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

from __future__ import unicode_literals

from unittest import TestCase

from num2words2 import num2words


class Num2WordsHRTest(TestCase):
    def test_basic_numbers(self):
        self.assertEqual(num2words(0, lang="hr"), "nula")
        self.assertEqual(num2words(1, lang="hr"), "jedan")
        self.assertEqual(num2words(2, lang="hr"), "dva")
        self.assertEqual(num2words(3, lang="hr"), "tri")
        self.assertEqual(num2words(4, lang="hr"), "četiri")
        self.assertEqual(num2words(5, lang="hr"), "pet")
        self.assertEqual(num2words(6, lang="hr"), "šest")
        self.assertEqual(num2words(7, lang="hr"), "sedam")
        self.assertEqual(num2words(8, lang="hr"), "osam")
        self.assertEqual(num2words(9, lang="hr"), "devet")
        self.assertEqual(num2words(10, lang="hr"), "deset")

    def test_teens(self):
        self.assertEqual(num2words(11, lang="hr"), "jedanaest")
        self.assertEqual(num2words(12, lang="hr"), "dvanaest")
        self.assertEqual(num2words(13, lang="hr"), "trinaest")
        self.assertEqual(num2words(14, lang="hr"), "četrnaest")
        self.assertEqual(num2words(15, lang="hr"), "petnaest")
        self.assertEqual(num2words(16, lang="hr"), "šesnaest")
        self.assertEqual(num2words(17, lang="hr"), "sedamnaest")
        self.assertEqual(num2words(18, lang="hr"), "osamnaest")
        self.assertEqual(num2words(19, lang="hr"), "devetnaest")

    def test_tens(self):
        self.assertEqual(num2words(20, lang="hr"), "dvadeset")
        self.assertEqual(num2words(21, lang="hr"), "dvadeset jedan")
        self.assertEqual(num2words(30, lang="hr"), "trideset")
        self.assertEqual(num2words(40, lang="hr"), "četrdeset")
        self.assertEqual(num2words(50, lang="hr"), "pedeset")
        self.assertEqual(num2words(60, lang="hr"), "šezdeset")
        self.assertEqual(num2words(70, lang="hr"), "sedamdeset")
        self.assertEqual(num2words(80, lang="hr"), "osamdeset")
        self.assertEqual(num2words(90, lang="hr"), "devedeset")
        self.assertEqual(num2words(99, lang="hr"), "devedeset devet")

    def test_hundreds(self):
        self.assertEqual(num2words(100, lang="hr"), "sto")
        self.assertEqual(num2words(101, lang="hr"), "sto jedan")
        self.assertEqual(num2words(110, lang="hr"), "sto deset")
        self.assertEqual(num2words(115, lang="hr"), "sto petnaest")
        self.assertEqual(num2words(123, lang="hr"), "sto dvadeset tri")
        self.assertEqual(num2words(200, lang="hr"), "dvjesto")
        self.assertEqual(num2words(300, lang="hr"), "tristo")
        self.assertEqual(num2words(400, lang="hr"), "četiristo")
        self.assertEqual(num2words(500, lang="hr"), "petsto")
        self.assertEqual(num2words(600, lang="hr"), "šesto")
        self.assertEqual(num2words(700, lang="hr"), "sedamsto")
        self.assertEqual(num2words(800, lang="hr"), "osamsto")
        self.assertEqual(num2words(900, lang="hr"), "devetsto")

    def test_thousands(self):
        self.assertEqual(num2words(1000, lang="hr"), "tisuća")
        self.assertEqual(num2words(1001, lang="hr"), "tisuća jedan")
        self.assertEqual(num2words(2000, lang="hr"), "dvije tisuće")
        self.assertEqual(num2words(3000, lang="hr"), "tri tisuće")
        self.assertEqual(num2words(4000, lang="hr"), "četiri tisuće")
        self.assertEqual(num2words(5000, lang="hr"), "pet tisuća")
        self.assertEqual(num2words(2012, lang="hr"), "dvije tisuće dvanaest")
        self.assertEqual(num2words(21000, lang="hr"), "dvadeset jedna tisuća")

    def test_large_numbers(self):
        self.assertEqual(num2words(1000000, lang="hr"), "milijun")
        self.assertEqual(num2words(2000000, lang="hr"), "dva milijuna")
        self.assertEqual(num2words(1000000000, lang="hr"), "milijarda")
        self.assertEqual(
            num2words(1234567890, lang="hr"),
            "milijarda dvjesto trideset četiri milijuna "
            "petsto šezdeset sedam tisuća osamsto devedeset",
        )

    def test_gender_forms(self):
        # Test feminine forms
        self.assertEqual(num2words(1, lang="hr", feminine=True), "jedna")
        self.assertEqual(num2words(2, lang="hr", feminine=True), "dvije")
        self.assertEqual(num2words(21, lang="hr", feminine=True), "dvadeset jedna")
        self.assertEqual(num2words(22, lang="hr", feminine=True), "dvadeset dvije")

    def test_decimals(self):
        self.assertEqual(num2words(10.02, lang="hr"), "deset zarez nula dva")
        self.assertEqual(num2words(15.007, lang="hr"), "petnaest zarez nula nula sedam")
        self.assertEqual(
            num2words(12519.85, lang="hr"),
            "dvanaest tisuća petsto devetnaest zarez osamdeset pet",
        )
        self.assertEqual(num2words(123.50, lang="hr"), "sto dvadeset tri zarez pet")

    def test_negative_numbers(self):
        self.assertEqual(num2words(-1, lang="hr"), "minus jedan")
        self.assertEqual(num2words(-15, lang="hr"), "minus petnaest")
        self.assertEqual(num2words(-123, lang="hr"), "minus sto dvadeset tri")

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="hr"), "minus nula zarez četiri")
        self.assertEqual(num2words(-0.5, lang="hr"), "minus nula zarez pet")
        self.assertEqual(num2words(-1.4, lang="hr"), "minus jedan zarez četiri")

    def test_currency_hrd(self):
        # Croatian Kuna (HRK) tests
        self.assertEqual(
            # kuna/lipa are feminine: jedna, dvije (#196).
            num2words(1, lang="hr", to="currency", currency="HRK"), "jedna kuna"
        )
        self.assertEqual(
            num2words(2, lang="hr", to="currency", currency="HRK"), "dvije kune"
        )
        self.assertEqual(
            num2words(5, lang="hr", to="currency", currency="HRK"), "pet kuna"
        )
        self.assertEqual(
            num2words(1234.56, lang="hr", to="currency", currency="HRK"),
            "tisuća dvjesto trideset četiri kune, pedeset šest lipa",
        )
        self.assertEqual(
            num2words(101.21, lang="hr", to="currency", currency="HRK", separator=" i"),
            "sto jedna kuna i dvadeset jedna lipa",
        )

    def test_currency_eur(self):
        # Euro currency tests
        self.assertEqual(
            num2words(10, lang="hr", to="currency", currency="EUR"), "deset eura"
        )
        self.assertEqual(
            num2words(1, lang="hr", to="currency", currency="EUR"), "jedan euro"
        )
        self.assertEqual(
            num2words(1234.56, lang="hr", to="currency", currency="EUR"),
            "tisuća dvjesto trideset četiri eura, pedeset šest centi",
        )
        self.assertEqual(
            num2words(101.11, lang="hr", to="currency", currency="EUR", separator=" i"),
            "sto jedan euro i jedanaest centi",
        )

    def test_currency_gender_agreement(self):
        # #196: kuna and lipa are feminine, euro and cent masculine.
        self.assertEqual(
            num2words(21, lang="hr", to="currency", currency="HRK"),
            "dvadeset jedna kuna",
        )
        self.assertEqual(
            num2words(2.02, lang="hr", to="currency", currency="HRK"),
            "dvije kune, dvije lipe",
        )
        self.assertEqual(
            num2words("2.02", lang="hr", to="currency", currency="HRK"),
            "dvije kune, dvije lipe",
        )
        self.assertEqual(
            num2words(2.02, lang="hr", to="currency", currency="EUR"),
            "dva eura, dva centa",
        )
        self.assertEqual(
            num2words(21.5, lang="hr", to="cheque", currency="HRK"),
            "DVADESET JEDNA AND 50/100 KUNA",
        )

    def test_currency_negative(self):
        self.assertEqual(
            num2words(-12519.85, lang="hr", to="currency", cents=False),
            "minus dvanaest tisuća petsto devetnaest eura, 85 centi",
        )
        self.assertEqual(
            num2words(-1.50, lang="hr", to="currency", currency="HRK"),
            "minus jedna kuna, pedeset lipa",
        )

    def test_currency_no_cents(self):
        self.assertEqual(
            num2words(19.50, lang="hr", to="currency", cents=False),
            "devetnaest eura, 50 centi",
        )
        self.assertEqual(
            num2words(100, lang="hr", to="currency", currency="HRK"), "sto kuna"
        )
        self.assertEqual(
            num2words(10000, lang="hr", to="currency", currency="HRK"),
            "deset tisuća kuna",
        )

    def test_ordinal(self):
        # Test ordinal numbers
        self.assertEqual(num2words(1, lang="hr", to="ordinal"), "prvi")
        self.assertEqual(num2words(2, lang="hr", to="ordinal"), "drugi")
        self.assertEqual(num2words(3, lang="hr", to="ordinal"), "treći")
        self.assertEqual(num2words(4, lang="hr", to="ordinal"), "četvrti")
        self.assertEqual(num2words(5, lang="hr", to="ordinal"), "peti")
        self.assertEqual(num2words(10, lang="hr", to="ordinal"), "deseti")
        self.assertEqual(num2words(100, lang="hr", to="ordinal"), "stoti")
        self.assertEqual(num2words(21, lang="hr", to="ordinal"), "dvadeset prvi")

    def test_complex_numbers(self):
        # Test some complex number conversions
        self.assertEqual(
            num2words(987654321, lang="hr"),
            "devetsto osamdeset sedam milijuna šesto pedeset "
            "četiri tisuće tristo dvadeset jedan",
        )
        self.assertEqual(
            num2words(50505050, lang="hr"), "pedeset milijuna petsto pet tisuća pedeset"
        )

    def test_edge_cases(self):
        # Test edge cases
        self.assertEqual(num2words(1001, lang="hr"), "tisuća jedan")
        self.assertEqual(num2words(1010, lang="hr"), "tisuća deset")
        self.assertEqual(num2words(1100, lang="hr"), "tisuća sto")
        self.assertEqual(num2words(11000, lang="hr"), "jedanaest tisuća")
        self.assertEqual(num2words(11001, lang="hr"), "jedanaest tisuća jedan")

    def test_cheque_names_the_unit(self):
        # gladiaio/num2words2#189: the cheque ended in the gender flag
        # ("FALSE"/"TRUE") instead of the currency name.
        self.assertEqual(
            num2words(12.5, lang="hr", to="cheque"), "DVANAEST AND 50/100 EURA"
        )
        self.assertEqual(
            num2words(1234.56, lang="hr", to="cheque", currency="EUR"),
            "TISUĆA DVJESTO TRIDESET ČETIRI AND 56/100 EURA",
        )
        self.assertEqual(
            num2words(12.5, lang="hr", to="cheque", currency="USD"),
            "DVANAEST AND 50/100 DOLARA",
        )
        self.assertEqual(
            num2words(-12.5, lang="hr", to="cheque", currency="HRK"),
            "MINUS DVANAEST AND 50/100 KUNA",
        )
        with self.assertRaises(NotImplementedError):
            num2words(12.5, lang="hr", to="cheque", currency="JPY")
