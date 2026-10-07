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


class Num2WordsNLTest(TestCase):
    """Comprehensive test cases for Dutch language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="nl"), "nul")
        self.assertEqual(num2words(1, lang="nl"), "één")
        self.assertEqual(num2words(2, lang="nl"), "twee")
        self.assertEqual(num2words(3, lang="nl"), "drie")
        self.assertEqual(num2words(4, lang="nl"), "vier")
        self.assertEqual(num2words(5, lang="nl"), "vijf")
        self.assertEqual(num2words(6, lang="nl"), "zes")
        self.assertEqual(num2words(7, lang="nl"), "zeven")
        self.assertEqual(num2words(8, lang="nl"), "acht")
        self.assertEqual(num2words(9, lang="nl"), "negen")
        self.assertEqual(num2words(10, lang="nl"), "tien")
        self.assertEqual(num2words(11, lang="nl"), "elf")
        self.assertEqual(num2words(12, lang="nl"), "twaalf")
        self.assertEqual(num2words(13, lang="nl"), "dertien")
        self.assertEqual(num2words(14, lang="nl"), "veertien")
        self.assertEqual(num2words(15, lang="nl"), "vijftien")
        self.assertEqual(num2words(16, lang="nl"), "zestien")
        self.assertEqual(num2words(17, lang="nl"), "zeventien")
        self.assertEqual(num2words(18, lang="nl"), "achttien")
        self.assertEqual(num2words(19, lang="nl"), "negentien")
        self.assertEqual(num2words(20, lang="nl"), "twintig")
        self.assertEqual(num2words(21, lang="nl"), "eenentwintig")
        self.assertEqual(num2words(22, lang="nl"), "tweeëntwintig")
        self.assertEqual(num2words(23, lang="nl"), "drieëntwintig")
        self.assertEqual(num2words(24, lang="nl"), "vierentwintig")
        self.assertEqual(num2words(25, lang="nl"), "vijfentwintig")
        self.assertEqual(num2words(26, lang="nl"), "zesentwintig")
        self.assertEqual(num2words(27, lang="nl"), "zevenentwintig")
        self.assertEqual(num2words(28, lang="nl"), "achtentwintig")
        self.assertEqual(num2words(29, lang="nl"), "negenentwintig")
        self.assertEqual(num2words(30, lang="nl"), "dertig")
        self.assertEqual(num2words(31, lang="nl"), "eenendertig")
        self.assertEqual(num2words(35, lang="nl"), "vijfendertig")
        self.assertEqual(num2words(40, lang="nl"), "veertig")
        self.assertEqual(num2words(45, lang="nl"), "vijfenveertig")
        self.assertEqual(num2words(50, lang="nl"), "vijftig")
        self.assertEqual(num2words(55, lang="nl"), "vijfenvijftig")
        self.assertEqual(num2words(60, lang="nl"), "zestig")
        self.assertEqual(num2words(65, lang="nl"), "vijfenzestig")
        self.assertEqual(num2words(70, lang="nl"), "zeventig")
        self.assertEqual(num2words(75, lang="nl"), "vijfenzeventig")
        self.assertEqual(num2words(80, lang="nl"), "tachtig")
        self.assertEqual(num2words(85, lang="nl"), "vijfentachtig")
        self.assertEqual(num2words(90, lang="nl"), "negentig")
        self.assertEqual(num2words(95, lang="nl"), "vijfennegentig")
        self.assertEqual(num2words(99, lang="nl"), "negenennegentig")
        self.assertEqual(num2words(100, lang="nl"), "honderd")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="nl"), "honderdéén")
        self.assertEqual(num2words(110, lang="nl"), "honderdtien")
        self.assertEqual(num2words(111, lang="nl"), "honderdelf")
        self.assertEqual(num2words(120, lang="nl"), "honderdtwintig")
        self.assertEqual(num2words(125, lang="nl"), "honderdvijfentwintig")
        self.assertEqual(num2words(150, lang="nl"), "honderdvijftig")
        self.assertEqual(num2words(175, lang="nl"), "honderdvijfenzeventig")
        self.assertEqual(num2words(199, lang="nl"), "honderdnegenennegentig")
        self.assertEqual(num2words(200, lang="nl"), "tweehonderd")
        self.assertEqual(num2words(201, lang="nl"), "tweehonderdéén")
        self.assertEqual(num2words(210, lang="nl"), "tweehonderdtien")
        self.assertEqual(num2words(220, lang="nl"), "tweehonderdtwintig")
        self.assertEqual(num2words(250, lang="nl"), "tweehonderdvijftig")
        self.assertEqual(num2words(299, lang="nl"), "tweehonderdnegenennegentig")
        self.assertEqual(num2words(300, lang="nl"), "driehonderd")
        self.assertEqual(num2words(333, lang="nl"), "driehonderddrieëndertig")
        self.assertEqual(num2words(400, lang="nl"), "vierhonderd")
        self.assertEqual(num2words(444, lang="nl"), "vierhonderdvierenveertig")
        self.assertEqual(num2words(500, lang="nl"), "vijfhonderd")
        self.assertEqual(num2words(555, lang="nl"), "vijfhonderdvijfenvijftig")
        self.assertEqual(num2words(600, lang="nl"), "zeshonderd")
        self.assertEqual(num2words(666, lang="nl"), "zeshonderdzesenzestig")
        self.assertEqual(num2words(700, lang="nl"), "zevenhonderd")
        self.assertEqual(num2words(777, lang="nl"), "zevenhonderdzevenenzeventig")
        self.assertEqual(num2words(800, lang="nl"), "achthonderd")
        self.assertEqual(num2words(888, lang="nl"), "achthonderdachtentachtig")
        self.assertEqual(num2words(900, lang="nl"), "negenhonderd")
        self.assertEqual(num2words(999, lang="nl"), "negenhonderdnegenennegentig")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="nl"), "duizend")
        self.assertEqual(num2words(1001, lang="nl"), "duizend één")
        self.assertEqual(num2words(1010, lang="nl"), "duizend tien")
        self.assertEqual(num2words(1100, lang="nl"), "duizend honderd")
        self.assertEqual(num2words(1111, lang="nl"), "duizend honderdelf")
        self.assertEqual(num2words(1234, lang="nl"), "duizend tweehonderdvierendertig")
        self.assertEqual(num2words(1500, lang="nl"), "duizend vijfhonderd")
        self.assertEqual(
            num2words(1999, lang="nl"), "duizend negenhonderdnegenennegentig"
        )
        self.assertEqual(num2words(2000, lang="nl"), "tweeduizend")
        self.assertEqual(num2words(2001, lang="nl"), "tweeduizend één")
        self.assertEqual(num2words(2020, lang="nl"), "tweeduizend twintig")
        self.assertEqual(
            num2words(2222, lang="nl"), "tweeduizend tweehonderdtweeëntwintig"
        )
        self.assertEqual(num2words(3000, lang="nl"), "drieduizend")
        self.assertEqual(
            num2words(3333, lang="nl"), "drieduizend driehonderddrieëndertig"
        )
        self.assertEqual(num2words(4000, lang="nl"), "vierduizend")
        self.assertEqual(
            num2words(4444, lang="nl"), "vierduizend vierhonderdvierenveertig"
        )
        self.assertEqual(num2words(5000, lang="nl"), "vijfduizend")
        self.assertEqual(
            num2words(5555, lang="nl"), "vijfduizend vijfhonderdvijfenvijftig"
        )
        self.assertEqual(num2words(6000, lang="nl"), "zesduizend")
        self.assertEqual(num2words(6666, lang="nl"), "zesduizend zeshonderdzesenzestig")
        self.assertEqual(num2words(7000, lang="nl"), "zevenduizend")
        self.assertEqual(
            num2words(7777, lang="nl"), "zevenduizend zevenhonderdzevenenzeventig"
        )
        self.assertEqual(num2words(8000, lang="nl"), "achtduizend")
        self.assertEqual(
            num2words(8888, lang="nl"), "achtduizend achthonderdachtentachtig"
        )
        self.assertEqual(num2words(9000, lang="nl"), "negenduizend")
        self.assertEqual(
            num2words(9999, lang="nl"), "negenduizend negenhonderdnegenennegentig"
        )
        self.assertEqual(num2words(10000, lang="nl"), "tienduizend")
        self.assertEqual(num2words(10001, lang="nl"), "tienduizend één")
        self.assertEqual(num2words(11111, lang="nl"), "elfduizend honderdelf")
        self.assertEqual(
            num2words(12345, lang="nl"), "twaalfduizend driehonderdvijfenveertig"
        )
        self.assertEqual(num2words(20000, lang="nl"), "twintigduizend")
        self.assertEqual(num2words(50000, lang="nl"), "vijftigduizend")
        self.assertEqual(
            num2words(99999, lang="nl"),
            "negenennegentigduizend negenhonderdnegenennegentig",
        )
        self.assertEqual(num2words(100000, lang="nl"), "honderdduizend")
        self.assertEqual(
            num2words(123456, lang="nl"),
            "honderddrieëntwintigduizend vierhonderdzesenvijftig",
        )
        self.assertEqual(num2words(200000, lang="nl"), "tweehonderdduizend")
        self.assertEqual(num2words(500000, lang="nl"), "vijfhonderdduizend")
        self.assertEqual(
            num2words(654321, lang="nl"),
            "zeshonderdvierenvijftigduizend driehonderdeenentwintig",
        )
        self.assertEqual(
            num2words(999999, lang="nl"),
            "negenhonderdnegenennegentigduizend negenhonderdnegenennegentig",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="nl"), "een miljoen")
        self.assertEqual(num2words(1000001, lang="nl"), "een miljoen één")
        self.assertEqual(
            num2words(1111111, lang="nl"), "een miljoen honderdelfduizend honderdelf"
        )
        self.assertEqual(
            num2words(1234567, lang="nl"),
            "een miljoen tweehonderdvierendertigduizend vijfhonderdzevenenzestig",
        )
        self.assertEqual(num2words(2000000, lang="nl"), "twee miljoen")
        self.assertEqual(num2words(5000000, lang="nl"), "vijf miljoen")
        self.assertEqual(
            num2words(9999999, lang="nl"),
            "negen miljoen negenhonderdnegenennegentigduizend negenhonderdnegenennegentig",
        )
        self.assertEqual(num2words(10000000, lang="nl"), "tien miljoen")
        self.assertEqual(
            num2words(12345678, lang="nl"),
            "twaalf miljoen driehonderdvijfenveertigduizend zeshonderdachtenzeventig",
        )
        self.assertEqual(
            num2words(99999999, lang="nl"),
            "negenennegentig miljoen negenhonderdnegenennegentigduizend negenhonderdnegenennegentig",
        )
        self.assertEqual(num2words(100000000, lang="nl"), "honderd miljoen")
        self.assertEqual(
            num2words(123456789, lang="nl"),
            "honderddrieëntwintig miljoen vierhonderdzesenvijftigduizend zevenhonderdnegenentachtig",
        )
        self.assertEqual(
            num2words(999999999, lang="nl"),
            "negenhonderdnegenennegentig miljoen negenhonderdnegenennegentigduizend negenhonderdnegenennegentig",
        )
        self.assertEqual(num2words(1000000000, lang="nl"), "een miljard")
        self.assertEqual(
            num2words(1234567890, lang="nl"),
            "een miljard tweehonderdvierendertig miljoen vijfhonderdzevenenzestigduizend achthonderdnegentig",
        )
        self.assertEqual(
            num2words(9999999999, lang="nl"),
            "negen miljard negenhonderdnegenennegentig miljoen negenhonderdnegenennegentigduizend negenhonderdnegenennegentig",
        )
        self.assertEqual(num2words(10000000000, lang="nl"), "tien miljard")
        self.assertEqual(
            num2words(99999999999, lang="nl"),
            "negenennegentig miljard negenhonderdnegenennegentig miljoen negenhonderdnegenennegentigduizend negenhonderdnegenennegentig",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="nl"), "min één")
        self.assertEqual(num2words(-2, lang="nl"), "min twee")
        self.assertEqual(num2words(-5, lang="nl"), "min vijf")
        self.assertEqual(num2words(-10, lang="nl"), "min tien")
        self.assertEqual(num2words(-11, lang="nl"), "min elf")
        self.assertEqual(num2words(-20, lang="nl"), "min twintig")
        self.assertEqual(num2words(-50, lang="nl"), "min vijftig")
        self.assertEqual(num2words(-99, lang="nl"), "min negenennegentig")
        self.assertEqual(num2words(-100, lang="nl"), "min honderd")
        self.assertEqual(num2words(-101, lang="nl"), "min honderdéén")
        self.assertEqual(num2words(-200, lang="nl"), "min tweehonderd")
        self.assertEqual(num2words(-999, lang="nl"), "min negenhonderdnegenennegentig")
        self.assertEqual(num2words(-1000, lang="nl"), "min duizend")
        self.assertEqual(num2words(-1001, lang="nl"), "min duizend één")
        self.assertEqual(num2words(-10000, lang="nl"), "min tienduizend")
        self.assertEqual(num2words(-100000, lang="nl"), "min honderdduizend")
        self.assertEqual(num2words(-1000000, lang="nl"), "min een miljoen")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="nl"), "nul komma één")
        self.assertEqual(num2words(0.5, lang="nl"), "nul komma vijf")
        self.assertEqual(num2words(0.9, lang="nl"), "nul komma negen")
        self.assertEqual(num2words(1.1, lang="nl"), "één komma één")
        self.assertEqual(num2words(1.5, lang="nl"), "één komma vijf")
        self.assertEqual(num2words(2.5, lang="nl"), "twee komma vijf")
        self.assertEqual(num2words(3.14, lang="nl"), "drie komma één vier")
        self.assertEqual(num2words(10.5, lang="nl"), "tien komma vijf")
        self.assertEqual(num2words(11.11, lang="nl"), "elf komma één één")
        self.assertEqual(num2words(20.2, lang="nl"), "twintig komma twee")
        self.assertEqual(
            num2words(99.99, lang="nl"), "negenennegentig komma negen negen"
        )
        self.assertEqual(num2words(100.01, lang="nl"), "honderd komma nul één")
        self.assertEqual(num2words(100.5, lang="nl"), "honderd komma vijf")
        self.assertEqual(
            num2words(123.45, lang="nl"), "honderddrieëntwintig komma vier vijf"
        )
        self.assertEqual(num2words(1000.5, lang="nl"), "duizend komma vijf")
        self.assertEqual(
            num2words(1234.56, lang="nl"),
            "duizend tweehonderdvierendertig komma vijf zes",
        )
        self.assertEqual(num2words(10000.01, lang="nl"), "tienduizend komma nul één")
        self.assertEqual(num2words(-0.5, lang="nl"), "min nul komma vijf")
        self.assertEqual(num2words(-1.5, lang="nl"), "min één komma vijf")
        self.assertEqual(num2words(-10.5, lang="nl"), "min tien komma vijf")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="nl", ordinal=True), "eerste")
        self.assertEqual(num2words(2, lang="nl", ordinal=True), "tweede")
        self.assertEqual(num2words(3, lang="nl", ordinal=True), "derde")
        self.assertEqual(num2words(4, lang="nl", ordinal=True), "vierde")
        self.assertEqual(num2words(5, lang="nl", ordinal=True), "vijfde")
        self.assertEqual(num2words(6, lang="nl", ordinal=True), "zesde")
        self.assertEqual(num2words(7, lang="nl", ordinal=True), "zevende")
        self.assertEqual(num2words(8, lang="nl", ordinal=True), "achtste")
        self.assertEqual(num2words(9, lang="nl", ordinal=True), "negende")
        self.assertEqual(num2words(10, lang="nl", ordinal=True), "tiende")
        self.assertEqual(num2words(11, lang="nl", ordinal=True), "elfde")
        self.assertEqual(num2words(12, lang="nl", ordinal=True), "twaalfde")
        self.assertEqual(num2words(13, lang="nl", ordinal=True), "dertiende")
        self.assertEqual(num2words(14, lang="nl", ordinal=True), "veertiende")
        self.assertEqual(num2words(15, lang="nl", ordinal=True), "vijftiende")
        self.assertEqual(num2words(16, lang="nl", ordinal=True), "zestiende")
        self.assertEqual(num2words(17, lang="nl", ordinal=True), "zeventiende")
        self.assertEqual(num2words(18, lang="nl", ordinal=True), "achttiende")
        self.assertEqual(num2words(19, lang="nl", ordinal=True), "negentiende")
        self.assertEqual(num2words(20, lang="nl", ordinal=True), "twintigste")
        self.assertEqual(num2words(21, lang="nl", ordinal=True), "eenentwintigste")
        self.assertEqual(num2words(22, lang="nl", ordinal=True), "tweeëntwintigste")
        self.assertEqual(num2words(25, lang="nl", ordinal=True), "vijfentwintigste")
        self.assertEqual(num2words(30, lang="nl", ordinal=True), "dertigste")
        self.assertEqual(num2words(40, lang="nl", ordinal=True), "veertigste")
        self.assertEqual(num2words(50, lang="nl", ordinal=True), "vijftigste")
        self.assertEqual(num2words(60, lang="nl", ordinal=True), "zestigste")
        self.assertEqual(num2words(70, lang="nl", ordinal=True), "zeventigste")
        self.assertEqual(num2words(80, lang="nl", ordinal=True), "tachtigste")
        self.assertEqual(num2words(90, lang="nl", ordinal=True), "negentigste")
        self.assertEqual(num2words(100, lang="nl", ordinal=True), "honderdste")
        self.assertEqual(num2words(101, lang="nl", ordinal=True), "honderdeerste")
        self.assertEqual(num2words(200, lang="nl", ordinal=True), "tweehonderdste")
        self.assertEqual(num2words(500, lang="nl", ordinal=True), "vijfhonderdste")
        self.assertEqual(num2words(1000, lang="nl", ordinal=True), "duizendste")
        self.assertEqual(num2words(1001, lang="nl", ordinal=True), "duizend eerste")
        self.assertEqual(num2words(10000, lang="nl", ordinal=True), "tienduizendste")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="nl", to="currency", currency="EUR"), "nul euro"
        )
        self.assertEqual(
            num2words(0.01, lang="nl", to="currency", currency="EUR"),
            "nul euro en één cent",
        )
        self.assertEqual(
            num2words(0.5, lang="nl", to="currency", currency="EUR"),
            "nul euro en vijftig cent",
        )
        self.assertEqual(
            num2words(1, lang="nl", to="currency", currency="EUR"), "één euro"
        )
        self.assertEqual(
            num2words(1.5, lang="nl", to="currency", currency="EUR"),
            "één euro en vijftig cent",
        )
        self.assertEqual(
            num2words(0, lang="nl", to="currency", currency="GBP"), "nul pond"
        )
        self.assertEqual(
            num2words(0.01, lang="nl", to="currency", currency="GBP"),
            "nul pond en één penny",
        )
        self.assertEqual(
            num2words(0.5, lang="nl", to="currency", currency="GBP"),
            "nul pond en vijftig penny",
        )
        self.assertEqual(
            num2words(1, lang="nl", to="currency", currency="GBP"), "één pond"
        )
        self.assertEqual(
            num2words(1.5, lang="nl", to="currency", currency="GBP"),
            "één pond en vijftig penny",
        )
        self.assertEqual(
            num2words(0, lang="nl", to="currency", currency="USD"), "nul dollar"
        )
        self.assertEqual(
            num2words(0.01, lang="nl", to="currency", currency="USD"),
            "nul dollar en één cent",
        )
        self.assertEqual(
            num2words(0.5, lang="nl", to="currency", currency="USD"),
            "nul dollar en vijftig cent",
        )
        self.assertEqual(
            num2words(1, lang="nl", to="currency", currency="USD"), "één dollar"
        )
        self.assertEqual(
            num2words(1.5, lang="nl", to="currency", currency="USD"),
            "één dollar en vijftig cent",
        )
        self.assertEqual(
            num2words(0, lang="nl", to="currency", currency="CNY"), "nul yuan"
        )
        self.assertEqual(
            num2words(0.01, lang="nl", to="currency", currency="CNY"),
            "nul yuan en één jiao",
        )
        self.assertEqual(
            num2words(0.5, lang="nl", to="currency", currency="CNY"),
            "nul yuan en vijftig jiao",
        )
        self.assertEqual(
            num2words(1, lang="nl", to="currency", currency="CNY"), "één yuan"
        )
        self.assertEqual(
            num2words(1.5, lang="nl", to="currency", currency="CNY"),
            "één yuan en vijftig jiao",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="nl", to="year"), "duizend")
        self.assertEqual(num2words(1066, lang="nl", to="year"), "duizend zesenzestig")
        self.assertEqual(
            num2words(1492, lang="nl", to="year"), "veertienhonderdtweeënnegentig"
        )
        self.assertEqual(
            num2words(1776, lang="nl", to="year"), "zeventienhonderdzesenzeventig"
        )
        self.assertEqual(num2words(1800, lang="nl", to="year"), "achttienhonderd")
        self.assertEqual(num2words(1900, lang="nl", to="year"), "negentienhonderd")
        self.assertEqual(
            num2words(1984, lang="nl", to="year"), "negentienhonderdvierentachtig"
        )
        self.assertEqual(
            num2words(1999, lang="nl", to="year"), "negentienhonderdnegenennegentig"
        )
        self.assertEqual(num2words(2000, lang="nl", to="year"), "tweeduizend")
        self.assertEqual(num2words(2001, lang="nl", to="year"), "tweeduizend één")
        self.assertEqual(num2words(2010, lang="nl", to="year"), "tweeduizend tien")
        self.assertEqual(num2words(2020, lang="nl", to="year"), "tweeduizend twintig")
        self.assertEqual(
            num2words(2024, lang="nl", to="year"), "tweeduizend vierentwintig"
        )
        self.assertEqual(num2words(2100, lang="nl", to="year"), "eenentwintighonderd")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="nl"), "nul")
        self.assertEqual(num2words("1", lang="nl"), "één")
        self.assertEqual(num2words("10", lang="nl"), "tien")
        self.assertEqual(num2words("100", lang="nl"), "honderd")
        self.assertEqual(num2words("1000", lang="nl"), "duizend")
        self.assertEqual(num2words("10000", lang="nl"), "tienduizend")
        self.assertEqual(num2words("100000", lang="nl"), "honderdduizend")
        self.assertEqual(num2words("1000000", lang="nl"), "een miljoen")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="nl"), "nul")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="nl"), num2words("100", lang="nl"))
        self.assertEqual(num2words(1000, lang="nl"), num2words("1000", lang="nl"))


def test_nl_year_no_spaces_for_2100_plus():
    # Regression for num2words2#60 (ports savoirfairelinux/num2words#519).
    from num2words2 import num2words

    assert num2words(2100, lang="nl", to="year") == "eenentwintighonderd"
    assert num2words(2150, lang="nl", to="year") == "eenentwintighonderdvijftig"
    assert num2words(2500, lang="nl", to="year") == "vijfentwintighonderd"
    # 2000-2099 keeps tweeduizend form
    assert num2words(2024, lang="nl", to="year") == "tweeduizend vierentwintig"
