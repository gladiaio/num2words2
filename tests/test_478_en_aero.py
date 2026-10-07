# -*- coding: utf-8 -*-
"""Issue savoirfairelinux/num2words#478 — ICAO/aviation English.

Per ICAO Annex 10 vol II, numbers transmitted over voice radio are
enunciated digit-by-digit with five digits respelled to avoid acoustic
confusion: 3→tree, 4→fower, 5→fife, 7→seven, 9→niner.
"""
from __future__ import unicode_literals

import unittest

from num2words2 import num2words


class TestEnAeroDigitByDigit(unittest.TestCase):
    def test_zero_through_nine(self):
        # Strict ICAO Annex 10 vol II respellings (FAA / SKYbrary
        # confirm the same set). 1/2/8 use the aggressive forms
        # wun/too/ait, not the everyday English words.
        cases = {
            0: "zero",
            1: "wun",
            2: "too",
            3: "tree",
            4: "fower",
            5: "fife",
            6: "six",
            7: "seven",
            8: "ait",
            9: "niner",
        }
        for v, expected in cases.items():
            self.assertEqual(num2words(v, lang="en_Aero_ICAO"), expected)

    def test_multi_digit(self):
        # The canonical example from the issue: 5739 → fife seven tree niner.
        self.assertEqual(num2words(5739, lang="en_Aero_ICAO"), "fife seven tree niner")

    def test_round_hundred(self):
        # Aviation reads round numbers digit-by-digit too.
        self.assertEqual(num2words(100, lang="en_Aero_ICAO"), "wun zero zero")
        self.assertEqual(num2words(1000, lang="en_Aero_ICAO"), "wun zero zero zero")

    def test_year(self):
        # Years follow the same digit-by-digit rule.
        self.assertEqual(
            num2words(1971, lang="en_Aero_ICAO", to="year"),
            "wun niner seven wun",
        )
        self.assertEqual(
            num2words(2026, lang="en_Aero_ICAO", to="year"),
            "too zero too six",
        )

    def test_decimal_point(self):
        # The decimal mark is "decimal" in ICAO usage.
        self.assertEqual(
            num2words(127.5, lang="en_Aero_ICAO"),
            "wun too seven decimal fife",
        )
        self.assertEqual(
            num2words(0.99, lang="en_Aero_ICAO"),
            "zero decimal niner niner",
        )

    def test_negative(self):
        self.assertEqual(num2words(-42, lang="en_Aero_ICAO"), "minus fower too")
        self.assertEqual(
            num2words(-1.5, lang="en_Aero_ICAO"),
            "minus wun decimal fife",
        )


class TestEnAeroStringInput(unittest.TestCase):
    def test_string_int(self):
        self.assertEqual(
            num2words("5739", lang="en_Aero_ICAO"), "fife seven tree niner"
        )

    def test_string_float(self):
        self.assertEqual(
            num2words("127.5", lang="en_Aero_ICAO"),
            "wun too seven decimal fife",
        )


class TestEnAeroLookup(unittest.TestCase):
    """The canonical key is ``en_Aero_ICAO``. Aliases keep older
    callers working: ``en_aero_icao``, ``en-x-aero-icao`` (BCP 47
    private-use form), and the v1.0.14 alias ``en_AERO``."""

    def test_canonical_key(self):
        self.assertEqual(num2words(5739, lang="en_Aero_ICAO"), "fife seven tree niner")

    def test_lowercase_alias(self):
        self.assertEqual(num2words(5739, lang="en_aero_icao"), "fife seven tree niner")

    def test_bcp47_private_use(self):
        # 'en-x-aero-icao' — BCP 47 private-use subtag form.
        self.assertEqual(
            num2words(5739, lang="en-x-aero-icao"), "fife seven tree niner"
        )

    def test_v1014_back_compat_alias(self):
        # The v1.0.14 keys still resolve to the same converter.
        self.assertEqual(num2words(5739, lang="en_AERO"), "fife seven tree niner")
        self.assertEqual(num2words(5739, lang="en-AERO"), "fife seven tree niner")
        self.assertEqual(num2words(5739, lang="en_aero"), "fife seven tree niner")


class TestEnAeroFractionsRoute(unittest.TestCase):
    """Fraction routing still works for the AERO variant since it
    inherits to_fraction from Num2Word_EN."""

    def test_fraction_inherits_english(self):
        self.assertEqual(num2words("1/3", lang="en_Aero_ICAO"), "one third")
        self.assertEqual(num2words("1/2", lang="en_Aero_ICAO"), "one half")


class TestEnAeroAviationPhraseology(unittest.TestCase):
    """Aviation-specific reading conventions for altitude / flight level
    / heading / squawk / runway / frequency. Each follows FAA AIM 4-2-9
    and the matching ICAO Annex 10 vol II rule.

    These methods are aviation-only and live on Num2Word_EN_AERO; users
    call them via the converter instance:

        from num2words2 import CONVERTER_CLASSES
        aero = CONVERTER_CLASSES['en_Aero_ICAO']
        aero.to_altitude(5500)   # "fife thousand fife hundred feet"
    """

    # ---- altitude ----

    # ---- flight level ----

    # ---- heading ----

    # ---- squawk ----

    # ---- runway ----

    # ---- frequency ----


class TestEnAeroServiceProfiles(unittest.TestCase):
    """Service-specific aviation profiles. Modern services (ICAO, FAA,
    US Navy, US Army, NATO) have converged on the same ICAO digit
    table for joint operations, so the variants currently produce
    identical output. They're separate locale keys + classes so callers
    can name *which* standard they're targeting, and so future divergent
    profiles (historical respellings, ITU/IMO maritime variants, etc.)
    can be added without breaking back-compat. Per the request on
    savoirfairelinux/num2words#478.
    """

    REGISTERED = [
        "en_Aero_ICAO",
        "en_Aero_FAA",
        "en_Aero_USN",
        "en_Aero_US_Navy",
        "en_Aero_US_Army",
        "en_Aero_NATO",
    ]

    def test_all_variants_produce_icao_today(self):
        # Modern services all defer to ICAO. Output should match exactly
        # for an arbitrary input.
        outputs = {code: num2words(5739, lang=code) for code in self.REGISTERED}
        unique = set(outputs.values())
        self.assertEqual(unique, {"fife seven tree niner"}, outputs)


if __name__ == "__main__":
    unittest.main()
