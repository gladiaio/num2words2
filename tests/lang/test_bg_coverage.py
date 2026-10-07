from unittest import TestCase

from num2words2 import num2words

# -*- coding: utf-8 -*-
# Additional coverage tests for Bulgarian language


class Num2WordsBGCoverageTest(TestCase):
    """Additional tests to achieve 100% coverage for Bulgarian."""

    def test_to_cardinal_string_input(self):
        """Test to_cardinal with string input."""
        self.assertEqual(num2words("42", lang="bg"), "четиридесет и два")
        self.assertEqual(num2words("0", lang="bg"), "нула")

    def test_to_ordinal_string_input(self):
        """Test to_ordinal with string input."""
        self.assertEqual(num2words("5", lang="bg", to="ordinal"), "пети")

    def test_to_ordinal_num_string_input(self):
        """Test to_ordinal_num with string input."""
        self.assertEqual(num2words("15", lang="bg", to="ordinal_num"), "15-ти")

    def test_to_ordinal_num_special_suffixes(self):
        """Test ordinal_num with special suffix cases."""
        # Test numbers ending in 1
        self.assertEqual(num2words(31, lang="bg", to="ordinal_num"), "31-ви")
        self.assertEqual(num2words(41, lang="bg", to="ordinal_num"), "41-ви")

        # Test numbers ending in 2
        self.assertEqual(num2words(32, lang="bg", to="ordinal_num"), "32-ри")
        self.assertEqual(num2words(42, lang="bg", to="ordinal_num"), "42-ри")

        # Test numbers ending in 7, 8
        self.assertEqual(num2words(27, lang="bg", to="ordinal_num"), "27-ми")
        self.assertEqual(num2words(28, lang="bg", to="ordinal_num"), "28-ми")

        # Numbers ending in 00
        self.assertEqual(num2words(200, lang="bg", to="ordinal_num"), "200-ти")

    def test_to_currency_not_implemented(self):
        """Test currency with unsupported currency code."""
        # This should raise NotImplementedError
        with self.assertRaises(NotImplementedError):
            num2words(100, lang="bg", to="currency", currency="XXX")

    def test_to_currency_negative(self):
        """Test negative currency amounts."""
        self.assertEqual(
            num2words(-50.50, lang="bg", to="currency", currency="BGN"),
            "минус петдесет лева и петдесет стотинки",
        )

    def test_to_year_small_numbers(self):
        """Test year conversion for numbers less than 1000."""
        self.assertEqual(
            num2words(999, lang="bg", to="year"), "деветстотин деветдесет и девет"
        )
        self.assertEqual(num2words(500, lang="bg", to="year"), "петстотин")

    def test_special_hundreds(self):
        """Test special forms for 200 and 300."""
        self.assertEqual(num2words(200, lang="bg"), "двеста")
        self.assertEqual(num2words(300, lang="bg"), "триста")
