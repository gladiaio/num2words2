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


class Num2WordsISTest(TestCase):
    """Comprehensive test cases for Icelandic language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="is"), "núll")
        self.assertEqual(num2words(1, lang="is"), "einn")
        self.assertEqual(num2words(2, lang="is"), "tveir")
        self.assertEqual(num2words(3, lang="is"), "þrír")
        self.assertEqual(num2words(4, lang="is"), "fjórir")
        self.assertEqual(num2words(5, lang="is"), "fimm")
        self.assertEqual(num2words(6, lang="is"), "sex")
        self.assertEqual(num2words(7, lang="is"), "sjö")
        self.assertEqual(num2words(8, lang="is"), "átta")
        self.assertEqual(num2words(9, lang="is"), "níu")
        self.assertEqual(num2words(10, lang="is"), "tíu")
        self.assertEqual(num2words(11, lang="is"), "ellefu")
        self.assertEqual(num2words(12, lang="is"), "tólf")
        self.assertEqual(num2words(13, lang="is"), "þrettán")
        self.assertEqual(num2words(14, lang="is"), "fjórtán")
        self.assertEqual(num2words(15, lang="is"), "fimmtán")
        self.assertEqual(num2words(16, lang="is"), "sextán")
        self.assertEqual(num2words(17, lang="is"), "sautján")
        self.assertEqual(num2words(18, lang="is"), "átján")
        self.assertEqual(num2words(19, lang="is"), "nítján")
        self.assertEqual(num2words(20, lang="is"), "tuttugu")
        self.assertEqual(num2words(21, lang="is"), "tuttugu og einn")
        self.assertEqual(num2words(22, lang="is"), "tuttugu og tveir")
        self.assertEqual(num2words(23, lang="is"), "tuttugu og þrír")
        self.assertEqual(num2words(24, lang="is"), "tuttugu og fjórir")
        self.assertEqual(num2words(25, lang="is"), "tuttugu og fimm")
        self.assertEqual(num2words(26, lang="is"), "tuttugu og sex")
        self.assertEqual(num2words(27, lang="is"), "tuttugu og sjö")
        self.assertEqual(num2words(28, lang="is"), "tuttugu og átta")
        self.assertEqual(num2words(29, lang="is"), "tuttugu og níu")
        self.assertEqual(num2words(30, lang="is"), "þrjátíu")
        self.assertEqual(num2words(31, lang="is"), "þrjátíu og einn")
        self.assertEqual(num2words(35, lang="is"), "þrjátíu og fimm")
        self.assertEqual(num2words(40, lang="is"), "fjörutíu")
        self.assertEqual(num2words(45, lang="is"), "fjörutíu og fimm")
        self.assertEqual(num2words(50, lang="is"), "fimmtíu")
        self.assertEqual(num2words(55, lang="is"), "fimmtíu og fimm")
        self.assertEqual(num2words(60, lang="is"), "sextíu")
        self.assertEqual(num2words(65, lang="is"), "sextíu og fimm")
        self.assertEqual(num2words(70, lang="is"), "sjötíu")
        self.assertEqual(num2words(75, lang="is"), "sjötíu og fimm")
        self.assertEqual(num2words(80, lang="is"), "áttatíu")
        self.assertEqual(num2words(85, lang="is"), "áttatíu og fimm")
        self.assertEqual(num2words(90, lang="is"), "níutíu")
        self.assertEqual(num2words(95, lang="is"), "níutíu og fimm")
        self.assertEqual(num2words(99, lang="is"), "níutíu og níu")
        self.assertEqual(num2words(100, lang="is"), "eitt hundrað")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="is"), "eitt hundrað og einn")
        self.assertEqual(num2words(110, lang="is"), "eitt hundrað og tíu")
        self.assertEqual(num2words(111, lang="is"), "eitt hundrað og ellefu")
        self.assertEqual(num2words(120, lang="is"), "eitt hundrað og tuttugu")
        self.assertEqual(num2words(125, lang="is"), "eitt hundrað tuttugu og fimm")
        self.assertEqual(num2words(150, lang="is"), "eitt hundrað og fimmtíu")
        self.assertEqual(num2words(175, lang="is"), "eitt hundrað sjötíu og fimm")
        self.assertEqual(num2words(199, lang="is"), "eitt hundrað níutíu og níu")
        self.assertEqual(num2words(200, lang="is"), "tvö hundruð")
        self.assertEqual(num2words(201, lang="is"), "tvö hundruð og einn")
        self.assertEqual(num2words(210, lang="is"), "tvö hundruð og tíu")
        self.assertEqual(num2words(220, lang="is"), "tvö hundruð og tuttugu")
        self.assertEqual(num2words(250, lang="is"), "tvö hundruð og fimmtíu")
        self.assertEqual(num2words(299, lang="is"), "tvö hundruð níutíu og níu")
        self.assertEqual(num2words(300, lang="is"), "þrjú hundruð")
        self.assertEqual(num2words(333, lang="is"), "þrjú hundruð þrjátíu og þrír")
        self.assertEqual(num2words(400, lang="is"), "fjögur hundruð")
        self.assertEqual(num2words(444, lang="is"), "fjögur hundruð fjörutíu og fjórir")
        self.assertEqual(num2words(500, lang="is"), "fimm hundruð")
        self.assertEqual(num2words(555, lang="is"), "fimm hundruð fimmtíu og fimm")
        self.assertEqual(num2words(600, lang="is"), "sex hundruð")
        self.assertEqual(num2words(666, lang="is"), "sex hundruð sextíu og sex")
        self.assertEqual(num2words(700, lang="is"), "sjö hundruð")
        self.assertEqual(num2words(777, lang="is"), "sjö hundruð sjötíu og sjö")
        self.assertEqual(num2words(800, lang="is"), "átta hundruð")
        self.assertEqual(num2words(888, lang="is"), "átta hundruð áttatíu og átta")
        self.assertEqual(num2words(900, lang="is"), "níu hundruð")
        self.assertEqual(num2words(999, lang="is"), "níu hundruð níutíu og níu")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="is"), "eitt þúsund")
        self.assertEqual(num2words(1001, lang="is"), "eitt þúsund og einn")
        self.assertEqual(num2words(1010, lang="is"), "eitt þúsund og tíu")
        self.assertEqual(num2words(1100, lang="is"), "eitt þúsund og eitt hundrað")
        self.assertEqual(
            num2words(1111, lang="is"), "eitt þúsund eitt hundrað og ellefu"
        )
        self.assertEqual(
            num2words(1234, lang="is"), "eitt þúsund tvö hundruð þrjátíu og fjórir"
        )
        self.assertEqual(num2words(1500, lang="is"), "eitt þúsund fimm hundruð")
        self.assertEqual(
            num2words(1999, lang="is"), "eitt þúsund níu hundruð níutíu og níu"
        )
        self.assertEqual(num2words(2000, lang="is"), "tvö þúsund")
        self.assertEqual(num2words(2001, lang="is"), "tvö þúsund og einn")
        self.assertEqual(num2words(2020, lang="is"), "tvö þúsund og tuttugu")
        self.assertEqual(
            num2words(2222, lang="is"), "tvö þúsund tvö hundruð tuttugu og tveir"
        )
        self.assertEqual(num2words(3000, lang="is"), "þrjú þúsund")
        self.assertEqual(
            num2words(3333, lang="is"), "þrjú þúsund þrjú hundruð þrjátíu og þrír"
        )
        self.assertEqual(num2words(4000, lang="is"), "fjögur þúsund")
        self.assertEqual(
            num2words(4444, lang="is"),
            "fjögur þúsund fjögur hundruð fjörutíu og fjórir",
        )
        self.assertEqual(num2words(5000, lang="is"), "fimm þúsund")
        self.assertEqual(
            num2words(5555, lang="is"), "fimm þúsund fimm hundruð fimmtíu og fimm"
        )
        self.assertEqual(num2words(6000, lang="is"), "sex þúsund")
        self.assertEqual(
            num2words(6666, lang="is"), "sex þúsund sex hundruð sextíu og sex"
        )
        self.assertEqual(num2words(7000, lang="is"), "sjö þúsund")
        self.assertEqual(
            num2words(7777, lang="is"), "sjö þúsund sjö hundruð sjötíu og sjö"
        )
        self.assertEqual(num2words(8000, lang="is"), "átta þúsund")
        self.assertEqual(
            num2words(8888, lang="is"), "átta þúsund átta hundruð áttatíu og átta"
        )
        self.assertEqual(num2words(9000, lang="is"), "níu þúsund")
        self.assertEqual(
            num2words(9999, lang="is"), "níu þúsund níu hundruð níutíu og níu"
        )
        self.assertEqual(num2words(10000, lang="is"), "tíu þúsund")
        self.assertEqual(num2words(10001, lang="is"), "tíu þúsund og einn")
        self.assertEqual(
            num2words(11111, lang="is"), "ellefu þúsund eitt hundrað og ellefu"
        )
        self.assertEqual(
            num2words(12345, lang="is"), "tólf þúsund þrjú hundruð fjörutíu og fimm"
        )
        self.assertEqual(num2words(20000, lang="is"), "tuttugu þúsund")
        self.assertEqual(num2words(50000, lang="is"), "fimmtíu þúsund")
        self.assertEqual(
            num2words(99999, lang="is"),
            "níutíu og níu þúsund níu hundruð níutíu og níu",
        )
        self.assertEqual(num2words(100000, lang="is"), "eitt hundrað þúsund")
        self.assertEqual(
            num2words(123456, lang="is"),
            "eitt hundrað tuttugu og þrjú þúsund fjögur hundruð fimmtíu og sex",
        )
        self.assertEqual(num2words(200000, lang="is"), "tvö hundruð þúsund")
        self.assertEqual(num2words(500000, lang="is"), "fimm hundruð þúsund")
        self.assertEqual(
            num2words(654321, lang="is"),
            "sex hundruð fimmtíu og fjögur þúsund þrjú hundruð tuttugu og einn",
        )
        self.assertEqual(
            num2words(999999, lang="is"),
            "níu hundruð níutíu og níu þúsund níu hundruð níutíu og níu",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="is"), "ein milljón")
        self.assertEqual(num2words(1000001, lang="is"), "ein milljón og einn")
        self.assertEqual(
            num2words(1111111, lang="is"),
            "ein milljón eitt hundrað og ellefu þúsund eitt hundrað og ellefu",
        )
        self.assertEqual(
            num2words(1234567, lang="is"),
            "ein milljón tvö hundruð þrjátíu og fjögur þúsund fimm hundruð sextíu og sjö",
        )
        self.assertEqual(num2words(2000000, lang="is"), "tvær milljónir")
        self.assertEqual(num2words(5000000, lang="is"), "fimm milljónir")
        self.assertEqual(
            num2words(9999999, lang="is"),
            "níu milljónir níu hundruð níutíu og níu þúsund níu hundruð níutíu og níu",
        )
        self.assertEqual(num2words(10000000, lang="is"), "tíu milljónir")
        self.assertEqual(
            num2words(12345678, lang="is"),
            "tólf milljónir þrjú hundruð fjörutíu og fimm þúsund sex hundruð sjötíu og átta",
        )
        self.assertEqual(
            num2words(99999999, lang="is"),
            "níutíu og níu milljónir níu hundruð níutíu og níu þúsund níu hundruð níutíu og níu",
        )
        self.assertEqual(num2words(100000000, lang="is"), "eitt hundrað milljónir")
        self.assertEqual(
            num2words(123456789, lang="is"),
            "eitt hundrað tuttugu og þrjár milljónir fjögur hundruð fimmtíu og sex þúsund sjö hundruð áttatíu og níu",
        )
        self.assertEqual(
            num2words(999999999, lang="is"),
            "níu hundruð níutíu og níu milljónir níu hundruð níutíu og níu þúsund níu hundruð níutíu og níu",
        )
        self.assertEqual(num2words(1000000000, lang="is"), "einn milljarður")
        self.assertEqual(
            num2words(1234567890, lang="is"),
            "einn milljarður tvö hundruð þrjátíu og fjórar milljónir fimm hundruð sextíu og sjö þúsund átta hundruð og níutíu",
        )
        self.assertEqual(
            num2words(9999999999, lang="is"),
            "níu milljarðar níu hundruð níutíu og níu milljónir níu hundruð níutíu og níu þúsund níu hundruð níutíu og níu",
        )
        self.assertEqual(num2words(10000000000, lang="is"), "tíu milljarðar")
        self.assertEqual(
            num2words(99999999999, lang="is"),
            "níutíu og níu milljarðar níu hundruð níutíu og níu milljónir níu hundruð níutíu og níu þúsund níu hundruð níutíu og níu",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="is"), "mínus einn")
        self.assertEqual(num2words(-2, lang="is"), "mínus tveir")
        self.assertEqual(num2words(-5, lang="is"), "mínus fimm")
        self.assertEqual(num2words(-10, lang="is"), "mínus tíu")
        self.assertEqual(num2words(-11, lang="is"), "mínus ellefu")
        self.assertEqual(num2words(-20, lang="is"), "mínus tuttugu")
        self.assertEqual(num2words(-50, lang="is"), "mínus fimmtíu")
        self.assertEqual(num2words(-99, lang="is"), "mínus níutíu og níu")
        self.assertEqual(num2words(-100, lang="is"), "mínus eitt hundrað")
        self.assertEqual(num2words(-101, lang="is"), "mínus eitt hundrað og einn")
        self.assertEqual(num2words(-200, lang="is"), "mínus tvö hundruð")
        self.assertEqual(num2words(-999, lang="is"), "mínus níu hundruð níutíu og níu")
        self.assertEqual(num2words(-1000, lang="is"), "mínus eitt þúsund")
        self.assertEqual(num2words(-1001, lang="is"), "mínus eitt þúsund og einn")
        self.assertEqual(num2words(-10000, lang="is"), "mínus tíu þúsund")
        self.assertEqual(num2words(-100000, lang="is"), "mínus eitt hundrað þúsund")
        self.assertEqual(num2words(-1000000, lang="is"), "mínus ein milljón")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="is"), "núll komma einn")
        self.assertEqual(num2words(0.5, lang="is"), "núll komma fimm")
        self.assertEqual(num2words(0.9, lang="is"), "núll komma níu")
        self.assertEqual(num2words(1.1, lang="is"), "einn komma einn")
        self.assertEqual(num2words(1.5, lang="is"), "einn komma fimm")
        self.assertEqual(num2words(2.5, lang="is"), "tveir komma fimm")
        self.assertEqual(num2words(3.14, lang="is"), "þrír komma einn fjórir")
        self.assertEqual(num2words(10.5, lang="is"), "tíu komma fimm")
        self.assertEqual(num2words(11.11, lang="is"), "ellefu komma einn einn")
        self.assertEqual(num2words(20.2, lang="is"), "tuttugu komma tveir")
        self.assertEqual(num2words(99.99, lang="is"), "níutíu og níu komma níu níu")
        self.assertEqual(num2words(100.01, lang="is"), "eitt hundrað komma núll einn")
        self.assertEqual(num2words(100.5, lang="is"), "eitt hundrað komma fimm")
        self.assertEqual(
            num2words(123.45, lang="is"),
            "eitt hundrað tuttugu og þrír komma fjórir fimm",
        )
        self.assertEqual(num2words(1000.5, lang="is"), "eitt þúsund komma fimm")
        self.assertEqual(
            num2words(1234.56, lang="is"),
            "eitt þúsund tvö hundruð þrjátíu og fjórir komma fimm sex",
        )
        self.assertEqual(num2words(10000.01, lang="is"), "tíu þúsund komma núll einn")
        self.assertEqual(num2words(-0.5, lang="is"), "mínus núll komma fimm")
        self.assertEqual(num2words(-1.5, lang="is"), "mínus einn komma fimm")
        self.assertEqual(num2words(-10.5, lang="is"), "mínus tíu komma fimm")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="is", ordinal=True), "fyrsti")
        self.assertEqual(num2words(2, lang="is", ordinal=True), "annar")
        self.assertEqual(num2words(3, lang="is", ordinal=True), "þriðji")
        self.assertEqual(num2words(4, lang="is", ordinal=True), "fjórði")
        self.assertEqual(num2words(5, lang="is", ordinal=True), "fimmti")
        self.assertEqual(num2words(6, lang="is", ordinal=True), "sjötti")
        self.assertEqual(num2words(7, lang="is", ordinal=True), "sjöundi")
        self.assertEqual(num2words(8, lang="is", ordinal=True), "áttundi")
        self.assertEqual(num2words(9, lang="is", ordinal=True), "níundi")
        self.assertEqual(num2words(10, lang="is", ordinal=True), "tíundi")
        self.assertEqual(num2words(11, lang="is", ordinal=True), "ellefti")
        self.assertEqual(num2words(12, lang="is", ordinal=True), "tólfti")
        self.assertEqual(num2words(13, lang="is", ordinal=True), "þrettánasti")
        self.assertEqual(num2words(14, lang="is", ordinal=True), "fjórtánasti")
        self.assertEqual(num2words(15, lang="is", ordinal=True), "fimmtánasti")
        self.assertEqual(num2words(16, lang="is", ordinal=True), "sextánasti")
        self.assertEqual(num2words(17, lang="is", ordinal=True), "sautjánasti")
        self.assertEqual(num2words(18, lang="is", ordinal=True), "átjánasti")
        self.assertEqual(num2words(19, lang="is", ordinal=True), "nítjánasti")
        self.assertEqual(num2words(20, lang="is", ordinal=True), "tuttugasti")
        self.assertEqual(num2words(21, lang="is", ordinal=True), "tuttugasti og fyrsti")
        self.assertEqual(num2words(22, lang="is", ordinal=True), "tuttugasti og annar")
        self.assertEqual(num2words(25, lang="is", ordinal=True), "tuttugasti og fimmti")
        self.assertEqual(num2words(30, lang="is", ordinal=True), "þrítugasti")
        self.assertEqual(num2words(40, lang="is", ordinal=True), "fertugasti")
        self.assertEqual(num2words(50, lang="is", ordinal=True), "fimmtugasti")
        self.assertEqual(num2words(60, lang="is", ordinal=True), "sextugasti")
        self.assertEqual(num2words(70, lang="is", ordinal=True), "sjötugasti")
        self.assertEqual(num2words(80, lang="is", ordinal=True), "áttugasti")
        self.assertEqual(num2words(90, lang="is", ordinal=True), "nítugasti")
        self.assertEqual(num2words(100, lang="is", ordinal=True), "hundraðasti")
        self.assertEqual(
            num2words(101, lang="is", ordinal=True), "eitt hundrað og einnasti"
        )
        self.assertEqual(num2words(200, lang="is", ordinal=True), "tvö hundruðasti")
        self.assertEqual(num2words(500, lang="is", ordinal=True), "fimm hundruðasti")
        self.assertEqual(num2words(1000, lang="is", ordinal=True), "þúsundasti")
        self.assertEqual(
            num2words(1001, lang="is", ordinal=True), "eitt þúsund og einnasti"
        )
        self.assertEqual(num2words(10000, lang="is", ordinal=True), "tíu þúsundasti")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="is", to="currency", currency="ISK"), "núll krónur"
        )
        self.assertEqual(
            num2words(0.01, lang="is", to="currency", currency="ISK"),
            "núll krónur, einn eyrir",
        )
        self.assertEqual(
            num2words(0.5, lang="is", to="currency", currency="ISK"),
            "núll krónur, fimmtíu aurar",
        )
        self.assertEqual(
            num2words(1, lang="is", to="currency", currency="ISK"), "ein króna"
        )
        self.assertEqual(
            num2words(1.5, lang="is", to="currency", currency="ISK"),
            "ein króna, fimmtíu aurar",
        )
        self.assertEqual(
            num2words(0, lang="is", to="currency", currency="EUR"), "núll evrur"
        )
        self.assertEqual(
            num2words(0.01, lang="is", to="currency", currency="EUR"),
            "núll evrur, eitt sent",
        )
        self.assertEqual(
            num2words(0.5, lang="is", to="currency", currency="EUR"),
            "núll evrur, fimmtíu sent",
        )
        self.assertEqual(
            num2words(1, lang="is", to="currency", currency="EUR"), "ein evra"
        )
        self.assertEqual(
            num2words(1.5, lang="is", to="currency", currency="EUR"),
            "ein evra, fimmtíu sent",
        )
        self.assertEqual(
            num2words(0, lang="is", to="currency", currency="USD"), "núll dalir"
        )
        self.assertEqual(
            num2words(0.01, lang="is", to="currency", currency="USD"),
            "núll dalir, eitt sent",
        )
        self.assertEqual(
            num2words(0.5, lang="is", to="currency", currency="USD"),
            "núll dalir, fimmtíu sent",
        )
        self.assertEqual(
            num2words(1, lang="is", to="currency", currency="USD"), "einn dalur"
        )
        self.assertEqual(
            num2words(1.5, lang="is", to="currency", currency="USD"),
            "einn dalur, fimmtíu sent",
        )

    def test_currency_float_picks_form(self):
        # gladiaio/num2words2#169: floats/strings printed the forms tuple.
        self.assertEqual(
            num2words(5.5, lang="is", to="currency"), "fimm krónur, fimmtíu aurar"
        )
        self.assertEqual(num2words("-5", lang="is", to="currency"), "mínus fimm krónur")
        self.assertEqual(
            num2words(-5.0, lang="is", to="currency"), "mínus fimm krónur, núll aurar"
        )
        # Singular after a count ending in 1 (but not 11), int and float alike.
        for v in (21, 21.0):
            self.assertTrue(
                num2words(v, lang="is", to="currency").startswith(
                    "tuttugu og ein króna"
                )
            )
        self.assertEqual(num2words(11, lang="is", to="currency"), "ellefu krónur")

    def test_currency_numeral_gender(self):
        # gladiaio/num2words2#185: 1-4 agree with the noun (króna/evra f,
        # dalur/eyrir m, sent n), also in compounds, on int/float/str alike.
        cases = [
            (1, "ISK", "ein króna"),
            (2, "ISK", "tvær krónur"),
            (3, "ISK", "þrjár krónur"),
            (4, "ISK", "fjórar krónur"),
            (21, "ISK", "tuttugu og ein króna"),
            (1001, "ISK", "eitt þúsund og ein króna"),
            (-1, "ISK", "mínus ein króna"),
            ("22", "ISK", "tuttugu og tvær krónur"),
            (0.01, "ISK", "núll krónur, einn eyrir"),
            (2.02, "ISK", "tvær krónur, tveir aurar"),
            ("21.03", "ISK", "tuttugu og ein króna, þrír aurar"),
            (3.04, "EUR", "þrjár evrur, fjögur sent"),
            (21.01, "EUR", "tuttugu og ein evra, eitt sent"),
            (2, "USD", "tveir dalir"),
            (2.02, "USD", "tveir dalir, tvö sent"),
        ]
        for value, cur, want in cases:
            self.assertEqual(
                num2words(value, lang="is", to="currency", currency=cur), want
            )
        self.assertEqual(
            num2words(2.5, lang="is", to="currency", currency="ISK", adjective=True),
            "tvær íslenskar krónur, fimmtíu aurar",
        )
        # The plain cardinal stays masculine.
        self.assertEqual(num2words(21, lang="is"), "tuttugu og einn")

    def test_cheque_and_adjective_agree_with_the_count(self):
        # #197: singular after counts ending in 1 but not 11.
        cheques = [
            (1.0, "EIN AND 00/100 KRÓNA"),
            (2.0, "TVÆR AND 00/100 KRÓNUR"),
            (21.5, "TUTTUGU OG EIN AND 50/100 KRÓNA"),
            (11.0, "ELLEFU AND 00/100 KRÓNUR"),
        ]
        for value, want in cheques:
            self.assertEqual(num2words(value, lang="is", to="cheque"), want)
        self.assertEqual(
            num2words(1.5, lang="is", to="currency", adjective=True),
            "ein íslensk króna, fimmtíu aurar",
        )
        self.assertEqual(
            num2words(21.0, lang="is", to="currency", adjective=True),
            "tuttugu og ein íslensk króna, núll aurar",
        )
        self.assertEqual(
            num2words(11.0, lang="is", to="currency", adjective=True),
            "ellefu íslenskar krónur, núll aurar",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="is", to="year"), "eitt þúsund")
        self.assertEqual(
            num2words(1066, lang="is", to="year"), "eitt þúsund sextíu og sex"
        )
        self.assertEqual(
            num2words(1492, lang="is", to="year"),
            "eitt þúsund fjögur hundruð níutíu og tveir",
        )
        self.assertEqual(
            num2words(1776, lang="is", to="year"),
            "eitt þúsund sjö hundruð sjötíu og sex",
        )
        self.assertEqual(
            num2words(1800, lang="is", to="year"), "eitt þúsund átta hundruð"
        )
        self.assertEqual(
            num2words(1900, lang="is", to="year"), "eitt þúsund níu hundruð"
        )
        self.assertEqual(
            num2words(1984, lang="is", to="year"),
            "eitt þúsund níu hundruð áttatíu og fjórir",
        )
        self.assertEqual(
            num2words(1999, lang="is", to="year"),
            "eitt þúsund níu hundruð níutíu og níu",
        )
        self.assertEqual(num2words(2000, lang="is", to="year"), "tvö þúsund")
        self.assertEqual(num2words(2001, lang="is", to="year"), "tvö þúsund og einn")
        self.assertEqual(num2words(2010, lang="is", to="year"), "tvö þúsund og tíu")
        self.assertEqual(num2words(2020, lang="is", to="year"), "tvö þúsund og tuttugu")
        self.assertEqual(
            num2words(2024, lang="is", to="year"), "tvö þúsund tuttugu og fjórir"
        )
        self.assertEqual(
            num2words(2100, lang="is", to="year"), "tvö þúsund og eitt hundrað"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="is"), "núll")
        self.assertEqual(num2words("1", lang="is"), "einn")
        self.assertEqual(num2words("10", lang="is"), "tíu")
        self.assertEqual(num2words("100", lang="is"), "eitt hundrað")
        self.assertEqual(num2words("1000", lang="is"), "eitt þúsund")
        self.assertEqual(num2words("10000", lang="is"), "tíu þúsund")
        self.assertEqual(num2words("100000", lang="is"), "eitt hundrað þúsund")
        self.assertEqual(num2words("1000000", lang="is"), "ein milljón")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="is"), "núll")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="is"), num2words("100", lang="is"))
        self.assertEqual(num2words(1000, lang="is"), num2words("1000", lang="is"))

