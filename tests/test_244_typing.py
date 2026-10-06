# -*- coding: utf-8 -*-
"""The package's typing surface (gladiaio/num2words2#244)."""
import ast
import inspect
import os
import typing

import num2words2
from num2words2 import _rust, grouping

STUB = os.path.join(os.path.dirname(num2words2.__file__), "_rust.pyi")


def _stub_names():
    tree = ast.parse(open(STUB, encoding="utf-8").read())
    return {n.name for n in tree.body
            if isinstance(n, (ast.FunctionDef, ast.ClassDef))}


def test_stub_covers_every_public_name_of_the_extension():
    exported = {n for n in dir(_rust) if not n.startswith("_")}
    assert exported - _stub_names() == set()
    assert _stub_names() - exported == set()


def test_public_functions_are_annotated():
    for fn in (num2words2.num2words, num2words2.num2words_sentence,
               num2words2.maxval, grouping.group_digits):
        sig = inspect.signature(fn)
        assert sig.return_annotation is not inspect.Signature.empty, fn
        for p in sig.parameters.values():
            assert p.annotation is not inspect.Parameter.empty, (fn, p)


def test_converter_literal_matches_converter_types():
    assert set(typing.get_args(num2words2.ConverterType)) == set(
        num2words2.CONVERTER_TYPES)


def test_errors_is_a_typed_keyword_only_option():
    # #228's errors= on both entry points, with their different defaults.
    for fn, default in ((num2words2.num2words, "raise"),
                        (num2words2.num2words_sentence, "ignore")):
        p = inspect.signature(fn).parameters["errors"]
        assert p.kind is inspect.Parameter.KEYWORD_ONLY
        assert p.default == default
        assert set(typing.get_args(p.annotation)) == {"raise", "ignore"}
