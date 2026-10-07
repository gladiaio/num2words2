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

from unittest import TestCase

from num2words2 import num2words


class Num2WordsELTest(TestCase):
    """Comprehensive test cases for Greek language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="el"), "μηδέν")
        self.assertEqual(num2words(1, lang="el"), "ένα")
        self.assertEqual(num2words(2, lang="el"), "δύο")
        self.assertEqual(num2words(3, lang="el"), "τρία")
        self.assertEqual(num2words(4, lang="el"), "τέσσερα")
        self.assertEqual(num2words(5, lang="el"), "πέντε")
        self.assertEqual(num2words(6, lang="el"), "έξι")
        self.assertEqual(num2words(7, lang="el"), "επτά")
        self.assertEqual(num2words(8, lang="el"), "οκτώ")
        self.assertEqual(num2words(9, lang="el"), "εννέα")
        self.assertEqual(num2words(10, lang="el"), "δέκα")
        self.assertEqual(num2words(11, lang="el"), "έντεκα")
        self.assertEqual(num2words(12, lang="el"), "δώδεκα")
        self.assertEqual(num2words(13, lang="el"), "δεκατρία")
        self.assertEqual(num2words(14, lang="el"), "δεκατέσσερα")
        self.assertEqual(num2words(15, lang="el"), "δεκαπέντε")
        self.assertEqual(num2words(16, lang="el"), "δεκαέξι")
        self.assertEqual(num2words(17, lang="el"), "δεκαεπτά")
        self.assertEqual(num2words(18, lang="el"), "δεκαοκτώ")
        self.assertEqual(num2words(19, lang="el"), "δεκαεννέα")
        self.assertEqual(num2words(20, lang="el"), "είκοσι")
        self.assertEqual(num2words(21, lang="el"), "είκοσι ένα")
        self.assertEqual(num2words(22, lang="el"), "είκοσι δύο")
        self.assertEqual(num2words(23, lang="el"), "είκοσι τρία")
        self.assertEqual(num2words(24, lang="el"), "είκοσι τέσσερα")
        self.assertEqual(num2words(25, lang="el"), "είκοσι πέντε")
        self.assertEqual(num2words(26, lang="el"), "είκοσι έξι")
        self.assertEqual(num2words(27, lang="el"), "είκοσι επτά")
        self.assertEqual(num2words(28, lang="el"), "είκοσι οκτώ")
        self.assertEqual(num2words(29, lang="el"), "είκοσι εννέα")
        self.assertEqual(num2words(30, lang="el"), "τριάντα")
        self.assertEqual(num2words(31, lang="el"), "τριάντα ένα")
        self.assertEqual(num2words(35, lang="el"), "τριάντα πέντε")
        self.assertEqual(num2words(40, lang="el"), "σαράντα")
        self.assertEqual(num2words(45, lang="el"), "σαράντα πέντε")
        self.assertEqual(num2words(50, lang="el"), "πενήντα")
        self.assertEqual(num2words(55, lang="el"), "πενήντα πέντε")
        self.assertEqual(num2words(60, lang="el"), "εξήντα")
        self.assertEqual(num2words(65, lang="el"), "εξήντα πέντε")
        self.assertEqual(num2words(70, lang="el"), "εβδομήντα")
        self.assertEqual(num2words(75, lang="el"), "εβδομήντα πέντε")
        self.assertEqual(num2words(80, lang="el"), "ογδόντα")
        self.assertEqual(num2words(85, lang="el"), "ογδόντα πέντε")
        self.assertEqual(num2words(90, lang="el"), "ενενήντα")
        self.assertEqual(num2words(95, lang="el"), "ενενήντα πέντε")
        self.assertEqual(num2words(99, lang="el"), "ενενήντα εννέα")
        self.assertEqual(num2words(100, lang="el"), "εκατό")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="el"), "εκατό ένα")
        self.assertEqual(num2words(110, lang="el"), "εκατό δέκα")
        self.assertEqual(num2words(111, lang="el"), "εκατό έντεκα")
        self.assertEqual(num2words(120, lang="el"), "εκατό είκοσι")
        self.assertEqual(num2words(125, lang="el"), "εκατό είκοσι πέντε")
        self.assertEqual(num2words(150, lang="el"), "εκατό πενήντα")
        self.assertEqual(num2words(175, lang="el"), "εκατό εβδομήντα πέντε")
        self.assertEqual(num2words(199, lang="el"), "εκατό ενενήντα εννέα")
        self.assertEqual(num2words(200, lang="el"), "διακόσια")
        self.assertEqual(num2words(201, lang="el"), "διακόσια ένα")
        self.assertEqual(num2words(210, lang="el"), "διακόσια δέκα")
        self.assertEqual(num2words(220, lang="el"), "διακόσια είκοσι")
        self.assertEqual(num2words(250, lang="el"), "διακόσια πενήντα")
        self.assertEqual(num2words(299, lang="el"), "διακόσια ενενήντα εννέα")
        self.assertEqual(num2words(300, lang="el"), "τριακόσια")
        self.assertEqual(num2words(333, lang="el"), "τριακόσια τριάντα τρία")
        self.assertEqual(num2words(400, lang="el"), "τετρακόσια")
        self.assertEqual(num2words(444, lang="el"), "τετρακόσια σαράντα τέσσερα")
        self.assertEqual(num2words(500, lang="el"), "πεντακόσια")
        self.assertEqual(num2words(555, lang="el"), "πεντακόσια πενήντα πέντε")
        self.assertEqual(num2words(600, lang="el"), "εξακόσια")
        self.assertEqual(num2words(666, lang="el"), "εξακόσια εξήντα έξι")
        self.assertEqual(num2words(700, lang="el"), "επτακόσια")
        self.assertEqual(num2words(777, lang="el"), "επτακόσια εβδομήντα επτά")
        self.assertEqual(num2words(800, lang="el"), "οκτακόσια")
        self.assertEqual(num2words(888, lang="el"), "οκτακόσια ογδόντα οκτώ")
        self.assertEqual(num2words(900, lang="el"), "εννιακόσια")
        self.assertEqual(num2words(999, lang="el"), "εννιακόσια ενενήντα εννέα")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="el"), "χίλια")
        self.assertEqual(num2words(1001, lang="el"), "χίλια ένα")
        self.assertEqual(num2words(1010, lang="el"), "χίλια δέκα")
        self.assertEqual(num2words(1100, lang="el"), "χίλια εκατό")
        self.assertEqual(num2words(1111, lang="el"), "χίλια εκατό έντεκα")
        self.assertEqual(num2words(1234, lang="el"), "χίλια διακόσια τριάντα τέσσερα")
        self.assertEqual(num2words(1500, lang="el"), "χίλια πεντακόσια")
        self.assertEqual(num2words(1999, lang="el"), "χίλια εννιακόσια ενενήντα εννέα")
        self.assertEqual(num2words(2000, lang="el"), "δύο χιλιάδες")
        self.assertEqual(num2words(2001, lang="el"), "δύο χιλιάδες ένα")
        self.assertEqual(num2words(2020, lang="el"), "δύο χιλιάδες είκοσι")
        self.assertEqual(num2words(2222, lang="el"), "δύο χιλιάδες διακόσια είκοσι δύο")
        self.assertEqual(num2words(3000, lang="el"), "τρεις χιλιάδες")
        self.assertEqual(
            num2words(3333, lang="el"), "τρεις χιλιάδες τριακόσια τριάντα τρία"
        )
        self.assertEqual(num2words(4000, lang="el"), "τέσσερις χιλιάδες")
        self.assertEqual(
            num2words(4444, lang="el"), "τέσσερις χιλιάδες τετρακόσια σαράντα τέσσερα"
        )
        self.assertEqual(num2words(5000, lang="el"), "πέντε χιλιάδες")
        self.assertEqual(
            num2words(5555, lang="el"), "πέντε χιλιάδες πεντακόσια πενήντα πέντε"
        )
        self.assertEqual(num2words(6000, lang="el"), "έξι χιλιάδες")
        self.assertEqual(num2words(6666, lang="el"), "έξι χιλιάδες εξακόσια εξήντα έξι")
        self.assertEqual(num2words(7000, lang="el"), "επτά χιλιάδες")
        self.assertEqual(
            num2words(7777, lang="el"), "επτά χιλιάδες επτακόσια εβδομήντα επτά"
        )
        self.assertEqual(num2words(8000, lang="el"), "οκτώ χιλιάδες")
        self.assertEqual(
            num2words(8888, lang="el"), "οκτώ χιλιάδες οκτακόσια ογδόντα οκτώ"
        )
        self.assertEqual(num2words(9000, lang="el"), "εννέα χιλιάδες")
        self.assertEqual(
            num2words(9999, lang="el"), "εννέα χιλιάδες εννιακόσια ενενήντα εννέα"
        )
        self.assertEqual(num2words(10000, lang="el"), "δέκα χιλιάδες")
        self.assertEqual(num2words(10001, lang="el"), "δέκα χιλιάδες ένα")
        self.assertEqual(num2words(11111, lang="el"), "έντεκα χιλιάδες εκατό έντεκα")
        self.assertEqual(
            num2words(12345, lang="el"), "δώδεκα χιλιάδες τριακόσια σαράντα πέντε"
        )
        self.assertEqual(num2words(20000, lang="el"), "είκοσι χιλιάδες")
        self.assertEqual(num2words(50000, lang="el"), "πενήντα χιλιάδες")
        self.assertEqual(
            num2words(99999, lang="el"),
            "ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )
        self.assertEqual(num2words(100000, lang="el"), "εκατό χιλιάδες")
        self.assertEqual(
            num2words(123456, lang="el"),
            "εκατό είκοσι τρία χιλιάδες τετρακόσια πενήντα έξι",
        )
        self.assertEqual(num2words(200000, lang="el"), "διακόσιες χιλιάδες")
        self.assertEqual(num2words(500000, lang="el"), "πεντακόσιες χιλιάδες")
        self.assertEqual(
            num2words(654321, lang="el"),
            "εξακόσιες πενήντα τέσσερα χιλιάδες τριακόσια είκοσι ένα",
        )
        self.assertEqual(
            num2words(999999, lang="el"),
            "εννιακόσιες ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="el"), "ένα εκατομμύριο")
        self.assertEqual(num2words(1000001, lang="el"), "ένα εκατομμύριο ένα")
        self.assertEqual(
            num2words(1111111, lang="el"),
            "ένα εκατομμύριο εκατό έντεκα χιλιάδες εκατό έντεκα",
        )
        self.assertEqual(
            num2words(1234567, lang="el"),
            "ένα εκατομμύριο διακόσιες τριάντα τέσσερα χιλιάδες πεντακόσια εξήντα επτά",
        )
        self.assertEqual(num2words(2000000, lang="el"), "δύο εκατομμύρια")
        self.assertEqual(num2words(5000000, lang="el"), "πέντε εκατομμύρια")
        self.assertEqual(
            num2words(9999999, lang="el"),
            "εννέα εκατομμύρια εννιακόσιες ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )
        self.assertEqual(num2words(10000000, lang="el"), "δέκα εκατομμύρια")
        self.assertEqual(
            num2words(12345678, lang="el"),
            "δώδεκα εκατομμύρια τριακόσιες σαράντα πέντε χιλιάδες εξακόσια εβδομήντα οκτώ",
        )
        self.assertEqual(
            num2words(99999999, lang="el"),
            "ενενήντα εννέα εκατομμύρια εννιακόσιες ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )
        self.assertEqual(num2words(100000000, lang="el"), "εκατό εκατομμύρια")
        self.assertEqual(
            num2words(123456789, lang="el"),
            "εκατό είκοσι τρία εκατομμύρια τετρακόσιες πενήντα έξι χιλιάδες επτακόσια ογδόντα εννέα",
        )
        self.assertEqual(
            num2words(999999999, lang="el"),
            "εννιακόσια ενενήντα εννέα εκατομμύρια εννιακόσιες ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )
        self.assertEqual(num2words(1000000000, lang="el"), "ένα δισεκατομμύριο")
        self.assertEqual(
            num2words(1234567890, lang="el"),
            "ένα δισεκατομμύριο διακόσια τριάντα τέσσερα εκατομμύρια πεντακόσιες εξήντα επτά χιλιάδες οκτακόσια ενενήντα",
        )
        self.assertEqual(
            num2words(9999999999, lang="el"),
            "εννέα δισεκατομμύρια εννιακόσια ενενήντα εννέα εκατομμύρια εννιακόσιες ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )
        self.assertEqual(num2words(10000000000, lang="el"), "δέκα δισεκατομμύρια")
        self.assertEqual(
            num2words(99999999999, lang="el"),
            "ενενήντα εννέα δισεκατομμύρια εννιακόσια ενενήντα εννέα εκατομμύρια εννιακόσιες ενενήντα εννέα χιλιάδες εννιακόσια ενενήντα εννέα",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="el"), "μείον ένα")
        self.assertEqual(num2words(-2, lang="el"), "μείον δύο")
        self.assertEqual(num2words(-5, lang="el"), "μείον πέντε")
        self.assertEqual(num2words(-10, lang="el"), "μείον δέκα")
        self.assertEqual(num2words(-11, lang="el"), "μείον έντεκα")
        self.assertEqual(num2words(-20, lang="el"), "μείον είκοσι")
        self.assertEqual(num2words(-50, lang="el"), "μείον πενήντα")
        self.assertEqual(num2words(-99, lang="el"), "μείον ενενήντα εννέα")
        self.assertEqual(num2words(-100, lang="el"), "μείον εκατό")
        self.assertEqual(num2words(-101, lang="el"), "μείον εκατό ένα")
        self.assertEqual(num2words(-200, lang="el"), "μείον διακόσια")
        self.assertEqual(num2words(-999, lang="el"), "μείον εννιακόσια ενενήντα εννέα")
        self.assertEqual(num2words(-1000, lang="el"), "μείον χίλια")
        self.assertEqual(num2words(-1001, lang="el"), "μείον χίλια ένα")
        self.assertEqual(num2words(-10000, lang="el"), "μείον δέκα χιλιάδες")
        self.assertEqual(num2words(-100000, lang="el"), "μείον εκατό χιλιάδες")
        self.assertEqual(num2words(-1000000, lang="el"), "μείον ένα εκατομμύριο")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="el"), "μηδέν κόμμα ένα")
        self.assertEqual(num2words(0.5, lang="el"), "μηδέν κόμμα πέντε")
        self.assertEqual(num2words(0.9, lang="el"), "μηδέν κόμμα εννέα")
        self.assertEqual(num2words(1.1, lang="el"), "ένα κόμμα ένα")
        self.assertEqual(num2words(1.5, lang="el"), "ένα κόμμα πέντε")
        self.assertEqual(num2words(2.5, lang="el"), "δύο κόμμα πέντε")
        self.assertEqual(num2words(3.14, lang="el"), "τρία κόμμα ένα τέσσερα")
        self.assertEqual(num2words(10.5, lang="el"), "δέκα κόμμα πέντε")
        self.assertEqual(num2words(11.11, lang="el"), "έντεκα κόμμα ένα ένα")
        self.assertEqual(num2words(20.2, lang="el"), "είκοσι κόμμα δύο")
        self.assertEqual(
            num2words(99.99, lang="el"), "ενενήντα εννέα κόμμα εννέα εννέα"
        )
        self.assertEqual(num2words(100.01, lang="el"), "εκατό κόμμα μηδέν ένα")
        self.assertEqual(num2words(100.5, lang="el"), "εκατό κόμμα πέντε")
        self.assertEqual(
            num2words(123.45, lang="el"), "εκατό είκοσι τρία κόμμα τέσσερα πέντε"
        )
        self.assertEqual(num2words(1000.5, lang="el"), "χίλια κόμμα πέντε")
        self.assertEqual(
            num2words(1234.56, lang="el"),
            "χίλια διακόσια τριάντα τέσσερα κόμμα πέντε έξι",
        )
        self.assertEqual(
            num2words(10000.01, lang="el"), "δέκα χιλιάδες κόμμα μηδέν ένα"
        )
        self.assertEqual(num2words(-0.5, lang="el"), "μείον μηδέν κόμμα πέντε")
        self.assertEqual(num2words(-1.5, lang="el"), "μείον ένα κόμμα πέντε")
        self.assertEqual(num2words(-10.5, lang="el"), "μείον δέκα κόμμα πέντε")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="el", ordinal=True), "πρώτος")
        self.assertEqual(num2words(2, lang="el", ordinal=True), "δεύτερος")
        self.assertEqual(num2words(3, lang="el", ordinal=True), "τρίτος")
        self.assertEqual(num2words(4, lang="el", ordinal=True), "τέταρτος")
        self.assertEqual(num2words(5, lang="el", ordinal=True), "πέμπτος")
        self.assertEqual(num2words(6, lang="el", ordinal=True), "έκτος")
        self.assertEqual(num2words(7, lang="el", ordinal=True), "έβδομος")
        self.assertEqual(num2words(8, lang="el", ordinal=True), "όγδοος")
        self.assertEqual(num2words(9, lang="el", ordinal=True), "ένατος")
        self.assertEqual(num2words(10, lang="el", ordinal=True), "δέκατος")
        self.assertEqual(num2words(11, lang="el", ordinal=True), "ενδέκατος")
        self.assertEqual(num2words(12, lang="el", ordinal=True), "δωδέκατος")
        self.assertEqual(num2words(13, lang="el", ordinal=True), "δέκατος τρίτος")
        self.assertEqual(num2words(14, lang="el", ordinal=True), "δέκατος τέταρτος")
        self.assertEqual(num2words(15, lang="el", ordinal=True), "δέκατος πέμπτος")
        self.assertEqual(num2words(16, lang="el", ordinal=True), "δέκατος έκτος")
        self.assertEqual(num2words(17, lang="el", ordinal=True), "δέκατος έβδομος")
        self.assertEqual(num2words(18, lang="el", ordinal=True), "δέκατος όγδοος")
        self.assertEqual(num2words(19, lang="el", ordinal=True), "δέκατος ένατος")
        self.assertEqual(num2words(20, lang="el", ordinal=True), "εικοστός")
        self.assertEqual(num2words(21, lang="el", ordinal=True), "εικοστός πρώτος")
        self.assertEqual(num2words(22, lang="el", ordinal=True), "εικοστός δεύτερος")
        self.assertEqual(num2words(25, lang="el", ordinal=True), "εικοστός πέμπτος")
        self.assertEqual(num2words(30, lang="el", ordinal=True), "τριακοστός")
        self.assertEqual(num2words(40, lang="el", ordinal=True), "τεσσαρακοστός")
        self.assertEqual(num2words(50, lang="el", ordinal=True), "πεντηκοστός")
        self.assertEqual(num2words(60, lang="el", ordinal=True), "εξηκοστός")
        self.assertEqual(num2words(70, lang="el", ordinal=True), "εβδομηκοστός")
        self.assertEqual(num2words(80, lang="el", ordinal=True), "ογδοηκοστός")
        self.assertEqual(num2words(90, lang="el", ordinal=True), "ενενηκοστός")
        self.assertEqual(num2words(100, lang="el", ordinal=True), "εκατοστός")
        self.assertEqual(num2words(101, lang="el", ordinal=True), "εκατοστός πρώτος")
        self.assertEqual(num2words(200, lang="el", ordinal=True), "διακοσιοστός")
        self.assertEqual(num2words(500, lang="el", ordinal=True), "πεντακοσιοστός")
        self.assertEqual(num2words(1000, lang="el", ordinal=True), "χιλιοστός")
        self.assertEqual(num2words(1001, lang="el", ordinal=True), "χιλιοστός πρώτος")
        # #251: every component is ordinal; 121..=129 used to collide.
        self.assertEqual(
            num2words(121, lang="el", ordinal=True), "εκατοστός εικοστός πρώτος"
        )
        self.assertEqual(
            num2words(129, lang="el", ordinal=True), "εκατοστός εικοστός ένατος"
        )
        self.assertEqual(num2words(2000, lang="el", ordinal=True), "δισχιλιοστός")
        self.assertEqual(
            num2words(2345, lang="el", ordinal=True),
            "δισχιλιοστός τριακοσιοστός τεσσαρακοστός πέμπτος",
        )
        self.assertEqual(num2words(10**6, lang="el", ordinal=True), "εκατομμυριοστός")
        self.assertEqual(num2words(10000, lang="el", ordinal=True), "δεκακισχιλιοστός")
        # No sourced compound past 9999 (δεκακισχιλιοστός πρώτος? ...).
        with self.assertRaises(OverflowError):
            num2words(10001, lang="el", ordinal=True)

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="el", to="currency", currency="EUR"), "μηδέν ευρώ"
        )
        self.assertEqual(
            num2words(0.01, lang="el", to="currency", currency="EUR"),
            "μηδέν ευρώ και ένα λεπτό",
        )
        self.assertEqual(
            num2words(0.5, lang="el", to="currency", currency="EUR"),
            "μηδέν ευρώ και πενήντα λεπτά",
        )
        self.assertEqual(
            num2words(1, lang="el", to="currency", currency="EUR"), "ένα ευρώ"
        )
        self.assertEqual(
            num2words(1.5, lang="el", to="currency", currency="EUR"),
            "ένα ευρώ και πενήντα λεπτά",
        )
        self.assertEqual(
            num2words(0, lang="el", to="currency", currency="USD"), "μηδέν δολάρια"
        )
        self.assertEqual(
            num2words(0.01, lang="el", to="currency", currency="USD"),
            "μηδέν δολάρια και ένα σεντ",
        )
        self.assertEqual(
            num2words(0.5, lang="el", to="currency", currency="USD"),
            "μηδέν δολάρια και πενήντα σεντς",
        )
        self.assertEqual(
            num2words(1, lang="el", to="currency", currency="USD"), "ένα δολάριο"
        )
        self.assertEqual(
            num2words(1.5, lang="el", to="currency", currency="USD"),
            "ένα δολάριο και πενήντα σεντς",
        )
        self.assertEqual(
            num2words(0, lang="el", to="currency", currency="GBP"), "μηδέν λίρες"
        )
        self.assertEqual(
            num2words(0.01, lang="el", to="currency", currency="GBP"),
            "μηδέν λίρες και μία πέννα",
        )
        self.assertEqual(
            num2words(0.5, lang="el", to="currency", currency="GBP"),
            "μηδέν λίρες και πενήντα πένες",
        )
        self.assertEqual(
            num2words(1, lang="el", to="currency", currency="GBP"), "μία λίρα"
        )
        self.assertEqual(
            num2words(1.5, lang="el", to="currency", currency="GBP"),
            "μία λίρα και πενήντα πένες",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="el", to="year"), "χίλια")
        self.assertEqual(num2words(1066, lang="el", to="year"), "χίλια εξήντα έξι")
        self.assertEqual(
            num2words(1492, lang="el", to="year"), "χίλια τετρακόσια ενενήντα δύο"
        )
        self.assertEqual(
            num2words(1776, lang="el", to="year"), "χίλια επτακόσια εβδομήντα έξι"
        )
        self.assertEqual(num2words(1800, lang="el", to="year"), "χίλια οκτακόσια")
        self.assertEqual(num2words(1900, lang="el", to="year"), "χίλια εννιακόσια")
        self.assertEqual(
            num2words(1984, lang="el", to="year"), "χίλια εννιακόσια ογδόντα τέσσερα"
        )
        self.assertEqual(
            num2words(1999, lang="el", to="year"), "χίλια εννιακόσια ενενήντα εννέα"
        )
        self.assertEqual(num2words(2000, lang="el", to="year"), "δύο χιλιάδες")
        self.assertEqual(num2words(2001, lang="el", to="year"), "δύο χιλιάδες ένα")
        self.assertEqual(num2words(2010, lang="el", to="year"), "δύο χιλιάδες δέκα")
        self.assertEqual(num2words(2020, lang="el", to="year"), "δύο χιλιάδες είκοσι")
        self.assertEqual(
            num2words(2024, lang="el", to="year"), "δύο χιλιάδες είκοσι τέσσερα"
        )
        self.assertEqual(num2words(2100, lang="el", to="year"), "δύο χιλιάδες εκατό")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="el"), "μηδέν")
        self.assertEqual(num2words("1", lang="el"), "ένα")
        self.assertEqual(num2words("10", lang="el"), "δέκα")
        self.assertEqual(num2words("100", lang="el"), "εκατό")
        self.assertEqual(num2words("1000", lang="el"), "χίλια")
        self.assertEqual(num2words("10000", lang="el"), "δέκα χιλιάδες")
        self.assertEqual(num2words("100000", lang="el"), "εκατό χιλιάδες")
        self.assertEqual(num2words("1000000", lang="el"), "ένα εκατομμύριο")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="el"), "μηδέν")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="el"), num2words("100", lang="el"))
        self.assertEqual(num2words(1000, lang="el"), num2words("1000", lang="el"))

        # Test ordinal with floats (should raise error)
        with self.assertRaises(TypeError):
            num2words(3.14, lang="el", ordinal=True)

    def test_more_ordinals(self):
        """Test additional ordinal numbers."""
        # Test special ordinals that follow the converter's logic
        self.assertEqual(num2words(1000000, lang="el", ordinal=True), "εκατομμυριοστός")

    def test_special_hundreds(self):
        """Test special hundreds handling in Greek."""
        # Test numbers with special hundreds rules
        self.assertEqual(num2words(111, lang="el"), "εκατό έντεκα")
        self.assertEqual(num2words(112, lang="el"), "εκατό δώδεκα")
        self.assertEqual(num2words(200, lang="el"), "διακόσια")
        self.assertEqual(num2words(300, lang="el"), "τριακόσια")
        self.assertEqual(num2words(400, lang="el"), "τετρακόσια")
        self.assertEqual(num2words(500, lang="el"), "πεντακόσια")
        self.assertEqual(num2words(600, lang="el"), "εξακόσια")
        self.assertEqual(num2words(700, lang="el"), "επτακόσια")
        self.assertEqual(num2words(800, lang="el"), "οκτακόσια")
        self.assertEqual(num2words(900, lang="el"), "εννιακόσια")
