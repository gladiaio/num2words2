#!/usr/bin/env python
# -*- coding: utf-8 -*-
# Copyright (c) 2003, Taro Ogawa.  All Rights Reserved.
# Copyright (c) 2013, Savoir-faire Linux inc.  All Rights Reserved.

# This library is free software; you can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Lesser General Public License for more details.
# You should have received a copy of the GNU Lesser General Public
# License along with this library; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
# MA 02110-1301 USA

from __future__ import unicode_literals

import collections
import os
import subprocess
import sys
import unittest

import num2words2 as num2words
from num2words2 import _rust as _RUST

CliResult = collections.namedtuple("CliResult", ["return_code", "out", "err"])


class CliCaller(object):
    """Drives the CLI the way a user would.

    This used to shell out to ``bin/num2words2``, which the maturin wheel
    never installed (``setup.py``'s ``scripts=`` is not read by the build
    backend) and which imported ``docopt``, not a dependency. The entry point
    is ``python -m num2words2`` now — see ``num2words2/__main__.py``.
    """

    def __init__(self):
        self.cmd_list = [sys.executable, "-m", "num2words2"]

    def run_cmd(self, *args):
        cmd_list = self.cmd_list + [str(arg) for arg in args]
        env = os.environ.copy()
        env["PYTHONIOENCODING"] = "utf-8"
        env["PYTHONUTF8"] = "1"
        proc = subprocess.run(
            cmd_list,
            capture_output=True,
            encoding="utf-8",
            env=env,
        )
        return CliResult(
            return_code=proc.returncode,
            out=proc.stdout,
            err=proc.stderr,
        )


class CliTestCase(unittest.TestCase):
    """Test the command line app"""

    def setUp(self):
        self.cli = CliCaller()

    def test_cli_help(self):
        """No arguments is a usage error.

        argparse exits 2 for a usage error and writes "usage: ..." to stderr,
        where docopt exited 1 and capitalised it.
        """
        output = self.cli.run_cmd()
        self.assertEqual(output.return_code, 2)
        self.assertTrue(output.err.lower().startswith("usage:"), output.err)

    def test_cli_list_langs(self):
        """-L lists each canonical code once, aliases after it (#245)."""
        aliases = _RUST.lang_aliases()
        canonical = sorted(c for c in _RUST.supported_langs() if c not in aliases)
        for flag in ("--list-languages", "-L"):
            output = self.cli.run_cmd(flag)
            lines = [out for out in output.out.strip().splitlines() if out]
            self.assertEqual(canonical, [ln.split(" ")[0] for ln in lines])
            self.assertIn("cs (cz)", lines)
            self.assertIn("zh (cn, zh_CN)", lines)
            listed = {ln.split(" ")[0] for ln in lines}
            for alias in ("cz", "dk", "jp", "jw", "cn", "uz_cyr", "en_aero_icao"):
                self.assertNotIn(alias, listed)
        # Every supported code is reachable from the listing.
        shown = set()
        for ln in lines:
            shown.update(ln.replace("(", "").replace(")", "").replace(",", "").split())
        self.assertEqual(shown, set(_RUST.supported_langs()))

    def test_cli_list_converters(self):
        """You should be able to list all available converters"""
        output = self.cli.run_cmd("--list-converters")
        self.assertEqual(
            sorted(list(num2words.CONVERTER_TYPES)),
            [out for out in output.out.strip().splitlines() if out],
        )
        output = self.cli.run_cmd("-C")
        self.assertEqual(
            sorted(list(num2words.CONVERTER_TYPES)),
            [out for out in output.out.strip().splitlines() if out],
        )

    def test_cli_default_lang(self):
        """Default to english"""
        output = self.cli.run_cmd(150)
        self.assertEqual(output.return_code, 0)
        self.assertEqual(output.out.strip(), "one hundred and fifty")

    def test_cli_with_lang(self):
        """You should be able to specify a language"""
        output = self.cli.run_cmd(150, "--lang", "es")
        self.assertEqual(output.return_code, 0)
        self.assertEqual(output.out.strip(), "ciento cincuenta")

    def test_cli_with_lang_to(self):
        """You should be able to specify a language and currency"""
        output = self.cli.run_cmd(150.55, "--lang", "es", "--to", "currency")
        self.assertEqual(output.return_code, 0)
        self.assertEqual(
            (
                output.out.decode("utf-8")
                if hasattr(output.out, "decode")
                else output.out
            ).strip(),
            "ciento cincuenta euros con cincuenta y cinco céntimos",
        )

    def test_cli_error_is_one_clean_line(self):
        """No Python reprs in errors (#245)."""
        for args, msg in [
            (("abc",), "not a number"),
            (("0.5", "-t", "fraction"), "expects a fraction"),
            (("1e400",), "too large"),
        ]:
            output = self.cli.run_cmd(*args)
            self.assertEqual(output.return_code, 1)
            self.assertIn(msg, output.err)
            self.assertEqual(len(output.err.strip().splitlines()), 1, output.err)
            self.assertNotIn("<class", output.err)

    def test_cli_negative_numbers(self):
        """'-1e3' and '-0.5' are numbers, not options (#245)."""
        self.assertEqual(self.cli.run_cmd("-1e3").out.strip(), "minus one thousand")
        self.assertEqual(self.cli.run_cmd("-0.5").out.strip(), "minus zero point five")
        self.assertEqual(self.cli.run_cmd("-5", "-l", "fr").out.strip(), "moins cinq")
        self.assertEqual(
            self.cli.run_cmd("-l", "fr", "-1e3").out.strip(), "moins mille"
        )
        self.assertEqual(self.cli.run_cmd("-1x").return_code, 2)

    def test_cli_currency_options(self):
        """--currency/-c, --cents, --adjective and --style reach num2words."""
        out = self.cli.run_cmd("2.14", "-t", "currency", "-c", "USD").out
        self.assertEqual(out.strip(), "two dollars, fourteen cents")
        out = self.cli.run_cmd(
            "2.14", "-t", "currency", "--currency", "USD", "--adjective"
        ).out
        self.assertEqual(out.strip(), "two US dollars, fourteen cents")
        out = self.cli.run_cmd(
            "2.14", "-t", "currency", "--cents", "terse", "-c", "USD"
        ).out
        self.assertEqual(
            out.strip(),
            num2words.num2words("2.14", to="currency", currency="USD", cents="terse"),
        )
        out = self.cli.run_cmd("101", "-t", "ordinal", "--style", "terse").out
        self.assertEqual(out.strip(), "hundred and first")

    def test_cli_errors_option(self):
        """--errors passes num2words()'s errors= through (#228)."""
        output = self.cli.run_cmd("50%")
        self.assertEqual(output.return_code, 1)
        self.assertIn("cannot convert '50%'", output.err)
        output = self.cli.run_cmd("50%", "--errors", "raise")
        self.assertEqual(output.return_code, 1)
        output = self.cli.run_cmd("50%", "--errors", "ignore")
        self.assertEqual(output.return_code, 0, output.err)
        self.assertEqual(output.out.strip(), "50%")
        self.assertEqual(self.cli.run_cmd("5", "--errors", "nope").return_code, 2)

    def test_cli_non_utf8_stdout(self):
        """A cp1252 stdout prints UTF-8 rather than failing (#245)."""
        env = os.environ.copy()
        env.pop("PYTHONUTF8", None)
        env["PYTHONIOENCODING"] = "cp1252"
        proc = subprocess.run(
            self.cli.cmd_list + ["3", "-l", "ru"],
            capture_output=True,
            env=env,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(proc.stdout.decode("utf-8").strip(), "три")
