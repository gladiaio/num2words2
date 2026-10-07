# The zero word of a currency reading is the language's own (fix/a5-latin).
# These languages raise for USD/EUR since #222 (no native noun), so the
# zero word is checked with each language's default currency instead.
import pytest

from num2words2 import num2words

CASES = [
    ("br", "mann euroioù"),
    ("haw", "'ole kālā"),
    ("mg", "aotra ariary"),
    ("mi", "kore tāra"),
    ("tk", "nol manat"),
    ("yo", "òdo náírà"),
    ("ln", "libúngútulú faranga"),
    ("wo", "tus dërëm"),
]


@pytest.mark.parametrize("lang, expected", CASES)
def test_zero_currency_uses_native_zero(lang, expected):
    assert num2words(0, lang=lang, to="currency") == expected
