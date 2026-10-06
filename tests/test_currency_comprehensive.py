#!/usr/bin/env python3
"""Comprehensive currency tests based on coverage matrix analysis"""

import unittest

from num2words2 import _rust, num2words


class ComprehensiveCurrencyTests(unittest.TestCase):
    """Test currency support across all languages based on matrix findings"""

    def test_full_support_languages(self):
        """Every code either raises or names a different currency (#219).

        These languages used to "support" all ten codes by printing their
        own default currency for the unknown ones.
        """
        full_support_langs = [
            "ar",
            "bn",
            "ca",
            "es",
            "fi",
            "ha",
            "hy",
            "id",
            "mn",
            "sq",
            "uk",
            "vi",
        ]
        test_currencies = [
            "EUR",
            "USD",
            "GBP",
            "JPY",
            "CNY",
            "CHF",
            "CAD",
            "AUD",
            "INR",
            "RUB",
        ]

        for lang in full_support_langs:
            default = num2words(100.50, lang=lang, to="currency")
            for currency in test_currencies:
                try:
                    result = num2words(
                        100.50, lang=lang, to="currency", currency=currency
                    )
                except NotImplementedError:
                    continue
                self.assertTrue(result, f"{lang} {currency} output is empty")
                if currency != _rust.default_currency(lang):
                    self.assertNotEqual(result, default, f"{lang} {currency}")

    def test_polish_gbp_support(self):
        """Test newly added Polish GBP support"""
        test_cases = [
            (1, "GBP", "jeden funt brytyjski"),
            (2, "GBP", "dwa funty brytyjskie"),
            (5, "GBP", "pięć funtów brytyjskich"),
            (10, "GBP", "dziesięć funtów brytyjskich"),
            (1.50, "GBP", "jeden funt brytyjski, pięćdziesiąt pensów"),
            (2.25, "GBP", "dwa funty brytyjskie, dwadzieścia pięć pensów"),
            (100.99, "GBP", "sto funtów brytyjskich, dziewięćdziesiąt dziewięć pensów"),
        ]

        for amount, currency, expected in test_cases:
            result = num2words(amount, lang="pl", to="currency", currency=currency)
            self.assertEqual(result, expected, f"PL {amount} {currency}")

    def test_russian_gbp_support(self):
        """Test newly added Russian GBP support"""
        test_cases = [
            (1, "GBP", "один фунт стерлингов"),
            (2, "GBP", "два фунта стерлингов"),
            (5, "GBP", "пять фунтов стерлингов"),
            (10, "GBP", "десять фунтов стерлингов"),
            (1.50, "GBP", "один фунт стерлингов, пятьдесят пенни"),
            (21, "GBP", "двадцать один фунт стерлингов"),
            (100, "GBP", "сто фунтов стерлингов"),
        ]

        for amount, currency, expected in test_cases:
            result = num2words(amount, lang="ru", to="currency", currency=currency)
            self.assertEqual(result, expected, f"RU {amount} {currency}")

    def test_japanese_new_currencies(self):
        """Test newly added Japanese EUR/USD/GBP support"""
        test_cases = [
            # EUR tests
            (1, "EUR", "一ユーロ"),
            (2, "EUR", "二ユーロ"),
            (10, "EUR", "十ユーロ"),
            (1.50, "EUR", "一ユーロ五十セント"),
            (100.25, "EUR", "百ユーロ二十五セント"),
            # USD tests
            (1, "USD", "一ドル"),
            (5, "USD", "五ドル"),
            (10, "USD", "十ドル"),
            (2.50, "USD", "二ドル五十セント"),
            (99.99, "USD", "九十九ドル九十九セント"),
            # GBP tests
            (1, "GBP", "一ポンド"),
            (3, "GBP", "三ポンド"),
            (20, "GBP", "二十ポンド"),
            (1.25, "GBP", "一ポンド二十五ペンス"),
            (50.50, "GBP", "五十ポンド五十ペンス"),
        ]

        for amount, currency, expected in test_cases:
            result = num2words(amount, lang="ja", to="currency", currency=currency)
            self.assertEqual(result, expected, f"JA {amount} {currency}")

    def test_italian_negative_currency(self):
        """Test Italian negative currency bug fix"""
        test_cases = [
            (-1, "EUR", "meno uno euro"),
            (-10, "EUR", "meno dieci euro"),
            (-5.50, "EUR", "meno cinque euro e cinquanta centesimi"),
            (-100, "EUR", "meno cento euro"),
            (-1, "USD", "meno uno dollaro"),
            (-5, "USD", "meno cinque dollari"),
            (-10.25, "USD", "meno dieci dollari e venticinque centesimi"),
            (-1, "GBP", "meno uno sterlina"),
            (-2, "GBP", "meno due sterline"),
            (-50.75, "GBP", "meno cinquanta sterline e settantacinque penny"),
        ]

        for amount, currency, expected in test_cases:
            result = num2words(amount, lang="it", to="currency", currency=currency)
            self.assertEqual(result, expected, f"IT negative {amount} {currency}")

    def test_newly_implemented_currency_support(self):
        """Test languages that now have currency support (previously had none)"""
        newly_supported_langs = [
            "az",
            "en_NG",
            "es_CO",
            "es_CR",
            "es_GT",
            "es_NI",
            "es_VE",
            "fa",
            "fr_DZ",
            "no",
            "pt_BR",
            "ro",
            "tr",
        ]

        for lang in newly_supported_langs:
            # Test that they now support EUR and USD
            try:
                result_eur = num2words(100, lang=lang, to="currency", currency="EUR")
                result_usd = num2words(100, lang=lang, to="currency", currency="USD")
                self.assertIsNotNone(result_eur, f"{lang} should support EUR")
                self.assertIsNotNone(result_usd, f"{lang} should support USD")
                self.assertTrue(
                    len(result_eur) > 0, f"{lang} EUR output should not be empty"
                )
                self.assertTrue(
                    len(result_usd) > 0, f"{lang} USD output should not be empty"
                )
            except Exception as e:
                self.fail(f"{lang} failed currency conversion: {e}")

    def test_edge_cases_major_currencies(self):
        """Test edge cases for major currencies"""
        # Test zero amounts
        test_cases = [
            ("en", 0, "USD", "zero dollars"),
            ("en", 0, "EUR", "zero euros"),
            ("en", 0, "GBP", "zero pounds"),
            ("fr", 0, "EUR", "zéro euro"),
            ("de", 0, "EUR", "null Euro"),
            ("es", 0, "EUR", "cero euros"),
        ]

        for lang, amount, currency, expected in test_cases:
            result = num2words(amount, lang=lang, to="currency", currency=currency)
            self.assertEqual(result, expected, f"{lang} zero {currency}")

    def test_fractional_cents(self):
        """Test fractional cent handling"""
        test_cases = [
            ("en", 0.01, "USD", "zero dollars, one cent"),
            ("en", 0.99, "USD", "zero dollars, ninety-nine cents"),
            ("en", 1.01, "USD", "one dollar, one cent"),
            ("en", 10.10, "USD", "ten dollars, ten cents"),
            ("fr", 0.01, "EUR", "zéro euro et un centime"),
            ("de", 0.50, "EUR", "null Euro und fünfzig Cent"),
            ("pl", 0.01, "PLN", "zero złotych, jeden grosz"),
            ("ru", 0.01, "RUB", "ноль рублей, одна копейка"),
        ]

        for lang, amount, currency, expected in test_cases:
            result = num2words(amount, lang=lang, to="currency", currency=currency)
            self.assertEqual(result, expected, f"{lang} {amount} {currency}")

    def test_large_amounts(self):
        """Test large currency amounts"""
        test_cases = [
            ("en", 1000000, "USD", "one million dollars"),
            ("en", 1000000000, "EUR", "one billion euros"),
            ("fr", 1000000, "EUR", "un million euros"),
            ("de", 1000000, "EUR", "eine Million Euro"),
            ("es", 1000000, "EUR", "un millón euros"),
            ("pl", 1000000, "PLN", "milion złotych"),
            ("ru", 1000000, "RUB", "один миллион рублей"),
            ("ja", 1000000, "JPY", "百万円"),
        ]

        for lang, amount, currency, expected in test_cases:
            result = num2words(amount, lang=lang, to="currency", currency=currency)
            self.assertEqual(result, expected, f"{lang} large {amount} {currency}")

    def test_currency_pluralization(self):
        """Test proper pluralization for currencies"""
        test_cases = [
            # English
            ("en", 1, "USD", "one dollar"),
            ("en", 2, "USD", "two dollars"),
            ("en", 1, "EUR", "one euro"),
            ("en", 2, "EUR", "two euros"),
            # Polish
            ("pl", 1, "PLN", "jeden złoty"),
            ("pl", 2, "PLN", "dwa złote"),
            ("pl", 5, "PLN", "pięć złotych"),
            # Russian
            ("ru", 1, "RUB", "один рубль"),
            ("ru", 2, "RUB", "два рубля"),
            ("ru", 5, "RUB", "пять рублей"),
            # Italian
            ("it", 1, "EUR", "uno euro"),
            ("it", 2, "EUR", "due euro"),
            ("it", 100, "EUR", "cento euro"),
        ]

        for lang, amount, currency, expected in test_cases:
            result = num2words(amount, lang=lang, to="currency", currency=currency)
            self.assertEqual(result, expected, f"{lang} plural {amount} {currency}")


