"""Tier-2 v1.0.7 features: cents=mode and style='terse'."""

from num2words2 import num2words


def test_cents_omit_drops_cents_portion():
    # Issue #554 / #190
    assert (
        num2words(55.45, lang="en", to="currency", cents="omit") == "fifty-five euros"
    )
    assert (
        num2words(100.99, lang="en", to="currency", cents="omit") == "one hundred euros"
    )


def test_cents_verbose_full_words():
    # 'verbose' alias = legacy True
    assert (
        num2words(55.45, lang="en", to="currency", cents="verbose")
        == "fifty-five euros, forty-five cents"
    )


def test_cents_terse_keeps_legacy_digit_form():
    # 'terse' alias = legacy False (digits for cents)
    assert "45" in num2words(55.45, lang="en", to="currency", cents="terse")


def test_legacy_cents_bool_unchanged():
    # cents=True/False legacy semantics unchanged.
    assert (
        num2words(55.45, lang="en", to="currency", cents=True)
        == "fifty-five euros, forty-five cents"
    )


def test_terse_ordinal_drops_leading_one():
    # Issue #535 / #147
    assert num2words(100, ordinal=True, lang="en", style="terse") == "hundredth"
    assert num2words(1000, ordinal=True, lang="en", style="terse") == "thousandth"
    # 1 stays "first" (not stripped because it's exactly "first")
    assert num2words(1, ordinal=True, lang="en", style="terse") == "first"


def test_terse_ordinal_default_unchanged():
    assert num2words(100, ordinal=True, lang="en") == "one hundredth"
