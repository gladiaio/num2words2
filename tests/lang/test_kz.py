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


class Num2WordsKZTest(TestCase):
    # "kz" is an alias of "kk" (gladiaio/num2words2#166). Hundreds carry
    # their multiplier ("бір жүз"), the form "kk" always used; whether plain
    # "жүз" is preferred is an open question for a native speaker.

    def test_kz_is_an_alias_of_kk(self):
        for value in (0, 7, 101, 1.5, -1, 10**30):
            self.assertEqual(num2words(value, lang="kz"), num2words(value, lang="kk"))
        for to in ("ordinal", "ordinal_num", "year", "currency"):
            self.assertEqual(
                num2words(21, lang="kz", to=to), num2words(21, lang="kk", to=to)
            )

    def test_default_currency_is_kzt(self):
        # Used to raise NotImplementedError: the default was EUR (#166).
        self.assertEqual(num2words(1, lang="kz", to="currency"), "бір теңге")

    def test_to_cardinal(self):
        self.maxDiff = None
        self.assertEqual(num2words(7, lang="kz"), "жеті")
        self.assertEqual(num2words(23, lang="kz"), "жиырма үш")
        self.assertEqual(num2words(145, lang="kz"), "бір жүз қырық бес")
        self.assertEqual(num2words(2869, lang="kz"), "екі мың сегіз жүз алпыс тоғыз")
        self.assertEqual(
            num2words(-789000125, lang="kz"),
            "минус жеті жүз сексен тоғыз миллион бір жүз жиырма бес",
        )
        self.assertEqual(
            num2words(84932, lang="kz"), "сексен төрт мың тоғыз жүз отыз екі"
        )

    def test_to_cardinal_floats(self):
        self.assertEqual(num2words(100.67, lang="kz"), "бір жүз бүтін алпыс жеті")
        self.assertEqual(num2words(0.7, lang="kz"), "нөл бүтін жеті")
        self.assertEqual(num2words(1.73, lang="kz"), "бір бүтін жетпіс үш")
        self.assertEqual(num2words(10.02, lang="kz"), "он бүтін нөл екі")
        self.assertEqual(num2words(15.007, lang="kz"), "он бес бүтін нөл нөл жеті")

    def test_to_ordinal(self):
        self.assertEqual(num2words(1, lang="kz", to="ordinal"), "бірінші")
        self.assertEqual(num2words(2, lang="kz", to="ordinal"), "екінші")
        self.assertEqual(num2words(3, lang="kz", to="ordinal"), "үшінші")
        self.assertEqual(num2words(5, lang="kz", to="ordinal"), "бесінші")
        self.assertEqual(num2words(10, lang="kz", to="ordinal"), "оныншы")
        self.assertEqual(num2words(21, lang="kz", to="ordinal"), "жиырма бірінші")
        self.assertEqual(num2words(100, lang="kz", to="ordinal"), "бір жүзінші")

    def test_to_currency(self):
        self.assertEqual(
            num2words(25.24, lang="kz", to="currency", currency="KZT"),
            "жиырма бес теңге, жиырма төрт тиын",
        )
        self.assertEqual(
            num2words(1996.4, lang="kz", to="currency", currency="KZT"),
            "бір мың тоғыз жүз тоқсан алты теңге, қырық тиын",
        )
        self.assertEqual(
            num2words(632924.51, lang="kz", to="currency", currency="KZT"),
            "алты жүз отыз екі мың тоғыз жүз жиырма төрт теңге, елу бір тиын",
        )
        self.assertEqual(
            num2words(632924.513, lang="kz", to="currency", currency="KZT"),
            "алты жүз отыз екі мың тоғыз жүз жиырма төрт теңге, елу бір бүтін үш тиын",
        )
        self.assertEqual(
            num2words(987654321.123, lang="kz", to="currency", currency="KZT"),
            "тоғыз жүз сексен жеті миллион алты жүз елу төрт мың "
            "үш жүз жиырма бір теңге, он екі бүтін үш тиын",
        )

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="kz"), "минус нөл бүтін төрт")
        self.assertEqual(num2words(-0.5, lang="kz"), "минус нөл бүтін бес")
        self.assertEqual(num2words(-1.4, lang="kz"), "минус бір бүтін төрт")