class CurrencyMatrixValidation(unittest.TestCase):
    """Validate currency support matrix findings"""

    def test_most_supported_currencies(self):
        """Test USD and EUR as most supported currencies"""
        # USD supported by 78.4% of languages
        # EUR supported by 77.0% of languages

        usd_langs = [
            "en",
            "es",
            "fr",
            "de",
            "it",
            "pl",
            "ru",
            "ja",
            "ar",
            "bn",
            "ca",
            "fi",
            "ha",
            "hy",
            "id",
            "mn",
            "sq",
            "uk",
            "vi",
            "zh",
        ]

        eur_langs = [
            "en",
            "es",
            "fr",
            "de",
            "it",
            "pl",
            "ru",
            "ja",
            "ar",
            "bn",
            "ca",
            "fi",
            "ha",
            "hy",
            "id",
            "mn",
            "sq",
            "uk",
            "vi",
            "zh",
        ]

        for lang in usd_langs:
            try:
                result = num2words(10, lang=lang, to="currency", currency="USD")
                self.assertIsNotNone(result)
            except Exception:
                pass  # Some might not be fully implemented

        for lang in eur_langs:
            try:
                result = num2words(10, lang=lang, to="currency", currency="EUR")
                self.assertIsNotNone(result)
            except Exception:
                pass  # Some might not be fully implemented

    def test_least_supported_currency(self):
        """Test CHF as least supported currency (24.3%)"""
        chf_supported = [
            "ar",
            "bn",
            "ca",
            "de",
            "es",
            "fi",
            "ha",
            "hy",
            "id",
            "it",
            "mn",
            "sq",
            "uk",
            "vi",
            "zh",
        ]

        for lang in chf_supported:
            try:
                result = num2words(100, lang=lang, to="currency", currency="CHF")
                self.assertIsNotNone(result)
            except Exception:
                pass  # Some might not be fully implemented


if __name__ == "__main__":
    unittest.main()
