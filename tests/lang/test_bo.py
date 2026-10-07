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


class Num2WordsBOTest(TestCase):
    """Comprehensive test cases for Tibetan language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="bo"), "ཀླད་ཀོར་")
        self.assertEqual(num2words(1, lang="bo"), "གཅིག་")
        self.assertEqual(num2words(2, lang="bo"), "གཉིས་")
        self.assertEqual(num2words(3, lang="bo"), "གསུམ་")
        self.assertEqual(num2words(4, lang="bo"), "བཞི་")
        self.assertEqual(num2words(5, lang="bo"), "ལྔ་")
        self.assertEqual(num2words(6, lang="bo"), "དྲུག་")
        self.assertEqual(num2words(7, lang="bo"), "བདུན་")
        self.assertEqual(num2words(8, lang="bo"), "བརྒྱད་")
        self.assertEqual(num2words(9, lang="bo"), "དགུ་")
        self.assertEqual(num2words(10, lang="bo"), "བཅུ་")
        self.assertEqual(num2words(11, lang="bo"), "བཅུ་གཅིག་")
        self.assertEqual(num2words(12, lang="bo"), "བཅུ་གཉིས་")
        self.assertEqual(num2words(13, lang="bo"), "བཅུ་གསུམ་")
        self.assertEqual(num2words(14, lang="bo"), "བཅུ་བཞི་")
        self.assertEqual(num2words(15, lang="bo"), "བཅུ་ལྔ་")
        self.assertEqual(num2words(16, lang="bo"), "བཅུ་དྲུག་")
        self.assertEqual(num2words(17, lang="bo"), "བཅུ་བདུན་")
        self.assertEqual(num2words(18, lang="bo"), "བཅུ་བརྒྱད་")
        self.assertEqual(num2words(19, lang="bo"), "བཅུ་དགུ་")
        self.assertEqual(num2words(20, lang="bo"), "ཉི་ཤུ་")
        self.assertEqual(num2words(21, lang="bo"), "ཉི་ཤུ་གཅིག་")
        self.assertEqual(num2words(22, lang="bo"), "ཉི་ཤུ་གཉིས་")
        self.assertEqual(num2words(23, lang="bo"), "ཉི་ཤུ་གསུམ་")
        self.assertEqual(num2words(24, lang="bo"), "ཉི་ཤུ་བཞི་")
        self.assertEqual(num2words(25, lang="bo"), "ཉི་ཤུ་ལྔ་")
        self.assertEqual(num2words(26, lang="bo"), "ཉི་ཤུ་དྲུག་")
        self.assertEqual(num2words(27, lang="bo"), "ཉི་ཤུ་བདུན་")
        self.assertEqual(num2words(28, lang="bo"), "ཉི་ཤུ་བརྒྱད་")
        self.assertEqual(num2words(29, lang="bo"), "ཉི་ཤུ་དགུ་")
        self.assertEqual(num2words(30, lang="bo"), "སུམ་ཅུ་")
        self.assertEqual(num2words(31, lang="bo"), "སུམ་ཅུ་གཅིག་")
        self.assertEqual(num2words(35, lang="bo"), "སུམ་ཅུ་ལྔ་")
        self.assertEqual(num2words(40, lang="bo"), "བཞི་བཅུ་")
        self.assertEqual(num2words(45, lang="bo"), "བཞི་བཅུ་ལྔ་")
        self.assertEqual(num2words(50, lang="bo"), "ལྔ་བཅུ་")
        self.assertEqual(num2words(55, lang="bo"), "ལྔ་བཅུ་ལྔ་")
        self.assertEqual(num2words(60, lang="bo"), "དྲུག་ཅུ་")
        self.assertEqual(num2words(65, lang="bo"), "དྲུག་ཅུ་ལྔ་")
        self.assertEqual(num2words(70, lang="bo"), "བདུན་ཅུ་")
        self.assertEqual(num2words(75, lang="bo"), "བདུན་ཅུ་ལྔ་")
        self.assertEqual(num2words(80, lang="bo"), "བརྒྱད་ཅུ་")
        self.assertEqual(num2words(85, lang="bo"), "བརྒྱད་ཅུ་ལྔ་")
        self.assertEqual(num2words(90, lang="bo"), "དགུ་བཅུ་")
        self.assertEqual(num2words(95, lang="bo"), "དགུ་བཅུ་ལྔ་")
        self.assertEqual(num2words(99, lang="bo"), "དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(100, lang="bo"), "གཅིག་བརྒྱ་")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="bo"), "གཅིག་བརྒྱ་གཅིག་")
        self.assertEqual(num2words(110, lang="bo"), "གཅིག་བརྒྱ་བཅུ་")
        self.assertEqual(num2words(111, lang="bo"), "གཅིག་བརྒྱ་བཅུ་གཅིག་")
        self.assertEqual(num2words(120, lang="bo"), "གཅིག་བརྒྱ་ཉི་ཤུ་")
        self.assertEqual(num2words(125, lang="bo"), "གཅིག་བརྒྱ་ཉི་ཤུ་ལྔ་")
        self.assertEqual(num2words(150, lang="bo"), "གཅིག་བརྒྱ་ལྔ་བཅུ་")
        self.assertEqual(num2words(175, lang="bo"), "གཅིག་བརྒྱ་བདུན་ཅུ་ལྔ་")
        self.assertEqual(num2words(199, lang="bo"), "གཅིག་བརྒྱ་དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(200, lang="bo"), "གཉིས་བརྒྱ་")
        self.assertEqual(num2words(201, lang="bo"), "གཉིས་བརྒྱ་གཅིག་")
        self.assertEqual(num2words(210, lang="bo"), "གཉིས་བརྒྱ་བཅུ་")
        self.assertEqual(num2words(220, lang="bo"), "གཉིས་བརྒྱ་ཉི་ཤུ་")
        self.assertEqual(num2words(250, lang="bo"), "གཉིས་བརྒྱ་ལྔ་བཅུ་")
        self.assertEqual(num2words(299, lang="bo"), "གཉིས་བརྒྱ་དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(300, lang="bo"), "གསུམ་བརྒྱ་")
        self.assertEqual(num2words(333, lang="bo"), "གསུམ་བརྒྱ་སུམ་ཅུ་གསུམ་")
        self.assertEqual(num2words(400, lang="bo"), "བཞི་བརྒྱ་")
        self.assertEqual(num2words(444, lang="bo"), "བཞི་བརྒྱ་བཞི་བཅུ་བཞི་")
        self.assertEqual(num2words(500, lang="bo"), "ལྔ་བརྒྱ་")
        self.assertEqual(num2words(555, lang="bo"), "ལྔ་བརྒྱ་ལྔ་བཅུ་ལྔ་")
        self.assertEqual(num2words(600, lang="bo"), "དྲུག་བརྒྱ་")
        self.assertEqual(num2words(666, lang="bo"), "དྲུག་བརྒྱ་དྲུག་ཅུ་དྲུག་")
        self.assertEqual(num2words(700, lang="bo"), "བདུན་བརྒྱ་")
        self.assertEqual(num2words(777, lang="bo"), "བདུན་བརྒྱ་བདུན་ཅུ་བདུན་")
        self.assertEqual(num2words(800, lang="bo"), "བརྒྱད་བརྒྱ་")
        self.assertEqual(num2words(888, lang="bo"), "བརྒྱད་བརྒྱ་བརྒྱད་ཅུ་བརྒྱད་")
        self.assertEqual(num2words(900, lang="bo"), "དགུ་བརྒྱ་")
        self.assertEqual(num2words(999, lang="bo"), "དགུ་བརྒྱ་དགུ་བཅུ་དགུ་")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="bo"), "གཅིག་སྟོང་")
        self.assertEqual(num2words(1001, lang="bo"), "གཅིག་སྟོང་གཅིག་")
        self.assertEqual(num2words(1010, lang="bo"), "གཅིག་སྟོང་བཅུ་")
        self.assertEqual(num2words(1100, lang="bo"), "གཅིག་སྟོང་གཅིག་བརྒྱ་")
        self.assertEqual(num2words(1111, lang="bo"), "གཅིག་སྟོང་གཅིག་བརྒྱ་བཅུ་གཅིག་")
        self.assertEqual(num2words(1234, lang="bo"), "གཅིག་སྟོང་གཉིས་བརྒྱ་སུམ་ཅུ་བཞི་")
        self.assertEqual(num2words(1500, lang="bo"), "གཅིག་སྟོང་ལྔ་བརྒྱ་")
        self.assertEqual(num2words(1999, lang="bo"), "གཅིག་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(2000, lang="bo"), "གཉིས་སྟོང་")
        self.assertEqual(num2words(2001, lang="bo"), "གཉིས་སྟོང་གཅིག་")
        self.assertEqual(num2words(2020, lang="bo"), "གཉིས་སྟོང་ཉི་ཤུ་")
        self.assertEqual(num2words(2222, lang="bo"), "གཉིས་སྟོང་གཉིས་བརྒྱ་ཉི་ཤུ་གཉིས་")
        self.assertEqual(num2words(3000, lang="bo"), "གསུམ་སྟོང་")
        self.assertEqual(num2words(3333, lang="bo"), "གསུམ་སྟོང་གསུམ་བརྒྱ་སུམ་ཅུ་གསུམ་")
        self.assertEqual(num2words(4000, lang="bo"), "བཞི་སྟོང་")
        self.assertEqual(num2words(4444, lang="bo"), "བཞི་སྟོང་བཞི་བརྒྱ་བཞི་བཅུ་བཞི་")
        self.assertEqual(num2words(5000, lang="bo"), "ལྔ་སྟོང་")
        self.assertEqual(num2words(5555, lang="bo"), "ལྔ་སྟོང་ལྔ་བརྒྱ་ལྔ་བཅུ་ལྔ་")
        self.assertEqual(num2words(6000, lang="bo"), "དྲུག་སྟོང་")
        self.assertEqual(
            num2words(6666, lang="bo"), "དྲུག་སྟོང་དྲུག་བརྒྱ་དྲུག་ཅུ་དྲུག་"
        )
        self.assertEqual(num2words(7000, lang="bo"), "བདུན་སྟོང་")
        self.assertEqual(
            num2words(7777, lang="bo"), "བདུན་སྟོང་བདུན་བརྒྱ་བདུན་ཅུ་བདུན་"
        )
        self.assertEqual(num2words(8000, lang="bo"), "བརྒྱད་སྟོང་")
        self.assertEqual(
            num2words(8888, lang="bo"), "བརྒྱད་སྟོང་བརྒྱད་བརྒྱ་བརྒྱད་ཅུ་བརྒྱད་"
        )
        self.assertEqual(num2words(9000, lang="bo"), "དགུ་སྟོང་")
        self.assertEqual(num2words(9999, lang="bo"), "དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(10000, lang="bo"), "གཅིག་ཁྲི་")
        self.assertEqual(num2words(10001, lang="bo"), "གཅིག་ཁྲི་གཅིག་")
        self.assertEqual(
            num2words(11111, lang="bo"), "གཅིག་ཁྲི་གཅིག་སྟོང་གཅིག་བརྒྱ་བཅུ་གཅིག་"
        )
        self.assertEqual(
            num2words(12345, lang="bo"), "གཅིག་ཁྲི་གཉིས་སྟོང་གསུམ་བརྒྱ་བཞི་བཅུ་ལྔ་"
        )
        self.assertEqual(num2words(20000, lang="bo"), "གཉིས་ཁྲི་")
        self.assertEqual(num2words(50000, lang="bo"), "ལྔ་ཁྲི་")
        self.assertEqual(
            num2words(99999, lang="bo"), "དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་"
        )
        self.assertEqual(num2words(100000, lang="bo"), "གཅིག་འབུམ་")
        self.assertEqual(
            num2words(123456, lang="bo"),
            "གཅིག་འབུམ་གཉིས་ཁྲི་གསུམ་སྟོང་བཞི་བརྒྱ་ལྔ་བཅུ་དྲུག་",
        )
        self.assertEqual(num2words(200000, lang="bo"), "གཉིས་འབུམ་")
        self.assertEqual(num2words(500000, lang="bo"), "ལྔ་འབུམ་")
        self.assertEqual(
            num2words(654321, lang="bo"),
            "དྲུག་འབུམ་ལྔ་ཁྲི་བཞི་སྟོང་གསུམ་བརྒྱ་ཉི་ཤུ་གཅིག་",
        )
        self.assertEqual(
            num2words(999999, lang="bo"),
            "དགུ་འབུམ་དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="bo"), "གཅིག་ས་ཡ་")
        self.assertEqual(num2words(1000001, lang="bo"), "གཅིག་ས་ཡ་གཅིག་")
        self.assertEqual(
            num2words(1111111, lang="bo"),
            "གཅིག་ས་ཡ་གཅིག་འབུམ་གཅིག་ཁྲི་གཅིག་སྟོང་གཅིག་བརྒྱ་བཅུ་གཅིག་",
        )
        self.assertEqual(
            num2words(1234567, lang="bo"),
            "གཅིག་ས་ཡ་གཉིས་འབུམ་གསུམ་ཁྲི་བཞི་སྟོང་ལྔ་བརྒྱ་དྲུག་ཅུ་བདུན་",
        )
        self.assertEqual(num2words(2000000, lang="bo"), "གཉིས་ས་ཡ་")
        self.assertEqual(num2words(5000000, lang="bo"), "ལྔ་ས་ཡ་")
        self.assertEqual(
            num2words(9999999, lang="bo"),
            "དགུ་ས་ཡ་དགུ་འབུམ་དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )
        self.assertEqual(num2words(10000000, lang="bo"), "གཅིག་བྱེ་བ་")
        self.assertEqual(
            num2words(12345678, lang="bo"),
            "གཅིག་བྱེ་བ་གཉིས་ས་ཡ་གསུམ་འབུམ་བཞི་ཁྲི་ལྔ་སྟོང་དྲུག་བརྒྱ་བདུན་ཅུ་བརྒྱད་",
        )
        self.assertEqual(
            num2words(99999999, lang="bo"),
            "དགུ་བྱེ་བ་དགུ་ས་ཡ་དགུ་འབུམ་དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )
        self.assertEqual(num2words(100000000, lang="bo"), "གཅིག་དུང་ཕྱུར་")
        self.assertEqual(
            num2words(123456789, lang="bo"),
            "གཅིག་དུང་ཕྱུར་གཉིས་བྱེ་བ་གསུམ་ས་ཡ་བཞི་འབུམ་ལྔ་ཁྲི་དྲུག་སྟོང་བདུན་བརྒྱ་བརྒྱད་ཅུ་དགུ་",
        )
        self.assertEqual(
            num2words(999999999, lang="bo"),
            "དགུ་དུང་ཕྱུར་དགུ་བྱེ་བ་དགུ་ས་ཡ་དགུ་འབུམ་དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )
        self.assertEqual(num2words(1000000000, lang="bo"), "བཅུ་དུང་ཕྱུར་")
        self.assertEqual(
            num2words(1234567890, lang="bo"),
            "བཅུ་གཉིས་དུང་ཕྱུར་གསུམ་བྱེ་བ་བཞི་ས་ཡ་ལྔ་འབུམ་དྲུག་ཁྲི་བདུན་སྟོང་བརྒྱད་བརྒྱ་དགུ་བཅུ་",
        )
        self.assertEqual(
            num2words(9999999999, lang="bo"),
            "དགུ་བཅུ་དགུ་དུང་ཕྱུར་དགུ་བྱེ་བ་དགུ་ས་ཡ་དགུ་འབུམ་དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )
        self.assertEqual(num2words(10000000000, lang="bo"), "གཅིག་བརྒྱ་དུང་ཕྱུར་")
        self.assertEqual(
            num2words(99999999999, lang="bo"),
            "དགུ་བརྒྱ་དགུ་བཅུ་དགུ་དུང་ཕྱུར་དགུ་བྱེ་བ་དགུ་ས་ཡ་དགུ་འབུམ་དགུ་ཁྲི་དགུ་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="bo"), "མེད་ཆ་ གཅིག་")
        self.assertEqual(num2words(-2, lang="bo"), "མེད་ཆ་ གཉིས་")
        self.assertEqual(num2words(-5, lang="bo"), "མེད་ཆ་ ལྔ་")
        self.assertEqual(num2words(-10, lang="bo"), "མེད་ཆ་ བཅུ་")
        self.assertEqual(num2words(-11, lang="bo"), "མེད་ཆ་ བཅུ་གཅིག་")
        self.assertEqual(num2words(-20, lang="bo"), "མེད་ཆ་ ཉི་ཤུ་")
        self.assertEqual(num2words(-50, lang="bo"), "མེད་ཆ་ ལྔ་བཅུ་")
        self.assertEqual(num2words(-99, lang="bo"), "མེད་ཆ་ དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(-100, lang="bo"), "མེད་ཆ་ གཅིག་བརྒྱ་")
        self.assertEqual(num2words(-101, lang="bo"), "མེད་ཆ་ གཅིག་བརྒྱ་གཅིག་")
        self.assertEqual(num2words(-200, lang="bo"), "མེད་ཆ་ གཉིས་བརྒྱ་")
        self.assertEqual(num2words(-999, lang="bo"), "མེད་ཆ་ དགུ་བརྒྱ་དགུ་བཅུ་དགུ་")
        self.assertEqual(num2words(-1000, lang="bo"), "མེད་ཆ་ གཅིག་སྟོང་")
        self.assertEqual(num2words(-1001, lang="bo"), "མེད་ཆ་ གཅིག་སྟོང་གཅིག་")
        self.assertEqual(num2words(-10000, lang="bo"), "མེད་ཆ་ གཅིག་ཁྲི་")
        self.assertEqual(num2words(-100000, lang="bo"), "མེད་ཆ་ གཅིག་འབུམ་")
        self.assertEqual(num2words(-1000000, lang="bo"), "མེད་ཆ་ གཅིག་ས་ཡ་")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="bo"), "ཀླད་ཀོར་ ཚེག་ གཅིག་")
        self.assertEqual(num2words(0.5, lang="bo"), "ཀླད་ཀོར་ ཚེག་ ལྔ་")
        self.assertEqual(num2words(0.9, lang="bo"), "ཀླད་ཀོར་ ཚེག་ དགུ་")
        self.assertEqual(num2words(1.1, lang="bo"), "གཅིག་ ཚེག་ གཅིག་")
        self.assertEqual(num2words(1.5, lang="bo"), "གཅིག་ ཚེག་ ལྔ་")
        self.assertEqual(num2words(2.5, lang="bo"), "གཉིས་ ཚེག་ ལྔ་")
        self.assertEqual(num2words(3.14, lang="bo"), "གསུམ་ ཚེག་ གཅིག་ བཞི་")
        self.assertEqual(num2words(10.5, lang="bo"), "བཅུ་ ཚེག་ ལྔ་")
        self.assertEqual(num2words(11.11, lang="bo"), "བཅུ་གཅིག་ ཚེག་ གཅིག་ གཅིག་")
        self.assertEqual(num2words(20.2, lang="bo"), "ཉི་ཤུ་ ཚེག་ གཉིས་")
        self.assertEqual(num2words(99.99, lang="bo"), "དགུ་བཅུ་དགུ་ ཚེག་ དགུ་ དགུ་")
        self.assertEqual(num2words(100.01, lang="bo"), "གཅིག་བརྒྱ་ ཚེག་ ཀླད་ཀོར་ གཅིག་")
        self.assertEqual(num2words(100.5, lang="bo"), "གཅིག་བརྒྱ་ ཚེག་ ལྔ་")
        self.assertEqual(
            num2words(123.45, lang="bo"), "གཅིག་བརྒྱ་ཉི་ཤུ་གསུམ་ ཚེག་ བཞི་ ལྔ་"
        )
        self.assertEqual(num2words(1000.5, lang="bo"), "གཅིག་སྟོང་ ཚེག་ ལྔ་")
        self.assertEqual(
            num2words(1234.56, lang="bo"),
            "གཅིག་སྟོང་གཉིས་བརྒྱ་སུམ་ཅུ་བཞི་ ཚེག་ ལྔ་ དྲུག་",
        )
        self.assertEqual(
            num2words(10000.01, lang="bo"), "གཅིག་ཁྲི་ ཚེག་ ཀླད་ཀོར་ གཅིག་"
        )
        self.assertEqual(num2words(-0.5, lang="bo"), "མེད་ཆ་ ཀླད་ཀོར་ ཚེག་ ལྔ་")
        self.assertEqual(num2words(-1.5, lang="bo"), "མེད་ཆ་ གཅིག་ ཚེག་ ལྔ་")
        self.assertEqual(num2words(-10.5, lang="bo"), "མེད་ཆ་ བཅུ་ ཚེག་ ལྔ་")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="bo", ordinal=True), "གཅིག་པ་")
        self.assertEqual(num2words(2, lang="bo", ordinal=True), "གཉིས་པ་")
        self.assertEqual(num2words(3, lang="bo", ordinal=True), "གསུམ་པ་")
        self.assertEqual(num2words(4, lang="bo", ordinal=True), "བཞི་པ་")
        self.assertEqual(num2words(5, lang="bo", ordinal=True), "ལྔ་པ་")
        self.assertEqual(num2words(6, lang="bo", ordinal=True), "དྲུག་པ་")
        self.assertEqual(num2words(7, lang="bo", ordinal=True), "བདུན་པ་")
        self.assertEqual(num2words(8, lang="bo", ordinal=True), "བརྒྱད་པ་")
        self.assertEqual(num2words(9, lang="bo", ordinal=True), "དགུ་པ་")
        self.assertEqual(num2words(10, lang="bo", ordinal=True), "བཅུ་པ་")
        self.assertEqual(num2words(11, lang="bo", ordinal=True), "བཅུ་གཅིག་པ་")
        self.assertEqual(num2words(12, lang="bo", ordinal=True), "བཅུ་གཉིས་པ་")
        self.assertEqual(num2words(13, lang="bo", ordinal=True), "བཅུ་གསུམ་པ་")
        self.assertEqual(num2words(14, lang="bo", ordinal=True), "བཅུ་བཞི་པ་")
        self.assertEqual(num2words(15, lang="bo", ordinal=True), "བཅུ་ལྔ་པ་")
        self.assertEqual(num2words(16, lang="bo", ordinal=True), "བཅུ་དྲུག་པ་")
        self.assertEqual(num2words(17, lang="bo", ordinal=True), "བཅུ་བདུན་པ་")
        self.assertEqual(num2words(18, lang="bo", ordinal=True), "བཅུ་བརྒྱད་པ་")
        self.assertEqual(num2words(19, lang="bo", ordinal=True), "བཅུ་དགུ་པ་")
        self.assertEqual(num2words(20, lang="bo", ordinal=True), "ཉི་ཤུ་པ་")
        self.assertEqual(num2words(21, lang="bo", ordinal=True), "ཉི་ཤུ་གཅིག་པ་")
        self.assertEqual(num2words(22, lang="bo", ordinal=True), "ཉི་ཤུ་གཉིས་པ་")
        self.assertEqual(num2words(25, lang="bo", ordinal=True), "ཉི་ཤུ་ལྔ་པ་")
        self.assertEqual(num2words(30, lang="bo", ordinal=True), "སུམ་ཅུ་པ་")
        self.assertEqual(num2words(40, lang="bo", ordinal=True), "བཞི་བཅུ་པ་")
        self.assertEqual(num2words(50, lang="bo", ordinal=True), "ལྔ་བཅུ་པ་")
        self.assertEqual(num2words(60, lang="bo", ordinal=True), "དྲུག་ཅུ་པ་")
        self.assertEqual(num2words(70, lang="bo", ordinal=True), "བདུན་ཅུ་པ་")
        self.assertEqual(num2words(80, lang="bo", ordinal=True), "བརྒྱད་ཅུ་པ་")
        self.assertEqual(num2words(90, lang="bo", ordinal=True), "དགུ་བཅུ་པ་")
        self.assertEqual(num2words(100, lang="bo", ordinal=True), "གཅིག་བརྒྱ་པ་")
        self.assertEqual(num2words(101, lang="bo", ordinal=True), "གཅིག་བརྒྱ་གཅིག་པ་")
        self.assertEqual(num2words(200, lang="bo", ordinal=True), "གཉིས་བརྒྱ་པ་")
        self.assertEqual(num2words(500, lang="bo", ordinal=True), "ལྔ་བརྒྱ་པ་")
        self.assertEqual(num2words(1000, lang="bo", ordinal=True), "གཅིག་སྟོང་པ་")
        self.assertEqual(num2words(1001, lang="bo", ordinal=True), "གཅིག་སྟོང་གཅིག་པ་")
        self.assertEqual(num2words(10000, lang="bo", ordinal=True), "གཅིག་ཁྲི་པ་")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="bo", to="currency", currency="CNY"), "ཀླད་ཀོར་ སྒོར་"
        )
        self.assertEqual(
            num2words(0.01, lang="bo", to="currency", currency="CNY"),
            "ཀླད་ཀོར་ སྒོར་ དང་ གཅིག་ ཕན་",
        )
        self.assertEqual(
            num2words(0.5, lang="bo", to="currency", currency="CNY"),
            "ཀླད་ཀོར་ སྒོར་ དང་ ལྔ་བཅུ་ ཕན་",
        )
        self.assertEqual(
            num2words(1, lang="bo", to="currency", currency="CNY"), "གཅིག་ སྒོར་"
        )
        self.assertEqual(
            num2words(1.5, lang="bo", to="currency", currency="CNY"),
            "གཅིག་ སྒོར་ དང་ ལྔ་བཅུ་ ཕན་",
        )
        self.assertEqual(
            num2words(0, lang="bo", to="currency", currency="USD"), "ཀླད་ཀོར་ ཌོ་ལར་"
        )
        self.assertEqual(
            num2words(0.01, lang="bo", to="currency", currency="USD"),
            "ཀླད་ཀོར་ ཌོ་ལར་ དང་ གཅིག་ སེན་",
        )
        self.assertEqual(
            num2words(0.5, lang="bo", to="currency", currency="USD"),
            "ཀླད་ཀོར་ ཌོ་ལར་ དང་ ལྔ་བཅུ་ སེན་",
        )
        self.assertEqual(
            num2words(1, lang="bo", to="currency", currency="USD"), "གཅིག་ ཌོ་ལར་"
        )
        self.assertEqual(
            num2words(1.5, lang="bo", to="currency", currency="USD"),
            "གཅིག་ ཌོ་ལར་ དང་ ལྔ་བཅུ་ སེན་",
        )
        self.assertEqual(
            num2words(0, lang="bo", to="currency", currency="EUR"), "ཀླད་ཀོར་ ཡུ་རོ་"
        )
        self.assertEqual(
            num2words(0.01, lang="bo", to="currency", currency="EUR"),
            "ཀླད་ཀོར་ ཡུ་རོ་ དང་ གཅིག་ སེན་",
        )
        self.assertEqual(
            num2words(0.5, lang="bo", to="currency", currency="EUR"),
            "ཀླད་ཀོར་ ཡུ་རོ་ དང་ ལྔ་བཅུ་ སེན་",
        )
        self.assertEqual(
            num2words(1, lang="bo", to="currency", currency="EUR"), "གཅིག་ ཡུ་རོ་"
        )
        self.assertEqual(
            num2words(1.5, lang="bo", to="currency", currency="EUR"),
            "གཅིག་ ཡུ་རོ་ དང་ ལྔ་བཅུ་ སེན་",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="bo", to="year"), "སྤྱི་ལོ་གཅིག་སྟོང་")
        self.assertEqual(
            num2words(1066, lang="bo", to="year"), "སྤྱི་ལོ་གཅིག་སྟོང་དྲུག་ཅུ་དྲུག་"
        )
        self.assertEqual(
            num2words(1492, lang="bo", to="year"),
            "སྤྱི་ལོ་གཅིག་སྟོང་བཞི་བརྒྱ་དགུ་བཅུ་གཉིས་",
        )
        self.assertEqual(
            num2words(1776, lang="bo", to="year"),
            "སྤྱི་ལོ་གཅིག་སྟོང་བདུན་བརྒྱ་བདུན་ཅུ་དྲུག་",
        )
        self.assertEqual(
            num2words(1800, lang="bo", to="year"), "སྤྱི་ལོ་གཅིག་སྟོང་བརྒྱད་བརྒྱ་"
        )
        self.assertEqual(
            num2words(1900, lang="bo", to="year"), "སྤྱི་ལོ་གཅིག་སྟོང་དགུ་བརྒྱ་"
        )
        self.assertEqual(
            num2words(1984, lang="bo", to="year"),
            "སྤྱི་ལོ་གཅིག་སྟོང་དགུ་བརྒྱ་བརྒྱད་ཅུ་བཞི་",
        )
        self.assertEqual(
            num2words(1999, lang="bo", to="year"),
            "སྤྱི་ལོ་གཅིག་སྟོང་དགུ་བརྒྱ་དགུ་བཅུ་དགུ་",
        )
        self.assertEqual(num2words(2000, lang="bo", to="year"), "སྤྱི་ལོ་གཉིས་སྟོང་")
        self.assertEqual(
            num2words(2001, lang="bo", to="year"), "སྤྱི་ལོ་གཉིས་སྟོང་གཅིག་"
        )
        self.assertEqual(
            num2words(2010, lang="bo", to="year"), "སྤྱི་ལོ་གཉིས་སྟོང་བཅུ་"
        )
        self.assertEqual(
            num2words(2020, lang="bo", to="year"), "སྤྱི་ལོ་གཉིས་སྟོང་ཉི་ཤུ་"
        )
        self.assertEqual(
            num2words(2024, lang="bo", to="year"), "སྤྱི་ལོ་གཉིས་སྟོང་ཉི་ཤུ་བཞི་"
        )
        self.assertEqual(
            num2words(2100, lang="bo", to="year"), "སྤྱི་ལོ་གཉིས་སྟོང་གཅིག་བརྒྱ་"
        )

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="bo"), "ཀླད་ཀོར་")
        self.assertEqual(num2words("1", lang="bo"), "གཅིག་")
        self.assertEqual(num2words("10", lang="bo"), "བཅུ་")
        self.assertEqual(num2words("100", lang="bo"), "གཅིག་བརྒྱ་")
        self.assertEqual(num2words("1000", lang="bo"), "གཅིག་སྟོང་")
        self.assertEqual(num2words("10000", lang="bo"), "གཅིག་ཁྲི་")
        self.assertEqual(num2words("100000", lang="bo"), "གཅིག་འབུམ་")
        self.assertEqual(num2words("1000000", lang="bo"), "གཅིག་ས་ཡ་")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="bo"), "ཀླད་ཀོར་")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="bo"), num2words("100", lang="bo"))
        self.assertEqual(num2words(1000, lang="bo"), num2words("1000", lang="bo"))
