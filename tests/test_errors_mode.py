"""errors="raise" | "ignore" for numeric tokens that cannot be converted
(Part of #228 / #230)."""

import pytest

from num2words2 import num2words, num2words_sentence

# --- num2words(): errors="raise" by default ---------------------------------


def test_mixed_text_still_converts():
    # #83: text with numbers goes through the sentence converter.
    assert num2words("I have 3 cats") == "I have three cats"
    assert num2words("I have 3 cats", errors="raise") == "I have three cats"


@pytest.mark.parametrize(
    "text,lang,token",
    [
        ("50%", "en", "50%"),
        ("¥100", "ru", "¥100"),
        ("5 m²", "en", "m²"),
        ("call 555-1234.", "en", "555-1234"),
        ("v2.0.1", "en", "v2.0.1"),
    ],
)
def test_num2words_raises_by_default(text, lang, token):
    with pytest.raises(ValueError) as exc:
        num2words(text, lang=lang)
    msg = str(exc.value)
    assert f"'{token}'" in msg
    assert f"lang='{lang}'" in msg
    assert "errors='ignore'" in msg


@pytest.mark.parametrize(
    "text,lang,expected",
    [
        ("50%", "en", "50%"),
        ("¥100", "ru", "¥100"),
        ("5 m²", "en", "Five m²"),
    ],
)
def test_num2words_ignore_returns_token(text, lang, expected):
    assert num2words(text, lang=lang, errors="ignore") == expected


def test_errors_not_forwarded_to_converters():
    # Numeric inputs and kwargs-taking modes must not see the keyword.
    assert num2words(5, errors="raise") == "five"
    assert num2words(5.5, errors="ignore") == "five point five"
    assert num2words("3.50", errors="raise") == "three point five zero"
    assert num2words(5, lang="es", to="currency", errors="ignore") == "cinco euros"
    assert num2words(2, lang="ru", to="ordinal", gender="f", errors="raise") == "вторая"


# --- num2words_sentence(): errors="ignore" by default ----------------------


def test_sentence_ignores_by_default():
    assert num2words_sentence("50% or 3") == "50% or three"


def test_sentence_raise():
    with pytest.raises(ValueError, match="'50%'"):
        num2words_sentence("50% or 3", errors="raise")
    assert num2words_sentence("I have 3 cats", errors="raise") == "I have three cats"


@pytest.mark.parametrize("func", [num2words, num2words_sentence])
@pytest.mark.parametrize("value", ["strict", "coerce", "", None])
def test_invalid_errors_value(func, value):
    with pytest.raises(ValueError, match="'raise' or 'ignore'"):
        func("3", errors=value)
