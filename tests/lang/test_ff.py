# -*- coding: utf-8 -*-
"""Fula (ff): word order of the scale nouns (gladiaio/num2words2#263)."""
from __future__ import unicode_literals

from unittest import TestCase

from num2words2 import num2words


class Num2WordsFFTest(TestCase):
    def test_count_follows_scale_noun(self):
        # The count follows the noun, as in CLDR's ff spell-out rules
        # (miliyo <<, miliyaari <<) and Pulaar usage ("miliyaaruuji ɗiɗi");
        # thousands and hundreds already did (ujunere ɗiɗi).
        cases = {
            2000: "ujunere ɗiɗi",
            10**6: "miliyon go'o",
            2 * 10**6: "miliyon ɗiɗi",
            10**9: "miliyaar go'o",
            1002000000: "miliyaar go'o e miliyon ɗiɗi",
        }
        for n, words in cases.items():
            with self.subTest(n=n):
                self.assertEqual(num2words(n, lang="ff"), words)
