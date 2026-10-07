"""v1.0.8: style='us' drops 'and' in English."""
from num2words2 import num2words


def test_us_drops_and_in_cardinal():
    assert (
        num2words(1234, lang="en", style="us")
        == "one thousand, two hundred thirty-four"
    )
    assert num2words(250, lang="en", style="us") == "two hundred fifty"


def test_us_drops_and_in_ordinal():
    assert num2words(101, ordinal=True, lang="en", style="us") == "one hundred first"


def test_us_default_unchanged():
    assert num2words(1234, lang="en") == "one thousand, two hundred and thirty-four"
