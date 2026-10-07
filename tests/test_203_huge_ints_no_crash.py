# -*- coding: utf-8 -*-
"""A huge integer never kills the interpreter (gladiaio/num2words2#203).

Languages whose converter recursed over a scale word with no ceiling
overflowed the native stack on ~10**50000 and died with SIGSEGV, which no
``try``/``except`` can catch. Each language runs in its own subprocess (a
few at a time) so a crash is reported per language instead of taking the
test session down. Any Python exception is fine here; only a dead process
is a failure.
"""
from __future__ import unicode_literals

import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

from num2words2 import _rust

SCRIPT = r"""
import sys
from num2words2 import num2words
lang = sys.argv[1]
for to in ("cardinal", "ordinal", "year", "currency"):
    try:
        num2words(10**50000, lang=lang, to=to)
    except Exception:
        pass
"""


def _run(lang):
    proc = subprocess.run(
        [sys.executable, "-c", SCRIPT, lang],
        capture_output=True,
        text=True,
        timeout=120,
    )
    return lang, proc.returncode, proc.stderr[-300:]


def test_huge_integer_does_not_crash_the_process():
    langs = sorted(_rust.supported_langs())
    with ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
        results = list(pool.map(_run, langs))
    crashed = [r for r in results if r[1] != 0]
    assert not crashed, crashed
