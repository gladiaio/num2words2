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


class Num2WordsSKTest(TestCase):
    """Comprehensive test cases for Slovak language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="sk"), "nula")
        self.assertEqual(num2words(1, lang="sk"), "jeden")
        self.assertEqual(num2words(2, lang="sk"), "dva")
        self.assertEqual(num2words(3, lang="sk"), "tri")
        self.assertEqual(num2words(4, lang="sk"), "štyri")
        self.assertEqual(num2words(5, lang="sk"), "päť")
        self.assertEqual(num2words(6, lang="sk"), "šesť")
        self.assertEqual(num2words(7, lang="sk"), "sedem")
        self.assertEqual(num2words(8, lang="sk"), "osem")
        self.assertEqual(num2words(9, lang="sk"), "deväť")
        self.assertEqual(num2words(10, lang="sk"), "desať")
        self.assertEqual(num2words(11, lang="sk"), "jedenásť")
        self.assertEqual(num2words(12, lang="sk"), "dvanásť")
        self.assertEqual(num2words(13, lang="sk"), "trinásť")
        self.assertEqual(num2words(14, lang="sk"), "štrnásť")
        self.assertEqual(num2words(15, lang="sk"), "pätnásť")
        self.assertEqual(num2words(16, lang="sk"), "šestnásť")
        self.assertEqual(num2words(17, lang="sk"), "sedemnásť")
        self.assertEqual(num2words(18, lang="sk"), "osemnásť")
        self.assertEqual(num2words(19, lang="sk"), "devätnásť")
        self.assertEqual(num2words(20, lang="sk"), "dvadsať")
        self.assertEqual(num2words(21, lang="sk"), "dvadsať jeden")
        self.assertEqual(num2words(22, lang="sk"), "dvadsať dva")
        self.assertEqual(num2words(23, lang="sk"), "dvadsať tri")
        self.assertEqual(num2words(24, lang="sk"), "dvadsať štyri")
        self.assertEqual(num2words(25, lang="sk"), "dvadsať päť")
        self.assertEqual(num2words(26, lang="sk"), "dvadsať šesť")
        self.assertEqual(num2words(27, lang="sk"), "dvadsať sedem")
        self.assertEqual(num2words(28, lang="sk"), "dvadsať osem")
        self.assertEqual(num2words(29, lang="sk"), "dvadsať deväť")
        self.assertEqual(num2words(30, lang="sk"), "tridsať")
        self.assertEqual(num2words(31, lang="sk"), "tridsať jeden")
        self.assertEqual(num2words(35, lang="sk"), "tridsať päť")
        self.assertEqual(num2words(40, lang="sk"), "štyridsať")
        self.assertEqual(num2words(45, lang="sk"), "štyridsať päť")
        self.assertEqual(num2words(50, lang="sk"), "päťdesiat")
        self.assertEqual(num2words(55, lang="sk"), "päťdesiat päť")
        self.assertEqual(num2words(60, lang="sk"), "šesťdesiat")
        self.assertEqual(num2words(65, lang="sk"), "šesťdesiat päť")
        self.assertEqual(num2words(70, lang="sk"), "sedemdesiat")
        self.assertEqual(num2words(75, lang="sk"), "sedemdesiat päť")
        self.assertEqual(num2words(80, lang="sk"), "osemdesiat")
        self.assertEqual(num2words(85, lang="sk"), "osemdesiat päť")
        self.assertEqual(num2words(90, lang="sk"), "deväťdesiat")
        self.assertEqual(num2words(95, lang="sk"), "deväťdesiat päť")
        self.assertEqual(num2words(99, lang="sk"), "deväťdesiat deväť")
        self.assertEqual(num2words(100, lang="sk"), "sto")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="sk"), "sto jeden")
        self.assertEqual(num2words(110, lang="sk"), "sto desať")
        self.assertEqual(num2words(111, lang="sk"), "sto jedenásť")
        self.assertEqual(num2words(120, lang="sk"), "sto dvadsať")
        self.assertEqual(num2words(125, lang="sk"), "sto dvadsať päť")
        self.assertEqual(num2words(150, lang="sk"), "sto päťdesiat")
        self.assertEqual(num2words(175, lang="sk"), "sto sedemdesiat päť")
        self.assertEqual(num2words(199, lang="sk"), "sto deväťdesiat deväť")
        self.assertEqual(num2words(200, lang="sk"), "dvesto")
        self.assertEqual(num2words(201, lang="sk"), "dvesto jeden")
        self.assertEqual(num2words(210, lang="sk"), "dvesto desať")
        self.assertEqual(num2words(220, lang="sk"), "dvesto dvadsať")
        self.assertEqual(num2words(250, lang="sk"), "dvesto päťdesiat")
        self.assertEqual(num2words(299, lang="sk"), "dvesto deväťdesiat deväť")
        self.assertEqual(num2words(300, lang="sk"), "tristo")
        self.assertEqual(num2words(333, lang="sk"), "tristo tridsať tri")
        self.assertEqual(num2words(400, lang="sk"), "štyristo")
        self.assertEqual(num2words(444, lang="sk"), "štyristo štyridsať štyri")
        self.assertEqual(num2words(500, lang="sk"), "päťsto")
        self.assertEqual(num2words(555, lang="sk"), "päťsto päťdesiat päť")
        self.assertEqual(num2words(600, lang="sk"), "šesťsto")
        self.assertEqual(num2words(666, lang="sk"), "šesťsto šesťdesiat šesť")
        self.assertEqual(num2words(700, lang="sk"), "sedemsto")
        self.assertEqual(num2words(777, lang="sk"), "sedemsto sedemdesiat sedem")
        self.assertEqual(num2words(800, lang="sk"), "osemsto")
        self.assertEqual(num2words(888, lang="sk"), "osemsto osemdesiat osem")
        self.assertEqual(num2words(900, lang="sk"), "deväťsto")
        self.assertEqual(num2words(999, lang="sk"), "deväťsto deväťdesiat deväť")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="sk"), "tisíc")
        self.assertEqual(num2words(1001, lang="sk"), "tisíc jeden")
        self.assertEqual(num2words(1010, lang="sk"), "tisíc desať")
        self.assertEqual(num2words(1100, lang="sk"), "tisíc sto")
        self.assertEqual(num2words(1111, lang="sk"), "tisíc sto jedenásť")
        self.assertEqual(num2words(1234, lang="sk"), "tisíc dvesto tridsať štyri")
        self.assertEqual(num2words(1500, lang="sk"), "tisíc päťsto")
        self.assertEqual(num2words(1999, lang="sk"), "tisíc deväťsto deväťdesiat deväť")
        self.assertEqual(num2words(2000, lang="sk"), "dve tisíc")
        self.assertEqual(num2words(2001, lang="sk"), "dve tisíc jeden")
        self.assertEqual(num2words(2020, lang="sk"), "dve tisíc dvadsať")
        self.assertEqual(num2words(2222, lang="sk"), "dve tisíc dvesto dvadsať dva")
        self.assertEqual(num2words(3000, lang="sk"), "tri tisíc")
        self.assertEqual(num2words(3333, lang="sk"), "tri tisíc tristo tridsať tri")
        self.assertEqual(num2words(4000, lang="sk"), "štyri tisíc")
        self.assertEqual(
            num2words(4444, lang="sk"), "štyri tisíc štyristo štyridsať štyri"
        )
        self.assertEqual(num2words(5000, lang="sk"), "päť tisíc")
        self.assertEqual(num2words(5555, lang="sk"), "päť tisíc päťsto päťdesiat päť")
        self.assertEqual(num2words(6000, lang="sk"), "šesť tisíc")
        self.assertEqual(
            num2words(6666, lang="sk"), "šesť tisíc šesťsto šesťdesiat šesť"
        )
        self.assertEqual(num2words(7000, lang="sk"), "sedem tisíc")
        self.assertEqual(
            num2words(7777, lang="sk"), "sedem tisíc sedemsto sedemdesiat sedem"
        )
        self.assertEqual(num2words(8000, lang="sk"), "osem tisíc")
        self.assertEqual(
            num2words(8888, lang="sk"), "osem tisíc osemsto osemdesiat osem"
        )
        self.assertEqual(num2words(9000, lang="sk"), "deväť tisíc")
        self.assertEqual(
            num2words(9999, lang="sk"), "deväť tisíc deväťsto deväťdesiat deväť"
        )
        self.assertEqual(num2words(10000, lang="sk"), "desať tisíc")
        self.assertEqual(num2words(10001, lang="sk"), "desať tisíc jeden")
        self.assertEqual(num2words(11111, lang="sk"), "jedenásť tisíc sto jedenásť")
        self.assertEqual(
            num2words(12345, lang="sk"), "dvanásť tisíc tristo štyridsať päť"
        )
        self.assertEqual(num2words(20000, lang="sk"), "dvadsať tisíc")
        self.assertEqual(num2words(50000, lang="sk"), "päťdesiat tisíc")
        self.assertEqual(
            num2words(99999, lang="sk"),
            "deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )
        self.assertEqual(num2words(100000, lang="sk"), "sto tisíc")
        self.assertEqual(
            num2words(123456, lang="sk"),
            "sto dvadsať tri tisíc štyristo päťdesiat šesť",
        )
        self.assertEqual(num2words(200000, lang="sk"), "dvesto tisíc")
        self.assertEqual(num2words(500000, lang="sk"), "päťsto tisíc")
        self.assertEqual(
            num2words(654321, lang="sk"),
            "šesťsto päťdesiat štyri tisíc tristo dvadsať jeden",
        )
        self.assertEqual(
            num2words(999999, lang="sk"),
            "deväťsto deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="sk"), "milión")
        self.assertEqual(num2words(1000001, lang="sk"), "milión jeden")
        self.assertEqual(
            num2words(1111111, lang="sk"), "milión sto jedenásť tisíc sto jedenásť"
        )
        self.assertEqual(
            num2words(1234567, lang="sk"),
            "milión dvesto tridsať štyri tisíc päťsto šesťdesiat sedem",
        )
        self.assertEqual(num2words(2000000, lang="sk"), "dva milióny")
        self.assertEqual(num2words(5000000, lang="sk"), "päť miliónov")
        self.assertEqual(
            num2words(9999999, lang="sk"),
            "deväť miliónov deväťsto deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )
        self.assertEqual(num2words(10000000, lang="sk"), "desať miliónov")
        self.assertEqual(
            num2words(12345678, lang="sk"),
            "dvanásť miliónov tristo štyridsať päť tisíc šesťsto sedemdesiat osem",
        )
        self.assertEqual(
            num2words(99999999, lang="sk"),
            "deväťdesiat deväť miliónov deväťsto deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )
        self.assertEqual(num2words(100000000, lang="sk"), "sto miliónov")
        self.assertEqual(
            num2words(123456789, lang="sk"),
            "sto dvadsať tri miliónov štyristo päťdesiat šesť tisíc sedemsto osemdesiat deväť",
        )
        self.assertEqual(
            num2words(999999999, lang="sk"),
            "deväťsto deväťdesiat deväť miliónov deväťsto deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )
        self.assertEqual(num2words(1000000000, lang="sk"), "miliarda")
        self.assertEqual(
            num2words(1234567890, lang="sk"),
            "miliarda dvesto tridsať štyri miliónov päťsto šesťdesiat sedem tisíc osemsto deväťdesiat",
        )
        self.assertEqual(
            num2words(9999999999, lang="sk"),
            "deväť miliárd deväťsto deväťdesiat deväť miliónov deväťsto deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )
        self.assertEqual(num2words(10000000000, lang="sk"), "desať miliárd")
        self.assertEqual(
            num2words(99999999999, lang="sk"),
            "deväťdesiat deväť miliárd deväťsto deväťdesiat deväť miliónov deväťsto deväťdesiat deväť tisíc deväťsto deväťdesiat deväť",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="sk"), "mínus jeden")
        self.assertEqual(num2words(-2, lang="sk"), "mínus dva")
        self.assertEqual(num2words(-5, lang="sk"), "mínus päť")
        self.assertEqual(num2words(-10, lang="sk"), "mínus desať")
        self.assertEqual(num2words(-11, lang="sk"), "mínus jedenásť")
        self.assertEqual(num2words(-20, lang="sk"), "mínus dvadsať")
        self.assertEqual(num2words(-50, lang="sk"), "mínus päťdesiat")
        self.assertEqual(num2words(-99, lang="sk"), "mínus deväťdesiat deväť")
        self.assertEqual(num2words(-100, lang="sk"), "mínus sto")
        self.assertEqual(num2words(-101, lang="sk"), "mínus sto jeden")
        self.assertEqual(num2words(-200, lang="sk"), "mínus dvesto")
        self.assertEqual(num2words(-999, lang="sk"), "mínus deväťsto deväťdesiat deväť")
        self.assertEqual(num2words(-1000, lang="sk"), "mínus tisíc")
        self.assertEqual(num2words(-1001, lang="sk"), "mínus tisíc jeden")
        self.assertEqual(num2words(-10000, lang="sk"), "mínus desať tisíc")
        self.assertEqual(num2words(-100000, lang="sk"), "mínus sto tisíc")
        self.assertEqual(num2words(-1000000, lang="sk"), "mínus milión")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="sk"), "nula celých jeden")
        self.assertEqual(num2words(0.5, lang="sk"), "nula celých päť")
        self.assertEqual(num2words(0.9, lang="sk"), "nula celých deväť")
        self.assertEqual(num2words(1.1, lang="sk"), "jeden celých jeden")
        self.assertEqual(num2words(1.5, lang="sk"), "jeden celých päť")
        self.assertEqual(num2words(2.5, lang="sk"), "dva celých päť")
        self.assertEqual(num2words(3.14, lang="sk"), "tri celých štrnásť")
        self.assertEqual(num2words(10.5, lang="sk"), "desať celých päť")
        self.assertEqual(num2words(11.11, lang="sk"), "jedenásť celých jedenásť")
        self.assertEqual(num2words(20.2, lang="sk"), "dvadsať celých dva")
        self.assertEqual(
            num2words(99.99, lang="sk"), "deväťdesiat deväť celých deväťdesiat deväť"
        )
        self.assertEqual(num2words(100.01, lang="sk"), "sto celých nula jeden")
        self.assertEqual(num2words(100.5, lang="sk"), "sto celých päť")
        self.assertEqual(
            num2words(123.45, lang="sk"), "sto dvadsať tri celých štyridsať päť"
        )
        self.assertEqual(num2words(1000.5, lang="sk"), "tisíc celých päť")
        self.assertEqual(
            num2words(1234.56, lang="sk"),
            "tisíc dvesto tridsať štyri celých päťdesiat šesť",
        )
        self.assertEqual(
            num2words(10000.01, lang="sk"), "desať tisíc celých nula jeden"
        )
        self.assertEqual(num2words(-0.5, lang="sk"), "mínus nula celých päť")
        self.assertEqual(num2words(-1.5, lang="sk"), "mínus jeden celých päť")
        self.assertEqual(num2words(-10.5, lang="sk"), "mínus desať celých päť")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="sk", ordinal=True), "prvý")
        self.assertEqual(num2words(2, lang="sk", ordinal=True), "druhý")
        self.assertEqual(num2words(3, lang="sk", ordinal=True), "tretí")
        self.assertEqual(num2words(4, lang="sk", ordinal=True), "štvrtý")
        self.assertEqual(num2words(5, lang="sk", ordinal=True), "piaty")
        self.assertEqual(num2words(6, lang="sk", ordinal=True), "šiesty")
        self.assertEqual(num2words(7, lang="sk", ordinal=True), "siedmy")
        self.assertEqual(num2words(8, lang="sk", ordinal=True), "ôsmy")
        self.assertEqual(num2words(9, lang="sk", ordinal=True), "deviaty")
        self.assertEqual(num2words(10, lang="sk", ordinal=True), "desiaty")
        self.assertEqual(num2words(11, lang="sk", ordinal=True), "jedenásty")
        self.assertEqual(num2words(12, lang="sk", ordinal=True), "dvanásty")
        self.assertEqual(num2words(13, lang="sk", ordinal=True), "trinásty")
        self.assertEqual(num2words(14, lang="sk", ordinal=True), "štrnásty")
        self.assertEqual(num2words(15, lang="sk", ordinal=True), "pätnásty")
        self.assertEqual(num2words(16, lang="sk", ordinal=True), "šestnásty")
        self.assertEqual(num2words(17, lang="sk", ordinal=True), "sedemnásty")
        self.assertEqual(num2words(18, lang="sk", ordinal=True), "osemnásty")
        self.assertEqual(num2words(19, lang="sk", ordinal=True), "devätnásty")
        self.assertEqual(num2words(20, lang="sk", ordinal=True), "dvadsiaty")
        self.assertEqual(num2words(21, lang="sk", ordinal=True), "dvadsať jedený")
        self.assertEqual(num2words(22, lang="sk", ordinal=True), "dvadsať dvaý")
        self.assertEqual(num2words(25, lang="sk", ordinal=True), "dvadsať päťý")
        self.assertEqual(num2words(30, lang="sk", ordinal=True), "tridsiaty")
        self.assertEqual(num2words(40, lang="sk", ordinal=True), "štyridsiaty")
        self.assertEqual(num2words(50, lang="sk", ordinal=True), "päťdesiaty")
        self.assertEqual(num2words(60, lang="sk", ordinal=True), "šesťdesiaty")
        self.assertEqual(num2words(70, lang="sk", ordinal=True), "sedemdesiaty")
        self.assertEqual(num2words(80, lang="sk", ordinal=True), "osemdesiaty")
        self.assertEqual(num2words(90, lang="sk", ordinal=True), "deväťdesiaty")
        self.assertEqual(num2words(100, lang="sk", ordinal=True), "stý")
        self.assertEqual(num2words(101, lang="sk", ordinal=True), "sto jedený")
        self.assertEqual(num2words(200, lang="sk", ordinal=True), "dvestoý")
        self.assertEqual(num2words(500, lang="sk", ordinal=True), "päťstoý")
        self.assertEqual(num2words(1000, lang="sk", ordinal=True), "tisíci")
        self.assertEqual(num2words(1001, lang="sk", ordinal=True), "tisíc jedený")
        self.assertEqual(num2words(10000, lang="sk", ordinal=True), "desať tisícý")

    def test_currency(self):
        """Test currency conversion."""
        # Int zero takes the genitive plural like the float path (#187).
        self.assertEqual(
            num2words(0, lang="sk", to="currency", currency="EUR"), "nula eur"
        )
        self.assertEqual(
            num2words(0.01, lang="sk", to="currency", currency="EUR"),
            "nula eur, jeden cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sk", to="currency", currency="EUR"),
            "nula eur, päťdesiat centov",
        )
        self.assertEqual(
            num2words(1, lang="sk", to="currency", currency="EUR"), "jeden euro"
        )
        self.assertEqual(
            num2words(1.5, lang="sk", to="currency", currency="EUR"),
            "jeden euro, päťdesiat centov",
        )
        self.assertEqual(
            num2words(0, lang="sk", to="currency", currency="CZK"), "nula korún"
        )
        self.assertEqual(
            num2words(0.01, lang="sk", to="currency", currency="CZK"),
            "nula korún, jeden halier",
        )
        self.assertEqual(
            num2words(0.5, lang="sk", to="currency", currency="CZK"),
            "nula korún, päťdesiat halierov",
        )
        self.assertEqual(
            num2words(1, lang="sk", to="currency", currency="CZK"), "jeden koruna"
        )
        self.assertEqual(
            num2words(1.5, lang="sk", to="currency", currency="CZK"),
            "jeden koruna, päťdesiat halierov",
        )
        self.assertEqual(
            num2words(0, lang="sk", to="currency", currency="USD"), "nula dolárov"
        )
        self.assertEqual(
            num2words(0.01, lang="sk", to="currency", currency="USD"),
            "nula dolárov, jeden cent",
        )
        self.assertEqual(
            num2words(0.5, lang="sk", to="currency", currency="USD"),
            "nula dolárov, päťdesiat centov",
        )
        self.assertEqual(
            num2words(1, lang="sk", to="currency", currency="USD"), "jeden dolár"
        )
        self.assertEqual(
            num2words(1.5, lang="sk", to="currency", currency="USD"),
            "jeden dolár, päťdesiat centov",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="sk", to="year"), "tisíc")
        self.assertEqual(num2words(1066, lang="sk", to="year"), "tisíc šesťdesiat šesť")
        self.assertEqual(
            num2words(1492, lang="sk", to="year"), "tisíc štyristo deväťdesiat dva"
        )
        self.assertEqual(
            num2words(1776, lang="sk", to="year"), "tisíc sedemsto sedemdesiat šesť"
        )
        self.assertEqual(num2words(1800, lang="sk", to="year"), "tisíc osemsto")
        self.assertEqual(num2words(1900, lang="sk", to="year"), "tisíc deväťsto")
        self.assertEqual(
            num2words(1984, lang="sk", to="year"), "tisíc deväťsto osemdesiat štyri"
        )
        self.assertEqual(
            num2words(1999, lang="sk", to="year"), "tisíc deväťsto deväťdesiat deväť"
        )
        self.assertEqual(num2words(2000, lang="sk", to="year"), "dve tisíc")
        self.assertEqual(num2words(2001, lang="sk", to="year"), "dve tisíc jeden")
        self.assertEqual(num2words(2010, lang="sk", to="year"), "dve tisíc desať")
        self.assertEqual(num2words(2020, lang="sk", to="year"), "dve tisíc dvadsať")
        self.assertEqual(
            num2words(2024, lang="sk", to="year"), "dve tisíc dvadsať štyri"
        )
        self.assertEqual(num2words(2100, lang="sk", to="year"), "dve tisíc sto")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="sk"), "nula")
        self.assertEqual(num2words("1", lang="sk"), "jeden")
        self.assertEqual(num2words("10", lang="sk"), "desať")
        self.assertEqual(num2words("100", lang="sk"), "sto")
        self.assertEqual(num2words("1000", lang="sk"), "tisíc")
        self.assertEqual(num2words("10000", lang="sk"), "desať tisíc")
        self.assertEqual(num2words("100000", lang="sk"), "sto tisíc")
        self.assertEqual(num2words("1000000", lang="sk"), "milión")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="sk"), "nula")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="sk"), num2words("100", lang="sk"))
        self.assertEqual(num2words(1000, lang="sk"), num2words("1000", lang="sk"))

    def test_currency_int_plural_matches_float(self):
        # gladiaio/num2words2#187: the int path used the 2-4 form for 5+.
        self.assertEqual(num2words(5, lang="sk", to="currency"), "päť eur")
        self.assertEqual(
            num2words(5.0, lang="sk", to="currency"), "päť eur, nula centov"
        )
        self.assertEqual(num2words(2, lang="sk", to="currency"), "dva eurá")
        self.assertEqual(num2words(1, lang="sk", to="currency"), "jeden euro")
        self.assertEqual(num2words(-1, lang="sk", to="currency"), "mínus jeden euro")
        self.assertEqual(
            num2words(100, lang="sk", to="currency", currency="USD"),
            "sto dolárov",
        )
        self.assertEqual(
            num2words(3, lang="sk", to="currency", currency="CZK"), "tri koruny"
        )
        for n in (0, 1, 2, 4, 5, 11, 21, 100):
            self.assertEqual(
                num2words(n, lang="sk", to="currency"),
                num2words(float(n), lang="sk", to="currency").split(",")[0],
            )
