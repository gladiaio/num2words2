# -*- coding: utf-8 -*-
"""Input normalisation in the shared dispatcher, checked for every language.

Values are normalised once, before any language converter sees them:
exponent notation (gladiaio/num2words2#211, #199, #210), integral values of
any input type in the integer modes (#213), non-integral ordinals and years
(#214), numeric types (#236) and input edge cases (#237).
"""
from __future__ import unicode_literals

from decimal import Decimal

import pytest

from num2words2 import _rust, num2words

LANGS = sorted(_rust.supported_langs())


def _call(x, **kw):
    try:
        return num2words(x, **kw)
    except Exception as e:  # noqa: BLE001 - comparing outcomes is the point
        return type(e)


# ---- #211 / #199 / #210: exponent notation --------------------------------

@pytest.mark.parametrize("lang", LANGS)
@pytest.mark.parametrize("to", ["cardinal", "ordinal", "ordinal_num", "year"])
def test_integer_in_exponent_form_reads_as_the_integer(lang, to):
    want = _call(1000, lang=lang, to=to)
    for x in (Decimal("1E+3"), "1e3", "1E3", Decimal("1.0E+3")):
        assert _call(x, lang=lang, to=to) == want, (x, lang, to)
    assert _call(1e21, lang=lang, to=to) == _call(10**21, lang=lang, to=to)


@pytest.mark.parametrize("lang", LANGS)
def test_small_float_reads_like_its_decimal(lang):
    # repr(1e-05) == '1e-05': read like Decimal('0.00001'), not its mantissa.
    assert (_call(1e-05, lang=lang) == _call(Decimal("0.00001"), lang=lang)
            == _call("1e-5", lang=lang))


@pytest.mark.parametrize("lang", ["pl", "uk", "en", "en_AERO", "fr"])
def test_currency_in_exponent_form(lang):
    assert (num2words(Decimal("1E+3"), lang=lang, to="currency")
            == num2words(Decimal("1000"), lang=lang, to="currency"))


def test_issue_examples():
    assert num2words(Decimal("1E+3"), lang="pl") == "tysiąc"
    assert num2words("1e3", lang="gl") == "un mil"
    assert num2words(1e21, lang="uk") == "один секстильйон"
    # #199: these raised IndexError
    assert num2words(1e21, lang="ce") == num2words(10**21, lang="ce")
    assert num2words(1e21, lang="cy") == num2words(10**21, lang="cy")
    assert num2words(1e21, lang="rm") == num2words(10**21, lang="rm")
    assert num2words(1.5e20, lang="ce") == num2words(15 * 10**19, lang="ce")


@pytest.mark.parametrize("lang", [
    "en_AERO", "en_Aero_ICAO", "en_aero_icao", "en_x_aero_icao", "en_Aero_FAA",
    "en_Aero_NATO", "en_Aero_USN", "en_Aero_US_Navy", "en_Aero_US_Army",
])
def test_en_aero_exponent_and_non_finite(lang):
    # #210: '1e-05' was read digit by digit ("wun zero fife").
    assert (num2words(1e-05, lang=lang)
            == num2words(Decimal("0.00001"), lang=lang))
    assert num2words(1e21, lang=lang) == num2words(10**21, lang=lang)
    # Non-finite strings raise as they do in en, instead of '' / 'minus'.
    for s in ("NaN", "inf", "-inf"):
        assert _call(s, lang=lang) == _call(s, lang="en")
        assert _call(s, lang=lang) in (ValueError, OverflowError)


# ---- #213: integral values of any type in the integer modes ---------------

@pytest.mark.parametrize("lang", LANGS)
@pytest.mark.parametrize("to", ["ordinal", "ordinal_num", "year"])
def test_integral_value_of_any_type_reads_as_the_integer(lang, to):
    want = _call(1999, lang=lang, to=to)
    for x in ("1999", Decimal("1999"), 1999.0, Decimal("1999.0"), "1999.0",
              Decimal("1.999E+3")):
        assert _call(x, lang=lang, to=to) == want, (x, lang, to)


def test_issue_213_examples():
    year = num2words(1999, lang="et", to="year")
    assert num2words("1999", lang="et", to="year") == year
    assert (num2words(1999.0, lang="ms", to="year")
            == num2words(1999, lang="ms", to="year"))
    assert (num2words(Decimal("1999"), lang="ta", to="year")
            == num2words(1999, lang="ta", to="year"))
    assert num2words("5", lang="bn", to="ordinal_num") == "পঞ্চম"
    assert (num2words(1999.0, lang="uk", to="ordinal")
            == num2words(1999, lang="uk", to="ordinal"))
    assert (num2words(Decimal("1999.0"), lang="hi", to="ordinal_num")
            == num2words(1999, lang="hi", to="ordinal_num"))
    # A whole Decimal no longer keeps its scale: "5th", not "5.00th".
    assert num2words(Decimal("5.00"), to="ordinal_num") == "5th"


# ---- #214: non-integral ordinals and years ---------------------------------

@pytest.mark.parametrize("lang", LANGS)
def test_fraction_is_a_type_error_in_integer_modes(lang):
    for to in ("ordinal", "ordinal_num", "year"):
        for x in (2.5, 2.9, -1.5, Decimal("2.5"), "2.5"):
            assert _call(x, lang=lang, to=to) is TypeError, (x, lang, to)
        # A whole float is an integer.
        assert _call(2.0, lang=lang, to=to) == _call(2, lang=lang, to=to)


def test_issue_214_examples():
    for lang in ("cs", "pt_BR", "mn", "br", "bn", "de", "es"):
        with pytest.raises(TypeError):
            num2words(2.5, lang=lang, to="ordinal")
    with pytest.raises(TypeError, match="Cannot treat float 2.5 as ordinal"):
        num2words(2.5, lang="cs", to="ordinal")
    with pytest.raises(TypeError, match="expects an integer"):
        num2words(2024.5, lang="de", to="year")


