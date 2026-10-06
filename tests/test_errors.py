# -*- coding: utf-8 -*-
# Copyright (c) 2003, Taro Ogawa.  All Rights Reserved.
# Copyright (c) 2013, Savoir-faire Linux inc.  All Rights Reserved.

# This library is free software; you can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Lesser General Public License for more details.
# You should have received a copy of the GNU Lesser General Public
# License along with this library; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
# MA 02110-1301 USA

from __future__ import unicode_literals

from unittest import TestCase

from num2words2 import num2words


class Num2WordsErrorsTest(TestCase):
    def test_NotImplementedError(self):
        with self.assertRaises(NotImplementedError):
            num2words(100, lang="unknown_lang")

    def test_types_NotImplementedError(self):
        with self.assertRaises(NotImplementedError):
            num2words(100, lang="en", to="babidibibidiboo!")

    def test_unknown_lang_message_names_the_code(self):
        with self.assertRaisesRegex(NotImplementedError, "'unknown_lang'"):
            num2words(100, lang="unknown_lang")

    def test_unknown_converter_message_lists_choices(self):
        with self.assertRaisesRegex(NotImplementedError, "babidi.*cardinal"):
            num2words(100, lang="en", to="babidibibidiboo!")

    def test_declined_kwarg_is_named(self):
        with self.assertRaisesRegex(NotImplementedError, "frobnicate="):
            num2words(1, lang="en", frobnicate=True)

    def test_number_too_large_is_an_overflow_error(self):
        from num2words2 import NumberTooLargeError

        self.assertTrue(issubclass(NumberTooLargeError, OverflowError))
        with self.assertRaises(OverflowError):
            num2words(10**700, lang="bn")
