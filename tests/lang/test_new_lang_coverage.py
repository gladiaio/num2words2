# Coverage gap-filler for the 23 new languages added in PR #14.
# Targets the three under-covered code paths shared across these
# implementations: > 1B fallback, currency-with-cents, and pluralize
# edge cases.
from unittest import TestCase

from num2words2 import maxval, num2words

# ksw is covered by tests/lang/test_ksw.py: since gladiaio/num2words2#143
# it spells 1..9999 only and raises for currency.
NEW_LANG_CODES = [
    "ban", "bm", "ceb", "ckb", "cnh", "ff", "fil", "hmn", "ki", "kok",
    "ku", "ky", "lg", "lus", "om", "or", "pap", "pli", "rw",
    "ti", "xh", "zu",
]


class TestLargeNumberFallback(TestCase):
    """Numbers >= 1e9: words, or OverflowError where a language has no
    attested scale word (gladiaio/num2words2#147; the digit check itself is
    `no_digit_output` in tests/test_invariants.py)."""

    def test_billion_plus(self):
        for code in NEW_LANG_CODES:
            with self.subTest(code=code):
                # 10^12 — beyond the explicit million scale in most templates.
                # A language with no attested word that high raises (#147).
                m = maxval(code)
                if m is not None and 10 ** 12 >= m:
                    with self.assertRaises(OverflowError):
                        num2words(10 ** 12, lang=code)
                    continue
                result = num2words(10 ** 12, lang=code)
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestCurrencyWithCents(TestCase):
    """Hit the `if cents and right:` branch in to_currency."""

    def test_currency_with_fractional(self):
        for code in NEW_LANG_CODES:
            with self.subTest(code=code):
                # Float with non-zero cents triggers the cents branch
                result = num2words(12.34, lang=code, to="currency")
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)

    def test_currency_negative_with_cents(self):
        for code in NEW_LANG_CODES:
            with self.subTest(code=code):
                result = num2words(-7.89, lang=code, to="currency")
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestPluralizeEdgeCases(TestCase):
    """Hit the pluralize() edge cases."""

