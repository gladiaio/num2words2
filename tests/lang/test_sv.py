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


class Num2WordsSVTest(TestCase):
    """Comprehensive test cases for Swedish language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sv"), "noll")
        self.assertEqual(num2words(1, lang="sv"), "ett")
        self.assertEqual(num2words(2, lang="sv"), "två")
        self.assertEqual(num2words(3, lang="sv"), "tre")
        self.assertEqual(num2words(4, lang="sv"), "fyra")
        self.assertEqual(num2words(5, lang="sv"), "fem")
        self.assertEqual(num2words(6, lang="sv"), "sex")
        self.assertEqual(num2words(7, lang="sv"), "sju")
        self.assertEqual(num2words(8, lang="sv"), "åtta")
        self.assertEqual(num2words(9, lang="sv"), "nio")
        self.assertEqual(num2words(10, lang="sv"), "tio")
        self.assertEqual(num2words(11, lang="sv"), "elva")
        self.assertEqual(num2words(12, lang="sv"), "tolv")
        self.assertEqual(num2words(13, lang="sv"), "tretton")
        self.assertEqual(num2words(14, lang="sv"), "fjorton")
        self.assertEqual(num2words(15, lang="sv"), "femton")
        self.assertEqual(num2words(16, lang="sv"), "sexton")
        self.assertEqual(num2words(17, lang="sv"), "sjutton")
        self.assertEqual(num2words(18, lang="sv"), "arton")
        self.assertEqual(num2words(19, lang="sv"), "nitton")
        self.assertEqual(num2words(20, lang="sv"), "tjugo")
        self.assertEqual(num2words(21, lang="sv"), "tjugoett")
        self.assertEqual(num2words(22, lang="sv"), "tjugotvå")
        self.assertEqual(num2words(23, lang="sv"), "tjugotre")
        self.assertEqual(num2words(24, lang="sv"), "tjugofyra")
        self.assertEqual(num2words(25, lang="sv"), "tjugofem")
        self.assertEqual(num2words(26, lang="sv"), "tjugosex")
        self.assertEqual(num2words(27, lang="sv"), "tjugosju")
        self.assertEqual(num2words(28, lang="sv"), "tjugoåtta")
        self.assertEqual(num2words(29, lang="sv"), "tjugonio")
        self.assertEqual(num2words(30, lang="sv"), "trettio")
        self.assertEqual(num2words(31, lang="sv"), "trettioett")
        self.assertEqual(num2words(35, lang="sv"), "trettiofem")
        self.assertEqual(num2words(40, lang="sv"), "fyrtio")
        self.assertEqual(num2words(45, lang="sv"), "fyrtiofem")
        self.assertEqual(num2words(50, lang="sv"), "femtio")
        self.assertEqual(num2words(55, lang="sv"), "femtiofem")
        self.assertEqual(num2words(60, lang="sv"), "sextio")
        self.assertEqual(num2words(65, lang="sv"), "sextiofem")
        self.assertEqual(num2words(70, lang="sv"), "sjuttio")
        self.assertEqual(num2words(75, lang="sv"), "sjuttiofem")
        self.assertEqual(num2words(80, lang="sv"), "åttio")
        self.assertEqual(num2words(85, lang="sv"), "åttiofem")
        self.assertEqual(num2words(90, lang="sv"), "nittio")
        self.assertEqual(num2words(95, lang="sv"), "nittiofem")
        self.assertEqual(num2words(99, lang="sv"), "nittionio")
        self.assertEqual(num2words(100, lang="sv"), "etthundra")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sv"), "etthundraett")
        self.assertEqual(num2words(110, lang="sv"), "etthundratio")
        self.assertEqual(num2words(111, lang="sv"), "etthundraelva")
        self.assertEqual(num2words(120, lang="sv"), "etthundratjugo")
        self.assertEqual(num2words(125, lang="sv"), "etthundratjugofem")
        self.assertEqual(num2words(150, lang="sv"), "etthundrafemtio")
        self.assertEqual(num2words(175, lang="sv"), "etthundrasjuttiofem")
        self.assertEqual(num2words(199, lang="sv"), "etthundranittionio")
        self.assertEqual(num2words(200, lang="sv"), "tvåhundra")
        self.assertEqual(num2words(201, lang="sv"), "tvåhundraett")
        self.assertEqual(num2words(210, lang="sv"), "tvåhundratio")
        self.assertEqual(num2words(220, lang="sv"), "tvåhundratjugo")
        self.assertEqual(num2words(250, lang="sv"), "tvåhundrafemtio")
        self.assertEqual(num2words(299, lang="sv"), "tvåhundranittionio")
        self.assertEqual(num2words(300, lang="sv"), "trehundra")
        self.assertEqual(num2words(333, lang="sv"), "trehundratrettiotre")
        self.assertEqual(num2words(400, lang="sv"), "fyrahundra")
        self.assertEqual(num2words(444, lang="sv"), "fyrahundrafyrtiofyra")
        self.assertEqual(num2words(500, lang="sv"), "femhundra")
        self.assertEqual(num2words(555, lang="sv"), "femhundrafemtiofem")
        self.assertEqual(num2words(600, lang="sv"), "sexhundra")
        self.assertEqual(num2words(666, lang="sv"), "sexhundrasextiosex")
        self.assertEqual(num2words(700, lang="sv"), "sjuhundra")
        self.assertEqual(num2words(777, lang="sv"), "sjuhundrasjuttiosju")
        self.assertEqual(num2words(800, lang="sv"), "åttahundra")
        self.assertEqual(num2words(888, lang="sv"), "åttahundraåttioåtta")
        self.assertEqual(num2words(900, lang="sv"), "niohundra")
        self.assertEqual(num2words(999, lang="sv"), "niohundranittionio")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sv"), "ettusen")
        self.assertEqual(num2words(1001, lang="sv"), "ettusenett")
        self.assertEqual(num2words(1010, lang="sv"), "ettusentio")
        self.assertEqual(num2words(1100, lang="sv"), "ettusen etthundra")
        self.assertEqual(num2words(1111, lang="sv"), "ettusen etthundraelva")
        self.assertEqual(num2words(1234, lang="sv"), "ettusen tvåhundratrettiofyra")
        self.assertEqual(num2words(1500, lang="sv"), "ettusen femhundra")
        self.assertEqual(num2words(1999, lang="sv"), "ettusen niohundranittionio")
        self.assertEqual(num2words(2000, lang="sv"), "tvåtusen")
        self.assertEqual(num2words(2001, lang="sv"), "tvåtusenett")
        self.assertEqual(num2words(2020, lang="sv"), "tvåtusentjugo")
        self.assertEqual(num2words(2222, lang="sv"), "tvåtusen tvåhundratjugotvå")
        self.assertEqual(num2words(3000, lang="sv"), "tretusen")
        self.assertEqual(num2words(3333, lang="sv"), "tretusen trehundratrettiotre")
        self.assertEqual(num2words(4000, lang="sv"), "fyratusen")
        self.assertEqual(num2words(4444, lang="sv"), "fyratusen fyrahundrafyrtiofyra")
        self.assertEqual(num2words(5000, lang="sv"), "femtusen")
        self.assertEqual(num2words(5555, lang="sv"), "femtusen femhundrafemtiofem")
        self.assertEqual(num2words(6000, lang="sv"), "sextusen")
        self.assertEqual(num2words(6666, lang="sv"), "sextusen sexhundrasextiosex")
        self.assertEqual(num2words(7000, lang="sv"), "sjutusen")
        self.assertEqual(num2words(7777, lang="sv"), "sjutusen sjuhundrasjuttiosju")
        self.assertEqual(num2words(8000, lang="sv"), "åttatusen")
        self.assertEqual(num2words(8888, lang="sv"), "åttatusen åttahundraåttioåtta")
        self.assertEqual(num2words(9000, lang="sv"), "niotusen")
        self.assertEqual(num2words(9999, lang="sv"), "niotusen niohundranittionio")
        self.assertEqual(num2words(10000, lang="sv"), "tiotusen")
        self.assertEqual(num2words(10001, lang="sv"), "tiotusenett")
        self.assertEqual(num2words(11111, lang="sv"), "elvatusen etthundraelva")
        self.assertEqual(num2words(12345, lang="sv"), "tolvtusen trehundrafyrtiofem")
        self.assertEqual(num2words(20000, lang="sv"), "tjugotusen")
        self.assertEqual(num2words(50000, lang="sv"), "femtiotusen")
        self.assertEqual(
            num2words(99999, lang="sv"), "nittioniotusen niohundranittionio"
        )
        self.assertEqual(num2words(100000, lang="sv"), "hundratusen")
        self.assertEqual(
            num2words(123456, lang="sv"), "etthundratjugotretusen fyrahundrafemtiosex"
        )
        self.assertEqual(num2words(200000, lang="sv"), "tvåhundratusen")
        self.assertEqual(num2words(500000, lang="sv"), "femhundratusen")
        self.assertEqual(
            num2words(654321, lang="sv"), "sexhundrafemtiofyratusen trehundratjugoett"
        )
        self.assertEqual(
            num2words(999999, lang="sv"), "niohundranittioniotusen niohundranittionio"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sv"), "en miljon")
        self.assertEqual(num2words(1000001, lang="sv"), "en miljonett")
        self.assertEqual(
            num2words(1111111, lang="sv"), "en miljon etthundraelvatusen etthundraelva"
        )
        self.assertEqual(
            num2words(1234567, lang="sv"),
            "en miljon tvåhundratrettiofyratusen femhundrasextiosju",
        )
        self.assertEqual(num2words(2000000, lang="sv"), "två miljoner")
        self.assertEqual(num2words(5000000, lang="sv"), "fem miljoner")
        self.assertEqual(
            num2words(9999999, lang="sv"),
            "nio miljoner niohundranittioniotusen niohundranittionio",
        )
        self.assertEqual(num2words(10000000, lang="sv"), "tio miljoner")
        self.assertEqual(
            num2words(12345678, lang="sv"),
            "tolv miljoner trehundrafyrtiofemtusen sexhundrasjuttioåtta",
        )
        self.assertEqual(
            num2words(99999999, lang="sv"),
            "nittionio miljoner niohundranittioniotusen niohundranittionio",
        )
        self.assertEqual(num2words(100000000, lang="sv"), "etthundra miljoner")
        self.assertEqual(
            num2words(123456789, lang="sv"),
            "etthundratjugotre miljoner fyrahundrafemtiosextusen sjuhundraåttionio",
        )
        self.assertEqual(
            num2words(999999999, lang="sv"),
            "niohundranittionio miljoner niohundranittioniotusen niohundranittionio",
        )
        self.assertEqual(num2words(1000000000, lang="sv"), "en miljard")
        self.assertEqual(
            num2words(1234567890, lang="sv"),
            "en miljard tvåhundratrettiofyra miljoner femhundrasextiosjutusen åttahundranittio",
        )
        self.assertEqual(
            num2words(9999999999, lang="sv"),
            "nio miljarder niohundranittionio miljoner niohundranittioniotusen niohundranittionio",
        )
        self.assertEqual(num2words(10000000000, lang="sv"), "tio miljarder")
        self.assertEqual(
            num2words(99999999999, lang="sv"),
            "nittionio miljarder niohundranittionio miljoner niohundranittioniotusen niohundranittionio",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sv"), "minus ett")
        self.assertEqual(num2words(-2, lang="sv"), "minus två")
        self.assertEqual(num2words(-5, lang="sv"), "minus fem")
        self.assertEqual(num2words(-10, lang="sv"), "minus tio")
        self.assertEqual(num2words(-11, lang="sv"), "minus elva")
        self.assertEqual(num2words(-20, lang="sv"), "minus tjugo")
        self.assertEqual(num2words(-50, lang="sv"), "minus femtio")
        self.assertEqual(num2words(-99, lang="sv"), "minus nittionio")
        self.assertEqual(num2words(-100, lang="sv"), "minus etthundra")
        self.assertEqual(num2words(-101, lang="sv"), "minus etthundraett")
        self.assertEqual(num2words(-200, lang="sv"), "minus tvåhundra")
        self.assertEqual(num2words(-999, lang="sv"), "minus niohundranittionio")
        self.assertEqual(num2words(-1000, lang="sv"), "minus ettusen")
        self.assertEqual(num2words(-1001, lang="sv"), "minus ettusenett")
        self.assertEqual(num2words(-10000, lang="sv"), "minus tiotusen")
        self.assertEqual(num2words(-100000, lang="sv"), "minus hundratusen")
        self.assertEqual(num2words(-1000000, lang="sv"), "minus en miljon")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sv"), "noll komma ett")
        self.assertEqual(num2words(0.5, lang="sv"), "noll komma fem")
        self.assertEqual(num2words(0.9, lang="sv"), "noll komma nio")
        self.assertEqual(num2words(1.1, lang="sv"), "ett komma ett")
        self.assertEqual(num2words(1.5, lang="sv"), "ett komma fem")
        self.assertEqual(num2words(2.5, lang="sv"), "två komma fem")
        self.assertEqual(num2words(3.14, lang="sv"), "tre komma ett fyra")
        self.assertEqual(num2words(10.5, lang="sv"), "tio komma fem")
        self.assertEqual(num2words(11.11, lang="sv"), "elva komma ett ett")
        self.assertEqual(num2words(20.2, lang="sv"), "tjugo komma två")
        self.assertEqual(num2words(99.99, lang="sv"), "nittionio komma nio nio")
        self.assertEqual(num2words(100.01, lang="sv"), "etthundra komma noll ett")
        self.assertEqual(num2words(100.5, lang="sv"), "etthundra komma fem")
        self.assertEqual(
            num2words(123.45, lang="sv"), "etthundratjugotre komma fyra fem"
        )
        self.assertEqual(num2words(1000.5, lang="sv"), "ettusen komma fem")
        self.assertEqual(
            num2words(1234.56, lang="sv"), "ettusen tvåhundratrettiofyra komma fem sex"
        )
        self.assertEqual(num2words(10000.01, lang="sv"), "tiotusen komma noll ett")
        self.assertEqual(num2words(-0.5, lang="sv"), "minus noll komma fem")
        self.assertEqual(num2words(-1.5, lang="sv"), "minus ett komma fem")
        self.assertEqual(num2words(-10.5, lang="sv"), "minus tio komma fem")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sv", ordinal=True), "första")
        self.assertEqual(num2words(2, lang="sv", ordinal=True), "andra")
        self.assertEqual(num2words(3, lang="sv", ordinal=True), "tredje")
        self.assertEqual(num2words(4, lang="sv", ordinal=True), "fjärde")
        self.assertEqual(num2words(5, lang="sv", ordinal=True), "femte")
        self.assertEqual(num2words(6, lang="sv", ordinal=True), "sjätte")
        self.assertEqual(num2words(7, lang="sv", ordinal=True), "sjunde")
        self.assertEqual(num2words(8, lang="sv", ordinal=True), "åttonde")
        self.assertEqual(num2words(9, lang="sv", ordinal=True), "nionde")
        self.assertEqual(num2words(10, lang="sv", ordinal=True), "tionde")
        self.assertEqual(num2words(11, lang="sv", ordinal=True), "elfte")
        self.assertEqual(num2words(12, lang="sv", ordinal=True), "tolfte")
        self.assertEqual(num2words(13, lang="sv", ordinal=True), "trettonde")
        self.assertEqual(num2words(14, lang="sv", ordinal=True), "fjortonde")
        self.assertEqual(num2words(15, lang="sv", ordinal=True), "femtonde")
        self.assertEqual(num2words(16, lang="sv", ordinal=True), "sextonde")
        self.assertEqual(num2words(17, lang="sv", ordinal=True), "sjuttonde")
        self.assertEqual(num2words(18, lang="sv", ordinal=True), "artonde")
        self.assertEqual(num2words(19, lang="sv", ordinal=True), "nittonde")
        self.assertEqual(num2words(20, lang="sv", ordinal=True), "tjugonde")  # 252
        self.assertEqual(num2words(21, lang="sv", ordinal=True), "tjugoförsta")
        self.assertEqual(num2words(22, lang="sv", ordinal=True), "tjugoandra")
        self.assertEqual(num2words(25, lang="sv", ordinal=True), "tjugofemte")
        self.assertEqual(num2words(30, lang="sv", ordinal=True), "trettionde")
        self.assertEqual(num2words(40, lang="sv", ordinal=True), "fyrtionde")
        self.assertEqual(num2words(50, lang="sv", ordinal=True), "femtionde")
        self.assertEqual(num2words(60, lang="sv", ordinal=True), "sextionde")
        self.assertEqual(num2words(70, lang="sv", ordinal=True), "sjuttionde")
        self.assertEqual(num2words(80, lang="sv", ordinal=True), "åttionde")
        self.assertEqual(num2words(90, lang="sv", ordinal=True), "nittionde")
        self.assertEqual(num2words(100, lang="sv", ordinal=True), "etthundrade")
        self.assertEqual(num2words(101, lang="sv", ordinal=True), "etthundraförsta")
        self.assertEqual(num2words(200, lang="sv", ordinal=True), "tvåhundrade")
        self.assertEqual(num2words(500, lang="sv", ordinal=True), "femhundrade")
        self.assertEqual(num2words(1000, lang="sv", ordinal=True), "ettusende")
        self.assertEqual(num2words(1001, lang="sv", ordinal=True), "ettusenförsta")
        self.assertEqual(num2words(10000, lang="sv", ordinal=True), "tiotusende")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="AUD"), "noll dollar"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="AUD"),
            "noll dollar, en cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="AUD"),
            "noll dollar, femtio cent",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="AUD"), "en dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="AUD"),
            "en dollar, femtio cent",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="BYN")
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="CAD"), "noll dollar"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="CAD"),
            "noll dollar, en cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="CAD"),
            "noll dollar, femtio cent",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="CAD"), "en dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="CAD"),
            "en dollar, femtio cent",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="EEK")
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="EUR"), "noll euro"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="EUR"),
            "noll euro, en cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="EUR"),
            "noll euro, femtio cent",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="EUR"), "en euro"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="EUR"),
            "en euro, femtio cent",
        )
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="GBP"), "noll pund"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="GBP"),
            "noll pund, en penny",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="GBP"),
            "noll pund, femtio pence",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="GBP"), "ett pund"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="GBP"),
            "ett pund, femtio pence",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="LTL")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="LVL")
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="USD"), "noll dollar"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="USD"),
            "noll dollar, en cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="USD"),
            "noll dollar, femtio cent",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="USD"), "en dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="USD"),
            "en dollar, femtio cent",
        )
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="RUB"), "noll rubel"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="RUB"),
            "noll rubel, en kopek",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="RUB"),
            "noll rubel, femtio kopek",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="RUB"), "en rubel"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="RUB"),
            "en rubel, femtio kopek",
        )
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="SEK"), "noll kronor"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="SEK"),
            "noll kronor, ett öre",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="SEK"),
            "noll kronor, femtio öre",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="SEK"), "en krona"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="SEK"),
            "en krona, femtio öre",
        )
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="NOK"), "noll kronor"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="NOK"),
            "noll kronor, ett öre",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="NOK"),
            "noll kronor, femtio öre",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="NOK"), "en krona"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="NOK"),
            "en krona, femtio öre",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="PLN")
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="MXN"), "noll peso"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="MXN"),
            "noll peso, en centavo",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="MXN"),
            "noll peso, femtio centavos",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="MXN"), "en peso"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="MXN"),
            "en peso, femtio centavos",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="RON")
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="INR"), "noll rupier"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="INR"),
            "noll rupier, en paisa",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="INR"),
            "noll rupier, femtio paise",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="INR"), "en rupie"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="INR"),
            "en rupie, femtio paise",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="HUF")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="ISK")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="UZS")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="SAR")
        self.assertEqual(
            num2words(0, lang="sv", to="currency", currency="JPY"), "noll yen"
        )
        self.assertEqual(
            num2words(0.01, lang="sv", to="currency", currency="JPY"),
            "noll yen, en sen",
        )
        self.assertEqual(
            num2words(0.5, lang="sv", to="currency", currency="JPY"),
            "noll yen, femtio sen",
        )
        self.assertEqual(
            num2words(1, lang="sv", to="currency", currency="JPY"), "en yen"
        )
        self.assertEqual(
            num2words(1.5, lang="sv", to="currency", currency="JPY"),
            "en yen, femtio sen",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="KRW")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="sv", to="currency", currency="NGN")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sv", to="year"), "tiohundra")
        self.assertEqual(num2words(1066, lang="sv", to="year"), "tiohundrasextiosex")
        self.assertEqual(
            num2words(1492, lang="sv", to="year"), "fjortonhundranittiotvå"
        )
        self.assertEqual(
            num2words(1776, lang="sv", to="year"), "sjuttonhundrasjuttiosex"
        )
        self.assertEqual(num2words(1800, lang="sv", to="year"), "artonhundra")
        self.assertEqual(num2words(1900, lang="sv", to="year"), "nittonhundra")
        self.assertEqual(num2words(1984, lang="sv", to="year"), "nittonhundraåttiofyra")
        self.assertEqual(num2words(1999, lang="sv", to="year"), "nittonhundranittionio")
        self.assertEqual(num2words(2000, lang="sv", to="year"), "tvåtusen")
        self.assertEqual(num2words(2001, lang="sv", to="year"), "tvåtusenett")
        self.assertEqual(num2words(2010, lang="sv", to="year"), "tvåtusentio")
        self.assertEqual(num2words(2020, lang="sv", to="year"), "tvåtusentjugo")
        self.assertEqual(num2words(2024, lang="sv", to="year"), "tvåtusentjugofyra")
        self.assertEqual(num2words(2100, lang="sv", to="year"), "tvåtusen etthundra")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sv"), "noll")
        self.assertEqual(num2words("1", lang="sv"), "ett")
        self.assertEqual(num2words("10", lang="sv"), "tio")
        self.assertEqual(num2words("100", lang="sv"), "etthundra")
        self.assertEqual(num2words("1000", lang="sv"), "ettusen")
        self.assertEqual(num2words("10000", lang="sv"), "tiotusen")
        self.assertEqual(num2words("100000", lang="sv"), "hundratusen")
        self.assertEqual(num2words("1000000", lang="sv"), "en miljon")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sv"), "noll")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sv"), num2words("100", lang="sv"))
        self.assertEqual(num2words(1000, lang="sv"), num2words("1000", lang="sv"))
