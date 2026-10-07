# Tests for legacy pre-ISO-639 language aliases.
#
# Refs:
#   savoirfairelinux/num2words#431 (cz -> cs)
#   savoirfairelinux/num2words#425 (dk -> da)
import unittest

from num2words2 import num2words


class LegacyAliasTest(unittest.TestCase):
    def test_cz_alias_routes_to_cs(self):
        self.assertEqual(num2words(5, lang="cz"), num2words(5, lang="cs"))
        self.assertEqual(num2words(1.5, lang="cz"), num2words(1.5, lang="cs"))

    def test_dk_alias_routes_to_da(self):
        self.assertEqual(num2words(5, lang="dk"), num2words(5, lang="da"))
        self.assertEqual(num2words(1829794, lang="dk"), num2words(1829794, lang="da"))

    def test_cz_no_longer_raises(self):
        # Previously raised NotImplementedError. Pre-ISO-639 callers had to
        # remap to "cs" themselves; this is the upstream pain point in #431.
        try:
            num2words(5, lang="cz")
        except NotImplementedError:  # pragma: no cover - regression guard
            self.fail("cz alias should not raise NotImplementedError")

    def test_dk_no_longer_raises(self):
        try:
            num2words(5, lang="dk")
        except NotImplementedError:  # pragma: no cover - regression guard
            self.fail("dk alias should not raise NotImplementedError")
