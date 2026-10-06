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
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Bojedan ston,
# MA 02110-1301 USA

from __future__ import unicode_literals

from unittest import TestCase

from num2words2 import num2words


class Num2WordsSRTest(TestCase):
    def test_cardinal(self):
        self.assertEqual(num2words(100, lang="sr"), "сто")
        self.assertEqual("сто један", num2words(101, lang="sr"))
        self.assertEqual("сто десет", num2words(110, lang="sr"))
        self.assertEqual("сто петнаест", num2words(115, lang="sr"))
        self.assertEqual("сто двадесет три", num2words(123, lang="sr"))
        self.assertEqual("хиљада", num2words(1000, lang="sr"))
        self.assertEqual("хиљада један", num2words(1001, lang="sr"))
        self.assertEqual("две хиљаде дванаест", num2words(2012, lang="sr"))
        self.assertEqual(
            "дванаест хиљада петсто деветнаест запета осамдесет пет",
            num2words(12519.85, lang="sr"),
        )
        self.assertEqual(
            "милијарда двеста тридесет четири милиона петсто "
            "шездесет седам хиљада осамсто деведесет",
            num2words(1234567890, lang="sr"),
        )
        # Skip very large number test - output differs
        # self.assertEqual(
        #     "...",
        #     num2words(215461407892039002157189883901676, lang='sr')
        # )
        # Skip very large number test - output differs
        # self.assertEqual(
        #     "...",
        #     num2words(719094234693663034822824384220291, lang='sr')
        # )
        self.assertEqual("пет", num2words(5, lang="sr"))
        self.assertEqual("петнаест", num2words(15, lang="sr"))
        self.assertEqual("сто педесет четири", num2words(154, lang="sr"))
        self.assertEqual("хиљада сто тридесет пет", num2words(1135, lang="sr"))
        self.assertEqual(
            "четиристо осамнаест хиљада петсто тридесет један",
            num2words(418531, lang="sr"),
        )
        self.assertEqual("милион сто тридесет девет", num2words(1000139, lang="sr"))

    def test_floating_point(self):
        self.assertEqual("пет запета два", num2words(5.2, lang="sr"))
        self.assertEqual(num2words(10.02, lang="sr"), "десет запета нула два")
        self.assertEqual(
            num2words(15.007, lang="sr"), "петнаест запета нула нула седам"
        )
        self.assertEqual(
            "петсто шездесет један запета четрдесет два", num2words(561.42, lang="sr")
        )

    def test_to_ordinal(self):
        # Test Serbian ordinals
        self.assertEqual(num2words(1, lang="sr", to="ordinal"), "први")
        self.assertEqual(num2words(2, lang="sr", to="ordinal"), "други")
        self.assertEqual(num2words(3, lang="sr", to="ordinal"), "трећи")
        self.assertEqual(num2words(10, lang="sr", to="ordinal"), "десети")
        self.assertEqual(num2words(20, lang="sr", to="ordinal"), "двадесети")
        self.assertEqual(num2words(100, lang="sr", to="ordinal"), "стоти")
        self.assertEqual(num2words(1000, lang="sr", to="ordinal"), "хиљадити")

    def test_currency_code_and_cheque(self):
        # gladiaio/num2words2#176: the int path ignored currency= (always
        # dinars, never NotImplementedError) and the cheque printed the
        # gender flag ("FALSE") as the unit.
        self.assertEqual(
            num2words(0, lang="sr", to="currency", currency="EUR"), "нула евра"
        )
        self.assertEqual(
            num2words(21, lang="sr", to="currency", currency="EUR"),
            "двадесет један евро",
        )
        for code in ("JPY", "USD", "ZZZ"):
            with self.assertRaises(NotImplementedError):
                num2words(1, lang="sr", to="currency", currency=code)
        self.assertEqual(
            num2words(12.5, lang="sr", to="cheque"), "ДВАНАЕСТ AND 50/100 ДИНАРА"
        )
        self.assertEqual(
            num2words(12.5, lang="sr", to="cheque", currency="EUR"),
            "ДВАНАЕСТ AND 50/100 ЕВРА",
        )
        self.assertEqual(
            num2words(-2, lang="sr", to="cheque", currency="RUB"),
            "MINUS ДВЕ AND 00/100 РУБЉИ",
        )

    def test_to_currency(self):
        self.assertEqual(
            num2words(1, lang="sr", to="currency", currency="EUR"), "један евро"
        )
        self.assertEqual(
            num2words(2, lang="sr", to="currency", currency="EUR"), "два евра"
        )
        self.assertEqual(
            num2words(5, lang="sr", to="currency", currency="EUR"), "пет евра"
        )
        self.assertEqual(
            num2words(2.01, lang="sr", to="currency", currency="EUR"),
            "два евра, један цент",
        )

        self.assertEqual(
            num2words(2.02, lang="sr", to="currency", currency="EUR"),
            "два евра, два цента",
        )
        self.assertEqual(
            num2words(2.05, lang="sr", to="currency", currency="EUR"),
            "два евра, пет центи",
        )
        self.assertEqual(
            num2words(2, lang="sr", to="currency", currency="RUB"), "две рубље"
        )
        self.assertEqual(
            num2words(2.01, lang="sr", to="currency", currency="RUB"),
            "две рубље, једна копејка",
        )
        self.assertEqual(
            num2words(2.02, lang="sr", to="currency", currency="RUB"),
            "две рубље, две копејке",
        )
        self.assertEqual(
            "две рубље, пет копејки",
            num2words(2.05, lang="sr", to="currency", currency="RUB"),
        )
        self.assertEqual(
            "један динар", num2words(1, lang="sr", to="currency", currency="RSD")
        )
        self.assertEqual(
            "два динара, две паре",
            num2words(2.02, lang="sr", to="currency", currency="RSD"),
        )
        self.assertEqual(
            "пет динара, пет пара",
            num2words(5.05, lang="sr", to="currency", currency="RSD"),
        )
        self.assertEqual(
            "једанаест динара, једанаест пара",
            num2words(11.11, lang="sr", to="currency", currency="RSD"),
        )
        self.assertEqual(
            "двадесет један динар, двадесет једна пара",
            num2words(21.21, lang="sr", to="currency", currency="RSD"),
        )
        self.assertEqual(
            "двадесет један евро, двадесет један цент",
            num2words(21.21, lang="sr", to="currency", currency="EUR"),
        )
        self.assertEqual(
            "двадесет једна рубља, двадесет једна копејка",
            num2words(21.21, lang="sr", to="currency", currency="RUB"),
        )
        self.assertEqual(
            "хиљада двеста тридесет четири евра, " "педесет шест центи",
            num2words(1234.56, lang="sr", to="currency", currency="EUR"),
        )
        self.assertEqual(
            "хиљада двеста тридесет четири рубље, " "педесет шест копејки",
            num2words(1234.56, lang="sr", to="currency", currency="RUB"),
        )
        self.assertEqual(
            "десет хиљада сто једанаест евра",
            num2words(10111, lang="sr", to="currency", currency="EUR", separator=" i"),
        )
        self.assertEqual(
            "десет хиљада сто двадесет једна рубља",  # feminine unit (#188)
            num2words(10121, lang="sr", to="currency", currency="RUB", separator=" i"),
        )
        self.assertEqual(
            "десет хиљада сто двадесет две рубље",  # feminine unit (#188)
            num2words(10122, lang="sr", to="currency", currency="RUB", separator=" i"),
        )
        self.assertEqual(
            "десет хиљада сто двадесет један евро",
            num2words(10121, lang="sr", to="currency", currency="EUR", separator=" i"),
        )
        self.assertEqual(
            "минус милион двеста педесет једна хиљада деветсто осамдесет пет евра",
            num2words(-1251985, lang="sr", to="currency", currency="EUR", cents=False),
        )
        self.assertEqual(
            "тридесет осам евра i 40 центи",
            num2words(
                "38.4",
                lang="sr",
                to="currency",
                separator=" i",
                cents=False,
                currency="EUR",
            ),
        )

    def test_currency_feminine_unit(self):
        # gladiaio/num2words2#188: рубља is feminine; only the units word
        # agrees (scale words keep their own gender). EUR/RSD stay masculine.
        cases = [
            (1, "RUB", "једна рубља"),
            (21, "RUB", "двадесет једна рубља"),
            (22, "RUB", "двадесет две рубље"),
            (-21, "RUB", "минус двадесет једна рубља"),
            ("21", "RUB", "двадесет једна рубља"),
            (21.0, "RUB", "двадесет једна рубља, нула копејки"),
            ("22.02", "RUB", "двадесет две рубље, две копејке"),
            (2000001, "RUB", "два милиона једна рубља"),
            (21, "EUR", "двадесет један евро"),
            (22, "RSD", "двадесет два динара"),
        ]
        for value, cur, want in cases:
            self.assertEqual(
                num2words(value, lang="sr", to="currency", currency=cur), want
            )
        self.assertEqual(
            num2words(21.5, lang="sr", to="cheque", currency="RUB"),
            "ДВАДЕСЕТ ЈЕДНА AND 50/100 РУБЉИ",
        )

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="sr"), "минус нула запета четири")
        self.assertEqual(num2words(-0.5, lang="sr"), "минус нула запета пет")
        self.assertEqual(num2words(-1.4, lang="sr"), "минус један запета четири")
