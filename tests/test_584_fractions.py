# -*- coding: utf-8 -*-
"""Issue savoirfairelinux/num2words#584 — fraction support.

Strings of the form ``"<int>/<int>"`` are routed to the converter's
``to_fraction(numerator, denominator)`` method. Idiomatic forms for
common denominators are implemented per language; everything else falls
back to ``<cardinal numerator> <ordinal denominator>[plural]``.
"""
from __future__ import unicode_literals

import unittest

from num2words2 import num2words


class TestEnglishFractions(unittest.TestCase):
    def test_unit_fractions(self):
        self.assertEqual(num2words("1/2", lang="en"), "one half")
        self.assertEqual(num2words("1/3", lang="en"), "one third")
        self.assertEqual(num2words("1/4", lang="en"), "one quarter")
        self.assertEqual(num2words("1/5", lang="en"), "one fifth")
        self.assertEqual(num2words("1/8", lang="en"), "one eighth")

    def test_plural_fractions(self):
        self.assertEqual(num2words("2/3", lang="en"), "two thirds")
        self.assertEqual(num2words("3/4", lang="en"), "three quarters")
        self.assertEqual(num2words("5/8", lang="en"), "five eighths")
        self.assertEqual(num2words("11/12", lang="en"), "eleven twelfths")

    def test_improper_fractions(self):
        self.assertEqual(num2words("5/3", lang="en"), "five thirds")
        self.assertEqual(num2words("7/4", lang="en"), "seven quarters")

    def test_zero_numerator(self):
        self.assertEqual(num2words("0/3", lang="en"), "zero")
        self.assertEqual(num2words("0/100", lang="en"), "zero")

    def test_denominator_one(self):
        self.assertEqual(num2words("1/1", lang="en"), "one")
        self.assertEqual(num2words("5/1", lang="en"), "five")

    def test_negative(self):
        self.assertEqual(num2words("-1/3", lang="en"), "minus one third")
        self.assertEqual(num2words("-3/4", lang="en"), "minus three quarters")
        # Sign on denominator works the same.
        self.assertEqual(num2words("1/-3", lang="en"), "minus one third")

    def test_zero_denominator_raises(self):
        with self.assertRaises(ZeroDivisionError):
            num2words("1/0", lang="en")

    def test_whitespace_tolerated(self):
        self.assertEqual(num2words(" 1 / 3 ", lang="en"), "one third")


class TestRomanceLanguageFractions(unittest.TestCase):
    def test_french_idiomatic(self):
        self.assertEqual(num2words("1/2", lang="fr"), "un demi")
        self.assertEqual(num2words("1/3", lang="fr"), "un tiers")
        self.assertEqual(num2words("1/4", lang="fr"), "un quart")
        self.assertEqual(num2words("2/3", lang="fr"), "deux tiers")
        self.assertEqual(num2words("3/4", lang="fr"), "trois quarts")

    def test_french_other(self):
        self.assertEqual(num2words("5/8", lang="fr"), "cinq huitièmes")

    def test_spanish_idiomatic(self):
        self.assertEqual(num2words("1/2", lang="es"), "un medio")
        self.assertEqual(num2words("1/3", lang="es"), "un tercio")
        self.assertEqual(num2words("1/4", lang="es"), "un cuarto")
        self.assertEqual(num2words("2/3", lang="es"), "dos tercios")

    def test_italian_with_i_plural(self):
        self.assertEqual(num2words("1/2", lang="it"), "un mezzo")
        self.assertEqual(num2words("1/3", lang="it"), "un terzo")
        self.assertEqual(num2words("2/3", lang="it"), "due terzi")
        self.assertEqual(num2words("3/4", lang="it"), "tre quarti")

    def test_portuguese_with_s_plural(self):
        self.assertEqual(num2words("1/2", lang="pt"), "um meio")
        self.assertEqual(num2words("1/3", lang="pt"), "um terço")
        self.assertEqual(num2words("2/3", lang="pt"), "dois terços")
        self.assertEqual(num2words("3/4", lang="pt"), "três quartos")

    def test_pt_br_inherits(self):
        self.assertEqual(num2words("1/3", lang="pt_BR"), "um terço")
        self.assertEqual(num2words("3/4", lang="pt_BR"), "três quartos")


