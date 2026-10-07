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


class Num2WordsHUTest(TestCase):
    """Comprehensive test cases for Hungarian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="hu"), "nulla")
        self.assertEqual(num2words(1, lang="hu"), "egy")
        self.assertEqual(num2words(2, lang="hu"), "kettő")
        self.assertEqual(num2words(3, lang="hu"), "három")
        self.assertEqual(num2words(4, lang="hu"), "négy")
        self.assertEqual(num2words(5, lang="hu"), "öt")
        self.assertEqual(num2words(6, lang="hu"), "hat")
        self.assertEqual(num2words(7, lang="hu"), "hét")
        self.assertEqual(num2words(8, lang="hu"), "nyolc")
        self.assertEqual(num2words(9, lang="hu"), "kilenc")
        self.assertEqual(num2words(10, lang="hu"), "tíz")
        self.assertEqual(num2words(11, lang="hu"), "tizenegy")
        self.assertEqual(num2words(12, lang="hu"), "tizenkettő")
        self.assertEqual(num2words(13, lang="hu"), "tizenhárom")
        self.assertEqual(num2words(14, lang="hu"), "tizennégy")
        self.assertEqual(num2words(15, lang="hu"), "tizenöt")
        self.assertEqual(num2words(16, lang="hu"), "tizenhat")
        self.assertEqual(num2words(17, lang="hu"), "tizenhét")
        self.assertEqual(num2words(18, lang="hu"), "tizennyolc")
        self.assertEqual(num2words(19, lang="hu"), "tizenkilenc")
        self.assertEqual(num2words(20, lang="hu"), "húsz")
        self.assertEqual(num2words(21, lang="hu"), "huszonegy")
        self.assertEqual(num2words(22, lang="hu"), "huszonkettő")
        self.assertEqual(num2words(23, lang="hu"), "huszonhárom")
        self.assertEqual(num2words(24, lang="hu"), "huszonnégy")
        self.assertEqual(num2words(25, lang="hu"), "huszonöt")
        self.assertEqual(num2words(26, lang="hu"), "huszonhat")
        self.assertEqual(num2words(27, lang="hu"), "huszonhét")
        self.assertEqual(num2words(28, lang="hu"), "huszonnyolc")
        self.assertEqual(num2words(29, lang="hu"), "huszonkilenc")
        self.assertEqual(num2words(30, lang="hu"), "harminc")
        self.assertEqual(num2words(31, lang="hu"), "harmincegy")
        self.assertEqual(num2words(35, lang="hu"), "harmincöt")
        self.assertEqual(num2words(40, lang="hu"), "negyven")
        self.assertEqual(num2words(45, lang="hu"), "negyvenöt")
        self.assertEqual(num2words(50, lang="hu"), "ötven")
        self.assertEqual(num2words(55, lang="hu"), "ötvenöt")
        self.assertEqual(num2words(60, lang="hu"), "hatvan")
        self.assertEqual(num2words(65, lang="hu"), "hatvanöt")
        self.assertEqual(num2words(70, lang="hu"), "hetven")
        self.assertEqual(num2words(75, lang="hu"), "hetvenöt")
        self.assertEqual(num2words(80, lang="hu"), "nyolcvan")
        self.assertEqual(num2words(85, lang="hu"), "nyolcvanöt")
        self.assertEqual(num2words(90, lang="hu"), "kilencven")
        self.assertEqual(num2words(95, lang="hu"), "kilencvenöt")
        self.assertEqual(num2words(99, lang="hu"), "kilencvenkilenc")
        self.assertEqual(num2words(100, lang="hu"), "száz")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="hu"), "százegy")
        self.assertEqual(num2words(110, lang="hu"), "száztíz")
        self.assertEqual(num2words(111, lang="hu"), "száztizenegy")
        self.assertEqual(num2words(120, lang="hu"), "százhúsz")
        self.assertEqual(num2words(125, lang="hu"), "százhuszonöt")
        self.assertEqual(num2words(150, lang="hu"), "százötven")
        self.assertEqual(num2words(175, lang="hu"), "százhetvenöt")
        self.assertEqual(num2words(199, lang="hu"), "százkilencvenkilenc")
        self.assertEqual(num2words(200, lang="hu"), "kétszáz")
        self.assertEqual(num2words(201, lang="hu"), "kétszázegy")
        self.assertEqual(num2words(210, lang="hu"), "kétszáztíz")
        self.assertEqual(num2words(220, lang="hu"), "kétszázhúsz")
        self.assertEqual(num2words(250, lang="hu"), "kétszázötven")
        self.assertEqual(num2words(299, lang="hu"), "kétszázkilencvenkilenc")
        self.assertEqual(num2words(300, lang="hu"), "háromszáz")
        self.assertEqual(num2words(333, lang="hu"), "háromszázharminchárom")
        self.assertEqual(num2words(400, lang="hu"), "négyszáz")
        self.assertEqual(num2words(444, lang="hu"), "négyszáznegyvennégy")
        self.assertEqual(num2words(500, lang="hu"), "ötszáz")
        self.assertEqual(num2words(555, lang="hu"), "ötszázötvenöt")
        self.assertEqual(num2words(600, lang="hu"), "hatszáz")
        self.assertEqual(num2words(666, lang="hu"), "hatszázhatvanhat")
        self.assertEqual(num2words(700, lang="hu"), "hétszáz")
        self.assertEqual(num2words(777, lang="hu"), "hétszázhetvenhét")
        self.assertEqual(num2words(800, lang="hu"), "nyolcszáz")
        self.assertEqual(num2words(888, lang="hu"), "nyolcszáznyolcvannyolc")
        self.assertEqual(num2words(900, lang="hu"), "kilencszáz")
        self.assertEqual(num2words(999, lang="hu"), "kilencszázkilencvenkilenc")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="hu"), "ezer")
        self.assertEqual(num2words(1001, lang="hu"), "ezeregy")
        self.assertEqual(num2words(1010, lang="hu"), "ezertíz")
        self.assertEqual(num2words(1100, lang="hu"), "ezerszáz")
        self.assertEqual(num2words(1111, lang="hu"), "ezerszáztizenegy")
        self.assertEqual(num2words(1234, lang="hu"), "ezerkétszázharmincnégy")
        self.assertEqual(num2words(1500, lang="hu"), "ezerötszáz")
        self.assertEqual(num2words(1999, lang="hu"), "ezerkilencszázkilencvenkilenc")
        self.assertEqual(num2words(2000, lang="hu"), "kétezer")
        self.assertEqual(num2words(2001, lang="hu"), "kétezer-egy")
        self.assertEqual(num2words(2020, lang="hu"), "kétezer-húsz")
        self.assertEqual(num2words(2222, lang="hu"), "kétezer-kétszázhuszonkettő")
        self.assertEqual(num2words(3000, lang="hu"), "háromezer")
        self.assertEqual(num2words(3333, lang="hu"), "háromezer-háromszázharminchárom")
        self.assertEqual(num2words(4000, lang="hu"), "négyezer")
        self.assertEqual(num2words(4444, lang="hu"), "négyezer-négyszáznegyvennégy")
        self.assertEqual(num2words(5000, lang="hu"), "ötezer")
        self.assertEqual(num2words(5555, lang="hu"), "ötezer-ötszázötvenöt")
        self.assertEqual(num2words(6000, lang="hu"), "hatezer")
        self.assertEqual(num2words(6666, lang="hu"), "hatezer-hatszázhatvanhat")
        self.assertEqual(num2words(7000, lang="hu"), "hétezer")
        self.assertEqual(num2words(7777, lang="hu"), "hétezer-hétszázhetvenhét")
        self.assertEqual(num2words(8000, lang="hu"), "nyolcezer")
        self.assertEqual(num2words(8888, lang="hu"), "nyolcezer-nyolcszáznyolcvannyolc")
        self.assertEqual(num2words(9000, lang="hu"), "kilencezer")
        self.assertEqual(
            num2words(9999, lang="hu"), "kilencezer-kilencszázkilencvenkilenc"
        )
        self.assertEqual(num2words(10000, lang="hu"), "tízezer")
        self.assertEqual(num2words(10001, lang="hu"), "tízezer-egy")
        self.assertEqual(num2words(11111, lang="hu"), "tizenegyezer-száztizenegy")
        self.assertEqual(
            num2words(12345, lang="hu"), "tizenkettőezer-háromszáznegyvenöt"
        )
        self.assertEqual(num2words(20000, lang="hu"), "húszezer")
        self.assertEqual(num2words(50000, lang="hu"), "ötvenezer")
        self.assertEqual(
            num2words(99999, lang="hu"), "kilencvenkilencezer-kilencszázkilencvenkilenc"
        )
        self.assertEqual(num2words(100000, lang="hu"), "százezer")
        self.assertEqual(
            num2words(123456, lang="hu"), "százhuszonháromezer-négyszázötvenhat"
        )
        self.assertEqual(num2words(200000, lang="hu"), "kétszázezer")
        self.assertEqual(num2words(500000, lang="hu"), "ötszázezer")
        self.assertEqual(
            num2words(654321, lang="hu"), "hatszázötvennégyezer-háromszázhuszonegy"
        )
        self.assertEqual(
            num2words(999999, lang="hu"),
            "kilencszázkilencvenkilencezer-kilencszázkilencvenkilenc",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="hu"), "egymillió")
        self.assertEqual(num2words(1000001, lang="hu"), "egymillió-egy")
        self.assertEqual(
            num2words(1111111, lang="hu"), "egymillió-száztizenegyezer-száztizenegy"
        )
        self.assertEqual(
            num2words(1234567, lang="hu"),
            "egymillió-kétszázharmincnégyezer-ötszázhatvanhét",
        )
        self.assertEqual(num2words(2000000, lang="hu"), "kétmillió")
        self.assertEqual(num2words(5000000, lang="hu"), "ötmillió")
        self.assertEqual(
            num2words(9999999, lang="hu"),
            "kilencmillió-kilencszázkilencvenkilencezer-kilencszázkilencvenkilenc",
        )
        self.assertEqual(num2words(10000000, lang="hu"), "tízmillió")
        self.assertEqual(
            num2words(12345678, lang="hu"),
            "tizenkettőmillió-háromszáznegyvenötezer-hatszázhetvennyolc",
        )
        self.assertEqual(
            num2words(99999999, lang="hu"),
            "kilencvenkilencmillió-kilencszázkilencvenkilencezer-kilencszázkilencvenkilenc",
        )
        self.assertEqual(num2words(100000000, lang="hu"), "százmillió")
        self.assertEqual(
            num2words(123456789, lang="hu"),
            "százhuszonhárommillió-négyszázötvenhatezer-hétszáznyolcvankilenc",
        )
        self.assertEqual(
            num2words(999999999, lang="hu"),
            "kilencszázkilencvenkilencmillió-kilencszázkilencvenkilencezer-kilencszázkilencvenkilenc",
        )
        self.assertEqual(num2words(1000000000, lang="hu"), "egymilliárd")
        self.assertEqual(
            num2words(1234567890, lang="hu"),
            "egymilliárd-kétszázharmincnégymillió-ötszázhatvanhétezer-nyolcszázkilencven",
        )
        self.assertEqual(
            num2words(9999999999, lang="hu"),
            "kilencmilliárd-kilencszázkilencvenkilencmillió-kilencszázkilencvenkilencezer-kilencszázkilencvenkilenc",
        )
        self.assertEqual(num2words(10000000000, lang="hu"), "tízmilliárd")
        self.assertEqual(
            num2words(99999999999, lang="hu"),
            "kilencvenkilencmilliárd-kilencszázkilencvenkilencmillió-kilencszázkilencvenkilencezer-kilencszázkilencvenkilenc",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="hu"), "mínusz egy")
        self.assertEqual(num2words(-2, lang="hu"), "mínusz kettő")
        self.assertEqual(num2words(-5, lang="hu"), "mínusz öt")
        self.assertEqual(num2words(-10, lang="hu"), "mínusz tíz")
        self.assertEqual(num2words(-11, lang="hu"), "mínusz tizenegy")
        self.assertEqual(num2words(-20, lang="hu"), "mínusz húsz")
        self.assertEqual(num2words(-50, lang="hu"), "mínusz ötven")
        self.assertEqual(num2words(-99, lang="hu"), "mínusz kilencvenkilenc")
        self.assertEqual(num2words(-100, lang="hu"), "mínusz száz")
        self.assertEqual(num2words(-101, lang="hu"), "mínusz százegy")
        self.assertEqual(num2words(-200, lang="hu"), "mínusz kétszáz")
        self.assertEqual(num2words(-999, lang="hu"), "mínusz kilencszázkilencvenkilenc")
        self.assertEqual(num2words(-1000, lang="hu"), "mínusz ezer")
        self.assertEqual(num2words(-1001, lang="hu"), "mínusz ezeregy")
        self.assertEqual(num2words(-10000, lang="hu"), "mínusz tízezer")
        self.assertEqual(num2words(-100000, lang="hu"), "mínusz százezer")
        self.assertEqual(num2words(-1000000, lang="hu"), "mínusz egymillió")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="hu"), "nulla egész egy tized")
        self.assertEqual(num2words(0.5, lang="hu"), "nulla egész öt tized")
        self.assertEqual(num2words(0.9, lang="hu"), "nulla egész kilenc tized")
        self.assertEqual(num2words(1.1, lang="hu"), "egy egész egy tized")
        self.assertEqual(num2words(1.5, lang="hu"), "egy egész öt tized")
        self.assertEqual(num2words(2.5, lang="hu"), "kettő egész öt tized")
        self.assertEqual(num2words(3.14, lang="hu"), "három egész tizennégy század")
        self.assertEqual(num2words(10.5, lang="hu"), "tíz egész öt tized")
        self.assertEqual(num2words(11.11, lang="hu"), "tizenegy egész tizenegy század")
        self.assertEqual(num2words(20.2, lang="hu"), "húsz egész kettő tized")
        self.assertEqual(
            num2words(99.99, lang="hu"), "kilencvenkilenc egész kilencvenkilenc század"
        )
        self.assertEqual(num2words(100.01, lang="hu"), "száz egész egy század")
        self.assertEqual(num2words(100.5, lang="hu"), "száz egész öt tized")
        self.assertEqual(
            num2words(123.45, lang="hu"), "százhuszonhárom egész negyvenöt század"
        )
        self.assertEqual(num2words(1000.5, lang="hu"), "ezer egész öt tized")
        self.assertEqual(
            num2words(1234.56, lang="hu"),
            "ezerkétszázharmincnégy egész ötvenhat század",
        )
        self.assertEqual(num2words(10000.01, lang="hu"), "tízezer egész egy század")
        self.assertEqual(num2words(-0.5, lang="hu"), "mínusz nulla egész öt tized")
        self.assertEqual(num2words(-1.5, lang="hu"), "mínusz egy egész öt tized")
        self.assertEqual(num2words(-10.5, lang="hu"), "mínusz tíz egész öt tized")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="hu", ordinal=True), "első")
        self.assertEqual(num2words(2, lang="hu", ordinal=True), "második")
        self.assertEqual(num2words(3, lang="hu", ordinal=True), "harmadik")
        self.assertEqual(num2words(4, lang="hu", ordinal=True), "negyedik")
        self.assertEqual(num2words(5, lang="hu", ordinal=True), "ötödik")
        self.assertEqual(num2words(6, lang="hu", ordinal=True), "hatodik")
        self.assertEqual(num2words(7, lang="hu", ordinal=True), "hetedik")
        self.assertEqual(num2words(8, lang="hu", ordinal=True), "nyolcadik")
        self.assertEqual(num2words(9, lang="hu", ordinal=True), "kilencedik")
        self.assertEqual(num2words(10, lang="hu", ordinal=True), "tizedik")
        self.assertEqual(num2words(11, lang="hu", ordinal=True), "tizenegyedik")
        self.assertEqual(num2words(12, lang="hu", ordinal=True), "tizenkettedik")
        self.assertEqual(num2words(13, lang="hu", ordinal=True), "tizenharmadik")
        self.assertEqual(num2words(14, lang="hu", ordinal=True), "tizennegyedik")
        self.assertEqual(num2words(15, lang="hu", ordinal=True), "tizenötödik")
        self.assertEqual(num2words(16, lang="hu", ordinal=True), "tizenhatodik")
        self.assertEqual(num2words(17, lang="hu", ordinal=True), "tizenhetedik")
        self.assertEqual(num2words(18, lang="hu", ordinal=True), "tizennyolcadik")
        self.assertEqual(num2words(19, lang="hu", ordinal=True), "tizenkilencedik")
        self.assertEqual(num2words(20, lang="hu", ordinal=True), "huszadik")
        self.assertEqual(num2words(21, lang="hu", ordinal=True), "huszonegyedik")
        self.assertEqual(num2words(22, lang="hu", ordinal=True), "huszonkettedik")
        self.assertEqual(num2words(25, lang="hu", ordinal=True), "huszonötödik")
        self.assertEqual(num2words(30, lang="hu", ordinal=True), "harmincadik")
        self.assertEqual(num2words(40, lang="hu", ordinal=True), "negyvenedik")
        self.assertEqual(num2words(50, lang="hu", ordinal=True), "ötvenedik")
        self.assertEqual(num2words(60, lang="hu", ordinal=True), "hatvanadik")
        self.assertEqual(num2words(70, lang="hu", ordinal=True), "hetvenedik")
        self.assertEqual(num2words(80, lang="hu", ordinal=True), "nyolcvanadik")
        self.assertEqual(num2words(90, lang="hu", ordinal=True), "kilencvenedik")
        self.assertEqual(num2words(100, lang="hu", ordinal=True), "századik")
        self.assertEqual(num2words(101, lang="hu", ordinal=True), "százegyedik")
        self.assertEqual(num2words(200, lang="hu", ordinal=True), "kétszázadik")
        self.assertEqual(num2words(500, lang="hu", ordinal=True), "ötszázadik")
        self.assertEqual(num2words(1000, lang="hu", ordinal=True), "ezredik")
        self.assertEqual(num2words(1001, lang="hu", ordinal=True), "ezeregyedik")
        self.assertEqual(num2words(10000, lang="hu", ordinal=True), "tízezredik")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="AUD"), "nulla dollár"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="AUD"),
            "nulla dollár, egy cent",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="AUD"),
            "nulla dollár, ötven cent",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="AUD"), "egy dollár"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="AUD"),
            "egy dollár, ötven cent",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="BYN"), "nulla rubel"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="BYN"),
            "nulla rubel, egy kopejka",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="BYN"),
            "nulla rubel, ötven kopejka",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="BYN"), "egy rubel"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="BYN"),
            "egy rubel, ötven kopejka",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="CAD"), "nulla dollár"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="CAD"),
            "nulla dollár, egy cent",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="CAD"),
            "nulla dollár, ötven cent",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="CAD"), "egy dollár"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="CAD"),
            "egy dollár, ötven cent",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="EEK")
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="EUR"), "nulla euró"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="EUR"),
            "nulla euró, egy cent",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="EUR"),
            "nulla euró, ötven cent",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="EUR"), "egy euró"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="EUR"),
            "egy euró, ötven cent",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="GBP"), "nulla font"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="GBP"),
            "nulla font, egy penny",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="GBP"),
            "nulla font, ötven penny",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="GBP"), "egy font"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="GBP"),
            "egy font, ötven penny",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="LTL")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="LVL")
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="USD"), "nulla dollár"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="USD"),
            "nulla dollár, egy cent",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="USD"),
            "nulla dollár, ötven cent",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="USD"), "egy dollár"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="USD"),
            "egy dollár, ötven cent",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="RUB"), "nulla rubel"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="RUB"),
            "nulla rubel, egy kopejka",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="RUB"),
            "nulla rubel, ötven kopejka",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="RUB"), "egy rubel"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="RUB"),
            "egy rubel, ötven kopejka",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="SEK"), "nulla korona"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="SEK"),
            "nulla korona, egy öre",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="SEK"),
            "nulla korona, ötven öre",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="SEK"), "egy korona"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="SEK"),
            "egy korona, ötven öre",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="NOK"), "nulla korona"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="NOK"),
            "nulla korona, egy øre",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="NOK"),
            "nulla korona, ötven øre",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="NOK"), "egy korona"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="NOK"),
            "egy korona, ötven øre",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="PLN"), "nulla złoty"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="PLN"),
            "nulla złoty, egy grosz",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="PLN"),
            "nulla złoty, ötven grosz",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="PLN"), "egy złoty"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="PLN"),
            "egy złoty, ötven grosz",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="MXN"), "nulla peso"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="MXN"),
            "nulla peso, egy centavo",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="MXN"),
            "nulla peso, ötven centavo",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="MXN"), "egy peso"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="MXN"),
            "egy peso, ötven centavo",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="RON")
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="INR"), "nulla rúpia"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="INR"),
            "nulla rúpia, egy paisa",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="INR"),
            "nulla rúpia, ötven paisa",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="INR"), "egy rúpia"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="INR"),
            "egy rúpia, ötven paisa",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="HUF"), "nulla forint"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="HUF"),
            "nulla forint, egy fillér",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="HUF"),
            "nulla forint, ötven fillér",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="HUF"), "egy forint"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="HUF"),
            "egy forint, ötven fillér",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="ISK")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="UZS")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="SAR")
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="JPY"), "nulla jen"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="JPY"),
            "nulla jen, egy szen",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="JPY"),
            "nulla jen, ötven szen",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="JPY"), "egy jen"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="JPY"),
            "egy jen, ötven szen",
        )
        self.assertEqual(
            num2words(0, lang="hu", to="currency", currency="KRW"), "nulla von"
        )
        self.assertEqual(
            num2words(0.01, lang="hu", to="currency", currency="KRW"),
            "nulla von, egy jeon",
        )
        self.assertEqual(
            num2words(0.5, lang="hu", to="currency", currency="KRW"),
            "nulla von, ötven jeon",
        )
        self.assertEqual(
            num2words(1, lang="hu", to="currency", currency="KRW"), "egy von"
        )
        self.assertEqual(
            num2words(1.5, lang="hu", to="currency", currency="KRW"),
            "egy von, ötven jeon",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="hu", to="currency", currency="NGN")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="hu", to="year"), "ezer")
        self.assertEqual(num2words(1066, lang="hu", to="year"), "ezerhatvanhat")
        self.assertEqual(
            num2words(1492, lang="hu", to="year"), "ezernégyszázkilencvenkettő"
        )
        self.assertEqual(num2words(1776, lang="hu", to="year"), "ezerhétszázhetvenhat")
        self.assertEqual(num2words(1800, lang="hu", to="year"), "ezernyolcszáz")
        self.assertEqual(num2words(1900, lang="hu", to="year"), "ezerkilencszáz")
        self.assertEqual(
            num2words(1984, lang="hu", to="year"), "ezerkilencszáznyolcvannégy"
        )
        self.assertEqual(
            num2words(1999, lang="hu", to="year"), "ezerkilencszázkilencvenkilenc"
        )
        self.assertEqual(num2words(2000, lang="hu", to="year"), "kétezer")
        self.assertEqual(num2words(2001, lang="hu", to="year"), "kétezer-egy")
        self.assertEqual(num2words(2010, lang="hu", to="year"), "kétezer-tíz")
        self.assertEqual(num2words(2020, lang="hu", to="year"), "kétezer-húsz")
        self.assertEqual(num2words(2024, lang="hu", to="year"), "kétezer-huszonnégy")
        self.assertEqual(num2words(2100, lang="hu", to="year"), "kétezer-száz")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="hu"), "nulla")
        self.assertEqual(num2words("1", lang="hu"), "egy")
        self.assertEqual(num2words("10", lang="hu"), "tíz")
        self.assertEqual(num2words("100", lang="hu"), "száz")
        self.assertEqual(num2words("1000", lang="hu"), "ezer")
        self.assertEqual(num2words("10000", lang="hu"), "tízezer")
        self.assertEqual(num2words("100000", lang="hu"), "százezer")
        self.assertEqual(num2words("1000000", lang="hu"), "egymillió")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="hu"), "nulla")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="hu"), num2words("100", lang="hu"))
        self.assertEqual(num2words(1000, lang="hu"), num2words("1000", lang="hu"))

