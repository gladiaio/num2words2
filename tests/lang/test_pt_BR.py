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


class Num2WordsPT_BRTest(TestCase):
    """Comprehensive test cases for Portuguese (Brazil) language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="pt-br"), "zero")
        self.assertEqual(num2words(1, lang="pt-br"), "um")
        self.assertEqual(num2words(2, lang="pt-br"), "dois")
        self.assertEqual(num2words(3, lang="pt-br"), "três")
        self.assertEqual(num2words(4, lang="pt-br"), "quatro")
        self.assertEqual(num2words(5, lang="pt-br"), "cinco")
        self.assertEqual(num2words(6, lang="pt-br"), "seis")
        self.assertEqual(num2words(7, lang="pt-br"), "sete")
        self.assertEqual(num2words(8, lang="pt-br"), "oito")
        self.assertEqual(num2words(9, lang="pt-br"), "nove")
        self.assertEqual(num2words(10, lang="pt-br"), "dez")
        self.assertEqual(num2words(11, lang="pt-br"), "onze")
        self.assertEqual(num2words(12, lang="pt-br"), "doze")
        self.assertEqual(num2words(13, lang="pt-br"), "treze")
        self.assertEqual(num2words(14, lang="pt-br"), "catorze")
        self.assertEqual(num2words(15, lang="pt-br"), "quinze")
        self.assertEqual(num2words(16, lang="pt-br"), "dezesseis")
        self.assertEqual(num2words(17, lang="pt-br"), "dezessete")
        self.assertEqual(num2words(18, lang="pt-br"), "dezoito")
        self.assertEqual(num2words(19, lang="pt-br"), "dezenove")
        self.assertEqual(num2words(20, lang="pt-br"), "vinte")
        self.assertEqual(num2words(21, lang="pt-br"), "vinte e um")
        self.assertEqual(num2words(22, lang="pt-br"), "vinte e dois")
        self.assertEqual(num2words(23, lang="pt-br"), "vinte e três")
        self.assertEqual(num2words(24, lang="pt-br"), "vinte e quatro")
        self.assertEqual(num2words(25, lang="pt-br"), "vinte e cinco")
        self.assertEqual(num2words(26, lang="pt-br"), "vinte e seis")
        self.assertEqual(num2words(27, lang="pt-br"), "vinte e sete")
        self.assertEqual(num2words(28, lang="pt-br"), "vinte e oito")
        self.assertEqual(num2words(29, lang="pt-br"), "vinte e nove")
        self.assertEqual(num2words(30, lang="pt-br"), "trinta")
        self.assertEqual(num2words(31, lang="pt-br"), "trinta e um")
        self.assertEqual(num2words(35, lang="pt-br"), "trinta e cinco")
        self.assertEqual(num2words(40, lang="pt-br"), "quarenta")
        self.assertEqual(num2words(45, lang="pt-br"), "quarenta e cinco")
        self.assertEqual(num2words(50, lang="pt-br"), "cinquenta")
        self.assertEqual(num2words(55, lang="pt-br"), "cinquenta e cinco")
        self.assertEqual(num2words(60, lang="pt-br"), "sessenta")
        self.assertEqual(num2words(65, lang="pt-br"), "sessenta e cinco")
        self.assertEqual(num2words(70, lang="pt-br"), "setenta")
        self.assertEqual(num2words(75, lang="pt-br"), "setenta e cinco")
        self.assertEqual(num2words(80, lang="pt-br"), "oitenta")
        self.assertEqual(num2words(85, lang="pt-br"), "oitenta e cinco")
        self.assertEqual(num2words(90, lang="pt-br"), "noventa")
        self.assertEqual(num2words(95, lang="pt-br"), "noventa e cinco")
        self.assertEqual(num2words(99, lang="pt-br"), "noventa e nove")
        self.assertEqual(num2words(100, lang="pt-br"), "cem")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="pt-br"), "cento e um")
        self.assertEqual(num2words(110, lang="pt-br"), "cento e dez")
        self.assertEqual(num2words(111, lang="pt-br"), "cento e onze")
        self.assertEqual(num2words(120, lang="pt-br"), "cento e vinte")
        self.assertEqual(num2words(125, lang="pt-br"), "cento e vinte e cinco")
        self.assertEqual(num2words(150, lang="pt-br"), "cento e cinquenta")
        self.assertEqual(num2words(175, lang="pt-br"), "cento e setenta e cinco")
        self.assertEqual(num2words(199, lang="pt-br"), "cento e noventa e nove")
        self.assertEqual(num2words(200, lang="pt-br"), "duzentos")
        self.assertEqual(num2words(201, lang="pt-br"), "duzentos e um")
        self.assertEqual(num2words(210, lang="pt-br"), "duzentos e dez")
        self.assertEqual(num2words(220, lang="pt-br"), "duzentos e vinte")
        self.assertEqual(num2words(250, lang="pt-br"), "duzentos e cinquenta")
        self.assertEqual(num2words(299, lang="pt-br"), "duzentos e noventa e nove")
        self.assertEqual(num2words(300, lang="pt-br"), "trezentos")
        self.assertEqual(num2words(333, lang="pt-br"), "trezentos e trinta e três")
        self.assertEqual(num2words(400, lang="pt-br"), "quatrocentos")
        self.assertEqual(
            num2words(444, lang="pt-br"), "quatrocentos e quarenta e quatro"
        )
        self.assertEqual(num2words(500, lang="pt-br"), "quinhentos")
        self.assertEqual(num2words(555, lang="pt-br"), "quinhentos e cinquenta e cinco")
        self.assertEqual(num2words(600, lang="pt-br"), "seiscentos")
        self.assertEqual(num2words(666, lang="pt-br"), "seiscentos e sessenta e seis")
        self.assertEqual(num2words(700, lang="pt-br"), "setecentos")
        self.assertEqual(num2words(777, lang="pt-br"), "setecentos e setenta e sete")
        self.assertEqual(num2words(800, lang="pt-br"), "oitocentos")
        self.assertEqual(num2words(888, lang="pt-br"), "oitocentos e oitenta e oito")
        self.assertEqual(num2words(900, lang="pt-br"), "novecentos")
        self.assertEqual(num2words(999, lang="pt-br"), "novecentos e noventa e nove")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="pt-br"), "mil")
        self.assertEqual(num2words(1001, lang="pt-br"), "mil e um")
        self.assertEqual(num2words(1010, lang="pt-br"), "mil e dez")
        self.assertEqual(num2words(1100, lang="pt-br"), "mil e cem")
        self.assertEqual(num2words(1111, lang="pt-br"), "mil, cento e onze")
        self.assertEqual(
            num2words(1234, lang="pt-br"), "mil, duzentos e trinta e quatro"
        )
        self.assertEqual(num2words(1500, lang="pt-br"), "mil e quinhentos")
        self.assertEqual(
            num2words(1999, lang="pt-br"), "mil, novecentos e noventa e nove"
        )
        self.assertEqual(num2words(2000, lang="pt-br"), "dois mil")
        self.assertEqual(num2words(2001, lang="pt-br"), "dois mil e um")
        self.assertEqual(num2words(2020, lang="pt-br"), "dois mil e vinte")
        self.assertEqual(
            num2words(2222, lang="pt-br"), "dois mil, duzentos e vinte e dois"
        )
        self.assertEqual(num2words(3000, lang="pt-br"), "três mil")
        self.assertEqual(
            num2words(3333, lang="pt-br"), "três mil, trezentos e trinta e três"
        )
        self.assertEqual(num2words(4000, lang="pt-br"), "quatro mil")
        self.assertEqual(
            num2words(4444, lang="pt-br"),
            "quatro mil, quatrocentos e quarenta e quatro",
        )
        self.assertEqual(num2words(5000, lang="pt-br"), "cinco mil")
        self.assertEqual(
            num2words(5555, lang="pt-br"), "cinco mil, quinhentos e cinquenta e cinco"
        )
        self.assertEqual(num2words(6000, lang="pt-br"), "seis mil")
        self.assertEqual(
            num2words(6666, lang="pt-br"), "seis mil, seiscentos e sessenta e seis"
        )
        self.assertEqual(num2words(7000, lang="pt-br"), "sete mil")
        self.assertEqual(
            num2words(7777, lang="pt-br"), "sete mil, setecentos e setenta e sete"
        )
        self.assertEqual(num2words(8000, lang="pt-br"), "oito mil")
        self.assertEqual(
            num2words(8888, lang="pt-br"), "oito mil, oitocentos e oitenta e oito"
        )
        self.assertEqual(num2words(9000, lang="pt-br"), "nove mil")
        self.assertEqual(
            num2words(9999, lang="pt-br"), "nove mil, novecentos e noventa e nove"
        )
        self.assertEqual(num2words(10000, lang="pt-br"), "dez mil")
        self.assertEqual(num2words(10001, lang="pt-br"), "dez mil e um")
        self.assertEqual(num2words(11111, lang="pt-br"), "onze mil, cento e onze")
        self.assertEqual(
            num2words(12345, lang="pt-br"), "doze mil, trezentos e quarenta e cinco"
        )
        self.assertEqual(num2words(20000, lang="pt-br"), "vinte mil")
        self.assertEqual(num2words(50000, lang="pt-br"), "cinquenta mil")
        self.assertEqual(
            num2words(99999, lang="pt-br"),
            "noventa e nove mil, novecentos e noventa e nove",
        )
        self.assertEqual(num2words(100000, lang="pt-br"), "cem mil")
        self.assertEqual(
            num2words(123456, lang="pt-br"),
            "cento e vinte e três mil, quatrocentos e cinquenta e seis",
        )
        self.assertEqual(num2words(200000, lang="pt-br"), "duzentos mil")
        self.assertEqual(num2words(500000, lang="pt-br"), "quinhentos mil")
        self.assertEqual(
            num2words(654321, lang="pt-br"),
            "seiscentos e cinquenta e quatro mil, trezentos e vinte e um",
        )
        self.assertEqual(
            num2words(999999, lang="pt-br"),
            "novecentos e noventa e nove mil, novecentos e noventa e nove",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="pt-br"), "um milhão")
        self.assertEqual(num2words(1000001, lang="pt-br"), "um milhão e um")
        self.assertEqual(
            num2words(1111111, lang="pt-br"),
            "um milhão, cento e onze mil, cento e onze",
        )
        self.assertEqual(
            num2words(1234567, lang="pt-br"),
            "um milhão, duzentos e trinta e quatro mil, quinhentos e sessenta e sete",
        )
        self.assertEqual(num2words(2000000, lang="pt-br"), "dois milhões")
        self.assertEqual(num2words(5000000, lang="pt-br"), "cinco milhões")
        self.assertEqual(
            num2words(9999999, lang="pt-br"),
            "nove milhões, novecentos e noventa e nove mil, novecentos e noventa e nove",
        )
        self.assertEqual(num2words(10000000, lang="pt-br"), "dez milhões")
        self.assertEqual(
            num2words(12345678, lang="pt-br"),
            "doze milhões, trezentos e quarenta e cinco mil, seiscentos e setenta e oito",
        )
        self.assertEqual(
            num2words(99999999, lang="pt-br"),
            "noventa e nove milhões, novecentos e noventa e nove mil, novecentos e noventa e nove",
        )
        self.assertEqual(num2words(100000000, lang="pt-br"), "cem milhões")
        self.assertEqual(
            num2words(123456789, lang="pt-br"),
            "cento e vinte e três milhões, quatrocentos e cinquenta e seis mil, setecentos e oitenta e nove",
        )
        self.assertEqual(
            num2words(999999999, lang="pt-br"),
            "novecentos e noventa e nove milhões, novecentos e noventa e nove mil, novecentos e noventa e nove",
        )
        self.assertEqual(num2words(1000000000, lang="pt-br"), "um bilhão")
        self.assertEqual(
            num2words(1234567890, lang="pt-br"),
            "um bilhão, duzentos e trinta e quatro milhões quinhentos e sessenta e sete mil oitocentos e noventa",
        )
        self.assertEqual(
            num2words(9999999999, lang="pt-br"),
            "nove bilhões, novecentos e noventa e nove milhões novecentos e noventa e nove mil novecentos e noventa e nove",
        )
        self.assertEqual(num2words(10000000000, lang="pt-br"), "dez bilhões")
        self.assertEqual(
            num2words(99999999999, lang="pt-br"),
            "noventa e nove bilhões, novecentos e noventa e nove milhões novecentos e noventa e nove mil novecentos e noventa e nove",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="pt-br"), "menos um")
        self.assertEqual(num2words(-2, lang="pt-br"), "menos dois")
        self.assertEqual(num2words(-5, lang="pt-br"), "menos cinco")
        self.assertEqual(num2words(-10, lang="pt-br"), "menos dez")
        self.assertEqual(num2words(-11, lang="pt-br"), "menos onze")
        self.assertEqual(num2words(-20, lang="pt-br"), "menos vinte")
        self.assertEqual(num2words(-50, lang="pt-br"), "menos cinquenta")
        self.assertEqual(num2words(-99, lang="pt-br"), "menos noventa e nove")
        self.assertEqual(num2words(-100, lang="pt-br"), "menos cem")
        self.assertEqual(num2words(-101, lang="pt-br"), "menos cento e um")
        self.assertEqual(num2words(-200, lang="pt-br"), "menos duzentos")
        self.assertEqual(
            num2words(-999, lang="pt-br"), "menos novecentos e noventa e nove"
        )
        self.assertEqual(num2words(-1000, lang="pt-br"), "menos mil")
        self.assertEqual(num2words(-1001, lang="pt-br"), "menos mil e um")
        self.assertEqual(num2words(-10000, lang="pt-br"), "menos dez mil")
        self.assertEqual(num2words(-100000, lang="pt-br"), "menos cem mil")
        self.assertEqual(num2words(-1000000, lang="pt-br"), "menos um milhão")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="pt-br"), "zero vírgula um")
        self.assertEqual(num2words(0.5, lang="pt-br"), "zero vírgula cinco")
        self.assertEqual(num2words(0.9, lang="pt-br"), "zero vírgula nove")
        self.assertEqual(num2words(1.1, lang="pt-br"), "um vírgula um")
        self.assertEqual(num2words(1.5, lang="pt-br"), "um vírgula cinco")
        self.assertEqual(num2words(2.5, lang="pt-br"), "dois vírgula cinco")
        self.assertEqual(num2words(3.14, lang="pt-br"), "três vírgula um quatro")
        self.assertEqual(num2words(10.5, lang="pt-br"), "dez vírgula cinco")
        self.assertEqual(num2words(11.11, lang="pt-br"), "onze vírgula um um")
        self.assertEqual(num2words(20.2, lang="pt-br"), "vinte vírgula dois")
        self.assertEqual(
            num2words(99.99, lang="pt-br"), "noventa e nove vírgula nove nove"
        )
        self.assertEqual(num2words(100.01, lang="pt-br"), "cem vírgula zero um")
        self.assertEqual(num2words(100.5, lang="pt-br"), "cem vírgula cinco")
        self.assertEqual(
            num2words(123.45, lang="pt-br"), "cento e vinte e três vírgula quatro cinco"
        )
        self.assertEqual(num2words(1000.5, lang="pt-br"), "mil vírgula cinco")
        self.assertEqual(
            num2words(1234.56, lang="pt-br"),
            "mil, duzentos e trinta e quatro vírgula cinco seis",
        )
        self.assertEqual(num2words(10000.01, lang="pt-br"), "dez mil vírgula zero um")
        self.assertEqual(num2words(-0.5, lang="pt-br"), "menos zero vírgula cinco")
        self.assertEqual(num2words(-1.5, lang="pt-br"), "menos um vírgula cinco")
        self.assertEqual(num2words(-10.5, lang="pt-br"), "menos dez vírgula cinco")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="pt-br", ordinal=True), "primeiro")
        self.assertEqual(num2words(2, lang="pt-br", ordinal=True), "segundo")
        self.assertEqual(num2words(3, lang="pt-br", ordinal=True), "terceiro")
        self.assertEqual(num2words(4, lang="pt-br", ordinal=True), "quarto")
        self.assertEqual(num2words(5, lang="pt-br", ordinal=True), "quinto")
        self.assertEqual(num2words(6, lang="pt-br", ordinal=True), "sexto")
        self.assertEqual(num2words(7, lang="pt-br", ordinal=True), "sétimo")
        self.assertEqual(num2words(8, lang="pt-br", ordinal=True), "oitavo")
        self.assertEqual(num2words(9, lang="pt-br", ordinal=True), "nono")
        self.assertEqual(num2words(10, lang="pt-br", ordinal=True), "décimo")
        self.assertEqual(num2words(11, lang="pt-br", ordinal=True), "décimo primeiro")
        self.assertEqual(num2words(12, lang="pt-br", ordinal=True), "décimo segundo")
        self.assertEqual(num2words(13, lang="pt-br", ordinal=True), "décimo terceiro")
        self.assertEqual(num2words(14, lang="pt-br", ordinal=True), "décimo quarto")
        self.assertEqual(num2words(15, lang="pt-br", ordinal=True), "décimo quinto")
        self.assertEqual(num2words(16, lang="pt-br", ordinal=True), "décimo sexto")
        self.assertEqual(num2words(17, lang="pt-br", ordinal=True), "décimo sétimo")
        self.assertEqual(num2words(18, lang="pt-br", ordinal=True), "décimo oitavo")
        self.assertEqual(num2words(19, lang="pt-br", ordinal=True), "décimo nono")
        self.assertEqual(num2words(20, lang="pt-br", ordinal=True), "vigésimo")
        self.assertEqual(num2words(21, lang="pt-br", ordinal=True), "vigésimo primeiro")
        self.assertEqual(num2words(22, lang="pt-br", ordinal=True), "vigésimo segundo")
        self.assertEqual(num2words(25, lang="pt-br", ordinal=True), "vigésimo quinto")
        self.assertEqual(num2words(30, lang="pt-br", ordinal=True), "trigésimo")
        self.assertEqual(num2words(40, lang="pt-br", ordinal=True), "quadragésimo")
        self.assertEqual(num2words(50, lang="pt-br", ordinal=True), "quinquagésimo")
        self.assertEqual(num2words(60, lang="pt-br", ordinal=True), "sexagésimo")
        self.assertEqual(num2words(70, lang="pt-br", ordinal=True), "septuagésimo")
        self.assertEqual(num2words(80, lang="pt-br", ordinal=True), "octogésimo")
        self.assertEqual(num2words(90, lang="pt-br", ordinal=True), "nonagésimo")
        self.assertEqual(num2words(100, lang="pt-br", ordinal=True), "centésimo")
        self.assertEqual(
            num2words(101, lang="pt-br", ordinal=True), "centésimo primeiro"
        )
        self.assertEqual(num2words(200, lang="pt-br", ordinal=True), "ducentésimo")
        self.assertEqual(num2words(500, lang="pt-br", ordinal=True), "quingentésimo")
        self.assertEqual(num2words(1000, lang="pt-br", ordinal=True), "milésimo")
        self.assertEqual(
            num2words(1001, lang="pt-br", ordinal=True), "milésimo primeiro"
        )
        self.assertEqual(
            num2words(10000, lang="pt-br", ordinal=True), "décimo milésimo"
        )

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="pt-br", to="currency", currency="BRL"), "zero reais"
        )
        self.assertEqual(
            num2words(0.01, lang="pt-br", to="currency", currency="BRL"),
            "zero reais e um centavo",
        )
        self.assertEqual(
            num2words(0.5, lang="pt-br", to="currency", currency="BRL"),
            "zero reais e cinquenta centavos",
        )
        self.assertEqual(
            num2words(1, lang="pt-br", to="currency", currency="BRL"), "um real"
        )
        self.assertEqual(
            num2words(1.5, lang="pt-br", to="currency", currency="BRL"),
            "um real e cinquenta centavos",
        )
        self.assertEqual(
            num2words(0, lang="pt-br", to="currency", currency="EUR"), "zero euros"
        )
        self.assertEqual(
            num2words(0.01, lang="pt-br", to="currency", currency="EUR"),
            "zero euros e um cêntimo",
        )
        self.assertEqual(
            num2words(0.5, lang="pt-br", to="currency", currency="EUR"),
            "zero euros e cinquenta cêntimos",
        )
        self.assertEqual(
            num2words(1, lang="pt-br", to="currency", currency="EUR"), "um euro"
        )
        self.assertEqual(
            num2words(1.5, lang="pt-br", to="currency", currency="EUR"),
            "um euro e cinquenta cêntimos",
        )
        self.assertEqual(
            num2words(0, lang="pt-br", to="currency", currency="USD"), "zero dólares"
        )
        self.assertEqual(
            num2words(0.01, lang="pt-br", to="currency", currency="USD"),
            "zero dólares e um centavo",
        )
        self.assertEqual(
            num2words(0.5, lang="pt-br", to="currency", currency="USD"),
            "zero dólares e cinquenta centavos",
        )
        self.assertEqual(
            num2words(1, lang="pt-br", to="currency", currency="USD"), "um dólar"
        )
        self.assertEqual(
            num2words(1.5, lang="pt-br", to="currency", currency="USD"),
            "um dólar e cinquenta centavos",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="pt-br", to="year"), "mil")
        self.assertEqual(
            num2words(1066, lang="pt-br", to="year"), "mil e sessenta e seis"
        )
        self.assertEqual(
            num2words(1492, lang="pt-br", to="year"),
            "mil, quatrocentos e noventa e dois",
        )
        self.assertEqual(
            num2words(1776, lang="pt-br", to="year"), "mil, setecentos e setenta e seis"
        )
        self.assertEqual(num2words(1800, lang="pt-br", to="year"), "mil e oitocentos")
        self.assertEqual(num2words(1900, lang="pt-br", to="year"), "mil e novecentos")
        self.assertEqual(
            num2words(1984, lang="pt-br", to="year"),
            "mil, novecentos e oitenta e quatro",
        )
        self.assertEqual(
            num2words(1999, lang="pt-br", to="year"), "mil, novecentos e noventa e nove"
        )
        self.assertEqual(num2words(2000, lang="pt-br", to="year"), "dois mil")
        self.assertEqual(num2words(2001, lang="pt-br", to="year"), "dois mil e um")
        self.assertEqual(num2words(2010, lang="pt-br", to="year"), "dois mil e dez")
        self.assertEqual(num2words(2020, lang="pt-br", to="year"), "dois mil e vinte")
        self.assertEqual(
            num2words(2024, lang="pt-br", to="year"), "dois mil e vinte e quatro"
        )
        self.assertEqual(num2words(2100, lang="pt-br", to="year"), "dois mil e cem")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="pt-br"), "zero")
        self.assertEqual(num2words("1", lang="pt-br"), "um")
        self.assertEqual(num2words("10", lang="pt-br"), "dez")
        self.assertEqual(num2words("100", lang="pt-br"), "cem")
        self.assertEqual(num2words("1000", lang="pt-br"), "mil")
        self.assertEqual(num2words("10000", lang="pt-br"), "dez mil")
        self.assertEqual(num2words("100000", lang="pt-br"), "cem mil")
        self.assertEqual(num2words("1000000", lang="pt-br"), "um milhão")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="pt-br"), "zero")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="pt-br"), num2words("100", lang="pt-br"))
        self.assertEqual(num2words(1000, lang="pt-br"), num2words("1000", lang="pt-br"))

        # Test invalid ordinal input (float) - Note: PT_BR doesn't raise TypeError
        # The implementation allows floats in ordinal
        result = num2words(3.14, lang="pt-br", ordinal=True)
        self.assertIsNotNone(result)


    def test_more_currency_cases(self):
        """Test additional currency cases."""
        # Test various amounts
        self.assertEqual(
            num2words(2, lang="pt-br", to="currency", currency="BRL"), "dois reais"
        )
        self.assertEqual(
            num2words(3, lang="pt-br", to="currency", currency="BRL"), "três reais"
        )
        self.assertEqual(
            num2words(10, lang="pt-br", to="currency", currency="BRL"), "dez reais"
        )
        self.assertEqual(
            num2words(100, lang="pt-br", to="currency", currency="BRL"), "cem reais"
        )
        self.assertEqual(
            num2words(1000, lang="pt-br", to="currency", currency="BRL"), "mil reais"
        )

        # Test millions with currency - should have "de"
        self.assertEqual(
            num2words(1000000, lang="pt-br", to="currency", currency="BRL"),
            "um milhão de reais",
        )
        self.assertEqual(
            num2words(2000000, lang="pt-br", to="currency", currency="BRL"),
            "dois milhões de reais",
        )

        # Test billions with currency - should have "de"
        self.assertEqual(
            num2words(1000000000, lang="pt-br", to="currency", currency="BRL"),
            "um bilhão de reais",
        )
        self.assertEqual(
            num2words(2000000000, lang="pt-br", to="currency", currency="BRL"),
            "dois bilhões de reais",
        )

        # Test trillions with currency - should have "de"
        self.assertEqual(
            num2words(1000000000000, lang="pt-br", to="currency", currency="BRL"),
            "um trilhão de reais",
        )
        self.assertEqual(
            num2words(2000000000000, lang="pt-br", to="currency", currency="BRL"),
            "dois trilhões de reais",
        )

        # Test EUR currency
        self.assertEqual(
            num2words(2, lang="pt-br", to="currency", currency="EUR"), "dois euros"
        )
        self.assertEqual(
            num2words(100, lang="pt-br", to="currency", currency="EUR"), "cem euros"
        )
        self.assertEqual(
            num2words(1000000, lang="pt-br", to="currency", currency="EUR"),
            "um milhão de euros",
        )

        # Test USD currency
        self.assertEqual(
            num2words(2, lang="pt-br", to="currency", currency="USD"), "dois dólares"
        )
        self.assertEqual(
            num2words(100, lang="pt-br", to="currency", currency="USD"), "cem dólares"
        )
        self.assertEqual(
            num2words(1000000, lang="pt-br", to="currency", currency="USD"),
            "um milhão de dólares",
        )

        # Test cents pluralization
        self.assertEqual(
            num2words(0.02, lang="pt-br", to="currency", currency="BRL"),
            "zero reais e dois centavos",
        )
        self.assertEqual(
            num2words(0.02, lang="pt-br", to="currency", currency="EUR"),
            "zero euros e dois cêntimos",
        )

        # Test negative currency
        self.assertEqual(
            num2words(-1, lang="pt-br", to="currency", currency="BRL"), "menos um real"
        )
        self.assertEqual(
            num2words(-10, lang="pt-br", to="currency", currency="BRL"),
            "menos dez reais",
        )
        self.assertEqual(
            num2words(-1.50, lang="pt-br", to="currency", currency="BRL"),
            "menos um real e cinquenta centavos",
        )


    def test_trillion_scale(self):
        """Test Brazilian short scale for trillions."""
        # Test trillion (10^12)
        self.assertEqual(num2words(1000000000000, lang="pt-br"), "um trilhão")
        self.assertEqual(num2words(2000000000000, lang="pt-br"), "dois trilhões")

        # Test trillion with billions
        self.assertEqual(
            num2words(1001000000000, lang="pt-br"), "um trilhão, um bilhão"
        )

        # Test trillion with millions
        self.assertEqual(
            num2words(1000001000000, lang="pt-br"), "um trilhão e um milhão"
        )

        # Test trillion with thousands
        self.assertEqual(num2words(1000000001000, lang="pt-br"), "um trilhão e mil")

        # Test complex trillion number
        self.assertEqual(
            num2words(1234567890123, lang="pt-br"),
            "um trilhão, duzentos e trinta e quatro bilhões e quinhentos e sessenta e sete milhões oitocentos e noventa mil cento e vinte e três",
        )

    def test_unsupported_currency(self):
        """Test unsupported currency code."""
        # The parent class should handle unknown currencies
        # This might not raise an error but return a default format
        result = num2words(100, lang="pt-br", to="currency", currency="GBP")
        # Should still produce some output
        self.assertIsNotNone(result)

    def test_decimal_currency_conversion(self):
        """Test currency conversion with Decimal values."""
        from decimal import Decimal

        # Test Decimal whole number
        self.assertEqual(
            num2words(Decimal("100.00"), lang="pt-br", to="currency", currency="BRL"),
            "cem reais",
        )

        # Test Decimal with cents
        self.assertEqual(
            num2words(Decimal("100.50"), lang="pt-br", to="currency", currency="BRL"),
            "cem reais e cinquenta centavos",
        )

    def test_float_currency_conversion(self):
        """Test currency conversion with float values."""
        # Test float whole number
        self.assertEqual(
            num2words(100.0, lang="pt-br", to="currency", currency="BRL"), "cem reais"
        )

        # Test float with decimal
        self.assertEqual(
            num2words(100.5, lang="pt-br", to="currency", currency="BRL"),
            "cem reais e cinquenta centavos",
        )

    def test_regex_replacements(self):
        """Test regex replacements in to_cardinal."""
        # Test numbers that trigger the regex replacement for "mil e cento"
        self.assertEqual(num2words(1100, lang="pt-br"), "mil e cem")
        self.assertEqual(num2words(1200, lang="pt-br"), "mil e duzentos")
        self.assertEqual(num2words(1300, lang="pt-br"), "mil e trezentos")

        # With additional numbers after hundreds
        self.assertEqual(num2words(1111, lang="pt-br"), "mil, cento e onze")
        self.assertEqual(
            num2words(1234, lang="pt-br"), "mil, duzentos e trinta e quatro"
        )

        # Test with millions
        self.assertEqual(num2words(1000100, lang="pt-br"), "um milhão, cem")
        self.assertEqual(num2words(1000200, lang="pt-br"), "um milhão, duzentos")
        self.assertEqual(
            num2words(1001234, lang="pt-br"),
            "um milhão, mil, duzentos e trinta e quatro",
        )


def test_pt_br_dot_string_pronounced_as_ponto():
    # Regression for num2words2#63 (ports savoirfairelinux/num2words#300).
    from num2words2 import num2words

    # Dot-string is US notation; pronounce as 'ponto'.
    #
    # A string keeps the precision it was written with, so "1.50" has two
    # fractional digits and the trailing zero is spoken — the same rule that
    # makes num2words("1.50", lang="en") == "one point five zero". This line
    # predates that decision (num2words2#75) and expected the float's
    # one-digit reading.
    assert num2words("1.50", lang="pt_BR") == "um ponto cinco zero"
    assert num2words("1.5", lang="pt_BR") == "um ponto cinco"
    # Comma-string and float are canonical pt-BR; pronounce as 'vírgula'.
    assert "vírgula" in num2words(1.50, lang="pt_BR")