@pytest.mark.parametrize("lang", LANGS)
def test_ordinal_and_ordinal_num_agree_on_negatives(lang):
    for x in (-3, "-3", -3.0):
        ordinal = _call(x, lang=lang, to="ordinal")
        numeral = _call(x, lang=lang, to="ordinal_num")
        if numeral is NotImplementedError:  # rm*: no ordinal_num at all
            continue
        assert isinstance(ordinal, type) == isinstance(numeral, type), (
            lang, x, ordinal, numeral)


def test_negative_ordinal_num_examples():
    with pytest.raises(TypeError, match="negative"):
        num2words(-3, lang="fi", to="ordinal_num")
    assert num2words(-3, lang="hu", to="ordinal_num") == "-3."


# ---- #236: numeric types ---------------------------------------------------

MODES = ["cardinal", "ordinal", "ordinal_num", "year", "currency", "cheque"]


class _Index(object):
    def __index__(self):
        return 5


class _Float(object):
    def __float__(self):
        return 2.5


def test_index_types_are_integers_in_every_mode():
    import enum

    class E(enum.IntEnum):
        FIVE = 5

    for to in MODES:
        want = num2words(5, to=to)
        assert num2words(E.FIVE, to=to) == want
        assert num2words(_Index(), to=to) == want


def test_float_types_are_floats_in_every_mode():
    for to in ("cardinal", "currency", "cheque"):
        assert num2words(_Float(), to=to) == num2words(2.5, to=to)
    with pytest.raises(TypeError):
        num2words(_Float(), to="ordinal")


def test_numpy_scalars():
    np = pytest.importorskip("numpy")
    for to in MODES:
        assert num2words(np.int64(5), to=to) == num2words(5, to=to)
        assert _call(np.int8(-3), to=to) == _call(-3, to=to)
    for to in ("cardinal", "currency"):
        assert num2words(np.float32(2.5), to=to) == num2words(2.5, to=to)
        assert num2words(np.float64(2.5), to=to) == num2words(2.5, to=to)


@pytest.mark.parametrize("to", MODES + ["fraction"])
def test_bool_is_rejected_in_every_mode(to):
    for b in (True, False):
        with pytest.raises(TypeError, match="bool is not a number"):
            num2words(b, to=to)


@pytest.mark.parametrize("x", [None, [1], object()])
def test_unsupported_types_raise_type_error(x):
    with pytest.raises(TypeError):
        num2words(x)


def test_non_finite_values_raise_clear_errors():
    for x in (float("nan"), Decimal("NaN")):
        for to in ("currency", "cheque"):
            with pytest.raises(ValueError, match="cannot convert NaN to"):
                num2words(x, to=to)
        assert _call(x) == _call("NaN") == ValueError
    for x in (float("inf"), float("-inf"), Decimal("Infinity")):
        with pytest.raises(ValueError, match="cannot convert Infinity to"):
            num2words(x, to="currency")
        assert _call(x) == _call("inf")


# ---- #237: input edge cases ------------------------------------------------

@pytest.mark.parametrize("lang", LANGS)
def test_negative_zero_reads_as_zero(lang):
    for to in ("cardinal", "ordinal", "ordinal_num", "year", "currency"):
        for neg, pos in ((-0.0, 0.0), ("-0.0", "0.0"),
                         (Decimal("-0.0"), Decimal("0.0")),
                         (Decimal("-0"), Decimal("0")), ("-0", "0")):
            assert _call(neg, lang=lang, to=to) == _call(pos, lang=lang, to=to)


def test_negative_zero_examples():
    assert num2words(-0.0, lang="cs") == num2words("-0.0", lang="cs") \
        == "nula čárka nula"
    assert num2words(Decimal("-0"), lang="pl") == "zero"


@pytest.mark.parametrize("s", ["0x10", "0b101", "1__0", "_1", "1_000_", "-_1",
                               "1..2"])
def test_malformed_numeric_strings_raise(s):
    with pytest.raises(ValueError, match="as a number"):
        num2words(s)


def test_well_formed_and_text_strings_unchanged():
    assert num2words("1_000") == "one thousand"
    # Text goes to the sentence converter, not the malformed-number check:
    # it names the token it cannot read (errors='raise', the default) or
    # returns it as written (errors='ignore', #228).
    with pytest.raises(ValueError, match="cannot convert 'H2O'"):
        num2words("H2O")
    assert num2words("H2O", errors="ignore") == "H2O"


@pytest.mark.parametrize("lang", ["hr", "kk", "kz", "lt", "lv", "sk", "sr",
                                  "sr_Cyrl", "sr_Latn", "uk"])
def test_all_zero_fraction_reads_the_digits_written(lang):
    zero = num2words(0, lang=lang)
    one_zero = num2words(1.0, lang=lang)
    assert one_zero.split().count(zero) == 1
    assert num2words("1.0", lang=lang) == one_zero
    assert num2words(Decimal("1.000"), lang=lang).split().count(zero) == 3
    # Leading zeros before a non-zero digit are unchanged.
    assert num2words(Decimal("1.05"), lang=lang).split().count(zero) == 1


def test_uk_all_zero_fraction():
    assert num2words(1.0, lang="uk") == "один кома нуль"


def test_ar_ordinal_overflow_is_checked_up_front():
    from num2words2 import maxval
    m = maxval("ar")
    for to in ("ordinal", "ordinal_num"):
        for x in (m, 10**10000):
            with pytest.raises(OverflowError):
                num2words(x, lang="ar", to=to)
        assert num2words(m - 1, lang="ar", to=to)
