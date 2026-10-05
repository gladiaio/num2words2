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

TEST_CASES_CARDINAL = (
    (70, "septante"),
    (79, "septante-neuf"),
    (89, "quatre-vingt-neuf"),
    (95, "nonante-cinq"),
    (729, "sept cent vingt-neuf"),
    (894, "huit cent nonante-quatre"),
    (999, "neuf cent nonante-neuf"),
    (7232, "sept mille deux cent trente-deux"),
    (8569, "huit mille cinq cent soixante-neuf"),
    (9539, "neuf mille cinq cent trente-neuf"),
    (1000000, "un million"),
    (1000001, "un million un"),
    (4000000, "quatre millions"),
    (10000000000000, "dix billions"),
    (100000000000000, "cent billions"),
    (1000000000000000000, "un trillion"),
    (1000000000000000000000, "un trilliard"),
    (10000000000000000000000000, "dix quadrillions"),
)

TEST_CASES_ORDINAL = (
    (1, "premier"),
    (8, "huitième"),
    (12, "douzième"),
    (14, "quatorzième"),
    (28, "vingt-huitième"),
    (100, "centième"),
    (1000, "millième"),
    (1000000, "millionième"),
    (1000000000000000, "billiardième"),
    (1000000000000000000, "trillionième"),  # over 1e18 is not supported
)

TEST_CASES_TO_CURRENCY_EUR = (
    (1, "un euro"),
    (2.01, "deux euros et un centime"),
    (8.10, "huit euros et dix centimes"),
    (12.26, "douze euros et vingt-six centimes"),
    (21.29, "vingt et un euros et vingt-neuf centimes"),
    (81.25, "quatre-vingt et un euros et vingt-cinq centimes"),
    (100, "cent euros"),
)

TEST_CASES_TO_CURRENCY_FRF = (
    (1, "un franc"),
    (2.01, "deux francs et un centime"),
    (8.10, "huit francs et dix centimes"),
    (12.27, "douze francs et vingt-sept centimes"),
    (21.29, "vingt et un francs et vingt-neuf centimes"),
    (81.25, "quatre-vingt et un francs et vingt-cinq centimes"),
    (100, "cent francs"),
)

# Lang to execute current test
LANG = "fr_BE"


class Num2WordsENTest(TestCase):
    def test_ordinal_special_joins(self):
        self.assertEqual(num2words(5, ordinal=True, lang=LANG), "cinquième")
        self.assertEqual(num2words(6, ordinal=True, lang=LANG), "sixième")
        self.assertEqual(num2words(35, ordinal=True, lang=LANG), "trente-cinquième")
        self.assertEqual(num2words(9, ordinal=True, lang=LANG), "neuvième")
        self.assertEqual(num2words(49, ordinal=True, lang=LANG), "quarante-neuvième")
        self.assertEqual(num2words(71, lang=LANG), "septante et un")
        self.assertEqual(num2words(81, lang=LANG), "quatre-vingt et un")
        self.assertEqual(num2words(80, lang=LANG), "quatre-vingt")
        self.assertEqual(num2words(880, lang=LANG), "huit cent quatre-vingt")
        self.assertEqual(num2words(91, ordinal=True, lang=LANG), "nonante et unième")
        self.assertEqual(num2words(53, lang=LANG), "cinquante-trois")

    def test_number(self):
        for test in TEST_CASES_CARDINAL:
            self.assertEqual(num2words(test[0], lang=LANG), test[1])

    def test_ordinal(self):
        for test in TEST_CASES_ORDINAL:
            self.assertEqual(num2words(test[0], lang=LANG, ordinal=True), test[1])

    def test_currency_eur(self):
        for test in TEST_CASES_TO_CURRENCY_EUR:
            self.assertEqual(num2words(test[0], lang=LANG, to="currency"), test[1])

    def test_currency_frf(self):
        for test in TEST_CASES_TO_CURRENCY_FRF:
            self.assertEqual(
                num2words(test[0], lang=LANG, to="currency", currency="FRF"), test[1]
            )

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="fr_BE"), "moins zéro virgule quatre")
        self.assertEqual(num2words(-0.5, lang="fr_BE"), "moins zéro virgule cinq")
        self.assertEqual(num2words(-1.4, lang="fr_BE"), "moins un virgule quatre")


def test_fr_be_corrections_from_upstream_532():
    # Regression for num2words2#70 (ports savoirfairelinux/num2words#532).
    from num2words2 import num2words

    # Cardinal: cents singular before continuation, million/trillion/trilliard
    # singular when count is 1.
    assert num2words(729, lang="fr_BE") == "sept cent vingt-neuf"
    assert num2words(894, lang="fr_BE") == "huit cent nonante-quatre"
    assert num2words(1_000_000, lang="fr_BE") == "un million"
    assert num2words(1_000_001, lang="fr_BE") == "un million un"
    assert num2words(10 ** 18, lang="fr_BE") == "un trillion"
    assert num2words(10 ** 21, lang="fr_BE") == "un trilliard"

    # Ordinal: drop leading 'un' and drop trailing 's' before -ième. The
    # -ième stays singular when count > 1 (num2words2#165).
    assert num2words(1_000_000, lang="fr_BE", to="ordinal") == "millionième"
    assert num2words(2_000_000, lang="fr_BE", to="ordinal") == "deux millionième"
    assert num2words(10 ** 15, lang="fr_BE", to="ordinal") == "billiardième"

    # Currency: zéro takes singular for both major and minor unit.
    assert num2words(1.00, lang="fr_BE", to="currency") == "un euro et zéro centime"
    assert num2words(100.00, lang="fr_BE", to="currency") == "cent euros et zéro centime"
    assert num2words(1.00, lang="fr_BE", to="currency", currency="FRF") == "un franc et zéro centime"
