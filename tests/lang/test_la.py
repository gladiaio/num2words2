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


class Num2WordsLATest(TestCase):
    """Comprehensive test cases for Latin language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="la"), "nullus")
        self.assertEqual(num2words(1, lang="la"), "ūnus")
        self.assertEqual(num2words(2, lang="la"), "duo")
        self.assertEqual(num2words(3, lang="la"), "trēs")
        self.assertEqual(num2words(4, lang="la"), "quattuor")
        self.assertEqual(num2words(5, lang="la"), "quīnque")
        self.assertEqual(num2words(6, lang="la"), "sex")
        self.assertEqual(num2words(7, lang="la"), "septem")
        self.assertEqual(num2words(8, lang="la"), "octō")
        self.assertEqual(num2words(9, lang="la"), "novem")
        self.assertEqual(num2words(10, lang="la"), "decem")
        self.assertEqual(num2words(11, lang="la"), "ūndecim")
        self.assertEqual(num2words(12, lang="la"), "duodecim")
        self.assertEqual(num2words(13, lang="la"), "tredecim")
        self.assertEqual(num2words(14, lang="la"), "quattuordecim")
        self.assertEqual(num2words(15, lang="la"), "quīndecim")
        self.assertEqual(num2words(16, lang="la"), "sēdecim")
        self.assertEqual(num2words(17, lang="la"), "septendecim")
        self.assertEqual(num2words(18, lang="la"), "duodēvīgintī")
        self.assertEqual(num2words(19, lang="la"), "ūndēvīgintī")
        self.assertEqual(num2words(20, lang="la"), "vīgintī")
        self.assertEqual(num2words(21, lang="la"), "vīgintī ūnus")
        self.assertEqual(num2words(22, lang="la"), "vīgintī duo")
        self.assertEqual(num2words(23, lang="la"), "vīgintī trēs")
        self.assertEqual(num2words(24, lang="la"), "vīgintī quattuor")
        self.assertEqual(num2words(25, lang="la"), "vīgintī quīnque")
        self.assertEqual(num2words(26, lang="la"), "vīgintī sex")
        self.assertEqual(num2words(27, lang="la"), "vīgintī septem")
        self.assertEqual(num2words(28, lang="la"), "vīgintī octō")
        self.assertEqual(num2words(29, lang="la"), "vīgintī novem")
        self.assertEqual(num2words(30, lang="la"), "trīgintā")
        self.assertEqual(num2words(31, lang="la"), "trīgintā ūnus")
        self.assertEqual(num2words(35, lang="la"), "trīgintā quīnque")
        self.assertEqual(num2words(40, lang="la"), "quadrāgintā")
        self.assertEqual(num2words(45, lang="la"), "quadrāgintā quīnque")
        self.assertEqual(num2words(50, lang="la"), "quīnquāgintā")
        self.assertEqual(num2words(55, lang="la"), "quīnquāgintā quīnque")
        self.assertEqual(num2words(60, lang="la"), "sexāgintā")
        self.assertEqual(num2words(65, lang="la"), "sexāgintā quīnque")
        self.assertEqual(num2words(70, lang="la"), "septuāgintā")
        self.assertEqual(num2words(75, lang="la"), "septuāgintā quīnque")
        self.assertEqual(num2words(80, lang="la"), "octōgintā")
        self.assertEqual(num2words(85, lang="la"), "octōgintā quīnque")
        self.assertEqual(num2words(90, lang="la"), "nōnāgintā")
        self.assertEqual(num2words(95, lang="la"), "nōnāgintā quīnque")
        self.assertEqual(num2words(99, lang="la"), "nōnāgintā novem")
        self.assertEqual(num2words(100, lang="la"), "centum")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="la"), "centum ūnus")
        self.assertEqual(num2words(110, lang="la"), "centum decem")
        self.assertEqual(num2words(111, lang="la"), "centum ūndecim")
        self.assertEqual(num2words(120, lang="la"), "centum vīgintī")
        self.assertEqual(num2words(125, lang="la"), "centum vīgintī quīnque")
        self.assertEqual(num2words(150, lang="la"), "centum quīnquāgintā")
        self.assertEqual(num2words(175, lang="la"), "centum septuāgintā quīnque")
        self.assertEqual(num2words(199, lang="la"), "centum nōnāgintā novem")
        self.assertEqual(num2words(200, lang="la"), "ducentī")
        self.assertEqual(num2words(201, lang="la"), "ducentī ūnus")
        self.assertEqual(num2words(210, lang="la"), "ducentī decem")
        self.assertEqual(num2words(220, lang="la"), "ducentī vīgintī")
        self.assertEqual(num2words(250, lang="la"), "ducentī quīnquāgintā")
        self.assertEqual(num2words(299, lang="la"), "ducentī nōnāgintā novem")
        self.assertEqual(num2words(300, lang="la"), "trecentī")
        self.assertEqual(num2words(333, lang="la"), "trecentī trīgintā trēs")
        self.assertEqual(num2words(400, lang="la"), "quadringentī")
        self.assertEqual(
            num2words(444, lang="la"), "quadringentī quadrāgintā quattuor"
        )
        self.assertEqual(num2words(500, lang="la"), "quīngentī")
        self.assertEqual(
            num2words(555, lang="la"), "quīngentī quīnquāgintā quīnque"
        )
        self.assertEqual(num2words(600, lang="la"), "sescentī")
        self.assertEqual(num2words(666, lang="la"), "sescentī sexāgintā sex")
        self.assertEqual(num2words(700, lang="la"), "septingentī")
        self.assertEqual(num2words(777, lang="la"), "septingentī septuāgintā septem")
        self.assertEqual(num2words(800, lang="la"), "octingentī")
        self.assertEqual(num2words(888, lang="la"), "octingentī octōgintā octō")
        self.assertEqual(num2words(900, lang="la"), "nōngentī")
        self.assertEqual(num2words(999, lang="la"), "nōngentī nōnāgintā novem")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="la"), "mīlle")
        self.assertEqual(num2words(1001, lang="la"), "mīlle ūnus")
        self.assertEqual(num2words(1010, lang="la"), "mīlle decem")
        self.assertEqual(num2words(1100, lang="la"), "mīlle centum")
        self.assertEqual(
            num2words(1111, lang="la"), "mīlle centum ūndecim"
        )
        self.assertEqual(
            num2words(1234, lang="la"), "mīlle ducentī trīgintā quattuor"
        )
        self.assertEqual(num2words(1500, lang="la"), "mīlle quīngentī")
        self.assertEqual(
            num2words(1999, lang="la"), "mīlle nōngentī nōnāgintā novem"
        )
        self.assertEqual(num2words(2000, lang="la"), "duo mīlia")
        self.assertEqual(num2words(2001, lang="la"), "duo mīlia ūnus")
        self.assertEqual(num2words(2020, lang="la"), "duo mīlia vīgintī")
        self.assertEqual(num2words(2222, lang="la"), "duo mīlia ducentī vīgintī duo")
        self.assertEqual(num2words(3000, lang="la"), "tria mīlia")
        self.assertEqual(
            num2words(3333, lang="la"), "tria mīlia trecentī trīgintā trēs"
        )
        self.assertEqual(num2words(4000, lang="la"), "quattuor mīlia")
        self.assertEqual(
            num2words(4444, lang="la"),
            "quattuor mīlia quadringentī quadrāgintā quattuor",
        )
        self.assertEqual(num2words(5000, lang="la"), "quīnque mīlia")
        self.assertEqual(
            num2words(5555, lang="la"),
            "quīnque mīlia quīngentī quīnquāgintā quīnque",
        )
        self.assertEqual(num2words(6000, lang="la"), "sex mīlia")
        self.assertEqual(
            num2words(6666, lang="la"), "sex mīlia sescentī sexāgintā sex"
        )
        self.assertEqual(num2words(7000, lang="la"), "septem mīlia")
        self.assertEqual(
            num2words(7777, lang="la"), "septem mīlia septingentī septuāgintā septem"
        )
        self.assertEqual(num2words(8000, lang="la"), "octō mīlia")
        self.assertEqual(
            num2words(8888, lang="la"), "octō mīlia octingentī octōgintā octō"
        )
        self.assertEqual(num2words(9000, lang="la"), "novem mīlia")
        self.assertEqual(
            num2words(9999, lang="la"), "novem mīlia nōngentī nōnāgintā novem"
        )
        self.assertEqual(num2words(10000, lang="la"), "decem mīlia")
        self.assertEqual(num2words(10001, lang="la"), "decem mīlia ūnus")
        self.assertEqual(
            num2words(11111, lang="la"), "ūndecim mīlia centum ūndecim"
        )
        self.assertEqual(
            num2words(12345, lang="la"),
            "duodecim mīlia trecentī quadrāgintā quīnque",
        )
        self.assertEqual(num2words(20000, lang="la"), "vīgintī mīlia")
        self.assertEqual(num2words(50000, lang="la"), "quīnquāgintā mīlia")
        self.assertEqual(
            num2words(99999, lang="la"),
            "nōnāgintā novem mīlia nōngentī nōnāgintā novem",
        )
        self.assertEqual(num2words(100000, lang="la"), "centum mīlia")
        self.assertEqual(
            num2words(123456, lang="la"),
            "centum vīgintī tria mīlia quadringentī quīnquāgintā sex",
        )
        self.assertEqual(num2words(200000, lang="la"), "ducenta mīlia")
        self.assertEqual(num2words(500000, lang="la"), "quīngenta mīlia")
        self.assertEqual(
            num2words(654321, lang="la"),
            "sescenta quīnquāgintā quattuor mīlia trecentī vīgintī ūnus",
        )
        self.assertEqual(
            num2words(999999, lang="la"),
            "nōngenta nōnāgintā novem mīlia nōngentī nōnāgintā novem",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="la"), "ūnus milio")
        self.assertEqual(
            num2words(1000001, lang="la"), "ūnus milio ūnus"
        )
        self.assertEqual(
            num2words(1111111, lang="la"),
            "ūnus milio centum ūndecim mīlia centum ūndecim",
        )
        self.assertEqual(
            num2words(1234567, lang="la"),
            "ūnus milio ducenta trīgintā quattuor mīlia quīngentī sexāgintā septem",
        )
        self.assertEqual(num2words(2000000, lang="la"), "duo miliones")
        self.assertEqual(num2words(5000000, lang="la"), "quīnque miliones")
        self.assertEqual(
            num2words(9999999, lang="la"),
            "novem miliones nōngenta nōnāgintā novem mīlia nōngentī nōnāgintā novem",
        )
        self.assertEqual(num2words(10000000, lang="la"), "decem miliones")
        self.assertEqual(
            num2words(12345678, lang="la"),
            "duodecim miliones trecenta quadrāgintā quīnque mīlia sescentī septuāgintā octō",
        )
        self.assertEqual(
            num2words(99999999, lang="la"),
            "nōnāgintā novem miliones nōngenta nōnāgintā novem mīlia nōngentī nōnāgintā novem",
        )
        self.assertEqual(
            num2words(100000000, lang="la"), "centum miliones"
        )
        self.assertEqual(
            num2words(123456789, lang="la"),
            "centum vīgintī trēs miliones quadringenta quīnquāgintā sex mīlia septingentī octōgintā novem",
        )
        self.assertEqual(
            num2words(999999999, lang="la"),
            "nōngentī nōnāgintā novem miliones nōngenta nōnāgintā novem mīlia nōngentī nōnāgintā novem",
        )
        self.assertEqual(num2words(1000000000, lang="la"), "ūnus miliardus")
        self.assertEqual(num2words(1234567890, lang="la"), "ūnus miliardus ducentī trīgintā quattuor miliones quīngenta sexāgintā septem mīlia octingentī nōnāgintā")
        self.assertEqual(num2words(9999999999, lang="la"), "novem miliardi nōngentī nōnāgintā novem miliones nōngenta nōnāgintā novem mīlia nōngentī nōnāgintā novem")
        self.assertEqual(num2words(10000000000, lang="la"), "decem miliardi")
        self.assertEqual(num2words(99999999999, lang="la"), "nōnāgintā novem miliardi nōngentī nōnāgintā novem miliones nōngenta nōnāgintā novem mīlia nōngentī nōnāgintā novem")

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="la"), "minus ūnus")
        self.assertEqual(num2words(-2, lang="la"), "minus duo")
        self.assertEqual(num2words(-5, lang="la"), "minus quīnque")
        self.assertEqual(num2words(-10, lang="la"), "minus decem")
        self.assertEqual(num2words(-11, lang="la"), "minus ūndecim")
        self.assertEqual(num2words(-20, lang="la"), "minus vīgintī")
        self.assertEqual(num2words(-50, lang="la"), "minus quīnquāgintā")
        self.assertEqual(num2words(-99, lang="la"), "minus nōnāgintā novem")
        self.assertEqual(num2words(-100, lang="la"), "minus centum")
        self.assertEqual(num2words(-101, lang="la"), "minus centum ūnus")
        self.assertEqual(num2words(-200, lang="la"), "minus ducentī")
        self.assertEqual(
            num2words(-999, lang="la"), "minus nōngentī nōnāgintā novem"
        )
        self.assertEqual(num2words(-1000, lang="la"), "minus mīlle")
        self.assertEqual(num2words(-1001, lang="la"), "minus mīlle ūnus")
        self.assertEqual(num2words(-10000, lang="la"), "minus decem mīlia")
        self.assertEqual(num2words(-100000, lang="la"), "minus centum mīlia")
        self.assertEqual(
            num2words(-1000000, lang="la"), "minus ūnus milio"
        )

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="la"), "nullus virgula ūnus")
        self.assertEqual(num2words(0.5, lang="la"), "nullus virgula quīnque")
        self.assertEqual(num2words(0.9, lang="la"), "nullus virgula novem")
        self.assertEqual(num2words(1.1, lang="la"), "ūnus virgula ūnus")
        self.assertEqual(num2words(1.5, lang="la"), "ūnus virgula quīnque")
        self.assertEqual(num2words(2.5, lang="la"), "duo virgula quīnque")
        self.assertEqual(num2words(3.14, lang="la"), "trēs virgula ūnus quattuor")
        self.assertEqual(num2words(10.5, lang="la"), "decem virgula quīnque")
        self.assertEqual(num2words(11.11, lang="la"), "ūndecim virgula ūnus ūnus")
        self.assertEqual(num2words(20.2, lang="la"), "vīgintī virgula duo")
        self.assertEqual(
            num2words(99.99, lang="la"), "nōnāgintā novem virgula novem novem"
        )
        self.assertEqual(num2words(100.01, lang="la"), "centum virgula nullus ūnus")
        self.assertEqual(num2words(100.5, lang="la"), "centum virgula quīnque")
        self.assertEqual(
            num2words(123.45, lang="la"),
            "centum vīgintī trēs virgula quattuor quīnque",
        )
        self.assertEqual(num2words(1000.5, lang="la"), "mīlle virgula quīnque")
        self.assertEqual(
            num2words(1234.56, lang="la"),
            "mīlle ducentī trīgintā quattuor virgula quīnque sex",
        )
        self.assertEqual(num2words(10000.01, lang="la"), "decem mīlia virgula nullus ūnus")
        self.assertEqual(num2words(-0.5, lang="la"), "minus nullus virgula quīnque")
        self.assertEqual(num2words(-1.5, lang="la"), "minus ūnus virgula quīnque")
        self.assertEqual(num2words(-10.5, lang="la"), "minus decem virgula quīnque")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="la", ordinal=True), "prīmus")
        self.assertEqual(num2words(2, lang="la", ordinal=True), "secundus")
        self.assertEqual(num2words(3, lang="la", ordinal=True), "tertius")
        self.assertEqual(num2words(4, lang="la", ordinal=True), "quārtus")
        self.assertEqual(num2words(5, lang="la", ordinal=True), "quīntus")
        self.assertEqual(num2words(6, lang="la", ordinal=True), "sextus")
        self.assertEqual(num2words(7, lang="la", ordinal=True), "septimus")
        self.assertEqual(num2words(8, lang="la", ordinal=True), "octāvus")
        self.assertEqual(num2words(9, lang="la", ordinal=True), "nōnus")
        self.assertEqual(num2words(10, lang="la", ordinal=True), "decimus")
        self.assertEqual(num2words(11, lang="la", ordinal=True), "ūndecimus")
        self.assertEqual(num2words(12, lang="la", ordinal=True), "duodecimus")
        self.assertEqual(num2words(13, lang="la", ordinal=True), "tertius decimus")
        self.assertEqual(num2words(14, lang="la", ordinal=True), "quārtus decimus")
        self.assertEqual(num2words(15, lang="la", ordinal=True), "quīntus decimus")
        self.assertEqual(num2words(16, lang="la", ordinal=True), "sextus decimus")
        self.assertEqual(num2words(17, lang="la", ordinal=True), "septimus decimus")
        self.assertEqual(num2words(18, lang="la", ordinal=True), "duodēvīcēsimus")
        self.assertEqual(num2words(19, lang="la", ordinal=True), "ūndēvīcēsimus")
        self.assertEqual(num2words(20, lang="la", ordinal=True), "vīcēsimus")
        self.assertEqual(num2words(30, lang="la", ordinal=True), "trīcēsimus")
        self.assertEqual(num2words(40, lang="la", ordinal=True), "quadrāgēsimus")
        self.assertEqual(num2words(50, lang="la", ordinal=True), "quīnquāgēsimus")
        self.assertEqual(num2words(60, lang="la", ordinal=True), "sexāgēsimus")
        self.assertEqual(num2words(70, lang="la", ordinal=True), "septuāgēsimus")
        self.assertEqual(num2words(80, lang="la", ordinal=True), "octōgēsimus")
        self.assertEqual(num2words(90, lang="la", ordinal=True), "nōnāgēsimus")
        self.assertEqual(num2words(100, lang="la", ordinal=True), "centēsimus")
        self.assertEqual(num2words(200, lang="la", ordinal=True), "ducentēsimus")
        self.assertEqual(num2words(500, lang="la", ordinal=True), "quīngentēsimus")
        self.assertEqual(num2words(1000, lang="la", ordinal=True), "mīllēsimus")

    def test_ordinal_compound(self):
        """Compound ordinals combine ordinal parts (gladiaio/num2words2#181)."""
        for n in (21, 22, 25, 101, 1001, 10000):
            self.assertNotEqual(
                num2words(n, lang="la", ordinal=True), num2words(n, lang="la")
            )
        cases = {
            21: "vīcēsimus prīmus",
            22: "vīcēsimus secundus",
            25: "vīcēsimus quīntus",
            101: "centēsimus prīmus",
            118: "centēsimus duodēvīcēsimus",
            121: "centēsimus vīcēsimus prīmus",
            1001: "mīllēsimus prīmus",
            2000: "bis mīllēsimus",
            2021: "bis mīllēsimus vīcēsimus prīmus",
            10000: "deciēs mīllēsimus",
            100000: "centiēs mīllēsimus",
            1000000: "deciēs centiēs mīllēsimus",
        }
        for n, expected in cases.items():
            self.assertEqual(num2words(n, lang="la", ordinal=True), expected)
        # Every part agrees in gender and case.
        self.assertEqual(
            num2words(21, lang="la", to="ordinal", gender="f", case="gen"),
            "vīcēsimae prīmae",
        )
        self.assertEqual(
            num2words(2000, lang="la", to="ordinal", gender="n", case="abl",
                      macrons=False),
            "bis millesimo",
        )
        self.assertEqual(num2words(21, lang="la", to="ordinal_num"), "XXI")
        with self.assertRaises(OverflowError):
            num2words(10**6 + 1, lang="la", ordinal=True)

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="la", to="currency", currency="EUR"), "zero eurones"
        )
        self.assertEqual(
            num2words(0.01, lang="la", to="currency", currency="EUR"),
            "zero eurones unus centesima",
        )
        self.assertEqual(
            num2words(0.5, lang="la", to="currency", currency="EUR"),
            "zero eurones quinquaginta centesimae",
        )
        self.assertEqual(
            num2words(1, lang="la", to="currency", currency="EUR"), "unus euro"
        )
        self.assertEqual(
            num2words(1.5, lang="la", to="currency", currency="EUR"),
            "unus euro quinquaginta centesimae",
        )
        self.assertEqual(
            num2words(0, lang="la", to="currency", currency="USD"), "zero dollaria"
        )
        self.assertEqual(
            num2words(0.01, lang="la", to="currency", currency="USD"),
            "zero dollaria unus centesima",
        )
        self.assertEqual(
            num2words(0.5, lang="la", to="currency", currency="USD"),
            "zero dollaria quinquaginta centesimae",
        )
        self.assertEqual(
            num2words(1, lang="la", to="currency", currency="USD"), "unus dollarium"
        )
        self.assertEqual(
            num2words(1.5, lang="la", to="currency", currency="USD"),
            "unus dollarium quinquaginta centesimae",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="la", to="year"), "mīlle")
        self.assertEqual(
            num2words(1066, lang="la", to="year"), "mīlle sexāgintā sex"
        )
        self.assertEqual(
            num2words(1492, lang="la", to="year"),
            "mīlle quadringentī nōnāgintā duo",
        )
        self.assertEqual(
            num2words(1776, lang="la", to="year"),
            "mīlle septingentī septuāgintā sex",
        )
        self.assertEqual(
            num2words(1800, lang="la", to="year"), "mīlle octingentī"
        )
        self.assertEqual(
            num2words(1900, lang="la", to="year"), "mīlle nōngentī"
        )
        self.assertEqual(
            num2words(1984, lang="la", to="year"),
            "mīlle nōngentī octōgintā quattuor",
        )
        self.assertEqual(
            num2words(1999, lang="la", to="year"),
            "mīlle nōngentī nōnāgintā novem",
        )
        self.assertEqual(num2words(2000, lang="la", to="year"), "duo mīlia")
        self.assertEqual(num2words(2001, lang="la", to="year"), "duo mīlia ūnus")
        self.assertEqual(num2words(2010, lang="la", to="year"), "duo mīlia decem")
        self.assertEqual(num2words(2020, lang="la", to="year"), "duo mīlia vīgintī")
        self.assertEqual(
            num2words(2024, lang="la", to="year"), "duo mīlia vīgintī quattuor"
        )
        self.assertEqual(num2words(2100, lang="la", to="year"), "duo mīlia centum")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="la"), "nullus")
        self.assertEqual(num2words("1", lang="la"), "ūnus")
        self.assertEqual(num2words("10", lang="la"), "decem")
        self.assertEqual(num2words("100", lang="la"), "centum")
        self.assertEqual(num2words("1000", lang="la"), "mīlle")
        self.assertEqual(num2words("10000", lang="la"), "decem mīlia")
        self.assertEqual(num2words("100000", lang="la"), "centum mīlia")
        self.assertEqual(num2words("1000000", lang="la"), "ūnus milio")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="la"), "nullus")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="la"), num2words("100", lang="la"))
        self.assertEqual(num2words(1000, lang="la"), num2words("1000", lang="la"))

