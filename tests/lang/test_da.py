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


class Num2WordsDATest(TestCase):
    """Comprehensive test cases for Danish language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="da"), "nul")
        self.assertEqual(num2words(1, lang="da"), "et")
        self.assertEqual(num2words(2, lang="da"), "to")
        self.assertEqual(num2words(3, lang="da"), "tre")
        self.assertEqual(num2words(4, lang="da"), "fire")
        self.assertEqual(num2words(5, lang="da"), "fem")
        self.assertEqual(num2words(6, lang="da"), "seks")
        self.assertEqual(num2words(7, lang="da"), "syv")
        self.assertEqual(num2words(8, lang="da"), "otte")
        self.assertEqual(num2words(9, lang="da"), "ni")
        self.assertEqual(num2words(10, lang="da"), "ti")
        self.assertEqual(num2words(11, lang="da"), "elleve")
        self.assertEqual(num2words(12, lang="da"), "tolv")
        self.assertEqual(num2words(13, lang="da"), "tretten")
        self.assertEqual(num2words(14, lang="da"), "fjorten")
        self.assertEqual(num2words(15, lang="da"), "femten")
        self.assertEqual(num2words(16, lang="da"), "seksten")
        self.assertEqual(num2words(17, lang="da"), "sytten")
        self.assertEqual(num2words(18, lang="da"), "atten")
        self.assertEqual(num2words(19, lang="da"), "nitten")
        self.assertEqual(num2words(20, lang="da"), "tyve")
        self.assertEqual(num2words(21, lang="da"), "enogtyve")
        self.assertEqual(num2words(22, lang="da"), "toogtyve")
        self.assertEqual(num2words(23, lang="da"), "treogtyve")
        self.assertEqual(num2words(24, lang="da"), "fireogtyve")
        self.assertEqual(num2words(25, lang="da"), "femogtyve")
        self.assertEqual(num2words(26, lang="da"), "seksogtyve")
        self.assertEqual(num2words(27, lang="da"), "syvogtyve")
        self.assertEqual(num2words(28, lang="da"), "otteogtyve")
        self.assertEqual(num2words(29, lang="da"), "niogtyve")
        self.assertEqual(num2words(30, lang="da"), "tredive")
        self.assertEqual(num2words(31, lang="da"), "enogtredive")
        self.assertEqual(num2words(35, lang="da"), "femogtredive")
        self.assertEqual(num2words(40, lang="da"), "fyrre")
        self.assertEqual(num2words(45, lang="da"), "femogfyrre")
        self.assertEqual(num2words(50, lang="da"), "halvtreds")
        self.assertEqual(num2words(55, lang="da"), "femoghalvtreds")
        self.assertEqual(num2words(60, lang="da"), "treds")
        self.assertEqual(num2words(65, lang="da"), "femogtreds")
        self.assertEqual(num2words(70, lang="da"), "halvfjerds")
        self.assertEqual(num2words(75, lang="da"), "femoghalvfjerds")
        self.assertEqual(num2words(80, lang="da"), "firs")
        self.assertEqual(num2words(85, lang="da"), "femogfirs")
        self.assertEqual(num2words(90, lang="da"), "halvfems")
        self.assertEqual(num2words(95, lang="da"), "femoghalvfems")
        self.assertEqual(num2words(99, lang="da"), "nioghalvfems")
        self.assertEqual(num2words(100, lang="da"), "ethundrede")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="da"), "ethundrede og et")
        self.assertEqual(num2words(110, lang="da"), "ethundrede og ti")
        self.assertEqual(num2words(111, lang="da"), "ethundrede og elleve")
        self.assertEqual(num2words(120, lang="da"), "ethundrede og tyve")
        self.assertEqual(num2words(125, lang="da"), "ethundrede og femogtyve")
        self.assertEqual(num2words(150, lang="da"), "ethundrede og halvtreds")
        self.assertEqual(num2words(175, lang="da"), "ethundrede og femoghalvfjerds")
        self.assertEqual(num2words(199, lang="da"), "ethundrede og nioghalvfems")
        self.assertEqual(num2words(200, lang="da"), "tohundrede")
        self.assertEqual(num2words(201, lang="da"), "tohundrede og et")
        self.assertEqual(num2words(210, lang="da"), "tohundrede og ti")
        self.assertEqual(num2words(220, lang="da"), "tohundrede og tyve")
        self.assertEqual(num2words(250, lang="da"), "tohundrede og halvtreds")
        self.assertEqual(num2words(299, lang="da"), "tohundrede og nioghalvfems")
        self.assertEqual(num2words(300, lang="da"), "trehundrede")
        self.assertEqual(num2words(333, lang="da"), "trehundrede og treogtredive")
        self.assertEqual(num2words(400, lang="da"), "firehundrede")
        self.assertEqual(num2words(444, lang="da"), "firehundrede og fireogfyrre")
        self.assertEqual(num2words(500, lang="da"), "femhundrede")
        self.assertEqual(num2words(555, lang="da"), "femhundrede og femoghalvtreds")
        self.assertEqual(num2words(600, lang="da"), "sekshundrede")
        self.assertEqual(num2words(666, lang="da"), "sekshundrede og seksogtreds")
        self.assertEqual(num2words(700, lang="da"), "syvhundrede")
        self.assertEqual(num2words(777, lang="da"), "syvhundrede og syvoghalvfjerds")
        self.assertEqual(num2words(800, lang="da"), "ottehundrede")
        self.assertEqual(num2words(888, lang="da"), "ottehundrede og otteogfirs")
        self.assertEqual(num2words(900, lang="da"), "nihundrede")
        self.assertEqual(num2words(999, lang="da"), "nihundrede og nioghalvfems")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="da"), "ettusind")
        self.assertEqual(num2words(1001, lang="da"), "ettusinde og et")
        self.assertEqual(num2words(1010, lang="da"), "ettusinde og ti")
        self.assertEqual(num2words(1100, lang="da"), "ettusinde og ethundrede")
        self.assertEqual(
            num2words(1111, lang="da"), "ettusinde og ethundrede og elleve"
        )
        self.assertEqual(
            num2words(1234, lang="da"), "ettusinde og tohundrede og fireogtredive"
        )
        self.assertEqual(num2words(1500, lang="da"), "ettusinde og femhundrede")
        self.assertEqual(
            num2words(1999, lang="da"), "ettusinde og nihundrede og nioghalvfems"
        )
        self.assertEqual(num2words(2000, lang="da"), "totusind")
        self.assertEqual(num2words(2001, lang="da"), "totusinde og et")
        self.assertEqual(num2words(2020, lang="da"), "totusinde og tyve")
        self.assertEqual(
            num2words(2222, lang="da"), "totusinde og tohundrede og toogtyve"
        )
        self.assertEqual(num2words(3000, lang="da"), "tretusind")
        self.assertEqual(
            num2words(3333, lang="da"), "tretusinde og trehundrede og treogtredive"
        )
        self.assertEqual(num2words(4000, lang="da"), "firetusind")
        self.assertEqual(
            num2words(4444, lang="da"), "firetusinde og firehundrede og fireogfyrre"
        )
        self.assertEqual(num2words(5000, lang="da"), "femtusind")
        self.assertEqual(
            num2words(5555, lang="da"), "femtusinde og femhundrede og femoghalvtreds"
        )
        self.assertEqual(num2words(6000, lang="da"), "sekstusind")
        self.assertEqual(
            num2words(6666, lang="da"), "sekstusinde og sekshundrede og seksogtreds"
        )
        self.assertEqual(num2words(7000, lang="da"), "syvtusind")
        self.assertEqual(
            num2words(7777, lang="da"), "syvtusinde og syvhundrede og syvoghalvfjerds"
        )
        self.assertEqual(num2words(8000, lang="da"), "ottetusind")
        self.assertEqual(
            num2words(8888, lang="da"), "ottetusinde og ottehundrede og otteogfirs"
        )
        self.assertEqual(num2words(9000, lang="da"), "nitusind")
        self.assertEqual(
            num2words(9999, lang="da"), "nitusinde og nihundrede og nioghalvfems"
        )
        self.assertEqual(num2words(10000, lang="da"), "ti tusind")
        self.assertEqual(num2words(10001, lang="da"), "ti tusinde og et")
        self.assertEqual(
            num2words(11111, lang="da"), "ellevetusinde og ethundrede og elleve"
        )
        self.assertEqual(
            num2words(12345, lang="da"), "tolvtusinde og trehundrede og femogfyrre"
        )
        self.assertEqual(num2words(20000, lang="da"), "tyvetusind")
        self.assertEqual(num2words(50000, lang="da"), "halvtredstusind")
        self.assertEqual(
            num2words(99999, lang="da"),
            "nioghalvfemstusinde og nihundrede og nioghalvfems",
        )
        self.assertEqual(num2words(100000, lang="da"), "ethundrede tusind")
        self.assertEqual(
            num2words(123456, lang="da"),
            "ethundrede og treogtyvetusindfirehundrede og seksoghalvtreds",
        )
        self.assertEqual(num2words(200000, lang="da"), "tohundredetusind")
        self.assertEqual(num2words(500000, lang="da"), "femhundredetusind")
        self.assertEqual(
            num2words(654321, lang="da"),
            "sekshundrede og fireoghalvtredstusindtrehundrede og enogtyve",
        )
        self.assertEqual(
            num2words(999999, lang="da"),
            "nihundrede og nioghalvfemstusindnihundrede og nioghalvfems",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="da"), "en million")
        self.assertEqual(num2words(1000001, lang="da"), "en million et")
        self.assertEqual(
            num2words(1111111, lang="da"),
            "en million ethundrede og ellevetusindethundrede og elleve",
        )
        self.assertEqual(
            num2words(1234567, lang="da"),
            "en million tohundrede og fireogtredivetusindfemhundrede og syvogtreds",
        )
        self.assertEqual(num2words(2000000, lang="da"), "to millioner")
        self.assertEqual(num2words(5000000, lang="da"), "fem millioner")
        self.assertEqual(
            num2words(9999999, lang="da"),
            "ni millioner nihundrede og nioghalvfemstusindnihundrede og nioghalvfems",
        )
        self.assertEqual(num2words(10000000, lang="da"), "ti millioner")
        self.assertEqual(
            num2words(12345678, lang="da"),
            "tolv millioner trehundrede og femogfyrretusindsekshundrede og otteoghalvfjerds",
        )
        self.assertEqual(
            num2words(99999999, lang="da"),
            "nioghalvfems millioner nihundrede og nioghalvfemstusindnihundrede og nioghalvfems",
        )
        self.assertEqual(num2words(100000000, lang="da"), "ethundrede millioner")
        self.assertEqual(
            num2words(123456789, lang="da"),
            "ethundrede og treogtyve millioner firehundrede og seksoghalvtredstusindsyvhundrede og niogfirs",
        )
        self.assertEqual(
            num2words(999999999, lang="da"),
            "nihundrede og nioghalvfems millioner nihundrede og nioghalvfemstusindnihundrede og nioghalvfems",
        )
        self.assertEqual(num2words(1000000000, lang="da"), "en milliard")
        self.assertEqual(
            num2words(1234567890, lang="da"),
            "en milliard tohundrede og fireogtredive millioner femhundrede og syvogtredstusindottehundrede og halvfems",
        )
        self.assertEqual(
            num2words(9999999999, lang="da"),
            "ni milliarder nihundrede og nioghalvfems millioner nihundrede og nioghalvfemstusindnihundrede og nioghalvfems",
        )
        self.assertEqual(num2words(10000000000, lang="da"), "ti milliarder")
        self.assertEqual(
            num2words(99999999999, lang="da"),
            "nioghalvfems milliarder nihundrede og nioghalvfems millioner nihundrede og nioghalvfemstusindnihundrede og nioghalvfems",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="da"), "minus et")
        self.assertEqual(num2words(-2, lang="da"), "minus to")
        self.assertEqual(num2words(-5, lang="da"), "minus fem")
        self.assertEqual(num2words(-10, lang="da"), "minus ti")
        self.assertEqual(num2words(-11, lang="da"), "minus elleve")
        self.assertEqual(num2words(-20, lang="da"), "minus tyve")
        self.assertEqual(num2words(-50, lang="da"), "minus halvtreds")
        self.assertEqual(num2words(-99, lang="da"), "minus nioghalvfems")
        self.assertEqual(num2words(-100, lang="da"), "minus ethundrede")
        self.assertEqual(num2words(-101, lang="da"), "minus ethundrede og et")
        self.assertEqual(num2words(-200, lang="da"), "minus tohundrede")
        self.assertEqual(num2words(-999, lang="da"), "minus nihundrede og nioghalvfems")
        self.assertEqual(num2words(-1000, lang="da"), "minus ettusind")
        self.assertEqual(num2words(-1001, lang="da"), "minus ettusinde og et")
        self.assertEqual(num2words(-10000, lang="da"), "minus ti tusind")
        self.assertEqual(num2words(-100000, lang="da"), "minus ethundrede tusind")
        self.assertEqual(num2words(-1000000, lang="da"), "minus en million")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="da"), "nul komma et")
        self.assertEqual(num2words(0.5, lang="da"), "nul komma fem")
        self.assertEqual(num2words(0.9, lang="da"), "nul komma ni")
        self.assertEqual(num2words(1.1, lang="da"), "et komma et")
        self.assertEqual(num2words(1.5, lang="da"), "et komma fem")
        self.assertEqual(num2words(2.5, lang="da"), "to komma fem")
        self.assertEqual(num2words(3.14, lang="da"), "tre komma et fire")
        self.assertEqual(num2words(10.5, lang="da"), "ti komma fem")
        self.assertEqual(num2words(11.11, lang="da"), "elleve komma et et")
        self.assertEqual(num2words(20.2, lang="da"), "tyve komma to")
        self.assertEqual(num2words(99.99, lang="da"), "nioghalvfems komma ni ni")
        self.assertEqual(num2words(100.01, lang="da"), "ethundrede komma nul et")
        self.assertEqual(num2words(100.5, lang="da"), "ethundrede komma fem")
        self.assertEqual(
            num2words(123.45, lang="da"), "ethundrede og treogtyve komma fire fem"
        )
        self.assertEqual(num2words(1000.5, lang="da"), "ettusind komma fem")
        self.assertEqual(
            num2words(1234.56, lang="da"),
            "ettusinde og tohundrede og fireogtredive komma fem seks",
        )
        self.assertEqual(num2words(10000.01, lang="da"), "ti tusind komma nul et")
        self.assertEqual(num2words(-0.5, lang="da"), "minus nul komma fem")
        self.assertEqual(num2words(-1.5, lang="da"), "minus et komma fem")
        self.assertEqual(num2words(-10.5, lang="da"), "minus ti komma fem")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="da", ordinal=True), "første")
        self.assertEqual(num2words(2, lang="da", ordinal=True), "anden")
        self.assertEqual(num2words(3, lang="da", ordinal=True), "tredje")
        self.assertEqual(num2words(4, lang="da", ordinal=True), "fjerde")
        self.assertEqual(num2words(5, lang="da", ordinal=True), "femte")
        self.assertEqual(num2words(6, lang="da", ordinal=True), "sjette")
        self.assertEqual(num2words(7, lang="da", ordinal=True), "syvende")
        self.assertEqual(num2words(8, lang="da", ordinal=True), "ottende")
        self.assertEqual(num2words(9, lang="da", ordinal=True), "niende")
        self.assertEqual(num2words(10, lang="da", ordinal=True), "tiende")
        self.assertEqual(num2words(11, lang="da", ordinal=True), "ellevte")
        self.assertEqual(num2words(12, lang="da", ordinal=True), "tolvte")
        self.assertEqual(num2words(13, lang="da", ordinal=True), "trettende")
        self.assertEqual(num2words(14, lang="da", ordinal=True), "fjortende")
        self.assertEqual(num2words(15, lang="da", ordinal=True), "femtende")
        self.assertEqual(num2words(16, lang="da", ordinal=True), "sekstende")
        self.assertEqual(num2words(17, lang="da", ordinal=True), "syttende")
        self.assertEqual(num2words(18, lang="da", ordinal=True), "attende")
        self.assertEqual(num2words(19, lang="da", ordinal=True), "nittende")
        self.assertEqual(num2words(20, lang="da", ordinal=True), "tyvende")
        self.assertEqual(num2words(21, lang="da", ordinal=True), "enogtyvende")
        self.assertEqual(num2words(22, lang="da", ordinal=True), "toogtyvende")
        self.assertEqual(num2words(25, lang="da", ordinal=True), "femogtyvende")
        self.assertEqual(num2words(30, lang="da", ordinal=True), "tredivte")  # 252
        self.assertEqual(num2words(40, lang="da", ordinal=True), "fyrrende")  # 252
        self.assertEqual(num2words(50, lang="da", ordinal=True), "halvtredsende")
        self.assertEqual(num2words(60, lang="da", ordinal=True), "tressende")
        self.assertEqual(num2words(70, lang="da", ordinal=True), "halvfjerdsende")
        self.assertEqual(num2words(80, lang="da", ordinal=True), "firsende")
        self.assertEqual(num2words(90, lang="da", ordinal=True), "halvfemsende")
        self.assertEqual(num2words(100, lang="da", ordinal=True), "hundrede")  # 252
        self.assertEqual(
            num2words(101, lang="da", ordinal=True), "ethundrede og første"
        )
        self.assertEqual(num2words(200, lang="da", ordinal=True), "tohundrede")
        self.assertEqual(num2words(500, lang="da", ordinal=True), "femhundrede")
        self.assertEqual(num2words(1000, lang="da", ordinal=True), "tusinde")  # 252
        self.assertEqual(
            num2words(1001, lang="da", ordinal=True), "ettusinde og første"
        )
        self.assertEqual(num2words(10000, lang="da", ordinal=True), "ti tusinde")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="da", to="currency", currency="DKK"), "nul kroner"
        )
        self.assertEqual(
            num2words(0.01, lang="da", to="currency", currency="DKK"),
            "nul kroner, et øre",
        )
        self.assertEqual(
            num2words(0.5, lang="da", to="currency", currency="DKK"),
            "nul kroner, halvtreds øre",
        )
        self.assertEqual(
            num2words(1, lang="da", to="currency", currency="DKK"), "et krone"
        )
        self.assertEqual(
            num2words(1.5, lang="da", to="currency", currency="DKK"),
            "et krone, halvtreds øre",
        )
        self.assertEqual(
            num2words(0, lang="da", to="currency", currency="EUR"), "nul euro"
        )
        self.assertEqual(
            num2words(0.01, lang="da", to="currency", currency="EUR"),
            "nul euro, et cent",
        )
        self.assertEqual(
            num2words(0.5, lang="da", to="currency", currency="EUR"),
            "nul euro, halvtreds cent",
        )
        self.assertEqual(
            num2words(1, lang="da", to="currency", currency="EUR"), "et euro"
        )
        self.assertEqual(
            num2words(1.5, lang="da", to="currency", currency="EUR"),
            "et euro, halvtreds cent",
        )
        self.assertEqual(
            num2words(0, lang="da", to="currency", currency="USD"), "nul dollars"
        )
        self.assertEqual(
            num2words(0.01, lang="da", to="currency", currency="USD"),
            "nul dollars, et cent",
        )
        self.assertEqual(
            num2words(0.5, lang="da", to="currency", currency="USD"),
            "nul dollars, halvtreds cent",
        )
        self.assertEqual(
            num2words(1, lang="da", to="currency", currency="USD"), "et dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="da", to="currency", currency="USD"),
            "et dollar, halvtreds cent",
        )
        self.assertEqual(
            num2words(0, lang="da", to="currency", currency="GBP"), "nul pund"
        )
        self.assertEqual(
            num2words(0.01, lang="da", to="currency", currency="GBP"),
            "nul pund, et penny",
        )
        self.assertEqual(
            num2words(0.5, lang="da", to="currency", currency="GBP"),
            "nul pund, halvtreds pence",
        )
        self.assertEqual(
            num2words(1, lang="da", to="currency", currency="GBP"), "et pund"
        )
        self.assertEqual(
            num2words(1.5, lang="da", to="currency", currency="GBP"),
            "et pund, halvtreds pence",
        )
        self.assertEqual(
            num2words(0, lang="da", to="currency", currency="SEK"), "nul kroner"
        )
        self.assertEqual(
            num2words(0.01, lang="da", to="currency", currency="SEK"),
            "nul kroner, et øre",
        )
        self.assertEqual(
            num2words(0.5, lang="da", to="currency", currency="SEK"),
            "nul kroner, halvtreds øre",
        )
        self.assertEqual(
            num2words(1, lang="da", to="currency", currency="SEK"), "et krone"
        )
        self.assertEqual(
            num2words(1.5, lang="da", to="currency", currency="SEK"),
            "et krone, halvtreds øre",
        )
        self.assertEqual(
            num2words(0, lang="da", to="currency", currency="NOK"), "nul kroner"
        )
        self.assertEqual(
            num2words(0.01, lang="da", to="currency", currency="NOK"),
            "nul kroner, et øre",
        )
        self.assertEqual(
            num2words(0.5, lang="da", to="currency", currency="NOK"),
            "nul kroner, halvtreds øre",
        )
        self.assertEqual(
            num2words(1, lang="da", to="currency", currency="NOK"), "et krone"
        )
        self.assertEqual(
            num2words(1.5, lang="da", to="currency", currency="NOK"),
            "et krone, halvtreds øre",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="da", to="year"), "ettusind")
        self.assertEqual(
            num2words(1066, lang="da", to="year"), "ettusinde og seksogtreds"
        )
        self.assertEqual(
            num2words(1492, lang="da", to="year"), "fjorten hundrede tooghalvfems"
        )
        self.assertEqual(
            num2words(1776, lang="da", to="year"), "sytten hundrede seksoghalvfjerds"
        )
        self.assertEqual(num2words(1800, lang="da", to="year"), "atten hundrede")
        self.assertEqual(num2words(1900, lang="da", to="year"), "nitten hundrede")
        self.assertEqual(
            num2words(1984, lang="da", to="year"), "nitten hundrede fireogfirs"
        )
        self.assertEqual(
            num2words(1999, lang="da", to="year"), "nitten hundrede nioghalvfems"
        )
        self.assertEqual(num2words(2000, lang="da", to="year"), "totusind")
        self.assertEqual(num2words(2001, lang="da", to="year"), "totusinde og et")
        self.assertEqual(num2words(2010, lang="da", to="year"), "totusinde og ti")
        self.assertEqual(num2words(2020, lang="da", to="year"), "totusinde og tyve")
        self.assertEqual(
            num2words(2024, lang="da", to="year"), "totusinde og fireogtyve"
        )
        self.assertEqual(num2words(2100, lang="da", to="year"), "enogtyve hundrede")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="da"), "nul")
        self.assertEqual(num2words("1", lang="da"), "et")
        self.assertEqual(num2words("10", lang="da"), "ti")
        self.assertEqual(num2words("100", lang="da"), "ethundrede")
        self.assertEqual(num2words("1000", lang="da"), "ettusind")
        self.assertEqual(num2words("10000", lang="da"), "ti tusind")
        self.assertEqual(num2words("100000", lang="da"), "ethundrede tusind")
        self.assertEqual(num2words("1000000", lang="da"), "en million")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="da"), "nul")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="da"), num2words("100", lang="da"))
        self.assertEqual(num2words(1000, lang="da"), num2words("1000", lang="da"))


    def test_singular_scale_after_en(self):
        """gladiaio/num2words2#163: "en" takes the singular scale word."""
        for lang in ("da", "dk"):
            self.assertEqual(num2words(10**6, lang=lang), "en million")
            self.assertEqual(num2words(10**9, lang=lang), "en milliard")
            self.assertEqual(num2words(10**12, lang=lang), "en billion")
            self.assertEqual(num2words(10**15, lang=lang), "en billiard")
            self.assertEqual(
                num2words(1829794, lang=lang),
                "en million ottehundrede og niogtyvetusindsyvhundrede og "
                "fireoghalvfems",
            )
            self.assertEqual(
                num2words(10**9 + 10**6, lang=lang), "en milliard en million"
            )
            # Plural from two up.
            self.assertEqual(num2words(2 * 10**6, lang=lang), "to millioner")
            self.assertEqual(num2words(3 * 10**9, lang=lang), "tre milliarder")
            self.assertEqual(
                num2words(10**6, lang=lang, to="currency", currency="DKK"),
                "en million kroner",
            )

    def test_ordinal_scale_words_singular(self):
        """gladiaio/num2words2#172: ordinals build on the singular scale word."""
        for lang in ("da", "dk"):
            self.assertEqual(num2words(10**6, lang=lang, to="ordinal"), "millionte")
            self.assertEqual(num2words(10**9, lang=lang, to="ordinal"), "milliardte")
            self.assertEqual(num2words(10**12, lang=lang, to="ordinal"), "billionte")
            self.assertEqual(num2words(10**15, lang=lang, to="ordinal"), "billiardte")
            self.assertEqual(
                num2words(2 * 10**6, lang=lang, to="ordinal"), "to millionte"
            )
            self.assertEqual(
                num2words(3 * 10**9, lang=lang, to="ordinal"), "tre milliardte"
            )
            self.assertEqual(num2words(10**6, lang=lang, to="ordinal_num"), "1000000te")

    def test_ordinal_compound_above_million(self):
        """gladiaio/num2words2#184: compounds keep the cardinal's count.

        Only the last word is ordinalised; the leading part is the cardinal
        (Python emitted "millioner første" for 10**6 + 1).
        """
        cases = {
            10**6 + 1: "en million første",
            10**6 + 21: "en million enogtyvende",
            2 * 10**6 + 3: "to millioner tredje",
            10**9 + 1: "en milliard første",
            10**9 + 10**6: "en milliard en millionte",
            10**9 + 10**6 + 1: "en milliard en million første",
            10**6 + 1000: "en million ettusinde",
            10**6 + 1001: "en million ettusinde og første",
            10**12 + 1: "en billion første",
        }
        for lang in ("da", "dk"):
            for value, expected in cases.items():
                self.assertEqual(
                    num2words(value, lang=lang, to="ordinal"), expected
                )
            # The words before the ordinalised one match the cardinal's.
            for value in cases:
                card = num2words(value, lang=lang).split(" ")
                ordw = num2words(value, lang=lang, to="ordinal").split(" ")
                self.assertEqual(ordw[:-1], card[:-1])
