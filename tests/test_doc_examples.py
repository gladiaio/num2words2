# -*- coding: utf-8 -*-
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
"""The examples in the user docs must print what the library prints.

Collected from README.rst, README_num2words2.md, REFERENCE.md and the CLI
``--help`` epilog (gladiaio/num2words2#240):

* ``>>> expr`` followed by its repr on the next line (reStructuredText);
* ``expr  # 'output'`` or ``expr`` then ``# 'output'`` on the next line
  in a fenced python block; ``print(expr)  # output`` compares ``str()``;
* ``$ num2words2 ARGS`` followed by the printed lines (shell blocks).

A block preceded by a ``<!-- doc-examples: skip -->`` (Markdown) or
``.. doc-examples: skip`` (reST) comment is not checked.
"""

import ast
import io
import os
import re
import shlex
import subprocess
import sys
import tokenize

import pytest

import num2words2
from num2words2 import group_digits, maxval, num2words, num2words_sentence
from num2words2.__main__ import EPILOG

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DOCS = ("README.rst", "README_num2words2.md", "REFERENCE.md")
NAMESPACE = {
    "num2words": num2words,
    "num2words_sentence": num2words_sentence,
    "maxval": maxval,
    "group_digits": group_digits,
}
FENCE = re.compile(r"^\s*```(\w*)\s*$")
# ``$ num2words2 ARGS`` in the docs; the epilog has no prompt.
CLI = re.compile(r"^\s*\$ num2words2 (.+)$")
EPILOG_CLI = re.compile(r"^\s*num2words2 (.+)$")
CALL = re.compile(r"[A-Za-z_]\w*\(")


def _split_comment(line):
    """``(code, comment)`` of one python line; comment is None if absent."""
    tokens = list(tokenize.generate_tokens(io.StringIO(line).readline))
    for tok in tokens:
        if tok.type == tokenize.COMMENT:
            return line[: tok.start[1]].strip(), tok.string[1:].strip()
    return line.strip(), None


def _expected(comment):
    """The value an output comment shows: its leading string literal (so
    trailing notes like ``'5'  (no suffix)`` are allowed), else a literal."""
    first = next(tokenize.generate_tokens(io.StringIO(comment).readline))
    if first.type == tokenize.STRING:
        return ast.literal_eval(first.string)
    return ast.literal_eval(comment)


def _python_examples(where, lines):
    for i, line in enumerate(lines):
        code, comment = _split_comment(line)
        printed = code.startswith("print(") and code.endswith(")")
        expr = code[len("print(") : -1] if printed else code
        if not code or not CALL.match(expr):
            continue
        if comment is None and i + 1 < len(lines):
            nxt = lines[i + 1].strip()
            comment = nxt[1:].strip() if nxt.startswith("#") else None
        if comment is not None:
            yield where(i), "print" if printed else "expr", expr, comment


def _cli_examples(where, lines, cli=CLI):
    for i, line in enumerate(lines):
        m = cli.match(line)
        if not m:
            continue
        out = []
        for nxt in lines[i + 1 :]:
            if not nxt.strip() or nxt.lstrip().startswith(("$", "#")):
                break
            if cli.match(nxt):
                break
            out.append(nxt.strip())
        yield where(i), "cli", m.group(1), "\n".join(out)


def _markdown(name, text):
    lines = text.splitlines()
    i = 0
    while i < len(lines):
        m = FENCE.match(lines[i])
        if not m:
            i += 1
            continue
        end = next(j for j in range(i + 1, len(lines)) if FENCE.match(lines[j]))
        before = [x for x in lines[:i] if x.strip()]
        if not (before and "doc-examples: skip" in before[-1]):
            body = lines[i + 1 : end]
            start = i + 2

            def where(k, start=start):
                return "%s:%d" % (name, start + k)

            if m.group(1) == "python":
                for ex in _python_examples(where, body):
                    yield ex
            else:
                for ex in _cli_examples(where, body):
                    yield ex
        i = end + 1


def _rst(name, text):
    lines = text.splitlines()
    kept = []
    skip = False
    for line in lines:
        if "doc-examples: skip" in line:
            skip = True
        elif line.strip() and not line[0].isspace() and line.endswith("::"):
            skip = False  # a new literal block starts after this paragraph
        kept.append("" if skip else line)

    def where(k):
        return "%s:%d" % (name, k + 1)

    for i, line in enumerate(kept):
        expr = line.strip()[4:]
        if line.strip().startswith(">>> ") and CALL.match(expr):
            yield where(i), "expr", expr, kept[i + 1].strip()
    for ex in _cli_examples(where, kept):
        yield ex


def _collect():
    found = []
    for name in DOCS:
        with open(os.path.join(ROOT, name), encoding="utf-8") as f:
            text = f.read()
        parse = _rst if name.endswith(".rst") else _markdown
        found.extend(parse(name, text))
    epilog = EPILOG.splitlines()
    found.extend(_cli_examples(lambda k: "EPILOG:%d" % (k + 1), epilog, EPILOG_CLI))
    return found


EXAMPLES = _collect()


def test_examples_were_found():
    kinds = {kind for _, kind, _, _ in EXAMPLES}
    assert kinds == {"expr", "print", "cli"}
    assert len(EXAMPLES) > 60


@pytest.mark.parametrize(
    "kind, code, comment",
    [ex[1:] for ex in EXAMPLES],
    ids=[ex[0] for ex in EXAMPLES],
)
def test_doc_example(kind, code, comment):
    if kind == "cli":
        result = subprocess.run(
            [sys.executable, "-m", "num2words2"] + shlex.split(code),
            capture_output=True,
            encoding="utf-8",
            env=dict(os.environ, PYTHONIOENCODING="utf-8"),
            check=True,
        )
        if comment:
            assert result.stdout.strip() == comment
        return
    value = eval(code, dict(NAMESPACE, num2words2=num2words2))
    if kind == "print":
        assert str(value) == comment
    else:
        assert value == _expected(comment)


def test_reference_lists_every_language_code():
    """REFERENCE.md's Locale codes section names every code the core
    accepts (gladiaio/num2words2#241: 30 were missing)."""
    with open(os.path.join(ROOT, "REFERENCE.md"), encoding="utf-8") as f:
        text = f.read()
    section = text[text.index("## Locale codes") : text.index("## String input")]
    listed = set(re.findall(r"`([^`]+)`", section))
    assert sorted(set(num2words2._rust.supported_langs()) - listed) == []
