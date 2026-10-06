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

from decimal import Decimal
from unittest import TestCase

from num2words2 import num2words


class Num2WordsPTTest(TestCase):

    def test_cardinal_integer(self):
        self.assertEqual(num2words(1, lang="pt"), "um")
        self.assertEqual(num2words(2, lang="pt"), "dois")
        self.assertEqual(num2words(3, lang="pt"), "três")
        self.assertEqual(num2words(4, lang="pt"), "quatro")
        self.assertEqual(num2words(5, lang="pt"), "cinco")
        self.assertEqual(num2words(6, lang="pt"), "seis")
        self.assertEqual(num2words(7, lang="pt"), "sete")
        self.assertEqual(num2words(8, lang="pt"), "oito")
        self.assertEqual(num2words(9, lang="pt"), "nove")
        self.assertEqual(num2words(10, lang="pt"), "dez")
        self.assertEqual(num2words(11, lang="pt"), "onze")
        self.assertEqual(num2words(12, lang="pt"), "doze")
        self.assertEqual(num2words(13, lang="pt"), "treze")
        self.assertEqual(num2words(14, lang="pt"), "catorze")
        self.assertEqual(num2words(15, lang="pt"), "quinze")
        self.assertEqual(num2words(16, lang="pt"), "dezasseis")
        self.assertEqual(num2words(17, lang="pt"), "dezassete")
        self.assertEqual(num2words(18, lang="pt"), "dezoito")
        self.assertEqual(num2words(19, lang="pt"), "dezanove")
        self.assertEqual(num2words(20, lang="pt"), "vinte")

        self.assertEqual(num2words(21, lang="pt"), "vinte e um")
        self.assertEqual(num2words(22, lang="pt"), "vinte e dois")
        self.assertEqual(num2words(35, lang="pt"), "trinta e cinco")
        self.assertEqual(num2words(99, lang="pt"), "noventa e nove")

        self.assertEqual(num2words(100, lang="pt"), "cem")
        self.assertEqual(num2words(101, lang="pt"), "cento e um")
        self.assertEqual(num2words(128, lang="pt"), "cento e vinte e oito")
        self.assertEqual(num2words(713, lang="pt"), "setecentos e treze")

        self.assertEqual(num2words(1000, lang="pt"), "mil")
        self.assertEqual(num2words(1001, lang="pt"), "mil e um")
        self.assertEqual(num2words(1111, lang="pt"), "mil cento e onze")
        self.assertEqual(num2words(2114, lang="pt"), "dois mil cento e catorze")
        self.assertEqual(num2words(2200, lang="pt"), "dois mil e duzentos")
        self.assertEqual(num2words(2230, lang="pt"), "dois mil duzentos e trinta")
        self.assertEqual(
            num2words(73400, lang="pt"), "setenta e três mil e quatrocentos"
        )
        self.assertEqual(
            num2words(73421, lang="pt"), "setenta e três mil quatrocentos e vinte e um"
        )
        self.assertEqual(num2words(100000, lang="pt"), "cem mil")
        self.assertEqual(
            num2words(250050, lang="pt"), "duzentos e cinquenta mil e cinquenta"
        )
        self.assertEqual(num2words(6000000, lang="pt"), "seis milhões")
        self.assertEqual(num2words(100000000, lang="pt"), "cem milhões")
        self.assertEqual(num2words(19000000000, lang="pt"), "dezanove mil milhões")
        self.assertEqual(
            num2words(145000000002, lang="pt"),
            "cento e quarenta e cinco mil milhões e dois",
        )
        self.assertEqual(
            num2words(4635102, lang="pt"),
            "quatro milhões seiscentos e trinta e cinco mil cento e dois",
        )
        self.assertEqual(
            num2words(145254635102, lang="pt"),
            "cento e quarenta e cinco mil duzentos e cinquenta e quatro "
            "milhões seiscentos e trinta e cinco mil cento e dois",
        )
        self.assertEqual(num2words(1000000000000, lang="pt"), "um bilião")
        self.assertEqual(num2words(2000000000000, lang="pt"), "dois biliões")
        self.assertEqual(num2words(1000000000000000, lang="pt"), "mil biliões")
        self.assertEqual(num2words(2000000000000000, lang="pt"), "dois mil biliões")
        self.assertEqual(num2words(1000000000000000000, lang="pt"), "um trilião")
        self.assertEqual(num2words(2000000000000000000, lang="pt"), "dois triliões")

    def test_cardinal_integer_negative(self):
        self.assertEqual(num2words(-1, lang="pt"), "menos um")
        self.assertEqual(
            num2words(-256, lang="pt"), "menos duzentos e cinquenta e seis"
        )
        self.assertEqual(num2words(-1000, lang="pt"), "menos mil")
        self.assertEqual(num2words(-1000000, lang="pt"), "menos um milhão")
        self.assertEqual(
            num2words(-1234567, lang="pt"),
            "menos um milhão duzentos e trinta e quatro mil quinhentos e "
            "sessenta e sete",
        )

    def test_cardinal_float(self):
        self.assertEqual(num2words(Decimal("1.00"), lang="pt"), "um")
        self.assertEqual(num2words(Decimal("1.01"), lang="pt"), "um vírgula zero um")
        self.assertEqual(
            num2words(Decimal("1.035"), lang="pt"), "um vírgula zero três cinco"
        )
        self.assertEqual(num2words(Decimal("1.35"), lang="pt"), "um vírgula três cinco")
        self.assertEqual(
            num2words(Decimal("3.14159"), lang="pt"),
            "três vírgula um quatro um cinco nove",
        )
        self.assertEqual(
            num2words(Decimal("101.22"), lang="pt"), "cento e um vírgula dois dois"
        )
        self.assertEqual(
            num2words(Decimal("2345.75"), lang="pt"),
            "dois mil trezentos e quarenta e cinco vírgula sete cinco",
        )

    def test_cardinal_float_negative(self):
        self.assertEqual(
            num2words(Decimal("-2.34"), lang="pt"), "menos dois vírgula três quatro"
        )
        self.assertEqual(
            num2words(Decimal("-9.99"), lang="pt"), "menos nove vírgula nove nove"
        )
        self.assertEqual(
            num2words(Decimal("-7.01"), lang="pt"), "menos sete vírgula zero um"
        )
        self.assertEqual(
            num2words(Decimal("-222.22"), lang="pt"),
            "menos duzentos e vinte e dois vírgula dois dois",
        )

    def test_ordinal(self):
        self.assertEqual(num2words(1, lang="pt", ordinal=True), "primeiro")
        self.assertEqual(num2words(2, lang="pt", ordinal=True), "segundo")
        self.assertEqual(num2words(3, lang="pt", ordinal=True), "terceiro")
        self.assertEqual(num2words(4, lang="pt", ordinal=True), "quarto")
        self.assertEqual(num2words(5, lang="pt", ordinal=True), "quinto")
        self.assertEqual(num2words(6, lang="pt", ordinal=True), "sexto")
        self.assertEqual(num2words(7, lang="pt", ordinal=True), "sétimo")
        self.assertEqual(num2words(8, lang="pt", ordinal=True), "oitavo")
        self.assertEqual(num2words(9, lang="pt", ordinal=True), "nono")
        self.assertEqual(num2words(10, lang="pt", ordinal=True), "décimo")
        self.assertEqual(num2words(11, lang="pt", ordinal=True), "décimo primeiro")
        self.assertEqual(num2words(12, lang="pt", ordinal=True), "décimo segundo")
        self.assertEqual(num2words(13, lang="pt", ordinal=True), "décimo terceiro")
        self.assertEqual(num2words(14, lang="pt", ordinal=True), "décimo quarto")
        self.assertEqual(num2words(15, lang="pt", ordinal=True), "décimo quinto")
        self.assertEqual(num2words(16, lang="pt", ordinal=True), "décimo sexto")
        self.assertEqual(num2words(17, lang="pt", ordinal=True), "décimo sétimo")
        self.assertEqual(num2words(18, lang="pt", ordinal=True), "décimo oitavo")
        self.assertEqual(num2words(19, lang="pt", ordinal=True), "décimo nono")
        self.assertEqual(num2words(20, lang="pt", ordinal=True), "vigésimo")

        self.assertEqual(num2words(21, lang="pt", ordinal=True), "vigésimo primeiro")
        self.assertEqual(num2words(22, lang="pt", ordinal=True), "vigésimo segundo")
        self.assertEqual(num2words(35, lang="pt", ordinal=True), "trigésimo quinto")
        self.assertEqual(num2words(99, lang="pt", ordinal=True), "nonagésimo nono")

        self.assertEqual(num2words(100, lang="pt", ordinal=True), "centésimo")
        self.assertEqual(num2words(101, lang="pt", ordinal=True), "centésimo primeiro")
        self.assertEqual(
            num2words(128, lang="pt", ordinal=True), "centésimo vigésimo oitavo"
        )
        self.assertEqual(
            num2words(713, lang="pt", ordinal=True), "septigentésimo décimo terceiro"
        )

        self.assertEqual(num2words(1000, lang="pt", ordinal=True), "milésimo")
        self.assertEqual(num2words(1001, lang="pt", ordinal=True), "milésimo primeiro")
        self.assertEqual(
            num2words(1111, lang="pt", ordinal=True),
            "milésimo centésimo décimo primeiro",
        )
        self.assertEqual(
            num2words(2114, lang="pt", ordinal=True),
            "segundo milésimo centésimo décimo quarto",
        )
        self.assertEqual(
            num2words(73421, lang="pt", ordinal=True),
            "septuagésimo terceiro milésimo quadrigentésimo vigésimo primeiro",
        )

        self.assertEqual(
            num2words(100000, lang="pt", ordinal=True), "centésimo milésimo"
        )
        self.assertEqual(
            num2words(250050, lang="pt", ordinal=True),
            "ducentésimo quinquagésimo milésimo quinquagésimo",
        )
        self.assertEqual(
            num2words(6000000, lang="pt", ordinal=True), "sexto milionésimo"
        )
        self.assertEqual(
            num2words(19000000000, lang="pt", ordinal=True),
            "décimo nono milésimo milionésimo",
        )
        self.assertEqual(
            num2words(145000000002, lang="pt", ordinal=True),
            "centésimo quadragésimo quinto milésimo milionésimo segundo",
        )








    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="pt"), "menos zero vírgula quatro")
        self.assertEqual(num2words(-0.5, lang="pt"), "menos zero vírgula cinco")
        self.assertEqual(num2words(-0.04, lang="pt"), "menos zero vírgula zero quatro")
        self.assertEqual(num2words(-1.4, lang="pt"), "menos um vírgula quatro")
        self.assertEqual(num2words(-10.25, lang="pt"), "menos dez vírgula dois cinco")


