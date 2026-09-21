"""Tier-1 v1.0.6 features: maxval helper, runpy entry, precision= kwarg."""

import contextlib
import io
import subprocess
import sys
from unittest import TestCase

import pytest

import num2words2
from num2words2.__main__ import main


def test_maxval_helper():
    # Issue #582
    assert isinstance(num2words2.maxval("en"), int)
    assert num2words2.maxval("en") > 10**100
    assert num2words2.maxval("en_IN") > 10**18
    # Unknown lang raises
    with pytest.raises(NotImplementedError):
        num2words2.maxval("zzz_unknown")


def test_runpy_invocation():
    # Issue #348 — python -m num2words2 N -l X
    out = subprocess.check_output(
        [sys.executable, "-m", "num2words2", "1234", "-l", "fr"]
    ).decode().strip()
    assert out == "mille deux cent trente-quatre"


def test_runpy_list_languages():
    out = subprocess.check_output(
        [sys.executable, "-m", "num2words2", "--list-languages"]
    ).decode()
    assert "en" in out.split()
    assert "fr" in out.split()


def test_precision_kwarg():
    # Issue #580 — precision= overrides default 2-digit fractional precision.
    assert num2words2.num2words(3.14159, lang="en", precision=5) == \
        "three point one four one five nine"
    # Default precision (2) unchanged when no kwarg.
    assert num2words2.num2words(3.14, lang="en") == "three point one four"


class TestCliEntryPoint(TestCase):
    """In-process cover for ``num2words2.__main__``.

    ``test_runpy_invocation`` and ``test_runpy_list_languages`` above exercise
    the same code through a subprocess, which is the honest end-to-end check
    but is invisible to ``coverage``. These call ``main()`` directly so the
    module is measured.

    The CLI itself ports savoirfairelinux/num2words#624 and #623 by @shamilbi.
    """

    def _run(self, argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = main(argv)
        return code, out.getvalue(), err.getvalue()

    def test_converts(self):
        code, out, _ = self._run(["1234", "-l", "fr"])
        self.assertEqual(code, 0)
        self.assertEqual(out.strip(), "mille deux cent trente-quatre")

    def test_defaults_to_english_cardinal(self):
        code, out, _ = self._run(["10001"])
        self.assertEqual(code, 0)
        self.assertEqual(out.strip(), "ten thousand and one")

    def test_to_converter(self):
        code, out, _ = self._run(["2.14", "-l", "es", "--to", "currency"])
        self.assertEqual(code, 0)
        self.assertEqual(out.strip(), "dos euros con catorce céntimos")

    def test_list_languages(self):
        code, out, _ = self._run(["--list-languages"])
        self.assertEqual(code, 0)
        langs = out.split()
        self.assertIn("en", langs)
        self.assertIn("fr", langs)
        # Sorted, and the full set the core accepts — not a hand-kept list.
        self.assertEqual(langs, sorted(langs))

    def test_list_converters(self):
        code, out, _ = self._run(["--list-converters"])
        self.assertEqual(code, 0)
        self.assertIn("cardinal", out.split())
        self.assertIn("ordinal", out.split())

    def test_bad_input_reports_and_exits_nonzero(self):
        code, out, err = self._run(["not-a-number"])
        self.assertEqual(code, 1)
        self.assertEqual(out, "")
        self.assertIn("not-a-number", err)

    def test_missing_number_is_a_usage_error(self):
        # argparse calls parser.error(), which raises SystemExit(2).
        with self.assertRaises(SystemExit) as ctx:
            self._run([])
        self.assertEqual(ctx.exception.code, 2)

    def test_version_flag_exits_zero(self):
        with self.assertRaises(SystemExit) as ctx:
            self._run(["--version"])
        self.assertEqual(ctx.exception.code, 0)
