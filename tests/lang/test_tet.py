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

import decimal
from decimal import Decimal
from unittest import TestCase

from num2words2 import num2words


class Num2WordsTETTest(TestCase):
    def test_cardinal_integer(self):
        self.assertEqual(num2words(1, lang="tet"), "ida")
        self.assertEqual(num2words(2, lang="tet"), "rua")
        self.assertEqual(num2words(3, lang="tet"), "tolu")
        self.assertEqual(num2words(4, lang="tet"), "haat")
        self.assertEqual(num2words(5, lang="tet"), "lima")
        self.assertEqual(num2words(6, lang="tet"), "neen")
        self.assertEqual(num2words(7, lang="tet"), "hitu")
        self.assertEqual(num2words(8, lang="tet"), "ualu")
        self.assertEqual(num2words(9, lang="tet"), "sia")
        self.assertEqual(num2words(10, lang="tet"), "sanulu")
        self.assertEqual(num2words(11, lang="tet"), "sanulu resin ida")
        self.assertEqual(num2words(12, lang="tet"), "sanulu resin rua")
        self.assertEqual(num2words(13, lang="tet"), "sanulu resin tolu")
        self.assertEqual(num2words(14, lang="tet"), "sanulu resin haat")
        self.assertEqual(num2words(15, lang="tet"), "sanulu resin lima")
        self.assertEqual(num2words(16, lang="tet"), "sanulu resin neen")
        self.assertEqual(num2words(17, lang="tet"), "sanulu resin hitu")
        self.assertEqual(num2words(18, lang="tet"), "sanulu resin ualu")
        self.assertEqual(num2words(19, lang="tet"), "sanulu resin sia")
        self.assertEqual(num2words(20, lang="tet"), "rua nulu")

        self.assertEqual(num2words(21, lang="tet"), "rua nulu resin ida")
        self.assertEqual(num2words(22, lang="tet"), "rua nulu resin rua")
        self.assertEqual(num2words(35, lang="tet"), "tolu nulu resin lima")
        self.assertEqual(num2words(99, lang="tet"), "sia nulu resin sia")

        self.assertEqual(num2words(100, lang="tet"), "atus ida")
        self.assertEqual(num2words(101, lang="tet"), "atus ida ida")
        self.assertEqual(num2words(107, lang="tet"), "atus ida hitu")
        self.assertEqual(num2words(110, lang="tet"), "atus ida sanulu")
        self.assertEqual(num2words(114, lang="tet"), "atus ida sanulu resin haat")
        self.assertEqual(num2words(128, lang="tet"), "atus ida rua nulu resin ualu")
        self.assertEqual(num2words(151, lang="tet"), "atus ida lima nulu resin ida")
        self.assertEqual(num2words(713, lang="tet"), "atus hitu sanulu resin tolu")
        self.assertEqual(num2words(999, lang="tet"), "atus sia sia nulu resin sia")

        self.assertEqual(num2words(1000, lang="tet"), "rihun ida")
        self.assertEqual(num2words(1001, lang="tet"), "rihun ida ida")
        self.assertEqual(num2words(1011, lang="tet"), "rihun ida sanulu resin ida")
        self.assertEqual(
            num2words(1111, lang="tet"), "rihun ida atus ida sanulu resin ida"
        )
        self.assertEqual(
            num2words(2357, lang="tet"), "rihun rua atus tolu lima nulu resin hitu"
        )
        self.assertEqual(num2words(2200, lang="tet"), "rihun rua atus rua")
        self.assertEqual(num2words(2230, lang="tet"), "rihun rua atus rua tolu nulu")
        self.assertEqual(
            num2words(73400, lang="tet"), "rihun hitu nulu resin tolu atus haat"
        )
        self.assertEqual(
            num2words(73421, lang="tet"),
            "rihun hitu nulu resin tolu atus haat rua nulu resin ida",
        )
        self.assertEqual(num2words(100000, lang="tet"), "rihun atus ida")
        self.assertEqual(
            num2words(250050, lang="tet"), "rihun atus rua lima nulu lima nulu"
        )
        self.assertEqual(num2words(6000000, lang="tet"), "miliaun neen")
        self.assertEqual(num2words(100000000, lang="tet"), "miliaun atus ida")
        self.assertEqual(
            num2words(19000000000, lang="tet"), "miliaun rihun sanulu resin sia"
        )
        self.assertEqual(
            num2words(145000000002, lang="tet"),
            "miliaun rihun atus ida haat nulu resin lima resin rua",
        )
        self.assertEqual(
            num2words(4635102, lang="tet"),
            "miliaun haat rihun atus neen tolu nulu resin lima atus ida rua",
        )
        self.assertEqual(
            num2words(145254635102, lang="tet"),
            "miliaun rihun atus ida haat nulu resin lima atus rua lima nulu \
resin haat rihun atus neen tolu nulu resin lima atus ida rua",
        )
        self.assertEqual(num2words(1000000000000, lang="tet"), "biliaun ida")
        self.assertEqual(num2words(2000000000000, lang="tet"), "biliaun rua")
        self.assertEqual(num2words(1000000000000000, lang="tet"), "biliaun rihun ida")
        self.assertEqual(num2words(2000000000000000, lang="tet"), "biliaun rihun rua")
        self.assertEqual(num2words(1000000000000000000, lang="tet"), "triliaun ida")
        self.assertEqual(num2words(2000000000000000000, lang="tet"), "triliaun rua")

    def test_cardinal_integer_negative(self):
        self.assertEqual(num2words(-1, lang="tet"), "menus ida")
        self.assertEqual(
            num2words(-256, lang="tet"), "menus atus rua lima nulu resin neen"
        )
        self.assertEqual(num2words(-1000, lang="tet"), "menus rihun ida")
        self.assertEqual(num2words(-1000000, lang="tet"), "menus miliaun ida")
        self.assertEqual(
            num2words(-1234567, lang="tet"),
            "menus miliaun ida rihun atus rua tolu nulu resin \
haat atus lima neen nulu resin hitu",
        )

    def test_cardinal_float(self):
        self.assertEqual(num2words(Decimal("1.00"), lang="tet"), "ida")
        self.assertEqual(
            num2words(Decimal("1.01"), lang="tet"), "ida vírgula mamuk ida"
        )
        self.assertEqual(
            num2words(Decimal("1.035"), lang="tet"), "ida vírgula mamuk tolu lima"
        )
        self.assertEqual(
            num2words(Decimal("1.35"), lang="tet"), "ida vírgula tolu lima"
        )
        self.assertEqual(
            num2words(Decimal("3.14159"), lang="tet"),
            "tolu vírgula ida haat ida lima sia",
        )
        self.assertEqual(
            num2words(Decimal("101.22"), lang="tet"), "atus ida ida vírgula rua rua"
        )
        self.assertEqual(
            num2words(Decimal("2345.75"), lang="tet"),
            "rihun rua atus tolu haat nulu resin lima vírgula hitu lima",
        )

    def test_cardinal_float_negative(self):
        self.assertEqual(
            num2words(Decimal("-2.34"), lang="tet"), "menus rua vírgula tolu haat"
        )
        self.assertEqual(
            num2words(Decimal("-9.99"), lang="tet"), "menus sia vírgula sia sia"
        )
        self.assertEqual(
            num2words(Decimal("-7.01"), lang="tet"), "menus hitu vírgula mamuk ida"
        )
        self.assertEqual(
            num2words(Decimal("-222.22"), lang="tet"),
            "menus atus rua rua nulu resin rua vírgula rua rua",
        )

    def test_ordinal(self):
        with self.assertRaises(decimal.InvalidOperation):
            num2words("hello", lang="tet", ordinal=True)
        with self.assertRaises(TypeError):
            num2words(5.1, lang="tet", ordinal=True)
        self.assertEqual(num2words(1, lang="tet", ordinal=True), "dahuluk")
        self.assertEqual(num2words(2, lang="tet", ordinal=True), "daruak")
        self.assertEqual(num2words(3, lang="tet", ordinal=True), "datoluk")
        self.assertEqual(num2words(4, lang="tet", ordinal=True), "dahaat")
        self.assertEqual(num2words(5, lang="tet", ordinal=True), "dalimak")
        self.assertEqual(num2words(6, lang="tet", ordinal=True), "daneen")
        self.assertEqual(num2words(7, lang="tet", ordinal=True), "dahituk")
        self.assertEqual(num2words(8, lang="tet", ordinal=True), "daualuk")
        self.assertEqual(num2words(9, lang="tet", ordinal=True), "dasiak")
        self.assertEqual(num2words(10, lang="tet", ordinal=True), "dasanuluk")
        self.assertEqual(num2words(11, lang="tet", ordinal=True), "dasanulu resin idak")
        self.assertEqual(num2words(12, lang="tet", ordinal=True), "dasanulu resin ruak")
        self.assertEqual(
            num2words(13, lang="tet", ordinal=True), "dasanulu resin toluk"
        )
        self.assertEqual(num2words(14, lang="tet", ordinal=True), "dasanulu resin haat")
        self.assertEqual(
            num2words(15, lang="tet", ordinal=True), "dasanulu resin limak"
        )
        self.assertEqual(num2words(16, lang="tet", ordinal=True), "dasanulu resin neen")
        self.assertEqual(
            num2words(17, lang="tet", ordinal=True), "dasanulu resin hituk"
        )
        self.assertEqual(
            num2words(18, lang="tet", ordinal=True), "dasanulu resin ualuk"
        )
        self.assertEqual(num2words(19, lang="tet", ordinal=True), "dasanulu resin siak")
        self.assertEqual(num2words(20, lang="tet", ordinal=True), "darua nuluk")

        self.assertEqual(
            num2words(21, lang="tet", ordinal=True), "darua nulu resin idak"
        )
        self.assertEqual(
            num2words(22, lang="tet", ordinal=True), "darua nulu resin ruak"
        )
        self.assertEqual(
            num2words(35, lang="tet", ordinal=True), "datolu nulu resin limak"
        )
        self.assertEqual(
            num2words(99, lang="tet", ordinal=True), "dasia nulu resin siak"
        )

        self.assertEqual(num2words(100, lang="tet", ordinal=True), "dahatus idak")
        self.assertEqual(num2words(101, lang="tet", ordinal=True), "dahatus ida idak")
        self.assertEqual(num2words(106, lang="tet", ordinal=True), "dahatus ida neen")
        self.assertEqual(
            num2words(128, lang="tet", ordinal=True), "dahatus ida rua nulu resin ualuk"
        )
        self.assertEqual(num2words(600, lang="tet", ordinal=True), "dahatus neen")
        self.assertEqual(
            num2words(713, lang="tet", ordinal=True), "dahatus hitu sanulu resin toluk"
        )

        self.assertEqual(num2words(1000, lang="tet", ordinal=True), "darihun idak")
        self.assertEqual(num2words(1001, lang="tet", ordinal=True), "darihun ida idak")
        self.assertEqual(
            num2words(1111, lang="tet", ordinal=True),
            "darihun ida atus ida sanulu resin idak",
        )
        self.assertEqual(
            num2words(2114, lang="tet", ordinal=True),
            "darihun rua atus ida sanulu resin haat",
        )
        self.assertEqual(
            num2words(73421, lang="tet", ordinal=True),
            "darihun hitu nulu resin tolu atus haat rua nulu resin idak",
        )

        self.assertEqual(
            num2words(100000, lang="tet", ordinal=True), "darihun atus idak"
        )
        self.assertEqual(
            num2words(250050, lang="tet", ordinal=True),
            "darihun atus rua lima nulu lima nuluk",
        )
        self.assertEqual(num2words(6000000, lang="tet", ordinal=True), "damiliaun neen")
        self.assertEqual(
            num2words(19000000000, lang="tet", ordinal=True),
            "damiliaun rihun sanulu resin siak",
        )
        self.assertEqual(
            num2words(145000000002, lang="tet", ordinal=True),
            "damiliaun rihun atus ida haat nulu resin lima resin ruak",
        )

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="tet"), "menus mamuk vírgula haat")
        self.assertEqual(num2words(-0.5, lang="tet"), "menus mamuk vírgula lima")
        self.assertEqual(num2words(-1.4, lang="tet"), "menus ida vírgula haat")
