from unittest import TestCase

from num2words2 import num2words

# -*- coding: utf-8 -*-
# Additional coverage tests for Estonian language


class Num2WordsETCoverageTest(TestCase):
    """Additional tests to achieve 100% coverage for Estonian."""

    def test_to_cardinal_string_input(self):
        """Test to_cardinal with string input."""
        self.assertEqual(num2words("42", lang="et"), "nelikümmend kaks")
        self.assertEqual(num2words("0", lang="et"), "null")

    def test_to_ordinal_string_input(self):
        """Test to_ordinal with string input."""
        self.assertEqual(num2words("5", lang="et", to="ordinal"), "viies")

    def test_to_ordinal_num_string_input(self):
        """Test to_ordinal_num with string input."""
        self.assertEqual(num2words("15", lang="et", to="ordinal_num"), "15.")

    def test_to_currency_not_implemented(self):
        """Test currency with unsupported currency code."""
        # This should raise NotImplementedError
        with self.assertRaises(NotImplementedError):
            num2words(100, lang="et", to="currency", currency="XXX")

    def test_to_currency_negative(self):
        """Test negative currency amounts."""
        self.assertEqual(
            num2words(-50.50, lang="et", to="currency", currency="EUR"),
            "miinus viiskümmend eurot ja viiskümmend senti",
        )

    def test_to_currency_cents_only(self):
        """Test currency with only cents."""
        self.assertEqual(
            num2words(0.25, lang="et", to="currency", currency="EUR"),
            "null eurot ja kakskümmend viis senti",
        )

    def test_to_year_small_numbers(self):
        """Test year conversion for numbers less than 1000."""
        self.assertEqual(
            num2words(999, lang="et", to="year"), "üheksasada üheksakümmend üheksa"
        )
        self.assertEqual(num2words(500, lang="et", to="year"), "viissada")

    def test_large_numbers_with_no_remainder(self):
        """Test large round numbers."""
        self.assertEqual(num2words(1000, lang="et"), "tuhat")
        self.assertEqual(num2words(10000, lang="et"), "kümme tuhat")
        self.assertEqual(num2words(100000, lang="et"), "sada tuhat")
