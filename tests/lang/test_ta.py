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


class Num2WordsTATest(TestCase):
    def test_cardinal_basics(self):
        """Test basic cardinal numbers in Tamil."""
        self.assertEqual(num2words(0, lang="ta"), "பூஜ்ஜியம்")
        self.assertEqual(num2words(1, lang="ta"), "ஒன்று")
        self.assertEqual(num2words(2, lang="ta"), "இரண்டு")
        self.assertEqual(num2words(3, lang="ta"), "மூன்று")
        self.assertEqual(num2words(4, lang="ta"), "நான்கு")
        self.assertEqual(num2words(5, lang="ta"), "ஐந்து")
        self.assertEqual(num2words(6, lang="ta"), "ஆறு")
        self.assertEqual(num2words(7, lang="ta"), "ஏழு")
        self.assertEqual(num2words(8, lang="ta"), "எட்டு")
        self.assertEqual(num2words(9, lang="ta"), "ஒன்பது")

    def test_cardinal_tens(self):
        """Test tens in Tamil."""
        self.assertEqual(num2words(10, lang="ta"), "பத்து")
        self.assertEqual(num2words(11, lang="ta"), "பதினொன்று")
        self.assertEqual(num2words(12, lang="ta"), "பன்னிரண்டு")
        self.assertEqual(num2words(13, lang="ta"), "பதின்மூன்று")
        self.assertEqual(num2words(14, lang="ta"), "பதினான்கு")
        self.assertEqual(num2words(15, lang="ta"), "பதினைந்து")
        self.assertEqual(num2words(16, lang="ta"), "பதினாறு")
        self.assertEqual(num2words(17, lang="ta"), "பதினேழு")
        self.assertEqual(num2words(18, lang="ta"), "பதினெட்டு")
        self.assertEqual(num2words(19, lang="ta"), "பத்தொன்பது")
        self.assertEqual(num2words(20, lang="ta"), "இருபது")
        self.assertEqual(num2words(30, lang="ta"), "முப்பது")
        self.assertEqual(num2words(40, lang="ta"), "நாற்பது")
        self.assertEqual(num2words(50, lang="ta"), "ஐம்பது")
        self.assertEqual(num2words(60, lang="ta"), "அறுபது")
        self.assertEqual(num2words(70, lang="ta"), "எழுபது")
        self.assertEqual(num2words(80, lang="ta"), "எண்பது")
        self.assertEqual(num2words(90, lang="ta"), "தொண்ணூறு")

    def test_cardinal_compound_tens(self):
        """Test compound numbers with tens."""
        self.assertEqual(num2words(21, lang="ta"), "இருபது ஒன்று")
        self.assertEqual(num2words(32, lang="ta"), "முப்பது இரண்டு")
        self.assertEqual(num2words(45, lang="ta"), "நாற்பது ஐந்து")
        self.assertEqual(num2words(67, lang="ta"), "அறுபது ஏழு")
        self.assertEqual(num2words(89, lang="ta"), "எண்பது ஒன்பது")
        self.assertEqual(num2words(99, lang="ta"), "தொண்ணூறு ஒன்பது")

    def test_cardinal_hundreds(self):
        """Test hundreds in Tamil."""
        self.assertEqual(num2words(100, lang="ta"), "நூறு")
        self.assertEqual(num2words(200, lang="ta"), "இருநூறு")
        self.assertEqual(num2words(300, lang="ta"), "முன்னூறு")
        self.assertEqual(num2words(400, lang="ta"), "நானூறு")
        self.assertEqual(num2words(500, lang="ta"), "ஐநூறு")
        self.assertEqual(num2words(600, lang="ta"), "அறுநூறு")
        self.assertEqual(num2words(700, lang="ta"), "எழுநூறு")
        self.assertEqual(num2words(800, lang="ta"), "எண்ணூறு")
        self.assertEqual(num2words(900, lang="ta"), "தொள்ளாயிரம்")

    def test_cardinal_complex_hundreds(self):
        """Test complex numbers with hundreds."""
        self.assertEqual(num2words(101, lang="ta"), "நூறு ஒன்று")
        self.assertEqual(num2words(111, lang="ta"), "நூறு பதினொன்று")
        self.assertEqual(num2words(212, lang="ta"), "இருநூறு பன்னிரண்டு")
        self.assertEqual(num2words(345, lang="ta"), "முன்னூறு நாற்பது ஐந்து")
        self.assertEqual(num2words(567, lang="ta"), "ஐநூறு அறுபது ஏழு")
        self.assertEqual(num2words(891, lang="ta"), "எண்ணூறு தொண்ணூறு ஒன்று")
        self.assertEqual(num2words(999, lang="ta"), "தொள்ளாயிரம் தொண்ணூறு ஒன்பது")

    def test_cardinal_thousands(self):
        """Test thousands in Tamil."""
        self.assertEqual(num2words(1000, lang="ta"), "ஆயிரம்")
        self.assertEqual(num2words(2000, lang="ta"), "இரண்டு ஆயிரம்")
        self.assertEqual(num2words(3000, lang="ta"), "மூன்று ஆயிரம்")
        self.assertEqual(num2words(5000, lang="ta"), "ஐந்து ஆயிரம்")
        self.assertEqual(num2words(10000, lang="ta"), "பத்து ஆயிரம்")
        self.assertEqual(num2words(15000, lang="ta"), "பதினைந்து ஆயிரம்")
        self.assertEqual(num2words(25000, lang="ta"), "இருபது ஐந்து ஆயிரம்")
        self.assertEqual(
            num2words(99999, lang="ta"),
            "தொண்ணூறு ஒன்பது ஆயிரம் தொள்ளாயிரம் தொண்ணூறு ஒன்பது",
        )

    def test_indian_numbering_system(self):
        """Test the lakh/crore system used in Tamil."""
        # Basic lakhs (1 lakh = 100,000)
        self.assertEqual(num2words(100000, lang="ta"), "ஒரு இலட்சம்")
        self.assertEqual(num2words(200000, lang="ta"), "இரண்டு இலட்சம்")
        self.assertEqual(num2words(550000, lang="ta"), "ஐந்து இலட்சம் ஐம்பது ஆயிரம்")
        self.assertEqual(
            num2words(999999, lang="ta"),
            "ஒன்பது இலட்சம் தொண்ணூறு ஒன்பது ஆயிரம் " "தொள்ளாயிரம் தொண்ணூறு ஒன்பது",
        )

        # Crores (1 crore = 10,000,000)
        self.assertEqual(num2words(10000000, lang="ta"), "ஒரு கோடி")
        self.assertEqual(num2words(50000000, lang="ta"), "ஐந்து கோடி")
        self.assertEqual(
            num2words(12500000, lang="ta"), "ஒரு கோடி இருபது ஐந்து இலட்சம்"
        )

    def test_negative_numbers(self):
        """Test negative numbers in Tamil."""
        self.assertEqual(num2words(-1, lang="ta"), "கழித்தல் ஒன்று")
        self.assertEqual(num2words(-10, lang="ta"), "கழித்தல் பத்து")
        self.assertEqual(num2words(-25, lang="ta"), "கழித்தல் இருபது ஐந்து")
        self.assertEqual(num2words(-100, lang="ta"), "கழித்தல் நூறு")
        self.assertEqual(num2words(-1000, lang="ta"), "கழித்தல் ஆயிரம்")

    def test_ordinal_basics(self):
        """Test basic ordinal numbers in Tamil."""
        self.assertEqual(num2words(1, lang="ta", to="ordinal"), "முதல்")
        self.assertEqual(num2words(2, lang="ta", to="ordinal"), "இரண்டாம்")
        self.assertEqual(num2words(3, lang="ta", to="ordinal"), "மூன்றாம்")
        self.assertEqual(num2words(4, lang="ta", to="ordinal"), "நான்காம்")
        self.assertEqual(num2words(5, lang="ta", to="ordinal"), "ஐந்தாம்")
        self.assertEqual(num2words(6, lang="ta", to="ordinal"), "ஆறாம்")
        self.assertEqual(num2words(7, lang="ta", to="ordinal"), "ஏழாம்")
        self.assertEqual(num2words(8, lang="ta", to="ordinal"), "எட்டாம்")
        self.assertEqual(num2words(9, lang="ta", to="ordinal"), "ஒன்பதாம்")
        self.assertEqual(num2words(10, lang="ta", to="ordinal"), "பத்தாம்")

    def test_ordinal_tens(self):
        """Test ordinal tens in Tamil."""
        self.assertEqual(num2words(11, lang="ta", to="ordinal"), "பதினொன்றுஆவது")
        self.assertEqual(num2words(12, lang="ta", to="ordinal"), "பன்னிரண்டுஆவது")
        self.assertEqual(num2words(20, lang="ta", to="ordinal"), "இருபதுஆவது")
        self.assertEqual(num2words(30, lang="ta", to="ordinal"), "முப்பதுஆவது")
        self.assertEqual(num2words(50, lang="ta", to="ordinal"), "ஐம்பதுஆவது")
        self.assertEqual(num2words(100, lang="ta", to="ordinal"), "நூறுஆவது")
        self.assertEqual(num2words(1000, lang="ta", to="ordinal"), "ஆயிரம்ஆவது")

    def test_ordinal_num(self):
        """Test ordinal number formatting in Tamil."""
        self.assertEqual(num2words(1, lang="ta", to="ordinal_num"), "1-வது")
        self.assertEqual(num2words(2, lang="ta", to="ordinal_num"), "2-வது")
        self.assertEqual(num2words(3, lang="ta", to="ordinal_num"), "3-வது")
        self.assertEqual(num2words(10, lang="ta", to="ordinal_num"), "10-வது")
        self.assertEqual(num2words(21, lang="ta", to="ordinal_num"), "21-வது")
        self.assertEqual(num2words(100, lang="ta", to="ordinal_num"), "100-வது")

    def test_currency(self):
        """Test currency in Tamil."""
        self.assertEqual(
            num2words(1, lang="ta", to="currency", currency="INR"), "ஒன்று ரூபாய்"
        )
        self.assertEqual(
            num2words(2, lang="ta", to="currency", currency="INR"), "இரண்டு ரூபாய்"
        )
        self.assertEqual(
            num2words(5.50, lang="ta", to="currency", currency="INR"),
            "ஐந்து ரூபாய் ஐம்பது பைசா",
        )
        self.assertEqual(
            num2words(10.01, lang="ta", to="currency", currency="INR"),
            "பத்து ரூபாய் ஒன்று பைசா",
        )
        self.assertEqual(
            num2words(100.25, lang="ta", to="currency", currency="INR"),
            "நூறு ரூபாய் இருபது ஐந்து பைசா",
        )
        self.assertEqual(
            num2words(1, lang="ta", to="currency", currency="USD"), "ஒன்று டாலர்"
        )
        self.assertEqual(
            num2words(5.25, lang="ta", to="currency", currency="USD"),
            "ஐந்து டாலர் இருபது ஐந்து சென்ட்",
        )

    def test_year(self):
        """Test year conversion in Tamil."""
        self.assertEqual(num2words(1900, lang="ta", to="year"), "ஆயிரத்து தொள்ளாயிரம்")
        self.assertEqual(
            num2words(1999, lang="ta", to="year"),
            "ஆயிரத்து தொள்ளாயிரம் தொண்ணூறு ஒன்பது",
        )
        self.assertEqual(num2words(2000, lang="ta", to="year"), "இரண்டு ஆயிரத்து")
        self.assertEqual(num2words(2001, lang="ta", to="year"), "இரண்டு ஆயிரத்து ஒன்று")
        self.assertEqual(
            num2words(2020, lang="ta", to="year"), "இரண்டு ஆயிரத்து இருபது"
        )
        self.assertEqual(
            num2words(2023, lang="ta", to="year"), "இரண்டு ஆயிரத்து இருபது மூன்று"
        )

    def test_fraction_is_read(self):
        # gladiaio/num2words2#206: the fraction was dropped (2.5 -> "இரண்டு").
        from decimal import Decimal
        for v in ("2.5", 2.5, Decimal("2.5")):
            self.assertEqual(num2words(v, lang="ta"), "இரண்டு புள்ளி ஐந்து")
        self.assertEqual(num2words(0.5, lang="ta"), "பூஜ்ஜியம் புள்ளி ஐந்து")
        self.assertEqual(num2words(-0.25, lang="ta"),
                         "கழித்தல் பூஜ்ஜியம் புள்ளி இரண்டு ஐந்து")
        self.assertEqual(num2words(5.0, lang="ta"), "ஐந்து")
