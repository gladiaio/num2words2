"""Issue #204: a Decimal with more than 65535 fractional digits panicked in
the Rust core (`format!` width out of range). pyo3 surfaced it as
`PanicException`, a `BaseException` that `except Exception` does not catch.
Any ordinary exception or a string is acceptable; a panic is not."""

import sys
from decimal import Decimal

import pytest

from num2words2 import num2words

TINY = Decimal("1E-70000")


def _long_fraction(digit, count):
    """``Decimal("0." + digit * count)``. The pure-Python decimal module
    (used by CPython builds without ``_decimal``, e.g. the 3.15 CI image)
    goes through ``int()``, which refuses more than 4300 digits by default,
    so lift that limit just while building the value."""
    old = (
        sys.get_int_max_str_digits() if hasattr(sys, "get_int_max_str_digits") else None
    )
    if old is not None:
        sys.set_int_max_str_digits(0)
    try:
        return Decimal("0." + digit * count)
    finally:
        if old is not None:
            sys.set_int_max_str_digits(old)


HA_LONG = _long_fraction("7", 70000)

CASES = (
    [
        (lang, "currency", TINY)
        for lang in ("as", "bm", "ki", "lus", "miz", "ne", "om", "zu", "hmn")
    ]
    + [
        (lang, to, TINY)
        for lang in ("ce", "cy", "en_Aero_US_Army", "rm", "sn")
        for to in ("cardinal", "year")
    ]
    + [
        # Not a panic but worse: unbounded recursion overflowed the native stack
        # and killed the interpreter. Now OverflowError.
        ("ha", "cardinal", HA_LONG),
    ]
)


@pytest.mark.parametrize("lang,to,value", CASES, ids=lambda c: str(c)[:20])
def test_huge_scale_never_panics(lang, to, value):
    try:
        out = num2words(value, lang=lang, to=to)
    except Exception:
        return  # a typed, catchable error is fine
    assert isinstance(out, str)


def test_ha_huge_value_is_overflow_error():
    # A 70000-digit fraction no longer recurses: since #205 Hausa reads the
    # fractional digits one by one, so it is words, not an overflow.
    assert num2words(HA_LONG, lang="ha").startswith("sifiri wajen bakwai bakwai")
    with pytest.raises(OverflowError):
        num2words(10**12010, lang="ha")
    assert num2words(10**12, lang="ha") == "tiriliyan"


def test_panic_backstop_is_a_runtime_error():
    # Hand-feeding a 70000-digit precision reaches fo's `{:.p$}` float
    # formatting, which still panics past 65535 (num2words() never passes
    # such a precision). The binder's catch_unwind backstop must turn any
    # panic into an ordinary Exception (RuntimeError), never PanicException.
    from num2words2 import _rust

    try:
        _rust.to_cardinal_float_raw("fo", 1.5, 70000, "", None)
    except Exception as e:
        if "internal error" in str(e):
            assert isinstance(e, RuntimeError)
