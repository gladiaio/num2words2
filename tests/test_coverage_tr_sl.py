# -*- coding: utf-8 -*-
"""Coverage-extending tests for lang_TR.py and lang_SL.py.

Targets the upstream tickets savoirfairelinux/num2words#128 (TR) and
#118 (SL): each file should be at >= 90% statement coverage. The tests
here drive specific code paths that the existing fixture-style tests
don't reach: TR multi-triplet ordinals, TR `_to_cardinal_impl` early
returns, TR `_insert_spaces`, the `spaced=`/`precision=`/`decimal_word=`
kwargs, and the SL morphology branches at million/billion/trillion
scale where 1/2/3/4 inflect the noun differently.
"""
from __future__ import unicode_literals

import unittest

from num2words2 import num2words


class TestTRCoverageExtra(unittest.TestCase):
    """Drive lang_TR.py branches the existing tests don't reach."""

    def test_zero(self):
        self.assertEqual(num2words(0, lang="tr"), "sıfır")

    def test_single_digit(self):
        for v, w in [(1, "bir"), (5, "beş"), (9, "dokuz")]:
            self.assertEqual(num2words(v, lang="tr"), w)

    def test_ten_to_ninety_nine(self):
        self.assertEqual(num2words(10, lang="tr"), "on")
        self.assertEqual(num2words(11, lang="tr"), "onbir")
        self.assertEqual(num2words(50, lang="tr"), "elli")
        self.assertEqual(num2words(99, lang="tr"), "doksandokuz")

    def test_hundreds(self):
        self.assertEqual(num2words(100, lang="tr"), "yüz")
        self.assertEqual(num2words(101, lang="tr"), "yüzbir")
        self.assertEqual(num2words(199, lang="tr"), "yüzdoksandokuz")
        self.assertEqual(num2words(500, lang="tr"), "beşyüz")

    def test_thousands(self):
        self.assertEqual(num2words(1000, lang="tr"), "bin")
        self.assertEqual(num2words(1001, lang="tr"), "binbir")
        self.assertEqual(num2words(2000, lang="tr"), "ikibin")
        self.assertEqual(num2words(10000, lang="tr"), "onbin")
        self.assertEqual(num2words(100000, lang="tr"), "yüzbin")

    def test_millions(self):
        self.assertEqual(num2words(1_000_000, lang="tr"), "birmilyon")
        self.assertEqual(num2words(2_000_000, lang="tr"), "ikimilyon")

    def test_billions_trillions(self):
        # Exercise CARDINAL_TRIPLETS at higher orders
        out = num2words(1_000_000_000, lang="tr")
        self.assertIn("milyar", out)
        out = num2words(1_000_000_000_000, lang="tr")
        self.assertIn("trilyon", out)

    def test_negative_int(self):
        self.assertTrue(num2words(-5, lang="tr").startswith("eksi"))
        self.assertIn("yüz", num2words(-100, lang="tr"))

    def test_float_basic(self):
        # Natural-precision reading: 1.5 has 1 fractional digit, so it
        # reads "bir virgül beş" (one point five), not the previously
        # padded "1.50" → "elli". Issue savoirfairelinux/num2words#487.
        self.assertEqual(num2words(1.5, lang="tr"), "birvirgülbeş")
        self.assertEqual(num2words(0.1, lang="tr"), "sıfırvirgülbir")

    def test_float_leading_zero_in_decimal(self):
        # Issue #487 — decimal beginning with zero must be preserved.
        self.assertEqual(num2words(0.01, lang="tr"), "sıfırvirgülsıfırbir")
        self.assertEqual(num2words(1.05, lang="tr"), "birvirgülsıfırbeş")

    def test_kwarg_spaced(self):
        # Issue #486 — `spaced=True` re-tokenizes with spaces.
        self.assertEqual(
            num2words(123, lang="tr", spaced=True), "yüz yirmi üç"
        )
        self.assertEqual(
            num2words(1234, lang="tr", spaced=True), "bin iki yüz otuz dört"
        )
        self.assertEqual(
            num2words(1_000_000, lang="tr", spaced=True), "bir milyon"
        )

    def test_kwarg_precision(self):
        # Issue #534 — precision= controls fractional digit count.
        self.assertEqual(
            num2words(1.234567, lang="tr", precision=2),
            "birvirgülyirmiüç",
        )
        # precision= cuts toward zero in every language (#218), as base's
        # float2tuple does: 0.234567 at 4 places is 2345, where TR used to
        # round to 2346.
        self.assertEqual(
            num2words(1.234567, lang="tr", precision=4),
            "birvirgülikibinüçyüzkırkbeş",
        )

    def test_kwarg_decimal_word(self):
        # Issue #534 — decimal_word= replaces the fractional separator.
        out = num2words(1.5, lang="tr", decimal_word="nokta")
        self.assertIn("nokta", out)
        self.assertNotIn("virgül", out)

    def test_kwarg_combined(self):
        out = num2words(
            1.25, lang="tr", spaced=True, precision=2, decimal_word="nokta"
        )
        self.assertIn(" nokta ", out)
        self.assertIn(" yirmi beş", out)

    def test_year_passthrough(self):
        self.assertEqual(num2words(1971, lang="tr", to="year"),
                         num2words(1971, lang="tr"))

    def test_ordinal_digits(self):
        # Single digit, two digit, three digit ordinals.
        self.assertEqual(num2words(1, lang="tr", to="ordinal"), "birinci")
        self.assertEqual(num2words(10, lang="tr", to="ordinal"), "onuncu")
        self.assertEqual(num2words(20, lang="tr", to="ordinal"), "yirminci")
        self.assertEqual(num2words(100, lang="tr", to="ordinal"), "yüzüncü")

    def test_ordinal_two_triplet(self):
        # 1234th — two triplets, exercises one of the multi-triplet
        # ordinal branches in _to_ordinal_impl.
        out = num2words(1234, lang="tr", to="ordinal")
        self.assertTrue(out.endswith("dördüncü"), out)

    def test_ordinal_round_thousand(self):
        # 1000th, 10000th, 100000th: triplet boundary ordinals.
        for v in (1000, 10000, 100000, 1_000_000):
            out = num2words(v, lang="tr", to="ordinal")
            # Must produce *something* and not raise.
            self.assertTrue(len(out) > 0)

    def test_ordinal_complex_multi_triplet(self):
        # Numbers exercising the 700-870 ordinal block: digits at every
        # triplet position.
        for v in (1_000_001, 1_001_001, 1_111_111, 100_100_100,
                  101_101_101, 999_999_999):
            out = num2words(v, lang="tr", to="ordinal")
            self.assertTrue(len(out) > 5)
            # Result should not contain the separator we use elsewhere.
            self.assertNotIn(",", out)

    def test_ordinal_num(self):
        # Standard Turkish: digit + apostrophe + suffix per TDK.
        # Issue savoirfairelinux/num2words#128.
        self.assertEqual(num2words(5, lang="tr", to="ordinal_num"), "5'inci")
        self.assertEqual(num2words(2, lang="tr", to="ordinal_num"), "2'nci")
        self.assertEqual(num2words(6, lang="tr", to="ordinal_num"), "6'ncı")
        self.assertEqual(num2words(100, lang="tr", to="ordinal_num"), "100'üncü")

    def test_to_cardinal_invalid_raises(self):
        # Non-numeric should raise — covers verify_cardinal early return.
        with self.assertRaises((TypeError, ValueError, Exception)):
            num2words("not a number", lang="tr")


