"""num2words_sentence runs in linear time (#256).

It used to rebuild the text prefix and splice the output once per number,
so 232k characters took ~87 s. The check compares a text with its 4x
repetition: linear time gives a ~4x ratio, the old quadratic one ~16x. A
ratio is used instead of a fixed budget because debug builds on a loaded
machine are an order of magnitude slower than release wheels.
"""

import time

from num2words2 import num2words_sentence

BASE = "On 3 May 2024 we paid $1,234.56 for 12 items and 7 boxes. "


def _best_of(text, runs=2):
    best = float("inf")
    for _ in range(runs):
        start = time.perf_counter()
        num2words_sentence(text)
        best = min(best, time.perf_counter() - start)
    return best


def test_sentence_scales_linearly():
    num2words_sentence(BASE)  # warm up the lazily compiled regexes
    small = BASE * 850  # ~50k chars
    large = BASE * 3400  # ~200k chars
    t_small = _best_of(small)
    t_large = _best_of(large, runs=1)
    assert t_large < 30, t_large
    assert t_large < 8 * t_small + 0.5, (t_small, t_large)


def test_long_text_output_is_per_sentence():
    # The single forward pass puts every replacement in its place.
    out = num2words_sentence(BASE * 200)
    first = num2words_sentence(BASE).strip()
    assert out.count(first.split(" ", 1)[1]) == 200
