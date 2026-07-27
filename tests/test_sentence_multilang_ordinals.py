"""Tests for multi-language ordinal + date detection in SentenceConverter.

Covers the lang_registry coverage extension landed in PR
"feat/sentence-multilang-ordinals". Each language asserts the canonical
ordinal/date forms a downstream TTS pipeline expects to read.
"""
import unittest

from num2words2 import num2words_sentence


class MultiLangOrdinalSurfaceFormTests(unittest.TestCase):
    """Standalone ordinal forms in each language's native surface."""

    def test_en_simple_ordinal(self):
        self.assertEqual(
            num2words_sentence("the 5th apple", lang="en"),
            "the fifth apple",
        )

    def test_en_first_second_third(self):
        self.assertEqual(
            num2words_sentence("1st 2nd 3rd 4th", lang="en"),
            "First second third fourth",
        )

    def test_fr_ordinal_e(self):
        self.assertEqual(
            num2words_sentence("le 5e étage", lang="fr"),
            "le cinquième étage",
        )

    def test_fr_ordinal_er(self):
        self.assertIn(
            "premier",
            num2words_sentence("le 1er mai", lang="fr"),
        )

    def test_es_ordinal_letter_indicator(self):
        # U+00BA (masculine ordinal indicator, "º") is unicode-letter, so
        # the legacy `(\d+)(?:º|°|ª)\b` regex catches "5º" → "quinto".
        out = num2words_sentence("el 5º piso", lang="es").lower()
        self.assertIn("quinto", out)

    def test_pt_ordinal_letter_indicator(self):
        out = num2words_sentence("o 5º andar", lang="pt").lower()
        self.assertIn("quinto", out)

    def test_it_ordinal_degree_symbol_is_inert(self):
        # U+00B0 ("°", DEGREE SIGN) is a symbol, not a letter, so the
        # `\b` after it never matches and the standalone-ordinal pass
        # leaves "5°" alone — matches the legacy CSV expectation
        # ("cinque°" rather than "quinto").
        out = num2words_sentence("il 5° piano", lang="it").lower()
        self.assertIn("cinque", out)

    def test_de_ordinal_with_period(self):
        # "1. Mal" → "erste Mal" (no following month so no case agreement).
        self.assertIn(
            "erst",
            num2words_sentence("das 1. Mal", lang="de").lower(),
        )

    def test_nl_ordinal(self):
        self.assertIn(
            "vijfde",
            num2words_sentence("de 5e plaats", lang="nl").lower(),
        )

    def test_id_prefix_ordinal(self):
        self.assertIn(
            "kelima",
            num2words_sentence("Saya ke-5 dalam barisan", lang="id").lower(),
        )

    def test_th_prefix_ordinal(self):
        self.assertIn(
            "ห้า",
            num2words_sentence("ที่ 5", lang="th"),
        )

    def test_vi_prefix_ordinal(self):
        self.assertIn(
            "ba",
            num2words_sentence("thứ 3", lang="vi").lower(),
        )

    def test_zh_prefix_ordinal(self):
        # 第5 → 第五 (ordinal in zh just spells the digit)
        self.assertIn(
            "五",
            num2words_sentence("第5名", lang="zh"),
        )


class MultiLangDateTests(unittest.TestCase):
    """Date phrases (day + month + optional year) in each language."""

    def test_en_month_day_with_ordinal_suffix(self):
        out = num2words_sentence("December 5th, 2026", lang="en")
        self.assertIn("December fifth", out)
        self.assertIn("twenty-six", out)

    def test_en_month_day_bare(self):
        # "April 5" — no ordinal suffix on the date, but EN spells day as ordinal.
        self.assertIn(
            "April fifth",
            num2words_sentence("April 5 was a Tuesday", lang="en"),
        )

    def test_fr_date_first_day(self):
        out = num2words_sentence("le 1er mai", lang="fr")
        self.assertIn("premier", out)
        self.assertIn("mai", out)

    def test_es_date(self):
        # Spanish keeps day cardinal: "5 de mayo" → "cinco de mayo".
        out = num2words_sentence("5 de mayo de 2026", lang="es").lower()
        self.assertIn("cinco de mayo", out)

    def test_pt_date(self):
        out = num2words_sentence("5 de maio de 2026", lang="pt").lower()
        self.assertIn("cinco de maio", out)

    def test_de_date_dative_marker_remains(self):
        out = num2words_sentence("Am 5. März", lang="de").lower()
        self.assertIn("fünft", out)
        self.assertIn("märz", out)

    def test_nl_date(self):
        out = num2words_sentence("5 mei 2026", lang="nl").lower()
        self.assertIn("vijfde", out)

    def test_sv_date(self):
        out = num2words_sentence("5 maj 2026", lang="sv").lower()
        self.assertIn("femte", out)

    def test_pl_date_no_partial_match(self):
        # Polish day-month: "5 stycznia" — ensure month-name boundary works
        # so neighbouring words like "marca" (March) don't bleed.
        out = num2words_sentence("5 stycznia 2024", lang="pl").lower()
        self.assertIn("stycznia", out)

    def test_ja_date_simple(self):
        out = num2words_sentence("2024年5月3日", lang="ja")
        # Just sanity-check digit conversion happens.
        self.assertNotIn("2024", out)
        self.assertNotIn("3日", out)

    def test_zh_date_simple(self):
        out = num2words_sentence("2024年5月3日", lang="zh")
        self.assertNotIn("2024", out)


class RegressionGuards(unittest.TestCase):
    """Anti-regression checks on prior existing behaviour."""

    def test_year_in_non_date_context_is_not_ordinal(self):
        out = num2words_sentence(
            "The year 2024 marks fifty years since 1974", lang="en"
        )
        # Plain cardinal — must NOT inflect as "twenty-fourth".
        self.assertNotIn("fourth", out)
        self.assertIn("twenty-four", out)

    def test_short_month_abbrev_does_not_partial_match(self):
        # "Mar" must not match "marks". Regression for the \b fix.
        out = num2words_sentence("2024 marks the date", lang="en")
        self.assertNotIn("twenty-fourth", out)

    def test_unsupported_lang_raises(self):
        # Bogus language code should still raise the standard error path,
        # i.e. registry-based dispatch did not silently swallow unknown langs.
        with self.assertRaises(NotImplementedError):
            num2words_sentence("the 5th day", lang="xx-test")


if __name__ == "__main__":
    unittest.main()
