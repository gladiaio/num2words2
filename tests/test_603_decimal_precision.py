# -*- coding: utf-8 -*-
"""Issue savoirfairelinux/num2words#603 — preserve Decimal precision.

When a value is passed as a string (or Decimal), the dispatcher routes it
through ``str_to_number`` which returns a Decimal. Before the fix, the
base ``to_cardinal_float`` immediately re-cast it to ``float()``, silently
losing precision at trillion scale (98_746_251_323_029.99 → .98).
"""
from __future__ import unicode_literals

import unittest
from decimal import Decimal

from num2words2 import num2words


class Test603DecimalPrecision(unittest.TestCase):
    def test_trillion_scale_string_input(self):
        # The exact float literal can't represent .99 at this magnitude,
        # but a string input goes through Decimal and must round-trip.
        out = num2words("98746251323029.99", lang="en")
        self.assertTrue(out.endswith("point nine nine"), out)
        self.assertIn("twenty-nine", out)

    def test_trillion_scale_decimal_input(self):
        out = num2words(Decimal("98746251323029.99"), lang="en")
        self.assertTrue(out.endswith("point nine nine"), out)

    def test_float_input_unchanged(self):
        # Float at trillion scale loses precision in the literal itself —
        # the library can't recover what Python's parser already lost.
        out = num2words(98746251323029.99, lang="en")
        self.assertTrue(
            out.endswith("point nine eight") or out.endswith("point nine nine"),
            out,
        )

    def test_small_decimals_unchanged(self):
        self.assertEqual(num2words("1.23", lang="en"), "one point two three")
        self.assertEqual(num2words("0.99", lang="en"), "zero point nine nine")
        self.assertEqual(
            num2words("-42.50", lang="en"),
            "minus forty-two point five zero",
        )

    def test_cross_language_string(self):
        # The fix lives in base.float2tuple/to_cardinal_float, so any
        # language using the base path benefits.
        for lang, suffix in [
            ("en", "point nine nine"),
            ("fr", "virgule neuf neuf"),
            ("es", "punto nueve nueve"),
            ("de", "Komma neun neun"),
        ]:
            out = num2words("98746251323029.99", lang=lang)
            self.assertTrue(out.endswith(suffix), (lang, out))


if __name__ == "__main__":
    unittest.main()
