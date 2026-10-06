# Type stubs for the compiled ``num2words2._rust`` extension
# (rust/num2words2-py/src/lib.rs, ``#[pymodule] fn _rust``).
# gladiaio/num2words2#244. Keep in step with the ``m.add*`` calls there.

from decimal import Decimal
from typing import Any, Literal

_ConverterType = Literal[
    "cardinal", "ordinal", "ordinal_num", "year", "currency", "cheque",
    "fraction",
]
_ErrorsMode = Literal["raise", "ignore"]
# A kwarg value as the binder extracts it (``PyKw``); ``None`` is allowed.
_KwValue = bool | int | str | list[str] | None
_Kwargs = list[tuple[str, _KwValue]]

class RustFallback(Exception): ...
class NumberTooLargeError(OverflowError): ...

def supported_langs() -> list[str]: ...
def default_currency(lang: str) -> str: ...
def lang_aliases() -> dict[str, str]: ...

# Low-level per-converter entry points. ``None`` mirrors the core's
# ``Option<String>`` (a historic bare-``None`` return of lang_VI).
def to_cardinal(lang: str, value: int, /) -> str | None: ...
def to_ordinal(lang: str, value: int, /) -> str | None: ...
def to_ordinal_num(lang: str, value: int, /) -> str | None: ...
def to_year(lang: str, value: int, /) -> str | None: ...
def to_fraction(lang: str, numerator: int, denominator: int, /) -> str | None: ...
def to_currency(
    lang: str,
    value: str,
    is_int: bool,
    has_decimal: bool,
    is_float: bool,
    currency: str | None,
    cents: bool,
    separator: str | None,
    adjective: bool | None,
) -> str | None: ...
def to_cheque(lang: str, value: str, currency: str | None = None) -> str | None: ...
def to_cardinal_float(
    lang: str,
    value: float,
    precision: int,
    decimal_str: str,
    precision_override: int | None,
) -> str | None: ...
def to_cardinal_float_raw(
    lang: str,
    value: float,
    precision: int,
    decimal_str: str,
    precision_override: int | None,
) -> str | None: ...
def to_float(
    lang: str,
    to: str,
    value: float,
    precision: int,
    decimal_str: str,
    repr_str: str,
    precision_override: int | None,
    kwargs: _Kwargs,
) -> str | None: ...
def to_cardinal_kw(lang: str, value: int, kwargs: _Kwargs, /) -> str | None: ...
def to_ordinal_kw(lang: str, value: int, kwargs: _Kwargs, /) -> str | None: ...
def to_ordinal_num_kw(lang: str, value: int, kwargs: _Kwargs, /) -> str | None: ...
def to_year_kw(lang: str, value: int, kwargs: _Kwargs, /) -> str | None: ...
def to_currency_kw(
    lang: str,
    value: str,
    is_int: bool,
    has_decimal: bool,
    is_float: bool,
    currency: str | None,
    cents: bool,
    separator: str | None,
    adjective: bool | None,
    kwargs: _Kwargs,
) -> str | None: ...
def from_string(
    lang: str,
    s: str,
    to: str,
    currency: str | None,
    cents: bool,
    separator: str | None,
    adjective: bool | None,
    kwargs: _Kwargs,
) -> tuple[int, str | None]: ...

# The public entry points re-exported by ``num2words2``.
# ``errors=`` is read from the kwargs by the binder (#228); the defaults
# differ: "raise" for one number, "ignore" for running text.
def num2words(
    number: int | float | Decimal | str,
    ordinal: bool = False,
    lang: str = "en",
    to: _ConverterType = "cardinal",
    *,
    errors: _ErrorsMode = "raise",
    **kwargs: Any,
) -> str | None: ...
def num2words_sentence(
    sentence: str,
    lang: str | None = "en",
    to: str = "cardinal",
    *,
    errors: _ErrorsMode = "ignore",
    **kwargs: Any,
) -> str: ...
def group_digits(value: int, locale: str, separator: str) -> str: ...
def maxval(lang: str) -> int | None: ...
def sentence(text: str, lang: str, to: str) -> str: ...
def sentence_auto(text: str, to: str) -> str: ...
def detect_language(text: str) -> str | None: ...
