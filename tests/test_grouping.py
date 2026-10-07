"""Tests for num2words2.grouping (issue #66)."""

import unittest

from num2words2 import group_digits


# NOTE: these assertions were previously written as bare pytest-style
# functions, which `unittest discover` (the CI runner) does not collect, so
# they never executed. Expressed as a unittest.TestCase they run under the
# CI runner and exercise num2words2.grouping. Assertions are unchanged.
class GroupDigitsTest(unittest.TestCase):
    def test_western_grouping(self):
        self.assertEqual(group_digits(1234567, locale="western"), "1,234,567")
        self.assertEqual(group_digits(1000), "1,000")

    def test_indian_grouping(self):
        self.assertEqual(group_digits(100000, locale="indian"), "1,00,000")
        self.assertEqual(group_digits(12345678, locale="indian"), "1,23,45,678")
        self.assertEqual(group_digits(999, locale="indian"), "999")

    def test_chinese_grouping(self):
        self.assertEqual(group_digits(12345678, locale="chinese"), "1234,5678")

    def test_negative(self):
        self.assertEqual(group_digits(-12345678, locale="indian"), "-1,23,45,678")

    def test_zero(self):
        self.assertEqual(group_digits(0, locale="indian"), "0")

    def test_custom_separator(self):
        self.assertEqual(
            group_digits(100000, locale="indian", separator=" "), "1 00 000"
        )

    def test_unknown_locale_raises(self):
        with self.assertRaises(ValueError):
            group_digits(100, locale="bogus")

    def test_non_int_raises(self):
        with self.assertRaises(TypeError):
            group_digits(1.5)
