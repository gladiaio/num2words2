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


class Num2WordsSLTest(TestCase):
    """Comprehensive test cases for Slovenian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sl"), "nič")
        self.assertEqual(num2words(1, lang="sl"), "ena")
        self.assertEqual(num2words(2, lang="sl"), "dve")
        self.assertEqual(num2words(3, lang="sl"), "tri")
        self.assertEqual(num2words(4, lang="sl"), "štiri")
        self.assertEqual(num2words(5, lang="sl"), "pet")
        self.assertEqual(num2words(6, lang="sl"), "šest")
        self.assertEqual(num2words(7, lang="sl"), "sedem")
        self.assertEqual(num2words(8, lang="sl"), "osem")
        self.assertEqual(num2words(9, lang="sl"), "devet")
        self.assertEqual(num2words(10, lang="sl"), "deset")
        self.assertEqual(num2words(11, lang="sl"), "enajst")
        self.assertEqual(num2words(12, lang="sl"), "dvanajst")
        self.assertEqual(num2words(13, lang="sl"), "trinajst")
        self.assertEqual(num2words(14, lang="sl"), "štirinajst")
        self.assertEqual(num2words(15, lang="sl"), "petnajst")
        self.assertEqual(num2words(16, lang="sl"), "šestnajst")
        self.assertEqual(num2words(17, lang="sl"), "sedemnajst")
        self.assertEqual(num2words(18, lang="sl"), "osemnajst")
        self.assertEqual(num2words(19, lang="sl"), "devetnajst")
        self.assertEqual(num2words(20, lang="sl"), "dvajset")
        self.assertEqual(num2words(21, lang="sl"), "enaindvajset")
        self.assertEqual(num2words(22, lang="sl"), "dvaindvajset")
        self.assertEqual(num2words(23, lang="sl"), "triindvajset")
        self.assertEqual(num2words(24, lang="sl"), "štiriindvajset")
        self.assertEqual(num2words(25, lang="sl"), "petindvajset")
        self.assertEqual(num2words(26, lang="sl"), "šestindvajset")
        self.assertEqual(num2words(27, lang="sl"), "sedemindvajset")
        self.assertEqual(num2words(28, lang="sl"), "osemindvajset")
        self.assertEqual(num2words(29, lang="sl"), "devetindvajset")
        self.assertEqual(num2words(30, lang="sl"), "trideset")
        self.assertEqual(num2words(31, lang="sl"), "enaintrideset")
        self.assertEqual(num2words(35, lang="sl"), "petintrideset")
        self.assertEqual(num2words(40, lang="sl"), "štirideset")
        self.assertEqual(num2words(45, lang="sl"), "petinštirideset")
        self.assertEqual(num2words(50, lang="sl"), "petdeset")
        self.assertEqual(num2words(55, lang="sl"), "petinpetdeset")
        self.assertEqual(num2words(60, lang="sl"), "šestdeset")
        self.assertEqual(num2words(65, lang="sl"), "petinšestdeset")
        self.assertEqual(num2words(70, lang="sl"), "sedemdeset")
        self.assertEqual(num2words(75, lang="sl"), "petinsedemdeset")
        self.assertEqual(num2words(80, lang="sl"), "osemdeset")
        self.assertEqual(num2words(85, lang="sl"), "petinosemdeset")
        self.assertEqual(num2words(90, lang="sl"), "devetdeset")
        self.assertEqual(num2words(95, lang="sl"), "petindevetdeset")
        self.assertEqual(num2words(99, lang="sl"), "devetindevetdeset")
        self.assertEqual(num2words(100, lang="sl"), "sto")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sl"), "sto ena")
        self.assertEqual(num2words(110, lang="sl"), "sto deset")
        self.assertEqual(num2words(111, lang="sl"), "sto enajst")
        self.assertEqual(num2words(120, lang="sl"), "sto dvajset")
        self.assertEqual(num2words(125, lang="sl"), "sto petindvajset")
        self.assertEqual(num2words(150, lang="sl"), "sto petdeset")
        self.assertEqual(num2words(175, lang="sl"), "sto petinsedemdeset")
        self.assertEqual(num2words(199, lang="sl"), "sto devetindevetdeset")
        self.assertEqual(num2words(200, lang="sl"), "dvesto")
        self.assertEqual(num2words(201, lang="sl"), "dvesto ena")
        self.assertEqual(num2words(210, lang="sl"), "dvesto deset")
        self.assertEqual(num2words(220, lang="sl"), "dvesto dvajset")
        self.assertEqual(num2words(250, lang="sl"), "dvesto petdeset")
        self.assertEqual(num2words(299, lang="sl"), "dvesto devetindevetdeset")
        self.assertEqual(num2words(300, lang="sl"), "tristo")
        self.assertEqual(num2words(333, lang="sl"), "tristo triintrideset")
        self.assertEqual(num2words(400, lang="sl"), "štiristo")
        self.assertEqual(num2words(444, lang="sl"), "štiristo štiriinštirideset")
        self.assertEqual(num2words(500, lang="sl"), "petsto")
        self.assertEqual(num2words(555, lang="sl"), "petsto petinpetdeset")
        self.assertEqual(num2words(600, lang="sl"), "šeststo")
        self.assertEqual(num2words(666, lang="sl"), "šeststo šestinšestdeset")
        self.assertEqual(num2words(700, lang="sl"), "sedemsto")
        self.assertEqual(num2words(777, lang="sl"), "sedemsto sedeminsedemdeset")
        self.assertEqual(num2words(800, lang="sl"), "osemsto")
        self.assertEqual(num2words(888, lang="sl"), "osemsto oseminosemdeset")
        self.assertEqual(num2words(900, lang="sl"), "devetsto")
        self.assertEqual(num2words(999, lang="sl"), "devetsto devetindevetdeset")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sl"), "tisoč")
        self.assertEqual(num2words(1001, lang="sl"), "tisoč ena")
        self.assertEqual(num2words(1010, lang="sl"), "tisoč deset")
        self.assertEqual(num2words(1100, lang="sl"), "tisoč sto")
        self.assertEqual(num2words(1111, lang="sl"), "tisoč sto enajst")
        self.assertEqual(num2words(1234, lang="sl"), "tisoč dvesto štiriintrideset")
        self.assertEqual(num2words(1500, lang="sl"), "tisoč petsto")
        self.assertEqual(num2words(1999, lang="sl"), "tisoč devetsto devetindevetdeset")
        self.assertEqual(num2words(2000, lang="sl"), "dva tisoč")
        self.assertEqual(num2words(2001, lang="sl"), "dva tisoč ena")
        self.assertEqual(num2words(2020, lang="sl"), "dva tisoč dvajset")
        self.assertEqual(num2words(2222, lang="sl"), "dva tisoč dvesto dvaindvajset")
        self.assertEqual(num2words(3000, lang="sl"), "tri tisoč")
        self.assertEqual(num2words(3333, lang="sl"), "tri tisoč tristo triintrideset")
        self.assertEqual(num2words(4000, lang="sl"), "štiri tisoč")
        self.assertEqual(
            num2words(4444, lang="sl"), "štiri tisoč štiristo štiriinštirideset"
        )
        self.assertEqual(num2words(5000, lang="sl"), "pet tisoč")
        self.assertEqual(num2words(5555, lang="sl"), "pet tisoč petsto petinpetdeset")
        self.assertEqual(num2words(6000, lang="sl"), "šest tisoč")
        self.assertEqual(
            num2words(6666, lang="sl"), "šest tisoč šeststo šestinšestdeset"
        )
        self.assertEqual(num2words(7000, lang="sl"), "sedem tisoč")
        self.assertEqual(
            num2words(7777, lang="sl"), "sedem tisoč sedemsto sedeminsedemdeset"
        )
        self.assertEqual(num2words(8000, lang="sl"), "osem tisoč")
        self.assertEqual(
            num2words(8888, lang="sl"), "osem tisoč osemsto oseminosemdeset"
        )
        self.assertEqual(num2words(9000, lang="sl"), "devet tisoč")
        self.assertEqual(
            num2words(9999, lang="sl"), "devet tisoč devetsto devetindevetdeset"
        )
        self.assertEqual(num2words(10000, lang="sl"), "deset tisoč")
        self.assertEqual(num2words(10001, lang="sl"), "deset tisoč ena")
        self.assertEqual(num2words(11111, lang="sl"), "enajst tisoč sto enajst")
        self.assertEqual(
            num2words(12345, lang="sl"), "dvanajst tisoč tristo petinštirideset"
        )
        self.assertEqual(num2words(20000, lang="sl"), "dvajset tisoč")
        self.assertEqual(num2words(50000, lang="sl"), "petdeset tisoč")
        self.assertEqual(
            num2words(99999, lang="sl"),
            "devetindevetdeset tisoč devetsto devetindevetdeset",
        )
        self.assertEqual(num2words(100000, lang="sl"), "sto tisoč")
        self.assertEqual(
            num2words(123456, lang="sl"),
            "sto triindvajset tisoč štiristo šestinpetdeset",
        )
        self.assertEqual(num2words(200000, lang="sl"), "dvesto tisoč")
        self.assertEqual(num2words(500000, lang="sl"), "petsto tisoč")
        self.assertEqual(
            num2words(654321, lang="sl"),
            "šeststo štiriinpetdeset tisoč tristo enaindvajset",
        )
        self.assertEqual(
            num2words(999999, lang="sl"),
            "devetsto devetindevetdeset tisoč devetsto devetindevetdeset",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sl"), "milijon")
        self.assertEqual(num2words(1000001, lang="sl"), "milijon ena")
        self.assertEqual(
            num2words(1111111, lang="sl"), "milijon sto enajst tisoč sto enajst"
        )
        self.assertEqual(
            num2words(1234567, lang="sl"),
            "milijon dvesto štiriintrideset tisoč petsto sedeminšestdeset",
        )
        self.assertEqual(num2words(2000000, lang="sl"), "dva milijona")
        self.assertEqual(num2words(5000000, lang="sl"), "pet milijon")
        self.assertEqual(
            num2words(9999999, lang="sl"),
            "devet milijon devetsto devetindevetdeset tisoč devetsto devetindevetdeset",
        )
        self.assertEqual(num2words(10000000, lang="sl"), "deset milijon")
        self.assertEqual(
            num2words(12345678, lang="sl"),
            "dvanajst milijon tristo petinštirideset tisoč šeststo oseminsedemdeset",
        )
        self.assertEqual(
            num2words(99999999, lang="sl"),
            "devetindevetdeset milijon devetsto devetindevetdeset tisoč devetsto devetindevetdeset",
        )
        self.assertEqual(num2words(100000000, lang="sl"), "sto milijon")
        self.assertEqual(
            num2words(123456789, lang="sl"),
            "sto triindvajset milijon štiristo šestinpetdeset tisoč sedemsto devetinosemdeset",
        )
        self.assertEqual(
            num2words(999999999, lang="sl"),
            "devetsto devetindevetdeset milijon devetsto devetindevetdeset tisoč devetsto devetindevetdeset",
        )
        self.assertEqual(num2words(1000000000, lang="sl"), "milijarda")
        self.assertEqual(
            num2words(1234567890, lang="sl"),
            "milijarda dvesto štiriintrideset milijon petsto sedeminšestdeset tisoč osemsto devetdeset",
        )
        self.assertEqual(
            num2words(9999999999, lang="sl"),
            "devet milijarda devetsto devetindevetdeset milijon devetsto devetindevetdeset tisoč devetsto devetindevetdeset",
        )
        self.assertEqual(num2words(10000000000, lang="sl"), "deset milijarda")
        self.assertEqual(
            num2words(99999999999, lang="sl"),
            "devetindevetdeset milijarda devetsto devetindevetdeset milijon devetsto devetindevetdeset tisoč devetsto devetindevetdeset",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sl"), "minus ena")
        self.assertEqual(num2words(-2, lang="sl"), "minus dve")
        self.assertEqual(num2words(-5, lang="sl"), "minus pet")
        self.assertEqual(num2words(-10, lang="sl"), "minus deset")
        self.assertEqual(num2words(-11, lang="sl"), "minus enajst")
        self.assertEqual(num2words(-20, lang="sl"), "minus dvajset")
        self.assertEqual(num2words(-50, lang="sl"), "minus petdeset")
        self.assertEqual(num2words(-99, lang="sl"), "minus devetindevetdeset")
        self.assertEqual(num2words(-100, lang="sl"), "minus sto")
        self.assertEqual(num2words(-101, lang="sl"), "minus sto ena")
        self.assertEqual(num2words(-200, lang="sl"), "minus dvesto")
        self.assertEqual(num2words(-999, lang="sl"), "minus devetsto devetindevetdeset")
        self.assertEqual(num2words(-1000, lang="sl"), "minus tisoč")
        self.assertEqual(num2words(-1001, lang="sl"), "minus tisoč ena")
        self.assertEqual(num2words(-10000, lang="sl"), "minus deset tisoč")
        self.assertEqual(num2words(-100000, lang="sl"), "minus sto tisoč")
        self.assertEqual(num2words(-1000000, lang="sl"), "minus milijon")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sl"), "nič vejica ena")
        self.assertEqual(num2words(0.5, lang="sl"), "nič vejica pet")
        self.assertEqual(num2words(0.9, lang="sl"), "nič vejica devet")
        self.assertEqual(num2words(1.1, lang="sl"), "ena vejica ena")
        self.assertEqual(num2words(1.5, lang="sl"), "ena vejica pet")
        self.assertEqual(num2words(2.5, lang="sl"), "dve vejica pet")
        self.assertEqual(num2words(3.14, lang="sl"), "tri vejica štirinajst")
        self.assertEqual(num2words(10.5, lang="sl"), "deset vejica pet")
        self.assertEqual(num2words(11.11, lang="sl"), "enajst vejica enajst")
        self.assertEqual(num2words(20.2, lang="sl"), "dvajset vejica dve")
        self.assertEqual(
            num2words(99.99, lang="sl"), "devetindevetdeset vejica devetindevetdeset"
        )
        self.assertEqual(num2words(100.01, lang="sl"), "sto vejica nič ena")
        self.assertEqual(num2words(100.5, lang="sl"), "sto vejica pet")
        self.assertEqual(
            num2words(123.45, lang="sl"), "sto triindvajset vejica petinštirideset"
        )
        self.assertEqual(num2words(1000.5, lang="sl"), "tisoč vejica pet")
        self.assertEqual(
            num2words(1234.56, lang="sl"),
            "tisoč dvesto štiriintrideset vejica šestinpetdeset",
        )
        self.assertEqual(num2words(10000.01, lang="sl"), "deset tisoč vejica nič ena")
        self.assertEqual(num2words(-0.5, lang="sl"), "minus nič vejica pet")
        self.assertEqual(num2words(-1.5, lang="sl"), "minus ena vejica pet")
        self.assertEqual(num2words(-10.5, lang="sl"), "minus deset vejica pet")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sl", ordinal=True), "prvi")
        self.assertEqual(num2words(2, lang="sl", ordinal=True), "drugi")
        self.assertEqual(num2words(3, lang="sl", ordinal=True), "tretji")
        self.assertEqual(num2words(4, lang="sl", ordinal=True), "četrti")
        self.assertEqual(num2words(5, lang="sl", ordinal=True), "peti")
        self.assertEqual(num2words(6, lang="sl", ordinal=True), "šesti")
        self.assertEqual(num2words(7, lang="sl", ordinal=True), "sedmi")
        self.assertEqual(num2words(8, lang="sl", ordinal=True), "osmi")
        self.assertEqual(num2words(9, lang="sl", ordinal=True), "deveti")
        self.assertEqual(num2words(10, lang="sl", ordinal=True), "deseti")
        self.assertEqual(num2words(11, lang="sl", ordinal=True), "enajsti")
        self.assertEqual(num2words(12, lang="sl", ordinal=True), "dvanajsti")
        self.assertEqual(num2words(13, lang="sl", ordinal=True), "trinajsti")
        self.assertEqual(num2words(14, lang="sl", ordinal=True), "štirinajsti")
        self.assertEqual(num2words(15, lang="sl", ordinal=True), "petnajsti")
        self.assertEqual(num2words(16, lang="sl", ordinal=True), "šestnajsti")
        self.assertEqual(num2words(17, lang="sl", ordinal=True), "sedemnajsti")
        self.assertEqual(num2words(18, lang="sl", ordinal=True), "osemnajsti")
        self.assertEqual(num2words(19, lang="sl", ordinal=True), "devetnajsti")
        self.assertEqual(num2words(20, lang="sl", ordinal=True), "dvajseti")
        self.assertEqual(num2words(21, lang="sl", ordinal=True), "enaindvajseti")
        self.assertEqual(num2words(22, lang="sl", ordinal=True), "dvaindvajseti")
        self.assertEqual(num2words(25, lang="sl", ordinal=True), "petindvajseti")
        self.assertEqual(num2words(30, lang="sl", ordinal=True), "trideseti")
        self.assertEqual(num2words(40, lang="sl", ordinal=True), "štirideseti")
        self.assertEqual(num2words(50, lang="sl", ordinal=True), "petdeseti")
        self.assertEqual(num2words(60, lang="sl", ordinal=True), "šestdeseti")
        self.assertEqual(num2words(70, lang="sl", ordinal=True), "sedemdeseti")
        self.assertEqual(num2words(80, lang="sl", ordinal=True), "osemdeseti")
        self.assertEqual(num2words(90, lang="sl", ordinal=True), "devetdeseti")
        self.assertEqual(num2words(100, lang="sl", ordinal=True), "stoti")
        self.assertEqual(num2words(101, lang="sl", ordinal=True), "stoprvi")
        self.assertEqual(num2words(200, lang="sl", ordinal=True), "dvestoti")
        self.assertEqual(num2words(500, lang="sl", ordinal=True), "petstoti")
        self.assertEqual(num2words(1000, lang="sl", ordinal=True), "tisoči")
        self.assertEqual(num2words(1001, lang="sl", ordinal=True), "tisočprvi")
        self.assertEqual(num2words(10000, lang="sl", ordinal=True), "desettisoči")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="sl", to="currency", currency="EUR"), "nič evrov"
        )
        self.assertEqual(
            num2words(0.01, lang="sl", to="currency", currency="EUR"),
            "nič evrov en cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sl", to="currency", currency="EUR"),
            "nič evrov petdeset centov",
        )
        self.assertEqual(
            num2words(1, lang="sl", to="currency", currency="EUR"), "en evro"
        )
        self.assertEqual(
            num2words(1.5, lang="sl", to="currency", currency="EUR"),
            "en evro petdeset centov",
        )
        self.assertEqual(
            num2words(0, lang="sl", to="currency", currency="USD"), "nič dolarjev"
        )
        self.assertEqual(
            num2words(0.01, lang="sl", to="currency", currency="USD"),
            "nič dolarjev en cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sl", to="currency", currency="USD"),
            "nič dolarjev petdeset centov",
        )
        self.assertEqual(
            num2words(1, lang="sl", to="currency", currency="USD"), "en dolar"
        )
        self.assertEqual(
            num2words(1.5, lang="sl", to="currency", currency="USD"),
            "en dolar petdeset centov",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sl", to="year"), "tisoč")
        self.assertEqual(num2words(1066, lang="sl", to="year"), "tisoč šestinšestdeset")
        self.assertEqual(
            num2words(1492, lang="sl", to="year"), "tisoč štiristo dvaindevetdeset"
        )
        self.assertEqual(
            num2words(1776, lang="sl", to="year"), "tisoč sedemsto šestinsedemdeset"
        )
        self.assertEqual(num2words(1800, lang="sl", to="year"), "tisoč osemsto")
        self.assertEqual(num2words(1900, lang="sl", to="year"), "tisoč devetsto")
        self.assertEqual(
            num2words(1984, lang="sl", to="year"), "tisoč devetsto štiriinosemdeset"
        )
        self.assertEqual(
            num2words(1999, lang="sl", to="year"), "tisoč devetsto devetindevetdeset"
        )
        self.assertEqual(num2words(2000, lang="sl", to="year"), "dva tisoč")
        self.assertEqual(num2words(2001, lang="sl", to="year"), "dva tisoč ena")
        self.assertEqual(num2words(2010, lang="sl", to="year"), "dva tisoč deset")
        self.assertEqual(num2words(2020, lang="sl", to="year"), "dva tisoč dvajset")
        self.assertEqual(
            num2words(2024, lang="sl", to="year"), "dva tisoč štiriindvajset"
        )
        self.assertEqual(num2words(2100, lang="sl", to="year"), "dva tisoč sto")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sl"), "nič")
        self.assertEqual(num2words("1", lang="sl"), "ena")
        self.assertEqual(num2words("10", lang="sl"), "deset")
        self.assertEqual(num2words("100", lang="sl"), "sto")
        self.assertEqual(num2words("1000", lang="sl"), "tisoč")
        self.assertEqual(num2words("10000", lang="sl"), "deset tisoč")
        self.assertEqual(num2words("100000", lang="sl"), "sto tisoč")
        self.assertEqual(num2words("1000000", lang="sl"), "milijon")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sl"), "nič")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sl"), num2words("100", lang="sl"))
        self.assertEqual(num2words(1000, lang="sl"), num2words("1000", lang="sl"))

        # Test invalid ordinal input (float)
        with self.assertRaises(TypeError):
            num2words(3.14, lang="sl", ordinal=True)

        # Test large numbers with ordinal flag
        self.assertEqual(num2words(10001, lang="sl", ordinal=True), "desettisočprvi")

    def test_more_ordinals(self):
        """Test additional ordinal numbers."""
        # Test compound ordinals
        self.assertEqual(num2words(21, lang="sl", ordinal=True), "enaindvajseti")
        self.assertEqual(num2words(22, lang="sl", ordinal=True), "dvaindvajseti")
        self.assertEqual(num2words(23, lang="sl", ordinal=True), "triindvajseti")
        self.assertEqual(num2words(24, lang="sl", ordinal=True), "štiriindvajseti")
        self.assertEqual(num2words(25, lang="sl", ordinal=True), "petindvajseti")
        self.assertEqual(num2words(31, lang="sl", ordinal=True), "enaintrideseti")
        self.assertEqual(num2words(99, lang="sl", ordinal=True), "devetindevetdeseti")
        self.assertEqual(num2words(101, lang="sl", ordinal=True), "stoprvi")
        self.assertEqual(num2words(110, lang="sl", ordinal=True), "stodeseti")
        self.assertEqual(num2words(111, lang="sl", ordinal=True), "stoenajsti")
        self.assertEqual(num2words(200, lang="sl", ordinal=True), "dvestoti")
        self.assertEqual(num2words(300, lang="sl", ordinal=True), "tristoti")
        self.assertEqual(num2words(400, lang="sl", ordinal=True), "štiristoti")
        self.assertEqual(num2words(500, lang="sl", ordinal=True), "petstoti")
        self.assertEqual(num2words(1000, lang="sl", ordinal=True), "tisoči")
        self.assertEqual(num2words(1001, lang="sl", ordinal=True), "tisočprvi")
        self.assertEqual(num2words(2000, lang="sl", ordinal=True), "dvatisoči")
        self.assertEqual(num2words(10000, lang="sl", ordinal=True), "desettisoči")
        self.assertEqual(num2words(100000, lang="sl", ordinal=True), "stotisoči")
        self.assertEqual(num2words(1000000, lang="sl", ordinal=True), "milijonti")

    def test_currency_with_fractional_cents(self):
        """Test currency with fractional cents."""
        # Test currency with fractional cents
        self.assertEqual(
            num2words(1.235, lang="sl", to="currency", currency="EUR"),
            "en evro triindvajset vejica pet centov",
        )
        self.assertEqual(
            num2words(10.999, lang="sl", to="currency", currency="EUR"),
            "deset evrov devetindevetdeset vejica devet centov",
        )

    def test_more_currency_cases(self):
        """Test additional currency cases."""
        # Test various amounts
        self.assertEqual(
            num2words(2, lang="sl", to="currency", currency="EUR"), "dva evra"
        )
        self.assertEqual(
            num2words(3, lang="sl", to="currency", currency="EUR"), "tri evre"
        )
        self.assertEqual(
            num2words(4, lang="sl", to="currency", currency="EUR"), "štiri evre"
        )
        self.assertEqual(
            num2words(5, lang="sl", to="currency", currency="EUR"), "pet evrov"
        )
        self.assertEqual(
            num2words(10, lang="sl", to="currency", currency="EUR"), "deset evrov"
        )
        self.assertEqual(
            num2words(100, lang="sl", to="currency", currency="EUR"), "sto evrov"
        )
        self.assertEqual(
            num2words(101, lang="sl", to="currency", currency="EUR"), "sto en evro"
        )
        self.assertEqual(
            num2words(102, lang="sl", to="currency", currency="EUR"), "sto dva evra"
        )
        self.assertEqual(
            num2words(103, lang="sl", to="currency", currency="EUR"), "sto tri evre"
        )
        self.assertEqual(
            num2words(104, lang="sl", to="currency", currency="EUR"), "sto štiri evre"
        )
        self.assertEqual(
            num2words(105, lang="sl", to="currency", currency="EUR"), "sto pet evrov"
        )

        # Test USD
        self.assertEqual(
            num2words(2, lang="sl", to="currency", currency="USD"), "dva dolarja"
        )
        self.assertEqual(
            num2words(3, lang="sl", to="currency", currency="USD"), "tri dolarje"
        )
        self.assertEqual(
            num2words(4, lang="sl", to="currency", currency="USD"), "štiri dolarje"
        )
        self.assertEqual(
            num2words(5, lang="sl", to="currency", currency="USD"), "pet dolarjev"
        )

        # Test cents
        self.assertEqual(
            num2words(0.02, lang="sl", to="currency", currency="EUR"),
            "nič evrov dva centa",
        )
        self.assertEqual(
            num2words(0.03, lang="sl", to="currency", currency="EUR"),
            "nič evrov tri cente",
        )
        self.assertEqual(
            num2words(0.04, lang="sl", to="currency", currency="EUR"),
            "nič evrov štiri cente",
        )
        self.assertEqual(
            num2words(0.05, lang="sl", to="currency", currency="EUR"),
            "nič evrov pet centov",
        )

        # Test negative amounts
        self.assertEqual(
            num2words(-1, lang="sl", to="currency", currency="EUR"), "minus en evro"
        )
        self.assertEqual(
            num2words(-10, lang="sl", to="currency", currency="EUR"),
            "minus deset evrov",
        )
        self.assertEqual(
            num2words(-1.5, lang="sl", to="currency", currency="EUR"),
            "minus en evro petdeset centov",
        )

    def test_to_cardinal_float_negative_zero(self):
        """Test negative zero handling in float conversion."""
        # Test negative zero scenario
        self.assertEqual(num2words(-0.5, lang="sl"), "minus nič vejica pet")
        self.assertEqual(num2words(-0.1, lang="sl"), "minus nič vejica ena")
        self.assertEqual(
            num2words(-0.99, lang="sl"), "minus nič vejica devetindevetdeset"
        )