class TestSLCoverageExtra(unittest.TestCase):
    """Drive lang_SL.py branches the existing tests don't reach."""

    def test_basic_cardinals(self):
        self.assertEqual(num2words(0, lang="sl"), "nič")
        self.assertEqual(num2words(1, lang="sl"), "ena")
        self.assertEqual(num2words(2, lang="sl"), "dve")
        self.assertEqual(num2words(11, lang="sl"), "enajst")
        self.assertEqual(num2words(20, lang="sl"), "dvajset")

    def test_compound_tens(self):
        # The "twenty-five" → "petindvajset" pattern uses the ntext+'in'
        # branch in merge.
        out = num2words(25, lang="sl")
        self.assertIn("dvajset", out)
        out = num2words(99, lang="sl")
        self.assertIn("devetdeset", out)

    def test_hundreds_thousands(self):
        self.assertIn("sto", num2words(100, lang="sl"))
        self.assertIn("tisoč", num2words(1000, lang="sl"))
        self.assertIn("dvesto", num2words(200, lang="sl"))

    def test_one_million(self):
        # 1 × milijon — exercises ctext.endswith('en') + ntext ends 'n'
        # no-op branch (line 161-163).
        out = num2words(1_000_000, lang="sl")
        self.assertIn("milijon", out)

    def test_two_million(self):
        # 2 × milijon — cnum==2, ntext ends 'n', not 'd' → +'a'
        # → "dva milijona". Line 153.
        self.assertEqual(num2words(2_000_000, lang="sl"), "dva milijona")

    def test_three_to_four_million(self):
        # 3-4 × milijon — 2<cnum<5, ntext ends 'n' (not 'd') → +'i'.
        # Line 158-159.
        out = num2words(3_000_000, lang="sl")
        self.assertIn("milijon", out)
        out = num2words(4_000_000, lang="sl")
        self.assertIn("milijon", out)

    def test_five_million(self):
        # 5+ × milijon — falls through to else, ntext ends 'n' → no-op
        # then ctext fallback. Lines 174-176.
        out = num2words(5_000_000, lang="sl")
        self.assertIn("milijon", out)

    def test_one_billion(self):
        # 1 × milijarda — special path (cnum=1).
        out = num2words(1_000_000_000, lang="sl")
        self.assertIn("milijard", out)

    def test_two_billion(self):
        # 2 × milijard — cnum==2, ntext ends 'd' → +'i' → "milijardi".
        # Line 150-151.
        self.assertEqual(num2words(2_000_000_000, lang="sl"), "dve milijardi")

    def test_three_to_four_billion(self):
        # 3-4 × milijard — 2<cnum<5, ntext ends 'd' → +'e'. Line 156-157.
        out = num2words(3_000_000_000, lang="sl")
        self.assertIn("milijard", out)
        out = num2words(4_000_000_000, lang="sl")
        self.assertIn("milijard", out)

    def test_five_billion(self):
        # 5+ × milijard — fallback else, ntext ends 'd' → +'a'. Line 173-174.
        out = num2words(5_000_000_000, lang="sl")
        self.assertIn("milijard", out)

    def test_one_trillion(self):
        # 1 × bilijon — extends coverage at 10**12 boundary.
        out = num2words(1_000_000_000_000, lang="sl")
        self.assertIn("bilijon", out)

    def test_two_trillion(self):
        # 2 × bilijon — cnum=2, ntext ends 'n', ctext stays 'dve' (no
        # dve→dva because nnum >= 10^9). Line 165-167.
        out = num2words(2_000_000_000_000, lang="sl")
        self.assertIn("bilijon", out)

    def test_negative(self):
        out = num2words(-5, lang="sl")
        self.assertTrue(out.startswith("minus"))

    def test_float(self):
        out = num2words(1.5, lang="sl")
        self.assertIn("vejica", out)

    def test_float_negative(self):
        out = num2words(-0.5, lang="sl")
        self.assertIn("minus", out)
        self.assertIn("vejica", out)

    def test_to_ordinal_simple(self):
        self.assertEqual(num2words(1, lang="sl", to="ordinal"), "prvi")
        self.assertEqual(num2words(2, lang="sl", to="ordinal"), "drugi")
        self.assertEqual(num2words(3, lang="sl", to="ordinal"), "tretji")

    def test_to_ordinal_float_raises(self):
        # verify_ordinal must reject floats. Line 216.
        with self.assertRaises(TypeError):
            num2words(1.5, lang="sl", to="ordinal")

    def test_to_ordinal_negative_raises(self):
        with self.assertRaises(TypeError):
            num2words(-1, lang="sl", to="ordinal")

    def test_to_ordinal_num(self):
        self.assertEqual(num2words(5, lang="sl", to="ordinal_num"), "5.")

    def test_to_year(self):
        # to_year delegates to to_cardinal. Line 343-344.
        self.assertEqual(
            num2words(2024, lang="sl", to="year"),
            num2words(2024, lang="sl"),
        )

    def test_currency_eur_int(self):
        # Integer path, line 339-341.
        out = num2words(5, lang="sl", to="currency", currency="EUR")
        self.assertIn("evr", out)
        self.assertNotIn("cent", out)

    def test_currency_eur_float_zero_cents(self):
        out = num2words(5.00, lang="sl", to="currency", currency="EUR")
        self.assertIn("evr", out)
        self.assertIn("nič", out)

    def test_currency_eur_float_with_cents(self):
        out = num2words(5.25, lang="sl", to="currency", currency="EUR")
        self.assertIn("evr", out)
        self.assertIn("cent", out)

    def test_currency_usd(self):
        out = num2words(10, lang="sl", to="currency", currency="USD")
        self.assertIn("dolar", out)

    def test_currency_negative(self):
        out = num2words(-5.00, lang="sl", to="currency")
        self.assertTrue(out.startswith("minus"))

    def test_currency_unknown_raises(self):
        # Line 301-305.
        with self.assertRaises(NotImplementedError):
            num2words(5, lang="sl", to="currency", currency="XYZ")


if __name__ == "__main__":
    unittest.main()
