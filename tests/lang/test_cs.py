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


class Num2WordsCSTest(TestCase):
    def test_cardinal(self):
        self.assertEqual(num2words(100, lang="cs"), "sto")
        self.assertEqual(num2words(101, lang="cs"), "sto jedna")
        self.assertEqual(num2words(110, lang="cs"), "sto deset")
        self.assertEqual(num2words(115, lang="cs"), "sto patnáct")
        self.assertEqual(num2words(123, lang="cs"), "sto dvacet tři")
        self.assertEqual(num2words(1000, lang="cs"), "tisíc")
        self.assertEqual(num2words(1001, lang="cs"), "tisíc jedna")
        self.assertEqual(num2words(2012, lang="cs"), "dva tisíce dvanáct")
        self.assertEqual(num2words(10.02, lang="cs"), "deset čárka nula dva")
        self.assertEqual(num2words(15.007, lang="cs"), "patnáct čárka nula nula sedm")
        self.assertEqual(
            num2words(12519.85, lang="cs"),
            "dvanáct tisíc pět set devatenáct čárka osm pět",
        )
        self.assertEqual(num2words(123.50, lang="cs"), "sto dvacet tři čárka pět")
        self.assertEqual(
            num2words(1234567890, lang="cs"),
            "miliarda dvě stě třicet čtyři miliony pět set šedesát "
            "sedm tisíc osm set devadesát",
        )
        self.assertEqual(
            num2words(215461407892039002157189883901676, lang="cs"),
            "dvě stě patnáct quintillionů čtyři sta šedesát jedna kvadriliard "
            "čtyři sta sedm kvadrilionů osm set devadesát dvě triliardy třicet "
            "devět trilionů dvě biliardy sto padesát sedm bilionů sto "
            "osmdesát devět miliard osm set osmdesát tři miliony "
            "devět set jedna tisíc šest set sedmdesát šest",
        )
        self.assertEqual(
            num2words(719094234693663034822824384220291, lang="cs"),
            "sedm set devatenáct quintillionů devadesát "
            "čtyři kvadriliardy dvě stě třicet čtyři "
            "kvadriliony šest set devadesát tři triliardy "
            "šest set šedesát tři triliony třicet čtyři biliardy osm set "
            "dvacet dva biliony osm set dvacet čtyři "
            "miliardy tři sta osmdesát čtyři miliony dvě stě dvacet "
            "tisíc dvě stě devadesát jedna",
        )

    def test_to_ordinal(self):
        # Test Czech ordinals
        self.assertEqual(num2words(1, lang="cs", to="ordinal"), "první")
        self.assertEqual(num2words(2, lang="cs", to="ordinal"), "druhý")
        self.assertEqual(num2words(3, lang="cs", to="ordinal"), "třetí")
        self.assertEqual(num2words(10, lang="cs", to="ordinal"), "desátý")
        self.assertEqual(num2words(20, lang="cs", to="ordinal"), "dvacátý")
        self.assertEqual(num2words(100, lang="cs", to="ordinal"), "stý")
        self.assertEqual(num2words(1000, lang="cs", to="ordinal"), "tisící")

    def test_currency(self):
        self.assertEqual(
            num2words(10, lang="cs", to="currency", currency="EUR"), "deset euro"
        )
        self.assertEqual(
            num2words(1, lang="cs", to="currency", currency="CZK"), "jedna koruna"
        )
        self.assertEqual(
            num2words(1234.56, lang="cs", to="currency", currency="EUR"),
            "tisíc dvě stě třicet čtyři euro, padesát šest centů",
        )
        self.assertEqual(
            num2words(1234.56, lang="cs", to="currency", currency="CZK"),
            "tisíc dvě stě třicet čtyři koruny, padesát šest haléřů",
        )
        self.assertEqual(
            num2words(101.11, lang="cs", to="currency", currency="EUR", separator=" a"),
            "sto jedna euro a jedenáct centů",
        )
        self.assertEqual(
            num2words(101.21, lang="cs", to="currency", currency="CZK", separator=" a"),
            "sto jedna korun a dvacet jedna haléřů",
        )
        self.assertEqual(
            num2words(-12519.85, lang="cs", to="currency", cents=False),
            "mínus dvanáct tisíc pět set devatenáct euro, 85 centů",
        )
        self.assertEqual(
            num2words(123.50, lang="cs", to="currency", currency="CZK", separator=" a"),
            "sto dvacet tři koruny a padesát haléřů",
        )
        self.assertEqual(
            num2words(19.50, lang="cs", to="currency", cents=False),
            "devatenáct euro, 50 centů",
        )

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="cs"), "mínus nula čárka čtyři")
        self.assertEqual(num2words(-0.5, lang="cs"), "mínus nula čárka pět")
        self.assertEqual(num2words(-1.4, lang="cs"), "mínus jedna čárka čtyři")


def test_cs_string_and_float_decimal_consistency():
    # Regression for num2words2#75 (ports savoirfairelinux/num2words#458).
    from num2words2 import num2words
    assert num2words("1.50", lang="cs") == num2words(1.50, lang="cs") == "jedna čárka pět"
    assert num2words("1.00", lang="cs") == num2words(1.00, lang="cs") == "jedna čárka nula"
    assert num2words("1.123", lang="cs") == num2words(1.123, lang="cs")


def test_compound_ordinals_issue_216():
    # gladiaio/num2words2#216: every component of a compound ordinal is an
    # ordinal; Python glued "ý" onto the cardinal ("dvacet jednaý").
    from num2words2 import num2words
    cases = {
        0: "nultý",
        21: "dvacátý první",
        23: "dvacátý třetí",
        31: "třicátý první",
        101: "stý první",
        121: "stý dvacátý první",
        200: "dvoustý",
        345: "třístý čtyřicátý pátý",
        999: "devítistý devadesátý devátý",
        1001: "tisící první",
        2021: "dvoutisící dvacátý první",
        5000: "pětitisící",
        10**6: "miliontý",
        -1: "mínus první",
    }
    for n, want in cases.items():
        assert num2words(n, lang="cs", to="ordinal") == want, n