class TestGermanFractions(unittest.TestCase):
    def test_idiomatic(self):
        self.assertEqual(num2words("1/2", lang="de"), "ein halb")
        self.assertEqual(num2words("1/3", lang="de"), "ein Drittel")
        self.assertEqual(num2words("1/4", lang="de"), "ein Viertel")
        self.assertEqual(num2words("1/5", lang="de"), "ein Fünftel")
        self.assertEqual(num2words("1/7", lang="de"), "ein Siebtel")
        self.assertEqual(num2words("1/8", lang="de"), "ein Achtel")

    def test_invariant_plural(self):
        # German plural is invariant for these nouns: zwei Drittel, not
        # zwei Drittels.
        self.assertEqual(num2words("2/3", lang="de"), "zwei Drittel")
        self.assertEqual(num2words("3/4", lang="de"), "drei Viertel")
        self.assertEqual(num2words("5/8", lang="de"), "fünf Achtel")

    def test_round_powers_drop_ein(self):
        # The cardinal of 100 is 'einhundert' but the fraction noun is
        # 'Hundertstel' (no leading 'ein').
        self.assertEqual(num2words("1/100", lang="de"), "ein Hundertstel")
        self.assertEqual(num2words("1/1000", lang="de"), "ein Tausendstel")

    def test_above_twenty(self):
        self.assertEqual(num2words("1/20", lang="de"), "ein Zwanzigstel")
        self.assertEqual(num2words("1/13", lang="de"), "ein Dreizehntel")


class TestFractionEdgeCases(unittest.TestCase):
    def test_non_fraction_input_unchanged(self):
        # Plain ints/floats still go through the cardinal/ordinal path.
        self.assertEqual(num2words(42, lang="en"), "forty-two")
        self.assertEqual(num2words(3.14, lang="en"), "three point one four")

    def test_currency_path_unaffected(self):
        # Don't accidentally route '1/100' as a fraction when the user
        # really wants the currency conversion of integer 1 with currency
        # set elsewhere — no false positive in the dispatcher.
        out = num2words(1.50, lang="en", to="currency", currency="USD")
        self.assertIn("dollar", out)


class TestUnsupportedFractions(unittest.TestCase):
    """#217: no more "ordinal + s" fallback ('два третийs', '三 第四s')."""

    LANGS = [
        "ru",
        "zh",
        "ja",
        "tr",
        "cs",
        "nl",
        "pl",
        "bn",
        "dv",
        "id",
        "vi",
        "zh_TW",
        "rm",
        "sv",
    ]

    def test_unsupported_languages_raise(self):
        for lang in self.LANGS:
            for frac in ("1/2", "2/3", "-1/2"):
                with self.assertRaisesRegex(
                    NotImplementedError,
                    "lang='%s' does not support to='fraction'" % lang,
                ):
                    num2words(frac, lang=lang)

    def test_zero_denominator_is_zero_division_everywhere(self):
        for lang in self.LANGS + ["en", "de", "fr", "ca"]:
            with self.assertRaises(ZeroDivisionError):
                num2words("1/0", lang=lang)

    def test_non_string_input_is_type_error(self):
        from decimal import Decimal

        for value in (5, 1.5, Decimal("1.5"), "5", "1.5"):
            with self.assertRaisesRegex(TypeError, "'numerator/denominator'"):
                num2words(value, lang="en", to="fraction")

    def test_en_power_of_ten(self):
        self.assertEqual(num2words("1/100"), "one hundredth")
        self.assertEqual(num2words("3/1000"), "three thousandths")
        self.assertEqual(num2words("1/200"), "one two hundredth")

    def test_ca(self):
        self.assertEqual(num2words("1/2", lang="ca"), "un mig")
        self.assertEqual(num2words("2/3", lang="ca"), "dos terços")
        self.assertEqual(num2words("3/4", lang="ca"), "tres quarts")
        self.assertEqual(num2words("2/5", lang="ca"), "dos cinquens")
        self.assertEqual(num2words("1/10", lang="ca"), "un desè")


if __name__ == "__main__":
    unittest.main()
