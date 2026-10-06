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
"""Command-line entry point: ``num2words2`` and ``python -m num2words2``.

Ports savoirfairelinux/num2words#624 (``bin/num2words`` ->
``num2words/__main__.py``) and #623 (docopt -> argparse), both by @shamilbi.

Both changes are load-bearing here rather than cosmetic:

* **``__main__.py``.** This project builds with maturin, which reads
  ``pyproject.toml`` and ignores ``setup.py`` entirely — so ``setup.py``'s
  ``scripts=["bin/num2words2"]`` never reached a wheel and the documented
  ``num2words2`` command did not exist once installed. The console entry
  point is declared in ``pyproject.toml`` now, and points here.
* **argparse.** ``bin/num2words2`` imports ``docopt``, which is not a
  dependency of the wheel. Even if the script had shipped it would have died
  on the import. argparse is in the standard library.

``bin/num2words2`` also called ``num2words2.CONVERTER_CLASSES``, which this
package does not define — the Rust core exposes ``supported_langs()`` instead
— so ``--list-languages`` raised ``AttributeError``. Fixed here too.
"""
from __future__ import print_function, unicode_literals

import argparse
import decimal
import re
import sys

from . import CONVERTER_TYPES, __version__
from . import _rust as _RUST
from . import num2words

EPILOG = """\
examples:
  num2words2 10001
      ten thousand and one
  num2words2 24120.10 --lang es
      veinticuatro mil ciento veinte punto uno cero
  num2words2 2.14 --lang es --to currency
      dos euros con catorce céntimos
  num2words2 2.14 --to currency --currency USD
      two dollars, fourteen cents
  num2words2 -1e3
      minus one thousand
"""

# A negative number, including exponent forms argparse would otherwise take
# for an option ("-1e3", "-.5", "-2.5E-3").
NEGATIVE_NUMBER = re.compile(r"^-(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?$")


def get_languages():
    """Canonical language codes, sorted; aliases are not repeated."""
    aliases = _RUST.lang_aliases()
    return sorted(c for c in _RUST.supported_langs() if c not in aliases)


def get_language_lines():
    """``--list-languages`` lines: each canonical code, followed by its
    aliases in parentheses (``cs (cz)``)."""
    by_canonical = {}
    for alias, canonical in _RUST.lang_aliases().items():
        by_canonical.setdefault(canonical, []).append(alias)
    lines = []
    for code in get_languages():
        aliases = sorted(by_canonical.get(code, []))
        lines.append(
            "{} ({})".format(code, ", ".join(aliases)) if aliases else code
        )
    return lines


def get_converters():
    """Every ``--to`` value, sorted."""
    return sorted(CONVERTER_TYPES)


def build_parser():
    parser = argparse.ArgumentParser(
        prog="num2words2",
        description="Convert numbers into words.",
        epilog=EPILOG,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "number",
        nargs="?",
        help="number to convert into words",
    )
    parser.add_argument(
        "-L", "--list-languages",
        action="store_true",
        help="list every supported language code (aliases in parentheses) "
             "and exit",
    )
    parser.add_argument(
        "-C", "--list-converters",
        action="store_true",
        help="list every supported converter and exit",
    )
    parser.add_argument(
        "-l", "--lang",
        default="en",
        help="output language (default: %(default)s)",
    )
    parser.add_argument(
        "-t", "--to",
        default="cardinal",
        help="output converter (default: %(default)s)",
    )
    parser.add_argument(
        "-c", "--currency",
        help="ISO 4217 currency code for --to currency/cheque (e.g. USD)",
    )
    parser.add_argument(
        "--cents",
        choices=["verbose", "terse", "omit"],
        help="how to render the cents of a currency amount",
    )
    parser.add_argument(
        "--adjective",
        action="store_true",
        default=None,
        help="prefix the currency name with its adjective (e.g. US dollars)",
    )
    parser.add_argument(
        "--style",
        choices=["terse", "us"],
        help="presentation style: 'terse' ordinals, 'us' English without "
             "'and'",
    )
    parser.add_argument(
        "--errors",
        choices=["raise", "ignore"],
        help="input with no reading (50%%, v2.0.1): 'raise' an error "
             "(default) or 'ignore' and print it as written",
    )
    parser.add_argument(
        "-v", "--version",
        action="version",
        version="num2words2=={}".format(__version__),
    )
    return parser


def parse_args(parser, argv):
    """``parse_args`` that also takes a negative number such as ``-1e3`` as
    the positional ``number`` rather than an unknown option."""
    args, extras = parser.parse_known_args(argv)
    if extras and args.number is None and NEGATIVE_NUMBER.match(extras[0]):
        args.number = extras.pop(0)
    # Python 3.14's argparse hands a dash-prefixed token such as "-1x" to the
    # positional instead of rejecting it as an unknown option; reject it here
    # so the CLI behaves the same on every Python version.
    if (args.number is not None and args.number.startswith("-")
            and not NEGATIVE_NUMBER.match(args.number)):
        extras.insert(0, args.number)
        args.number = None
    if extras:
        parser.error("unrecognized arguments: {}".format(" ".join(extras)))
    return args


def error_message(err, args):
    """One human-readable line for a conversion error, never a Python repr."""
    if isinstance(err, decimal.InvalidOperation):
        return "not a number"
    if isinstance(err, OverflowError):
        return "number is too large for language {!r}".format(args.lang)
    if isinstance(err, ZeroDivisionError):
        return "division by zero"
    if isinstance(err, TypeError) and args.to == "fraction":
        return "--to fraction expects a fraction such as 3/4"
    lines = str(err).strip().splitlines()
    return lines[0] if lines else type(err).__name__


def write_line(text):
    """Print ``text``; if stdout cannot encode it (a cp1252 console),
    switch stdout to UTF-8 with replacement instead of failing."""
    try:
        print(text)
    except UnicodeEncodeError:
        if not hasattr(sys.stdout, "reconfigure"):
            raise
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
        print(text)


def main(argv=None):
    parser = build_parser()
    args = parse_args(parser, argv)

    if args.list_languages:
        for line in get_language_lines():
            write_line(line)
        return 0

    if args.list_converters:
        for converter in get_converters():
            print(converter)
        return 0

    if args.number is None:
        parser.error("the following arguments are required: number")

    kwargs = {
        key: getattr(args, key)
        for key in ("currency", "cents", "adjective", "style", "errors")
        if getattr(args, key) is not None
    }
    try:
        result = num2words(args.number, lang=args.lang, to=args.to, **kwargs)
    except Exception as err:
        # Keep the offending input in the message — the old script printed it
        # too, and it is the only context a shell user gets.
        print(
            "num2words2: cannot convert {!r}: {}".format(
                args.number, error_message(err, args)
            ),
            file=sys.stderr,
        )
        return 1
    write_line(result)
    return 0


if __name__ == "__main__":
    sys.exit(main())
