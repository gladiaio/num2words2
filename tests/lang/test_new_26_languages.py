# -*- coding: utf-8 -*-
# Smoke tests for the 26 new languages added in this PR.
# Each test verifies cardinal, ordinal, currency, year, decimal, and
# negative number conversions execute without error and produce the
# expected basic output.

from unittest import TestCase

from num2words2 import num2words

# Expected zero, one, ten for each new language code.
EXPECTED_BASICS = {
    "ban": ("nol", "siki", "dasa"),
    "bm": ("fu", "kelen", "tan"),
    "ceb": ("siro", "usa", "napulo"),
    "ckb": ("sifir", "yek", "de"),
    "cnh": ("zero", "pakhat", "pahra"),
    "ff": ("sufri", "go'o", "sappo"),
    "fil": ("sero", "isa", "sampu"),
    "hmn": ("xoom", "ib", "kaum"),
    "ki": ("wĩra", "ĩmwe", "ikũmi"),
    "kok": ("शून्य", "एक", "धा"),
    "ksw": ("lah", "ta", "tasi"),
    "ku": ("sifir", "yek", "deh"),
    "ky": ("nöl", "bir", "on"),
    "lg": ("nuli", "emu", "kkumi"),
    "lus": ("a awmlo", "pakhat", "sawm"),
    "om": ("zeeroo", "tokko", "kudhan"),
    "or": ("śūnya", "eka", "daśa"),
    "pap": ("sero", "un", "dies"),
    "pli": ("suñña", "eka", "dasa"),
    "rw": ("zeru", "rimwe", "icumi"),
    "ti": ("ባዶ", "ሓደ", "ዓሰርተ"),
    "xh": ("iqanda", "nye", "lishumi"),
    "zu": ("iqanda", "kunye", "ishumi"),
}

# Aliases — share converter with another language.
ALIASES = {
    "nb": "no",   # Norwegian Bokmål → Norwegian
    "jv": "jw",   # Modern Javanese code → existing converter
    "miz": "lus",  # Mizo alternate code → Mizo
}


class TestNewLanguageRegistration(TestCase):
    """Verify all 26 new codes resolve via num2words()."""

    def test_all_codes_resolve_cardinal(self):
        all_codes = list(EXPECTED_BASICS.keys()) + list(ALIASES.keys())
        for code in all_codes:
            with self.subTest(code=code):
                result = num2words(42, lang=code)
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestNewLanguageBasics(TestCase):
    """Verify zero, one, and ten produce the expected canonical words."""

    def test_basic_numerals(self):
        for code, (zero, one, ten) in EXPECTED_BASICS.items():
            with self.subTest(code=code):
                self.assertEqual(num2words(0, lang=code), zero)
                self.assertEqual(num2words(1, lang=code), one)
                self.assertEqual(num2words(10, lang=code), ten)


class TestNewLanguageRanges(TestCase):
    """Smoke-test that cardinals across magnitude ranges produce non-empty output."""

    def test_cardinal_range(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            for n in [5, 19, 23, 100, 999, 1000, 12345, 1000000]:
                with self.subTest(code=code, n=n):
                    result = num2words(n, lang=code)
                    self.assertIsInstance(result, str)
                    self.assertTrue(len(result) > 0)
                    # Should not contain raw digits (would mean fallback)
                    self.assertFalse(any(d in result for d in "0123456789"),
                                     f"{code}({n})={result!r} contains digits")


class TestNewLanguageOrdinal(TestCase):
    """Verify ordinal conversion runs for all new languages."""

    def test_ordinal(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            for n in [1, 2, 5, 10, 21]:
                with self.subTest(code=code, n=n):
                    result = num2words(n, lang=code, ordinal=True)
                    self.assertIsInstance(result, str)
                    self.assertTrue(len(result) > 0)


class TestNewLanguageOrdinalNum(TestCase):
    """Verify ordinal_num returns a string for each code."""

    def test_ordinal_num(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            for n in [1, 5, 21]:
                with self.subTest(code=code, n=n):
                    result = num2words(n, lang=code, to="ordinal_num")
                    self.assertIsInstance(result, str)


class TestNewLanguageCurrency(TestCase):
    """Verify currency conversion runs for default and USD currencies."""

    def test_currency_default(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            with self.subTest(code=code):
                result = num2words(1.5, lang=code, to="currency")
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)

    def test_currency_usd(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            with self.subTest(code=code):
                result = num2words(1.5, lang=code, to="currency", currency="USD")
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestNewLanguageYear(TestCase):
    """Verify year conversion runs."""

    def test_year(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            with self.subTest(code=code):
                result = num2words(2026, lang=code, to="year")
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestNewLanguageDecimal(TestCase):
    """Verify decimal conversion runs."""

    def test_decimal(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            with self.subTest(code=code):
                result = num2words(3.14, lang=code)
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestNewLanguageNegative(TestCase):
    """Verify negative cardinals run without crashing."""

    def test_negative(self):
        codes = list(EXPECTED_BASICS.keys())
        for code in codes:
            with self.subTest(code=code):
                result = num2words(-42, lang=code)
                self.assertIsInstance(result, str)
                self.assertTrue(len(result) > 0)


class TestAliases(TestCase):
    """Verify alias codes route to an existing converter and produce output."""

    def test_aliases_produce_output(self):
        for alias, target in ALIASES.items():
            with self.subTest(alias=alias):
                alias_result = num2words(42, lang=alias)
                target_result = num2words(42, lang=target)
                self.assertEqual(alias_result, target_result)
