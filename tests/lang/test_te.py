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


class Num2WordsTETest(TestCase):
    """Comprehensive test cases for Telugu language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="te"), "సున్న")
        self.assertEqual(num2words(1, lang="te"), "ఒకటి")
        self.assertEqual(num2words(2, lang="te"), "రెండు")
        self.assertEqual(num2words(3, lang="te"), "మూడు")
        self.assertEqual(num2words(4, lang="te"), "నాలుగు")
        self.assertEqual(num2words(5, lang="te"), "అయిదు")
        self.assertEqual(num2words(6, lang="te"), "ఆరు")
        self.assertEqual(num2words(7, lang="te"), "ఏడు")
        self.assertEqual(num2words(8, lang="te"), "ఎనిమిది")
        self.assertEqual(num2words(9, lang="te"), "తొమ్మిది")
        self.assertEqual(num2words(10, lang="te"), "పది")
        self.assertEqual(num2words(11, lang="te"), "పదకొండు")
        self.assertEqual(num2words(12, lang="te"), "పన్నెండు")
        self.assertEqual(num2words(13, lang="te"), "పదమూడు")
        self.assertEqual(num2words(14, lang="te"), "పధ్నాలుగు")
        self.assertEqual(num2words(15, lang="te"), "పదునయిదు")
        self.assertEqual(num2words(16, lang="te"), "పదహారు")
        self.assertEqual(num2words(17, lang="te"), "పదిహేడు")
        self.assertEqual(num2words(18, lang="te"), "పధ్ధెనిమిది")
        self.assertEqual(num2words(19, lang="te"), "పందొమ్మిది")
        self.assertEqual(num2words(20, lang="te"), "ఇరవై")
        self.assertEqual(num2words(21, lang="te"), "ఇరవై ఒకటి")
        self.assertEqual(num2words(22, lang="te"), "ఇరవై రెండు")
        self.assertEqual(num2words(23, lang="te"), "ఇరవై మూడు")
        self.assertEqual(num2words(24, lang="te"), "ఇరవై నాలుగు")
        self.assertEqual(num2words(25, lang="te"), "ఇరవై అయిదు")
        self.assertEqual(num2words(26, lang="te"), "ఇరవై ఆరు")
        self.assertEqual(num2words(27, lang="te"), "ఇరవై ఏడు")
        self.assertEqual(num2words(28, lang="te"), "ఇరవై ఎనిమిది")
        self.assertEqual(num2words(29, lang="te"), "ఇరవై తొమ్మిది")
        self.assertEqual(num2words(30, lang="te"), "ముప్పై")
        self.assertEqual(num2words(31, lang="te"), "ముప్పై ఒకటి")
        self.assertEqual(num2words(35, lang="te"), "ముప్పై ఐదు")
        self.assertEqual(num2words(40, lang="te"), "నలభై")
        self.assertEqual(num2words(45, lang="te"), "నలభై అయిదు")
        # 50 no longer carries a trailing space (#160).
        self.assertEqual(num2words(50, lang="te"), "యాభై")
        self.assertEqual(num2words(55, lang="te"), "యాభై అయిదు")
        self.assertEqual(num2words(60, lang="te"), "అరవై")
        self.assertEqual(num2words(65, lang="te"), "అరవై అయిదు")
        self.assertEqual(num2words(70, lang="te"), "డెబ్బై")
        self.assertEqual(num2words(75, lang="te"), "డెబ్బై అయిదు")
        self.assertEqual(num2words(80, lang="te"), "ఎనభై")
        self.assertEqual(num2words(85, lang="te"), "ఎనభై అయిదు")
        self.assertEqual(num2words(90, lang="te"), "తొంభై")
        self.assertEqual(num2words(95, lang="te"), "తొంభై అయిదు")
        self.assertEqual(num2words(99, lang="te"), "తొంభై తొమ్మిది")
        self.assertEqual(num2words(100, lang="te"), "ఒకటి వంద")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="te"), "ఒకటి వందల ఒకటి")
        self.assertEqual(num2words(110, lang="te"), "ఒకటి వందల పది")
        self.assertEqual(num2words(111, lang="te"), "ఒకటి వందల పదకొండు")
        self.assertEqual(num2words(120, lang="te"), "ఒకటి వందల ఇరవై")
        self.assertEqual(num2words(125, lang="te"), "ఒకటి వందల ఇరవై అయిదు")
        self.assertEqual(num2words(150, lang="te"), "ఒకటి వందల యాభై")
        self.assertEqual(num2words(175, lang="te"), "ఒకటి వందల డెబ్బై అయిదు")
        self.assertEqual(num2words(199, lang="te"), "ఒకటి వందల తొంభై తొమ్మిది")
        self.assertEqual(num2words(200, lang="te"), "రెండు వంద")
        self.assertEqual(num2words(201, lang="te"), "రెండు వందల ఒకటి")
        self.assertEqual(num2words(210, lang="te"), "రెండు వందల పది")
        self.assertEqual(num2words(220, lang="te"), "రెండు వందల ఇరవై")
        self.assertEqual(num2words(250, lang="te"), "రెండు వందల యాభై")
        self.assertEqual(num2words(299, lang="te"), "రెండు వందల తొంభై తొమ్మిది")
        self.assertEqual(num2words(300, lang="te"), "మూడు వంద")
        self.assertEqual(num2words(333, lang="te"), "మూడు వందల ముప్పై మూడు")
        self.assertEqual(num2words(400, lang="te"), "నాలుగు వంద")
        self.assertEqual(num2words(444, lang="te"), "నాలుగు వందల నలభై నాలుగు")
        self.assertEqual(num2words(500, lang="te"), "అయిదు వంద")
        self.assertEqual(num2words(555, lang="te"), "అయిదు వందల యాభై అయిదు")
        self.assertEqual(num2words(600, lang="te"), "ఆరు వంద")
        self.assertEqual(num2words(666, lang="te"), "ఆరు వందల అరవై ఆరు")
        self.assertEqual(num2words(700, lang="te"), "ఏడు వంద")
        self.assertEqual(num2words(777, lang="te"), "ఏడు వందల డెబ్బై ఏడు")
        self.assertEqual(num2words(800, lang="te"), "ఎనిమిది వంద")
        self.assertEqual(num2words(888, lang="te"), "ఎనిమిది వందల ఎనభై ఎనిమిది")
        self.assertEqual(num2words(900, lang="te"), "తొమ్మిది వంద")
        self.assertEqual(num2words(999, lang="te"), "తొమ్మిది వందల తొంభై తొమ్మిది")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="te"), "ఒకటి వేయి")
        self.assertEqual(num2words(1001, lang="te"), "ఒకటి వేయిల ఒకటి")
        self.assertEqual(num2words(1010, lang="te"), "ఒకటి వేయిల పది")
        self.assertEqual(num2words(1100, lang="te"), "ఒకటి వేయి ఒకటి వంద")
        self.assertEqual(num2words(1111, lang="te"), "ఒకటి వేయి ఒకటి వందల పదకొండు")
        self.assertEqual(
            num2words(1234, lang="te"), "ఒకటి వేయి రెండు వందల ముప్పై నాలుగు"
        )
        self.assertEqual(num2words(1500, lang="te"), "ఒకటి వేయి అయిదు వంద")
        self.assertEqual(
            num2words(1999, lang="te"), "ఒకటి వేయి తొమ్మిది వందల తొంభై తొమ్మిది"
        )
        self.assertEqual(num2words(2000, lang="te"), "రెండు వేయి")
        self.assertEqual(num2words(2001, lang="te"), "రెండు వేయిల ఒకటి")
        self.assertEqual(num2words(2020, lang="te"), "రెండు వేయిల ఇరవై")
        self.assertEqual(
            num2words(2222, lang="te"), "రెండు వేయి రెండు వందల ఇరవై రెండు"
        )
        self.assertEqual(num2words(3000, lang="te"), "మూడు వేయి")
        self.assertEqual(num2words(3333, lang="te"), "మూడు వేయి మూడు వందల ముప్పై మూడు")
        self.assertEqual(num2words(4000, lang="te"), "నాలుగు వేయి")
        self.assertEqual(
            num2words(4444, lang="te"), "నాలుగు వేయి నాలుగు వందల నలభై నాలుగు"
        )
        self.assertEqual(num2words(5000, lang="te"), "అయిదు వేయి")
        self.assertEqual(
            num2words(5555, lang="te"), "అయిదు వేయి అయిదు వందల యాభై అయిదు"
        )
        self.assertEqual(num2words(6000, lang="te"), "ఆరు వేయి")
        self.assertEqual(num2words(6666, lang="te"), "ఆరు వేయి ఆరు వందల అరవై ఆరు")
        self.assertEqual(num2words(7000, lang="te"), "ఏడు వేయి")
        self.assertEqual(num2words(7777, lang="te"), "ఏడు వేయి ఏడు వందల డెబ్బై ఏడు")
        self.assertEqual(num2words(8000, lang="te"), "ఎనిమిది వేయి")
        self.assertEqual(
            num2words(8888, lang="te"), "ఎనిమిది వేయి ఎనిమిది వందల ఎనభై ఎనిమిది"
        )
        self.assertEqual(num2words(9000, lang="te"), "తొమ్మిది వేయి")
        self.assertEqual(
            num2words(9999, lang="te"), "తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది"
        )
        self.assertEqual(num2words(10000, lang="te"), "పది వేయి")
        self.assertEqual(num2words(10001, lang="te"), "పది వేయిల ఒకటి")
        self.assertEqual(num2words(11111, lang="te"), "పదకొండు వేయి ఒకటి వందల పదకొండు")
        self.assertEqual(
            num2words(12345, lang="te"), "పన్నెండు వేయి మూడు వందల నలభై అయిదు"
        )
        self.assertEqual(num2words(20000, lang="te"), "ఇరవై వేయి")
        self.assertEqual(num2words(50000, lang="te"), "యాభై వేయి")
        self.assertEqual(
            num2words(99999, lang="te"),
            "తొంభై తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )
        self.assertEqual(num2words(100000, lang="te"), "ఒకటి లక్ష")
        self.assertEqual(
            num2words(123456, lang="te"),
            "ఒకటి లక్ష ఇరవై మూడు వేయి నాలుగు వందల యాభై ఆరు",
        )
        self.assertEqual(num2words(200000, lang="te"), "రెండు లక్ష")
        self.assertEqual(num2words(500000, lang="te"), "అయిదు లక్ష")
        self.assertEqual(
            num2words(654321, lang="te"),
            "ఆరు లక్ష యాభై నాలుగు వేయి మూడు వందల ఇరవై ఒకటి",
        )
        self.assertEqual(
            num2words(999999, lang="te"),
            "తొమ్మిది లక్ష తొంభై తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="te"), "పది లక్ష")
        self.assertEqual(num2words(1000001, lang="te"), "పది లక్షల ఒకటి")
        self.assertEqual(
            num2words(1111111, lang="te"), "పదకొండు లక్ష పదకొండు వేయి ఒకటి వందల పదకొండు"
        )
        self.assertEqual(
            num2words(1234567, lang="te"),
            "పన్నెండు లక్ష ముప్పై నాలుగు వేయి అయిదు వందల అరవై ఏడు",
        )
        self.assertEqual(num2words(2000000, lang="te"), "ఇరవై లక్ష")
        self.assertEqual(num2words(5000000, lang="te"), "యాభై లక్ష")
        self.assertEqual(
            num2words(9999999, lang="te"),
            "తొంభై తొమ్మిది లక్ష తొంభై తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )
        self.assertEqual(num2words(10000000, lang="te"), "ఒకటి కోట్ల")
        self.assertEqual(
            num2words(12345678, lang="te"),
            "ఒకటి కోట్ల ఇరవై మూడు లక్ష నలభై అయిదు వేయి ఆరు వందల డెబ్బై ఎనిమిది",
        )
        self.assertEqual(
            num2words(99999999, lang="te"),
            "తొమ్మిది కోట్ల తొంభై తొమ్మిది లక్ష తొంభై తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )
        self.assertEqual(num2words(100000000, lang="te"), "పది కోట్ల")
        self.assertEqual(
            num2words(123456789, lang="te"),
            "పన్నెండు కోట్ల ముప్పై నాలుగు లక్ష యాభై ఆరు వేయి ఏడు వందల ఎనభై తొమ్మిది",
        )
        self.assertEqual(
            num2words(999999999, lang="te"),
            "తొంభై తొమ్మిది కోట్ల తొంభై తొమ్మిది లక్ష తొంభై తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )
        self.assertEqual(num2words(1000000000, lang="te"), "ఒకటి వంద కోట్ల")
        self.assertEqual(
            num2words(1234567890, lang="te"),
            "ఒకటి వందల ఇరవై మూడు కోట్ల నలభై అయిదు లక్ష అరవై ఏడు వేయి ఎనిమిది వందల తొంభై",
        )
        self.assertEqual(
            num2words(9999999999, lang="te"),
            "తొమ్మిది వందల తొంభై తొమ్మిది కోట్ల తొంభై తొమ్మిది లక్ష తొంభై తొమ్మిది వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="te"), "(-) ఒకటి")
        self.assertEqual(num2words(-2, lang="te"), "(-) రెండు")
        self.assertEqual(num2words(-5, lang="te"), "(-) అయిదు")
        self.assertEqual(num2words(-10, lang="te"), "(-) పది")
        self.assertEqual(num2words(-11, lang="te"), "(-) పదకొండు")
        self.assertEqual(num2words(-20, lang="te"), "(-) ఇరవై")
        self.assertEqual(num2words(-50, lang="te"), "(-) యాభై")
        self.assertEqual(num2words(-99, lang="te"), "(-) తొంభై తొమ్మిది")
        self.assertEqual(num2words(-100, lang="te"), "(-) ఒకటి వంద")
        self.assertEqual(num2words(-101, lang="te"), "(-) ఒకటి వందల ఒకటి")
        self.assertEqual(num2words(-200, lang="te"), "(-) రెండు వంద")
        self.assertEqual(
            num2words(-999, lang="te"), "(-) తొమ్మిది వందల తొంభై తొమ్మిది"
        )
        self.assertEqual(num2words(-1000, lang="te"), "(-) ఒకటి వేయి")
        self.assertEqual(num2words(-1001, lang="te"), "(-) ఒకటి వేయిల ఒకటి")
        self.assertEqual(num2words(-10000, lang="te"), "(-) పది వేయి")
        self.assertEqual(num2words(-100000, lang="te"), "(-) ఒకటి లక్ష")
        self.assertEqual(num2words(-1000000, lang="te"), "(-) పది లక్ష")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="te"), "సున్న బిందువు ఒకటి")
        self.assertEqual(num2words(0.5, lang="te"), "సున్న బిందువు అయిదు")
        self.assertEqual(num2words(0.9, lang="te"), "సున్న బిందువు తొమ్మిది")
        self.assertEqual(num2words(1.1, lang="te"), "ఒకటి బిందువు ఒకటి")
        self.assertEqual(num2words(1.5, lang="te"), "ఒకటి బిందువు అయిదు")
        self.assertEqual(num2words(2.5, lang="te"), "రెండు బిందువు అయిదు")
        self.assertEqual(num2words(3.14, lang="te"), "మూడు బిందువు ఒకటి నాలుగు")
        self.assertEqual(num2words(10.5, lang="te"), "పది బిందువు అయిదు")
        self.assertEqual(num2words(11.11, lang="te"), "పదకొండు బిందువు ఒకటి ఒకటి")
        self.assertEqual(num2words(20.2, lang="te"), "ఇరవై బిందువు రెండు")
        self.assertEqual(
            num2words(99.99, lang="te"), "తొంభై తొమ్మిది బిందువు తొమ్మిది తొమ్మిది"
        )
        self.assertEqual(num2words(100.01, lang="te"), "ఒకటి వంద బిందువు సున్న ఒకటి")
        self.assertEqual(num2words(100.5, lang="te"), "ఒకటి వంద బిందువు అయిదు")
        self.assertEqual(
            num2words(123.45, lang="te"), "ఒకటి వందల ఇరవై మూడు బిందువు నాలుగు అయిదు"
        )
        self.assertEqual(num2words(1000.5, lang="te"), "ఒకటి వేయి బిందువు అయిదు")
        self.assertEqual(
            num2words(1234.56, lang="te"),
            "ఒకటి వేయి రెండు వందల ముప్పై నాలుగు బిందువు అయిదు ఆరు",
        )
        self.assertEqual(num2words(10000.01, lang="te"), "పది వేయి బిందువు సున్న ఒకటి")
        self.assertEqual(num2words(-0.5, lang="te"), "(-) సున్న బిందువు అయిదు")
        self.assertEqual(num2words(-1.5, lang="te"), "(-) ఒకటి బిందువు అయిదు")
        self.assertEqual(num2words(-10.5, lang="te"), "(-) పది బిందువు అయిదు")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="te", ordinal=True), "ఒకటివ")
        self.assertEqual(num2words(2, lang="te", ordinal=True), "రెండువ")
        self.assertEqual(num2words(3, lang="te", ordinal=True), "మూడువ")
        self.assertEqual(num2words(4, lang="te", ordinal=True), "నాలుగువ")
        self.assertEqual(num2words(5, lang="te", ordinal=True), "అయిదువ")
        self.assertEqual(num2words(6, lang="te", ordinal=True), "ఆరువ")
        self.assertEqual(num2words(7, lang="te", ordinal=True), "ఏడువ")
        self.assertEqual(num2words(8, lang="te", ordinal=True), "ఎనిమిదివ")
        self.assertEqual(num2words(9, lang="te", ordinal=True), "తొమ్మిదివ")
        self.assertEqual(num2words(10, lang="te", ordinal=True), "పదివ")
        self.assertEqual(num2words(11, lang="te", ordinal=True), "పదకొండువ")
        self.assertEqual(num2words(12, lang="te", ordinal=True), "పన్నెండువ")
        self.assertEqual(num2words(13, lang="te", ordinal=True), "పదమూడువ")
        self.assertEqual(num2words(14, lang="te", ordinal=True), "పధ్నాలుగువ")
        self.assertEqual(num2words(15, lang="te", ordinal=True), "పదునయిదువ")
        self.assertEqual(num2words(16, lang="te", ordinal=True), "పదహారువ")
        self.assertEqual(num2words(17, lang="te", ordinal=True), "పదిహేడువ")
        self.assertEqual(num2words(18, lang="te", ordinal=True), "పధ్ధెనిమిదివ")
        self.assertEqual(num2words(19, lang="te", ordinal=True), "పందొమ్మిదివ")
        self.assertEqual(num2words(20, lang="te", ordinal=True), "ఇరవైవ")
        self.assertEqual(num2words(21, lang="te", ordinal=True), "ఇరవై ఒకటివ")
        self.assertEqual(num2words(22, lang="te", ordinal=True), "ఇరవై రెండువ")
        self.assertEqual(num2words(25, lang="te", ordinal=True), "ఇరవై అయిదువ")
        self.assertEqual(num2words(30, lang="te", ordinal=True), "ముప్పైవ")
        self.assertEqual(num2words(40, lang="te", ordinal=True), "నలభైవ")
        self.assertEqual(num2words(50, lang="te", ordinal=True), "యాభైవ")
        self.assertEqual(num2words(60, lang="te", ordinal=True), "అరవైవ")
        self.assertEqual(num2words(70, lang="te", ordinal=True), "డెబ్బైవ")
        self.assertEqual(num2words(80, lang="te", ordinal=True), "ఎనభైవ")
        self.assertEqual(num2words(90, lang="te", ordinal=True), "తొంభైవ")
        self.assertEqual(num2words(100, lang="te", ordinal=True), "ఒకటి వందవ")
        self.assertEqual(num2words(101, lang="te", ordinal=True), "ఒకటి వందల ఒకటివ")
        self.assertEqual(num2words(200, lang="te", ordinal=True), "రెండు వందవ")
        self.assertEqual(num2words(500, lang="te", ordinal=True), "అయిదు వందవ")
        self.assertEqual(num2words(1000, lang="te", ordinal=True), "ఒకటి వేయివ")
        self.assertEqual(num2words(1001, lang="te", ordinal=True), "ఒకటి వేయిల ఒకటివ")
        self.assertEqual(num2words(10000, lang="te", ordinal=True), "పది వేయివ")

    def test_currency(self):
        """Test currency conversion."""
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="AUD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="BYN")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="CAD")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="EEK")
        self.assertEqual(
            num2words(0, lang="te", to="currency", currency="EUR"), "సున్న యూరోలు"
        )
        self.assertEqual(
            num2words(0.01, lang="te", to="currency", currency="EUR"),
            "సున్న యూరోలు, ఒకటి సెంట్",
        )
        self.assertEqual(
            num2words(0.5, lang="te", to="currency", currency="EUR"),
            "సున్న యూరోలు, యాభై సెంట్లు",
        )
        self.assertEqual(
            num2words(1, lang="te", to="currency", currency="EUR"), "ఒకటి యూరో"
        )
        self.assertEqual(
            num2words(1.5, lang="te", to="currency", currency="EUR"),
            "ఒకటి యూరో, యాభై సెంట్లు",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="GBP")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="LTL")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="LVL")
        self.assertEqual(
            num2words(0, lang="te", to="currency", currency="USD"), "సున్న డాలర్లు"
        )
        self.assertEqual(
            num2words(0.01, lang="te", to="currency", currency="USD"),
            "సున్న డాలర్లు, ఒకటి సెంట్",
        )
        self.assertEqual(
            num2words(0.5, lang="te", to="currency", currency="USD"),
            "సున్న డాలర్లు, యాభై సెంట్లు",
        )
        self.assertEqual(
            num2words(1, lang="te", to="currency", currency="USD"), "ఒకటి డాలర్"
        )
        self.assertEqual(
            num2words(1.5, lang="te", to="currency", currency="USD"),
            "ఒకటి డాలర్, యాభై సెంట్లు",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="RUB")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="SEK")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="NOK")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="PLN")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="MXN")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="RON")
        self.assertEqual(
            num2words(0, lang="te", to="currency", currency="INR"), "సున్న రూపాయలు"
        )
        self.assertEqual(
            num2words(0.01, lang="te", to="currency", currency="INR"),
            "సున్న రూపాయలు, ఒకటి పైసా",
        )
        self.assertEqual(
            num2words(0.5, lang="te", to="currency", currency="INR"),
            "సున్న రూపాయలు, యాభై పైసలు",
        )
        self.assertEqual(
            num2words(1, lang="te", to="currency", currency="INR"), "ఒకటి రూపాయి"
        )
        self.assertEqual(
            num2words(1.5, lang="te", to="currency", currency="INR"),
            "ఒకటి రూపాయి, యాభై పైసలు",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="HUF")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="ISK")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="UZS")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="SAR")
        self.assertEqual(
            num2words(0, lang="te", to="currency", currency="JPY"), "సున్న యెన్‌లు"
        )
        self.assertEqual(
            num2words(0.01, lang="te", to="currency", currency="JPY"),
            "సున్న యెన్‌లు, ఒకటి సెన్",
        )
        self.assertEqual(
            num2words(0.5, lang="te", to="currency", currency="JPY"),
            "సున్న యెన్‌లు, యాభై సెన్‌లు",
        )
        self.assertEqual(
            num2words(1, lang="te", to="currency", currency="JPY"), "ఒకటి యెన్"
        )
        self.assertEqual(
            num2words(1.5, lang="te", to="currency", currency="JPY"),
            "ఒకటి యెన్, యాభై సెన్‌లు",
        )
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="KRW")
        # No native noun for this code (gladiaio/num2words2#222).
        with self.assertRaises(NotImplementedError):
            num2words(0, lang="te", to="currency", currency="NGN")

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="te", to="year"), "ఒకటి వేయి")
        self.assertEqual(num2words(1066, lang="te", to="year"), "ఒకటి వేయిల అరవై ఆరు")
        self.assertEqual(
            num2words(1492, lang="te", to="year"), "ఒకటి వేయి నాలుగు వందల తొంభై రెండు"
        )
        self.assertEqual(
            num2words(1776, lang="te", to="year"), "ఒకటి వేయి ఏడు వందల డెబ్బై ఆరు"
        )
        self.assertEqual(num2words(1800, lang="te", to="year"), "ఒకటి వేయి ఎనిమిది వంద")
        self.assertEqual(
            num2words(1900, lang="te", to="year"), "ఒకటి వేయి తొమ్మిది వంద"
        )
        self.assertEqual(
            num2words(1984, lang="te", to="year"),
            "ఒకటి వేయి తొమ్మిది వందల ఎనభై నాలుగు",
        )
        self.assertEqual(
            num2words(1999, lang="te", to="year"),
            "ఒకటి వేయి తొమ్మిది వందల తొంభై తొమ్మిది",
        )
        self.assertEqual(num2words(2000, lang="te", to="year"), "రెండు వేయి")
        self.assertEqual(num2words(2001, lang="te", to="year"), "రెండు వేయిల ఒకటి")
        self.assertEqual(num2words(2010, lang="te", to="year"), "రెండు వేయిల పది")
        self.assertEqual(num2words(2020, lang="te", to="year"), "రెండు వేయిల ఇరవై")
        self.assertEqual(
            num2words(2024, lang="te", to="year"), "రెండు వేయిల ఇరవై నాలుగు"
        )
        self.assertEqual(num2words(2100, lang="te", to="year"), "రెండు వేయి ఒకటి వంద")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="te"), "సున్న")
        self.assertEqual(num2words("1", lang="te"), "ఒకటి")
        self.assertEqual(num2words("10", lang="te"), "పది")
        self.assertEqual(num2words("100", lang="te"), "ఒకటి వంద")
        self.assertEqual(num2words("1000", lang="te"), "ఒకటి వేయి")
        self.assertEqual(num2words("10000", lang="te"), "పది వేయి")
        self.assertEqual(num2words("100000", lang="te"), "ఒకటి లక్ష")
        self.assertEqual(num2words("1000000", lang="te"), "పది లక్ష")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="te"), "సున్న")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="te"), num2words("100", lang="te"))
        self.assertEqual(num2words(1000, lang="te"), num2words("1000", lang="te"))

