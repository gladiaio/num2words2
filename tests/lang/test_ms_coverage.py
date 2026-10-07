# -*- coding: utf-8 -*-
# Additional coverage tests for Malay language

from unittest import TestCase

from num2words2 import num2words


class Num2WordsMSCoverageTest(TestCase):
    """Additional tests to achieve 100% coverage for Malay."""

    def test_to_cardinal_string_input(self):
        """Test to_cardinal with string input."""
        self.assertEqual(num2words("42", lang="ms"), "empat puluh dua")
        self.assertEqual(num2words("0", lang="ms"), "kosong")

    def test_to_ordinal_string_input(self):
        """Test to_ordinal with string input."""
        self.assertEqual(num2words("5", lang="ms", to="ordinal"), "kelima")

    def test_to_ordinal_num_string_input(self):
        """Test to_ordinal_num with string input."""
        self.assertEqual(num2words("15", lang="ms", to="ordinal_num"), "ke-15")

    def test_to_currency_not_implemented(self):
        """Test currency with unsupported currency code."""
        # This should raise NotImplementedError
        with self.assertRaises(NotImplementedError):
            num2words(100, lang="ms", to="currency", currency="XXX")

    def test_to_currency_negative(self):
        """Test negative currency amounts."""
        self.assertEqual(
            num2words(-50.50, lang="ms", to="currency", currency="MYR"),
            "negatif lima puluh ringgit lima puluh sen",
        )

    def test_to_year_small_numbers(self):
        """Test year conversion for numbers less than 1000."""
        self.assertEqual(
            num2words(999, lang="ms", to="year"),
            "sembilan ratus sembilan puluh sembilan",
        )
        self.assertEqual(num2words(500, lang="ms", to="year"), "lima ratus")