def test_pt_handles_10_27_and_above():
    # Regression for num2words2#71 (ports savoirfairelinux/num2words#501).
    from num2words2 import num2words
    assert "quintilião" in num2words(10**30, lang="pt")
    assert "nonilião" in num2words(10**54, lang="pt")


def test_pt_ordinal_large_scale_words():
    # gladiaio/num2words2#173: KeyError('18') from 10**18; the ordinal now
    # follows the cardinal's long scale up to MAXVAL.
    import pytest

    from num2words2 import num2words
    assert num2words(10**18, lang="pt", to="ordinal") == "trilionésimo"
    assert num2words(2 * 10**21 + 5, lang="pt", to="ordinal") == (
        "segundo milésimo trilionésimo quinto"
    )
    assert num2words(10**24, lang="pt", to="ordinal") == "quatrilionésimo"
    assert num2words(10**36, lang="pt", to="ordinal") == "sextilionésimo"
    assert num2words(10**54, lang="pt", to="ordinal") == "nonilionésimo"
    assert num2words(10**57 - 1, lang="pt", to="ordinal").endswith("nono")
    with pytest.raises(OverflowError):
        num2words(10**57, lang="pt", to="ordinal")
    assert num2words(10**18, lang="pt", to="ordinal_num") == "%dº" % 10**18
