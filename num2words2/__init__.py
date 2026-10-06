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
"""num2words2 — a thin Python binder over the Rust conversion core.

Every conversion, and every piece of presentation logic that used to live
here (language-code resolution, the ``style=`` post-processing, the ``cents=``
mode mapping and the whole type dispatch), is now served by the compiled
``_rust`` extension. This module is a pass-through: it re-exports the public
entry points and the exception types, nothing more.
"""
from __future__ import unicode_literals

from decimal import Decimal
from typing import Any, Literal, Optional, Union

from . import _rust as _RUST
from .grouping import group_digits  # noqa: F401  (re-exported)

# Version information, read from the installed distribution's metadata.
# maturin stamps it from pyproject.toml (rewritten from the git tag at
# release), so it cannot drift from what pip reports.
try:
    from importlib.metadata import PackageNotFoundError
    from importlib.metadata import version as _dist_version

    __version__ = _dist_version("num2words2")
except PackageNotFoundError:
    # Imported from a source tree that was never installed.
    __version__ = "unknown"
__version_tuple__ = tuple(
    int(p) if p.isdigit() else p for p in __version__.split(".")
)

# Exception types defined in the compiled core and re-exported so
# ``from num2words2 import NumberTooLargeError`` (and ``except`` on it) keep
# working. RustFallback is the core's "declined" signal, kept importable for
# the same reason.
RustFallback = _RUST.RustFallback
NumberTooLargeError = _RUST.NumberTooLargeError


__all__ = [
    "num2words",
    "num2words_sentence",
    "convert_sentence",
    "sentence_to_words",
    "group_digits",
    "maxval",
    "NumberTooLargeError",
]

CONVERTES_TYPES = [
    "cardinal", "ordinal", "ordinal_num", "year", "currency", "cheque",
    "fraction",
]
CONVERTER_TYPES = CONVERTES_TYPES  # Alias for compatibility

# The values of CONVERTER_TYPES, for type checkers (#244).
ConverterType = Literal[
    "cardinal", "ordinal", "ordinal_num", "year", "currency", "cheque",
    "fraction",
]


def num2words(
    number: Union[int, float, Decimal, str],
    ordinal: bool = False,
    lang: str = "en",
    to: ConverterType = "cardinal",
    **kwargs: Any,
) -> str:
    return _RUST.num2words(number, ordinal, lang, to, **kwargs)


def num2words_sentence(
    sentence: str,
    lang: Optional[str] = "en",
    to: ConverterType = "cardinal",
    **kwargs: Any,
) -> str:
    return _RUST.num2words_sentence(sentence, lang, to, **kwargs)


def maxval(lang: str = "en") -> Optional[int]:
    return _RUST.maxval(lang)


# Aliases for num2words_sentence
convert_sentence = num2words_sentence
sentence_to_words = num2words_sentence
