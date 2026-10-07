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


class Num2WordsBSTest(TestCase):
    """Comprehensive test cases for Bosnian language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="bs"), "nula")
        self.assertEqual(num2words(1, lang="bs"), "jedan")
        self.assertEqual(num2words(2, lang="bs"), "dva")
        self.assertEqual(num2words(3, lang="bs"), "tri")
        self.assertEqual(num2words(4, lang="bs"), "četiri")
        self.assertEqual(num2words(5, lang="bs"), "pet")
        self.assertEqual(num2words(6, lang="bs"), "šest")
        self.assertEqual(num2words(7, lang="bs"), "sedam")
        self.assertEqual(num2words(8, lang="bs"), "osam")
        self.assertEqual(num2words(9, lang="bs"), "devet")
        self.assertEqual(num2words(10, lang="bs"), "deset")
        self.assertEqual(num2words(11, lang="bs"), "jedanaest")
        self.assertEqual(num2words(12, lang="bs"), "dvanaest")
        self.assertEqual(num2words(13, lang="bs"), "trinaest")
        self.assertEqual(num2words(14, lang="bs"), "četrnaest")
        self.assertEqual(num2words(15, lang="bs"), "petnaest")
        self.assertEqual(num2words(16, lang="bs"), "šesnaest")
        self.assertEqual(num2words(17, lang="bs"), "sedamnaest")
        self.assertEqual(num2words(18, lang="bs"), "osamnaest")
        self.assertEqual(num2words(19, lang="bs"), "devetnaest")
        self.assertEqual(num2words(20, lang="bs"), "dvadeset")
        self.assertEqual(num2words(21, lang="bs"), "dvadeset jedan")
        self.assertEqual(num2words(22, lang="bs"), "dvadeset dva")
        self.assertEqual(num2words(23, lang="bs"), "dvadeset tri")
        self.assertEqual(num2words(24, lang="bs"), "dvadeset četiri")
        self.assertEqual(num2words(25, lang="bs"), "dvadeset pet")
        self.assertEqual(num2words(26, lang="bs"), "dvadeset šest")
        self.assertEqual(num2words(27, lang="bs"), "dvadeset sedam")
        self.assertEqual(num2words(28, lang="bs"), "dvadeset osam")
        self.assertEqual(num2words(29, lang="bs"), "dvadeset devet")
        self.assertEqual(num2words(30, lang="bs"), "trideset")
        self.assertEqual(num2words(31, lang="bs"), "trideset jedan")
        self.assertEqual(num2words(35, lang="bs"), "trideset pet")
        self.assertEqual(num2words(40, lang="bs"), "četrdeset")
        self.assertEqual(num2words(45, lang="bs"), "četrdeset pet")
        self.assertEqual(num2words(50, lang="bs"), "pedeset")
        self.assertEqual(num2words(55, lang="bs"), "pedeset pet")
        self.assertEqual(num2words(60, lang="bs"), "šezdeset")
        self.assertEqual(num2words(65, lang="bs"), "šezdeset pet")
        self.assertEqual(num2words(70, lang="bs"), "sedamdeset")
        self.assertEqual(num2words(75, lang="bs"), "sedamdeset pet")
        self.assertEqual(num2words(80, lang="bs"), "osamdeset")
        self.assertEqual(num2words(85, lang="bs"), "osamdeset pet")
        self.assertEqual(num2words(90, lang="bs"), "devedeset")
        self.assertEqual(num2words(95, lang="bs"), "devedeset pet")
        self.assertEqual(num2words(99, lang="bs"), "devedeset devet")
        self.assertEqual(num2words(100, lang="bs"), "sto")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="bs"), "sto jedan")
        self.assertEqual(num2words(110, lang="bs"), "sto deset")
        self.assertEqual(num2words(111, lang="bs"), "sto jedanaest")
        self.assertEqual(num2words(120, lang="bs"), "sto dvadeset")
        self.assertEqual(num2words(125, lang="bs"), "sto dvadeset pet")
        self.assertEqual(num2words(150, lang="bs"), "sto pedeset")
        self.assertEqual(num2words(175, lang="bs"), "sto sedamdeset pet")
        self.assertEqual(num2words(199, lang="bs"), "sto devedeset devet")
        self.assertEqual(num2words(200, lang="bs"), "dvjesto")
        self.assertEqual(num2words(201, lang="bs"), "dvjesto jedan")
        self.assertEqual(num2words(210, lang="bs"), "dvjesto deset")
        self.assertEqual(num2words(220, lang="bs"), "dvjesto dvadeset")
        self.assertEqual(num2words(250, lang="bs"), "dvjesto pedeset")
        self.assertEqual(num2words(299, lang="bs"), "dvjesto devedeset devet")
        self.assertEqual(num2words(300, lang="bs"), "tristo")
        self.assertEqual(num2words(333, lang="bs"), "tristo trideset tri")
        self.assertEqual(num2words(400, lang="bs"), "četiristo")
        self.assertEqual(num2words(444, lang="bs"), "četiristo četrdeset četiri")
        self.assertEqual(num2words(500, lang="bs"), "petsto")
        self.assertEqual(num2words(555, lang="bs"), "petsto pedeset pet")
        self.assertEqual(num2words(600, lang="bs"), "šeststo")
        self.assertEqual(num2words(666, lang="bs"), "šeststo šezdeset šest")
        self.assertEqual(num2words(700, lang="bs"), "sedamsto")
        self.assertEqual(num2words(777, lang="bs"), "sedamsto sedamdeset sedam")
        self.assertEqual(num2words(800, lang="bs"), "osamsto")
        self.assertEqual(num2words(888, lang="bs"), "osamsto osamdeset osam")
        self.assertEqual(num2words(900, lang="bs"), "devetsto")
        self.assertEqual(num2words(999, lang="bs"), "devetsto devedeset devet")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="bs"), "hiljada")
        self.assertEqual(num2words(1001, lang="bs"), "hiljada jedan")
        self.assertEqual(num2words(1010, lang="bs"), "hiljada deset")
        self.assertEqual(num2words(1100, lang="bs"), "hiljada sto")
        self.assertEqual(num2words(1111, lang="bs"), "hiljada sto jedanaest")
        self.assertEqual(num2words(1234, lang="bs"), "hiljada dvjesto trideset četiri")
        self.assertEqual(num2words(1500, lang="bs"), "hiljada petsto")
        self.assertEqual(num2words(1999, lang="bs"), "hiljada devetsto devedeset devet")
        self.assertEqual(num2words(2000, lang="bs"), "dva hiljade")
        self.assertEqual(num2words(2001, lang="bs"), "dva hiljade jedan")
        self.assertEqual(num2words(2020, lang="bs"), "dva hiljade dvadeset")
        self.assertEqual(num2words(2222, lang="bs"), "dva hiljade dvjesto dvadeset dva")
        self.assertEqual(num2words(3000, lang="bs"), "tri hiljade")
        self.assertEqual(num2words(3333, lang="bs"), "tri hiljade tristo trideset tri")
        self.assertEqual(num2words(4000, lang="bs"), "četiri hiljade")
        self.assertEqual(
            num2words(4444, lang="bs"), "četiri hiljade četiristo četrdeset četiri"
        )
        self.assertEqual(num2words(5000, lang="bs"), "pet hiljada")
        self.assertEqual(num2words(5555, lang="bs"), "pet hiljada petsto pedeset pet")
        self.assertEqual(num2words(6000, lang="bs"), "šest hiljada")
        self.assertEqual(
            num2words(6666, lang="bs"), "šest hiljada šeststo šezdeset šest"
        )
        self.assertEqual(num2words(7000, lang="bs"), "sedam hiljada")
        self.assertEqual(
            num2words(7777, lang="bs"), "sedam hiljada sedamsto sedamdeset sedam"
        )
        self.assertEqual(num2words(8000, lang="bs"), "osam hiljada")
        self.assertEqual(
            num2words(8888, lang="bs"), "osam hiljada osamsto osamdeset osam"
        )
        self.assertEqual(num2words(9000, lang="bs"), "devet hiljada")
        self.assertEqual(
            num2words(9999, lang="bs"), "devet hiljada devetsto devedeset devet"
        )
        self.assertEqual(num2words(10000, lang="bs"), "deset hiljada")
        self.assertEqual(num2words(10001, lang="bs"), "deset hiljada jedan")
        self.assertEqual(num2words(11111, lang="bs"), "jedanaest hiljada sto jedanaest")
        self.assertEqual(
            num2words(12345, lang="bs"), "dvanaest hiljada tristo četrdeset pet"
        )
        self.assertEqual(num2words(20000, lang="bs"), "dvadeset hiljada")
        self.assertEqual(num2words(50000, lang="bs"), "pedeset hiljada")
        self.assertEqual(
            num2words(99999, lang="bs"),
            "devedeset devet hiljada devetsto devedeset devet",
        )
        self.assertEqual(num2words(100000, lang="bs"), "sto hiljada")
        self.assertEqual(
            num2words(123456, lang="bs"),
            "sto dvadeset tri hiljada četiristo pedeset šest",
        )
        self.assertEqual(num2words(200000, lang="bs"), "dvjesto hiljada")
        self.assertEqual(num2words(500000, lang="bs"), "petsto hiljada")
        self.assertEqual(
            num2words(654321, lang="bs"),
            "šeststo pedeset četiri hiljada tristo dvadeset jedan",
        )
        self.assertEqual(
            num2words(999999, lang="bs"),
            "devetsto devedeset devet hiljada devetsto devedeset devet",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="bs"), "milion")
        self.assertEqual(num2words(1000001, lang="bs"), "milion jedan")
        self.assertEqual(
            num2words(1111111, lang="bs"), "milion sto jedanaest hiljada sto jedanaest"
        )
        self.assertEqual(
            num2words(1234567, lang="bs"),
            "milion dvjesto trideset četiri hiljada petsto šezdeset sedam",
        )
        self.assertEqual(num2words(2000000, lang="bs"), "dva miliona")
        self.assertEqual(num2words(5000000, lang="bs"), "pet miliona")
        self.assertEqual(
            num2words(9999999, lang="bs"),
            "devet miliona devetsto devedeset devet hiljada devetsto devedeset devet",
        )
        self.assertEqual(num2words(10000000, lang="bs"), "deset miliona")
        self.assertEqual(
            num2words(12345678, lang="bs"),
            "dvanaest miliona tristo četrdeset pet hiljada šeststo sedamdeset osam",
        )
        self.assertEqual(
            num2words(99999999, lang="bs"),
            "devedeset devet miliona devetsto devedeset devet hiljada devetsto devedeset devet",
        )
        self.assertEqual(num2words(100000000, lang="bs"), "sto miliona")
        self.assertEqual(
            num2words(123456789, lang="bs"),
            "sto dvadeset tri miliona četiristo pedeset šest hiljada sedamsto osamdeset devet",
        )
        self.assertEqual(
            num2words(999999999, lang="bs"),
            "devetsto devedeset devet miliona devetsto devedeset devet hiljada devetsto devedeset devet",
        )
        # gladiaio/num2words2#147: words (or OverflowError), not digits.
        self.assertEqual(
            num2words(1000000000, lang="bs"),
            "milijarda",
        )
        self.assertEqual(
            num2words(1234567890, lang="bs"),
            "milijarda dvjesto trideset četiri miliona petsto šezdeset sedam hiljada osamsto devedeset",
        )
        self.assertEqual(
            num2words(9999999999, lang="bs"),
            "devet milijardi devetsto devedeset devet miliona devetsto devedeset devet hiljada devetsto devedeset devet",
        )
        self.assertEqual(
            num2words(10000000000, lang="bs"),
            "deset milijardi",
        )
        self.assertEqual(
            num2words(99999999999, lang="bs"),
            "devedeset devet milijardi devetsto devedeset devet miliona devetsto devedeset devet hiljada devetsto devedeset devet",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="bs"), "minus jedan")
        self.assertEqual(num2words(-2, lang="bs"), "minus dva")
        self.assertEqual(num2words(-5, lang="bs"), "minus pet")
        self.assertEqual(num2words(-10, lang="bs"), "minus deset")
        self.assertEqual(num2words(-11, lang="bs"), "minus jedanaest")
        self.assertEqual(num2words(-20, lang="bs"), "minus dvadeset")
        self.assertEqual(num2words(-50, lang="bs"), "minus pedeset")
        self.assertEqual(num2words(-99, lang="bs"), "minus devedeset devet")
        self.assertEqual(num2words(-100, lang="bs"), "minus sto")
        self.assertEqual(num2words(-101, lang="bs"), "minus sto jedan")
        self.assertEqual(num2words(-200, lang="bs"), "minus dvjesto")
        self.assertEqual(num2words(-999, lang="bs"), "minus devetsto devedeset devet")
        self.assertEqual(num2words(-1000, lang="bs"), "minus hiljada")
        self.assertEqual(num2words(-1001, lang="bs"), "minus hiljada jedan")
        self.assertEqual(num2words(-10000, lang="bs"), "minus deset hiljada")
        self.assertEqual(num2words(-100000, lang="bs"), "minus sto hiljada")
        self.assertEqual(num2words(-1000000, lang="bs"), "minus milion")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="bs"), "nula zarez jedan")
        self.assertEqual(num2words(0.5, lang="bs"), "nula zarez pet")
        self.assertEqual(num2words(0.9, lang="bs"), "nula zarez devet")
        self.assertEqual(num2words(1.1, lang="bs"), "jedan zarez jedan")
        self.assertEqual(num2words(1.5, lang="bs"), "jedan zarez pet")
        self.assertEqual(num2words(2.5, lang="bs"), "dva zarez pet")
        self.assertEqual(num2words(3.14, lang="bs"), "tri zarez jedan četiri")
        self.assertEqual(num2words(10.5, lang="bs"), "deset zarez pet")
        self.assertEqual(num2words(11.11, lang="bs"), "jedanaest zarez jedan jedan")
        self.assertEqual(num2words(20.2, lang="bs"), "dvadeset zarez dva")
        self.assertEqual(
            num2words(99.99, lang="bs"), "devedeset devet zarez devet devet"
        )
        self.assertEqual(num2words(100.01, lang="bs"), "sto zarez nula jedan")
        self.assertEqual(num2words(100.5, lang="bs"), "sto zarez pet")
        self.assertEqual(
            num2words(123.45, lang="bs"), "sto dvadeset tri zarez četiri pet"
        )
        self.assertEqual(num2words(1000.5, lang="bs"), "hiljada zarez pet")
        self.assertEqual(
            num2words(1234.56, lang="bs"),
            "hiljada dvjesto trideset četiri zarez pet šest",
        )
        self.assertEqual(
            num2words(10000.01, lang="bs"), "deset hiljada zarez nula jedan"
        )
        self.assertEqual(num2words(-0.5, lang="bs"), "minus nula zarez pet")
        self.assertEqual(num2words(-1.5, lang="bs"), "minus jedan zarez pet")
        self.assertEqual(num2words(-10.5, lang="bs"), "minus deset zarez pet")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="bs", ordinal=True), "prvi")
        self.assertEqual(num2words(2, lang="bs", ordinal=True), "drugi")
        self.assertEqual(num2words(3, lang="bs", ordinal=True), "treći")
        self.assertEqual(num2words(4, lang="bs", ordinal=True), "četvrti")
        self.assertEqual(num2words(5, lang="bs", ordinal=True), "peti")
        self.assertEqual(num2words(6, lang="bs", ordinal=True), "šesti")
        self.assertEqual(num2words(7, lang="bs", ordinal=True), "sedmi")
        self.assertEqual(num2words(8, lang="bs", ordinal=True), "osmi")
        self.assertEqual(num2words(9, lang="bs", ordinal=True), "deveti")
        self.assertEqual(num2words(10, lang="bs", ordinal=True), "deseti")
        self.assertEqual(num2words(11, lang="bs", ordinal=True), "jedanaesti")
        self.assertEqual(num2words(12, lang="bs", ordinal=True), "dvanaesti")
        self.assertEqual(num2words(13, lang="bs", ordinal=True), "trinaesti")
        self.assertEqual(num2words(14, lang="bs", ordinal=True), "četrnaesti")
        self.assertEqual(num2words(15, lang="bs", ordinal=True), "petnaesti")
        self.assertEqual(num2words(16, lang="bs", ordinal=True), "šesnaesti")
        self.assertEqual(num2words(17, lang="bs", ordinal=True), "sedamnaesti")
        self.assertEqual(num2words(18, lang="bs", ordinal=True), "osamnaesti")
        self.assertEqual(num2words(19, lang="bs", ordinal=True), "devetnaesti")
        self.assertEqual(num2words(20, lang="bs", ordinal=True), "dvadeseti")
        self.assertEqual(num2words(21, lang="bs", ordinal=True), "dvadeset prvi")
        self.assertEqual(num2words(22, lang="bs", ordinal=True), "dvadeset drugi")
        self.assertEqual(num2words(25, lang="bs", ordinal=True), "dvadeset peti")
        self.assertEqual(num2words(30, lang="bs", ordinal=True), "trideseti")
        self.assertEqual(num2words(40, lang="bs", ordinal=True), "četrdeseti")
        self.assertEqual(num2words(50, lang="bs", ordinal=True), "pedeseti")
        self.assertEqual(num2words(60, lang="bs", ordinal=True), "šezdeseti")
        self.assertEqual(num2words(70, lang="bs", ordinal=True), "sedamdeseti")
        self.assertEqual(num2words(80, lang="bs", ordinal=True), "osamdeseti")
        self.assertEqual(num2words(90, lang="bs", ordinal=True), "devedeseti")
        self.assertEqual(num2words(100, lang="bs", ordinal=True), "stoti")
        self.assertEqual(num2words(101, lang="bs", ordinal=True), "sto prvi")
        self.assertEqual(num2words(200, lang="bs", ordinal=True), "dvjestoti")
        self.assertEqual(num2words(500, lang="bs", ordinal=True), "petstoti")
        self.assertEqual(num2words(1000, lang="bs", ordinal=True), "hiljaditi")
        self.assertEqual(num2words(1001, lang="bs", ordinal=True), "hiljada prvi")
        self.assertEqual(num2words(10000, lang="bs", ordinal=True), "deset hiljada.")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="bs", to="currency", currency="BAM"), "nula maraka"
        )
        self.assertEqual(
            num2words(0.01, lang="bs", to="currency", currency="BAM"),
            "nula maraka jedan feninga",
        )
        self.assertEqual(
            num2words(0.5, lang="bs", to="currency", currency="BAM"),
            "nula maraka pedeset feninga",
        )
        self.assertEqual(
            num2words(1, lang="bs", to="currency", currency="BAM"), "jedan marka"
        )
        self.assertEqual(
            num2words(1.5, lang="bs", to="currency", currency="BAM"),
            "jedan marka pedeset feninga",
        )
        self.assertEqual(
            num2words(0, lang="bs", to="currency", currency="EUR"), "nula eura"
        )
        self.assertEqual(
            num2words(0.01, lang="bs", to="currency", currency="EUR"),
            "nula eura jedan cent",
        )
        self.assertEqual(
            num2words(0.5, lang="bs", to="currency", currency="EUR"),
            "nula eura pedeset centi",
        )
        self.assertEqual(
            num2words(1, lang="bs", to="currency", currency="EUR"), "jedan euro"
        )
        self.assertEqual(
            num2words(1.5, lang="bs", to="currency", currency="EUR"),
            "jedan euro pedeset centi",
        )
        self.assertEqual(
            num2words(0, lang="bs", to="currency", currency="USD"), "nula dolara"
        )
        self.assertEqual(
            num2words(0.01, lang="bs", to="currency", currency="USD"),
            "nula dolara jedan cent",
        )
        self.assertEqual(
            num2words(0.5, lang="bs", to="currency", currency="USD"),
            "nula dolara pedeset centi",
        )
        self.assertEqual(
            num2words(1, lang="bs", to="currency", currency="USD"), "jedan dolar"
        )
        self.assertEqual(
            num2words(1.5, lang="bs", to="currency", currency="USD"),
            "jedan dolar pedeset centi",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="bs", to="year"), "hiljada")
        self.assertEqual(num2words(1066, lang="bs", to="year"), "hiljada šezdeset šest")
        self.assertEqual(
            num2words(1492, lang="bs", to="year"), "hiljada četiristo devedeset dva"
        )
        self.assertEqual(
            num2words(1776, lang="bs", to="year"), "hiljada sedamsto sedamdeset šest"
        )
        self.assertEqual(num2words(1800, lang="bs", to="year"), "hiljada osamsto")
        self.assertEqual(num2words(1900, lang="bs", to="year"), "hiljada devetsto")
        self.assertEqual(
            num2words(1984, lang="bs", to="year"), "hiljada devetsto osamdeset četiri"
        )
        self.assertEqual(
            num2words(1999, lang="bs", to="year"), "hiljada devetsto devedeset devet"
        )
        self.assertEqual(num2words(2000, lang="bs", to="year"), "dva hiljade")
        self.assertEqual(num2words(2001, lang="bs", to="year"), "dva hiljade jedan")
        self.assertEqual(num2words(2010, lang="bs", to="year"), "dva hiljade deset")
        self.assertEqual(num2words(2020, lang="bs", to="year"), "dva hiljade dvadeset")
        self.assertEqual(
            num2words(2024, lang="bs", to="year"), "dva hiljade dvadeset četiri"
        )
        self.assertEqual(num2words(2100, lang="bs", to="year"), "dva hiljade sto")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="bs"), "nula")
        self.assertEqual(num2words("1", lang="bs"), "jedan")
        self.assertEqual(num2words("10", lang="bs"), "deset")
        self.assertEqual(num2words("100", lang="bs"), "sto")
        self.assertEqual(num2words("1000", lang="bs"), "hiljada")
        self.assertEqual(num2words("10000", lang="bs"), "deset hiljada")
        self.assertEqual(num2words("100000", lang="bs"), "sto hiljada")
        self.assertEqual(num2words("1000000", lang="bs"), "milion")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="bs"), "nula")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="bs"), num2words("100", lang="bs"))
        self.assertEqual(num2words(1000, lang="bs"), num2words("1000", lang="bs"))


def test_teens_issue_216():
    # gladiaio/num2words2#216: Python built 11-19 as ones + "aest".
    from num2words2 import num2words

    want = [
        "jedanaest",
        "dvanaest",
        "trinaest",
        "četrnaest",
        "petnaest",
        "šesnaest",
        "sedamnaest",
        "osamnaest",
        "devetnaest",
    ]
    assert [num2words(i, lang="bs") for i in range(11, 20)] == want
    assert num2words(12, lang="bs", to="year") == "dvanaest"
    assert num2words(112, lang="bs") == "sto dvanaest"


def test_scales_above_million_issue_147():
    """Words up to the scale table, then OverflowError -- never digits."""
    assert num2words(10**12, lang="bs") == "bilion"
    assert num2words(10**15, lang="bs") == "bilijarda"
    assert num2words(10**18, lang="bs") == "trilion"
    assert num2words(10**21, lang="bs") == "trilijarda"
    assert num2words(2 * 10**21, lang="bs") == "dva trilijarde"
    with pytest.raises(OverflowError):
        num2words(10**24, lang="bs")
