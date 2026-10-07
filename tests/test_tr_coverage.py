# -*- coding: utf-8 -*-
"""
Comprehensive tests to improve coverage for Turkish language module (lang_TR.py)
"""

from unittest import TestCase

from num2words2 import num2words


class TestTurkishCoverage(TestCase):
    """Tests to improve coverage for Turkish module."""

    def test_year_conversion(self):
        """Test year conversion."""
        # Common years
        self.assertEqual(num2words(2024, lang="tr", to="year"), "ikibinyirmidört")
        self.assertEqual(
            num2words(1999, lang="tr", to="year"), "bindokuzyüzdoksandokuz"
        )
        self.assertEqual(num2words(2000, lang="tr", to="year"), "ikibin")

    def test_main_api(self):
        """Test main API with various options."""
        # Cardinal
        self.assertEqual(num2words(42, lang="tr"), "kırkiki")

        # Ordinal
        self.assertEqual(num2words(42, lang="tr", to="ordinal"), "kırkikinci")

        # Ordinal num: TDK convention is digit + apostrophe + suffix, with
        # the suffix matching vowel harmony of the cardinal ('iki' ends in
        # a vowel → "nci" not "inci"). Issue savoirfairelinux/num2words#128.
        self.assertEqual(num2words(42, lang="tr", to="ordinal_num"), "42'nci")

        # Currency
        result = num2words(42, lang="tr", to="currency")
        self.assertIn("kırkiki", result)
        self.assertIn("lira", result)

        # Year
        self.assertEqual(num2words(2024, lang="tr", to="year"), "ikibinyirmidört")
