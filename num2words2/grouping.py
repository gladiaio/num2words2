# -*- coding: utf-8 -*-
# Copyright (c) 2026, num2words2 contributors. All Rights Reserved.
# Licensed under LGPL-2.1 (see COPYING).
"""Number-string grouping — thin binder over the Rust core.

Some locales group digits in patterns other than the standard ``,###``
Western convention.  :func:`group_digits` returns the locale-grouped form of an
integer; all logic lives in the ``num2words2._rust`` extension.

Currently supported groupings:

- ``"western"``      → ``1,234,567``
- ``"indian"``       → ``12,34,567``        (lakh / crore scale)
- ``"chinese"``      → ``123,4567``         (myriad scale, every 4 digits)

Issue #66; ports the request in savoirfairelinux/num2words#547.
"""

from __future__ import unicode_literals

from . import _rust as _RUST


def group_digits(value: int, locale: str = "western", separator: str = ",") -> str:
    """Format ``value`` as a digit-grouped string.

    Examples
    --------
    >>> group_digits(100000, locale="indian")
    '1,00,000'
    >>> group_digits(1234567, locale="western")
    '1,234,567'
    >>> group_digits(12345678, locale="chinese")
    '1234,5678'
    """
    return _RUST.group_digits(value, locale, separator)
