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
import sys

from . import CONVERTER_TYPES, __version__
from . import _rust as _RUST
from . import num2words

EPILOG = """\
examples:
  num2words2 10001
      ten thousand and one
  num2words2 24120.10 --lang es
      veinticuatro mil ciento veinte punto uno
  num2words2 2.14 --lang es --to currency
      dos euros con catorce céntimos
"""


def get_languages():
    """Every language code the compiled core will accept, sorted."""
    return sorted(_RUST.supported_langs())


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
        help="list every supported language code and exit",
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
        "-v", "--version",
        action="version",
        version="num2words2=={}".format(__version__),
    )
    return parser


def main(argv=None):
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.list_languages:
        for lang in get_languages():
            print(lang)
        return 0

    if args.list_converters:
        for converter in get_converters():
            print(converter)
        return 0

    if args.number is None:
        parser.error("the following arguments are required: number")

    try:
        print(num2words(args.number, lang=args.lang, to=args.to))
    except Exception as err:
        # Keep the offending input in the message — the old script printed it
        # too, and it is the only context a shell user gets.
        print(
            "num2words2: cannot convert {!r}: {}".format(args.number, err),
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
