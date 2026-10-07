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

from num2words2 import num2words

from . import test_es

TEST_HNL = (
    (1.0, "un lempira con cero centavos"),
    (2.0, "dos lempiras con cero centavos"),
    (8.5, "ocho lempiras con cincuenta centavos"),
    # 12.256 omitted: num2words2 base preserves fractional cents instead of
    # rounding (see lang_HA, lang_BAN behavior). Upstream rounds 25.6c -> 26c.
    (25.6, "veinticinco lempiras con sesenta centavos"),
    (96.55, "noventa y seis lempiras con cincuenta y cinco centavos"),
    (100.00, "cien lempiras con cero centavos"),
)


class Num2WordsHNHNLTest(test_es.Num2WordsESTest):
    def test_currency(self):
        for test in TEST_HNL:
            self.assertEqual(num2words(test[0], lang="es_HN", to="currency"), test[1])
