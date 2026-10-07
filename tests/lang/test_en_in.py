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


class Num2WordsENINTest(TestCase):
    def test_cardinal(self):
        self.assertEqual(num2words(1e5, lang="en_IN"), "one lakh")
        self.assertEqual(num2words(1e6, lang="en_IN"), "ten lakh")
        self.assertEqual(num2words(1e7, lang="en_IN"), "one crore")

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="en_IN"), "minus zero point four")
        self.assertEqual(num2words(-0.5, lang="en_IN"), "minus zero point five")
        self.assertEqual(num2words(-1.4, lang="en_IN"), "minus one point four")
        self.assertEqual(num2words(-10.25, lang="en_IN"), "minus ten point two five")

    def test_indian_numbering_system(self):
        """Test the unique lakh/crore system used in Indian subcontinent."""
        # Basic lakhs (1 lakh = 100,000)
        self.assertEqual(num2words(100000, lang="en_IN"), "one lakh")
        self.assertEqual(num2words(200000, lang="en_IN"), "two lakh")
        self.assertEqual(num2words(550000, lang="en_IN"), "five lakh, fifty thousand")
        self.assertEqual(
            num2words(999999, lang="en_IN"),
            "nine lakh, ninety-nine thousand, nine hundred and ninety-nine",
        )

        # Crores (1 crore = 10,000,000)
        self.assertEqual(num2words(10000000, lang="en_IN"), "one crore")
        self.assertEqual(num2words(50000000, lang="en_IN"), "five crore")
        self.assertEqual(
            num2words(12500000, lang="en_IN"), "one crore, twenty-five lakh"
        )

        # Mixed amounts showing typical Indian number grouping
        self.assertEqual(
            num2words(123456789, lang="en_IN"),
            "twelve crore, thirty-four lakh, fifty-six thousand, "
            "seven hundred and eighty-nine",
        )

    def test_basic_numbers(self):
        """Test basic numbers to ensure compatibility with Indian English."""
        # Single digits
        self.assertEqual(num2words(0, lang="en_IN"), "zero")
        self.assertEqual(num2words(5, lang="en_IN"), "five")
        self.assertEqual(num2words(9, lang="en_IN"), "nine")

        # Teens (same as standard English but important to verify)
        self.assertEqual(num2words(11, lang="en_IN"), "eleven")
        self.assertEqual(num2words(15, lang="en_IN"), "fifteen")
        self.assertEqual(num2words(19, lang="en_IN"), "nineteen")

        # Tens
        self.assertEqual(num2words(20, lang="en_IN"), "twenty")
        self.assertEqual(num2words(50, lang="en_IN"), "fifty")
        self.assertEqual(num2words(99, lang="en_IN"), "ninety-nine")

        # Hundreds (using 'and' as per British/Indian convention)
        self.assertEqual(num2words(100, lang="en_IN"), "one hundred")
        self.assertEqual(num2words(101, lang="en_IN"), "one hundred and one")
        self.assertEqual(num2words(555, lang="en_IN"), "five hundred and fifty-five")

        # Thousands (before lakh threshold)
        self.assertEqual(num2words(1000, lang="en_IN"), "one thousand")
        self.assertEqual(num2words(15000, lang="en_IN"), "fifteen thousand")
        self.assertEqual(
            num2words(99999, lang="en_IN"),
            "ninety-nine thousand, nine hundred and ninety-nine",
        )

    def test_ordinals(self):
        """Test ordinal numbers in Indian English."""
        self.assertEqual(num2words(1, lang="en_IN", to="ordinal"), "first")
        self.assertEqual(num2words(2, lang="en_IN", to="ordinal"), "second")
        self.assertEqual(num2words(3, lang="en_IN", to="ordinal"), "third")
        self.assertEqual(num2words(11, lang="en_IN", to="ordinal"), "eleventh")
        self.assertEqual(num2words(21, lang="en_IN", to="ordinal"), "twenty-first")
        self.assertEqual(num2words(100, lang="en_IN", to="ordinal"), "one hundredth")
        self.assertEqual(
            num2words(101, lang="en_IN", to="ordinal"), "one hundred and first"
        )

        # Ordinals with Indian numbers
        self.assertEqual(num2words(100000, lang="en_IN", to="ordinal"), "one lakhth")
        self.assertEqual(num2words(10000000, lang="en_IN", to="ordinal"), "one croreth")


def test_en_in_currency_defaults_to_inr():
    # Regression for savoirfairelinux/num2words#313 — en_IN default currency.
    from num2words2 import num2words

    assert "rupee" in num2words(1, to="currency", lang="en_IN")
    assert "rupee" in num2words(1.50, to="currency", lang="en_IN")
    assert num2words(639098, to="currency", lang="en_IN").endswith("rupees")
    # Explicit currency still wins.
    assert num2words(1, to="currency", currency="USD", lang="en_IN") == "one dollar"


def test_en_in_paise_subunit():
    from num2words2 import num2words

    assert "paise" in num2words(1.50, to="currency", lang="en_IN")
    assert "paisa" in num2words(0.01, to="currency", lang="en_IN")


def test_en_in_extends_past_kharab():
    # Regression for num2words2#72 (ports savoirfairelinux/num2words#414).
    from num2words2 import num2words

    assert num2words(10**11, lang="en_IN") == "one kharab"
    assert num2words(10**13, lang="en_IN") == "one neel"
    assert num2words(10**15, lang="en_IN") == "one padma"
    assert num2words(10**17, lang="en_IN") == "one shankh"
    assert num2words(10**19, lang="en_IN") == "one mahashankh"
