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


class Num2WordsGUTest(TestCase):
    """Comprehensive test cases for Gujarati language."""

    def test_cardinal_basic(self):
        """Test cardinal numbers from 0 to 100."""
        self.assertEqual(num2words(0, lang="gu"), "શૂન્ય")
        self.assertEqual(num2words(1, lang="gu"), "એક")
        self.assertEqual(num2words(2, lang="gu"), "બે")
        self.assertEqual(num2words(3, lang="gu"), "ત્રણ")
        self.assertEqual(num2words(4, lang="gu"), "ચાર")
        self.assertEqual(num2words(5, lang="gu"), "પાંચ")
        self.assertEqual(num2words(6, lang="gu"), "છ")
        self.assertEqual(num2words(7, lang="gu"), "સાત")
        self.assertEqual(num2words(8, lang="gu"), "આઠ")
        self.assertEqual(num2words(9, lang="gu"), "નવ")
        self.assertEqual(num2words(10, lang="gu"), "દસ")
        self.assertEqual(num2words(11, lang="gu"), "અગિયાર")
        self.assertEqual(num2words(12, lang="gu"), "બાર")
        self.assertEqual(num2words(13, lang="gu"), "તેર")
        self.assertEqual(num2words(14, lang="gu"), "ચૌદ")
        self.assertEqual(num2words(15, lang="gu"), "પંદર")
        self.assertEqual(num2words(16, lang="gu"), "સોળ")
        self.assertEqual(num2words(17, lang="gu"), "સત્તર")
        self.assertEqual(num2words(18, lang="gu"), "અઢાર")
        self.assertEqual(num2words(19, lang="gu"), "ઓગણીસ")
        self.assertEqual(num2words(20, lang="gu"), "વીસ")
        self.assertEqual(num2words(21, lang="gu"), "એકવીસ")
        self.assertEqual(num2words(22, lang="gu"), "બાવીસ")
        self.assertEqual(num2words(23, lang="gu"), "તેવીસ")
        self.assertEqual(num2words(24, lang="gu"), "ચોવીસ")
        self.assertEqual(num2words(25, lang="gu"), "પચ્ચીસ")
        self.assertEqual(num2words(26, lang="gu"), "છવીસ")
        self.assertEqual(num2words(27, lang="gu"), "સત્તાવીસ")
        self.assertEqual(num2words(28, lang="gu"), "અઠ્ઠાવીસ")
        self.assertEqual(num2words(29, lang="gu"), "ઓગણત્રીસ")
        self.assertEqual(num2words(30, lang="gu"), "ત્રીસ")
        self.assertEqual(num2words(31, lang="gu"), "એકત્રીસ")
        self.assertEqual(num2words(35, lang="gu"), "પાંત્રીસ")
        self.assertEqual(num2words(40, lang="gu"), "ચાલીસ")
        self.assertEqual(num2words(45, lang="gu"), "પિસ્તાલીસ")
        self.assertEqual(num2words(50, lang="gu"), "પચાસ")
        self.assertEqual(num2words(55, lang="gu"), "પંચાવન")
        self.assertEqual(num2words(60, lang="gu"), "સાઠ")
        self.assertEqual(num2words(65, lang="gu"), "પાંસઠ")
        self.assertEqual(num2words(70, lang="gu"), "સિત્તેર")
        self.assertEqual(num2words(75, lang="gu"), "પંચોતેર")
        self.assertEqual(num2words(80, lang="gu"), "એંસી")
        self.assertEqual(num2words(85, lang="gu"), "પંચાસી")
        self.assertEqual(num2words(90, lang="gu"), "નેવું")
        self.assertEqual(num2words(95, lang="gu"), "પંચાણું")
        self.assertEqual(num2words(99, lang="gu"), "નવ્વાણું")
        self.assertEqual(num2words(100, lang="gu"), "એક સો")

    def test_cardinal_hundreds(self):
        """Test cardinal numbers from 100 to 999."""
        self.assertEqual(num2words(101, lang="gu"), "એક સો એક")
        self.assertEqual(num2words(110, lang="gu"), "એક સો દસ")
        self.assertEqual(num2words(111, lang="gu"), "એક સો અગિયાર")
        self.assertEqual(num2words(120, lang="gu"), "એક સો વીસ")
        self.assertEqual(num2words(125, lang="gu"), "એક સો પચ્ચીસ")
        self.assertEqual(num2words(150, lang="gu"), "એક સો પચાસ")
        self.assertEqual(num2words(175, lang="gu"), "એક સો પંચોતેર")
        self.assertEqual(num2words(199, lang="gu"), "એક સો નવ્વાણું")
        self.assertEqual(num2words(200, lang="gu"), "બે સો")
        self.assertEqual(num2words(201, lang="gu"), "બે સો એક")
        self.assertEqual(num2words(210, lang="gu"), "બે સો દસ")
        self.assertEqual(num2words(220, lang="gu"), "બે સો વીસ")
        self.assertEqual(num2words(250, lang="gu"), "બે સો પચાસ")
        self.assertEqual(num2words(299, lang="gu"), "બે સો નવ્વાણું")
        self.assertEqual(num2words(300, lang="gu"), "ત્રણ સો")
        self.assertEqual(num2words(333, lang="gu"), "ત્રણ સો તેત્રીસ")
        self.assertEqual(num2words(400, lang="gu"), "ચાર સો")
        self.assertEqual(num2words(444, lang="gu"), "ચાર સો ચુંમાલીસ")
        self.assertEqual(num2words(500, lang="gu"), "પાંચ સો")
        self.assertEqual(num2words(555, lang="gu"), "પાંચ સો પંચાવન")
        self.assertEqual(num2words(600, lang="gu"), "છ સો")
        self.assertEqual(num2words(666, lang="gu"), "છ સો છાસઠ")
        self.assertEqual(num2words(700, lang="gu"), "સાત સો")
        self.assertEqual(num2words(777, lang="gu"), "સાત સો સિત્યોતેર")
        self.assertEqual(num2words(800, lang="gu"), "આઠ સો")
        self.assertEqual(num2words(888, lang="gu"), "આઠ સો ઈઠ્યાસી")
        self.assertEqual(num2words(900, lang="gu"), "નવ સો")
        self.assertEqual(num2words(999, lang="gu"), "નવ સો નવ્વાણું")

    def test_cardinal_thousands(self):
        """Test cardinal numbers from 1000 to 999999."""
        self.assertEqual(num2words(1000, lang="gu"), "એક હજાર")
        self.assertEqual(num2words(1001, lang="gu"), "એક હજાર એક")
        self.assertEqual(num2words(1010, lang="gu"), "એક હજાર દસ")
        self.assertEqual(num2words(1100, lang="gu"), "એક હજાર એક સો")
        self.assertEqual(num2words(1111, lang="gu"), "એક હજાર એક સો અગિયાર")
        self.assertEqual(num2words(1234, lang="gu"), "એક હજાર બે સો ચોત્રીસ")
        self.assertEqual(num2words(1500, lang="gu"), "એક હજાર પાંચ સો")
        self.assertEqual(num2words(1999, lang="gu"), "એક હજાર નવ સો નવ્વાણું")
        self.assertEqual(num2words(2000, lang="gu"), "બે હજાર")
        self.assertEqual(num2words(2001, lang="gu"), "બે હજાર એક")
        self.assertEqual(num2words(2020, lang="gu"), "બે હજાર વીસ")
        self.assertEqual(num2words(2222, lang="gu"), "બે હજાર બે સો બાવીસ")
        self.assertEqual(num2words(3000, lang="gu"), "ત્રણ હજાર")
        self.assertEqual(num2words(3333, lang="gu"), "ત્રણ હજાર ત્રણ સો તેત્રીસ")
        self.assertEqual(num2words(4000, lang="gu"), "ચાર હજાર")
        self.assertEqual(num2words(4444, lang="gu"), "ચાર હજાર ચાર સો ચુંમાલીસ")
        self.assertEqual(num2words(5000, lang="gu"), "પાંચ હજાર")
        self.assertEqual(num2words(5555, lang="gu"), "પાંચ હજાર પાંચ સો પંચાવન")
        self.assertEqual(num2words(6000, lang="gu"), "છ હજાર")
        self.assertEqual(num2words(6666, lang="gu"), "છ હજાર છ સો છાસઠ")
        self.assertEqual(num2words(7000, lang="gu"), "સાત હજાર")
        self.assertEqual(num2words(7777, lang="gu"), "સાત હજાર સાત સો સિત્યોતેર")
        self.assertEqual(num2words(8000, lang="gu"), "આઠ હજાર")
        self.assertEqual(num2words(8888, lang="gu"), "આઠ હજાર આઠ સો ઈઠ્યાસી")
        self.assertEqual(num2words(9000, lang="gu"), "નવ હજાર")
        self.assertEqual(num2words(9999, lang="gu"), "નવ હજાર નવ સો નવ્વાણું")
        self.assertEqual(num2words(10000, lang="gu"), "દસ હજાર")
        self.assertEqual(num2words(10001, lang="gu"), "દસ હજાર એક")
        self.assertEqual(num2words(11111, lang="gu"), "અગિયાર હજાર એક સો અગિયાર")
        self.assertEqual(num2words(12345, lang="gu"), "બાર હજાર ત્રણ સો પિસ્તાલીસ")
        self.assertEqual(num2words(20000, lang="gu"), "વીસ હજાર")
        self.assertEqual(num2words(50000, lang="gu"), "પચાસ હજાર")
        self.assertEqual(num2words(99999, lang="gu"), "નવ્વાણું હજાર નવ સો નવ્વાણું")
        self.assertEqual(num2words(100000, lang="gu"), "એક લાખ")
        self.assertEqual(
            num2words(123456, lang="gu"), "એક લાખ તેવીસ હજાર ચાર સો છપ્પન"
        )
        self.assertEqual(num2words(200000, lang="gu"), "બે લાખ")
        self.assertEqual(num2words(500000, lang="gu"), "પાંચ લાખ")
        self.assertEqual(
            num2words(654321, lang="gu"), "છ લાખ ચોપન હજાર ત્રણ સો એકવીસ"
        )
        self.assertEqual(
            num2words(999999, lang="gu"), "નવ લાખ નવ્વાણું હજાર નવ સો નવ્વાણું"
        )

    def test_cardinal_large(self):
        """Test large cardinal numbers (millions and billions)."""
        self.assertEqual(num2words(1000000, lang="gu"), "દસ લાખ")
        self.assertEqual(num2words(1000001, lang="gu"), "દસ લાખ એક")
        self.assertEqual(
            num2words(1111111, lang="gu"), "અગિયાર લાખ અગિયાર હજાર એક સો અગિયાર"
        )
        self.assertEqual(
            num2words(1234567, lang="gu"), "બાર લાખ ચોત્રીસ હજાર પાંચ સો સડસઠ"
        )
        self.assertEqual(num2words(2000000, lang="gu"), "વીસ લાખ")
        self.assertEqual(num2words(5000000, lang="gu"), "પચાસ લાખ")
        self.assertEqual(
            num2words(9999999, lang="gu"), "નવ્વાણું લાખ નવ્વાણું હજાર નવ સો નવ્વાણું"
        )
        self.assertEqual(num2words(10000000, lang="gu"), "એક કરોડ")
        self.assertEqual(
            num2words(12345678, lang="gu"),
            "એક કરોડ તેવીસ લાખ પિસ્તાલીસ હજાર છ સો ઇઠ્યોતેર",
        )
        self.assertEqual(
            num2words(99999999, lang="gu"),
            "નવ કરોડ નવ્વાણું લાખ નવ્વાણું હજાર નવ સો નવ્વાણું",
        )
        self.assertEqual(num2words(100000000, lang="gu"), "દસ કરોડ")
        self.assertEqual(
            num2words(123456789, lang="gu"),
            "બાર કરોડ ચોત્રીસ લાખ છપ્પન હજાર સાત સો નેવ્યાસી",
        )
        self.assertEqual(
            num2words(999999999, lang="gu"),
            "નવ્વાણું કરોડ નવ્વાણું લાખ નવ્વાણું હજાર નવ સો નવ્વાણું",
        )
        self.assertEqual(num2words(1000000000, lang="gu"), "એક અબજ")
        self.assertEqual(
            num2words(1234567890, lang="gu"),
            "એક અબજ તેવીસ કરોડ પિસ્તાલીસ લાખ સડસઠ હજાર આઠ સો નેવું",
        )
        self.assertEqual(
            num2words(9999999999, lang="gu"),
            "નવ અબજ નવ્વાણું કરોડ નવ્વાણું લાખ નવ્વાણું હજાર નવ સો નવ્વાણું",
        )
        self.assertEqual(num2words(10000000000, lang="gu"), "દસ અબજ")
        self.assertEqual(
            num2words(99999999999, lang="gu"),
            "નવ્વાણું અબજ નવ્વાણું કરોડ નવ્વાણું લાખ નવ્વાણું હજાર નવ સો નવ્વાણું",
        )

    def test_negative_numbers(self):
        """Test negative numbers."""
        self.assertEqual(num2words(-1, lang="gu"), "ઋણ એક")
        self.assertEqual(num2words(-2, lang="gu"), "ઋણ બે")
        self.assertEqual(num2words(-5, lang="gu"), "ઋણ પાંચ")
        self.assertEqual(num2words(-10, lang="gu"), "ઋણ દસ")
        self.assertEqual(num2words(-11, lang="gu"), "ઋણ અગિયાર")
        self.assertEqual(num2words(-20, lang="gu"), "ઋણ વીસ")
        self.assertEqual(num2words(-50, lang="gu"), "ઋણ પચાસ")
        self.assertEqual(num2words(-99, lang="gu"), "ઋણ નવ્વાણું")
        self.assertEqual(num2words(-100, lang="gu"), "ઋણ એક સો")
        self.assertEqual(num2words(-101, lang="gu"), "ઋણ એક સો એક")
        self.assertEqual(num2words(-200, lang="gu"), "ઋણ બે સો")
        self.assertEqual(num2words(-999, lang="gu"), "ઋણ નવ સો નવ્વાણું")
        self.assertEqual(num2words(-1000, lang="gu"), "ઋણ એક હજાર")
        self.assertEqual(num2words(-1001, lang="gu"), "ઋણ એક હજાર એક")
        self.assertEqual(num2words(-10000, lang="gu"), "ઋણ દસ હજાર")
        self.assertEqual(num2words(-100000, lang="gu"), "ઋણ એક લાખ")
        self.assertEqual(num2words(-1000000, lang="gu"), "ઋણ દસ લાખ")

    def test_decimal_numbers(self):
        """Test decimal numbers."""
        self.assertEqual(num2words(0.1, lang="gu"), "શૂન્ય દશાંશ એક")
        self.assertEqual(num2words(0.5, lang="gu"), "શૂન્ય દશાંશ પાંચ")
        self.assertEqual(num2words(0.9, lang="gu"), "શૂન્ય દશાંશ નવ")
        self.assertEqual(num2words(1.1, lang="gu"), "એક દશાંશ એક")
        self.assertEqual(num2words(1.5, lang="gu"), "એક દશાંશ પાંચ")
        self.assertEqual(num2words(2.5, lang="gu"), "બે દશાંશ પાંચ")
        self.assertEqual(num2words(3.14, lang="gu"), "ત્રણ દશાંશ એક ચાર")
        self.assertEqual(num2words(10.5, lang="gu"), "દસ દશાંશ પાંચ")
        self.assertEqual(num2words(11.11, lang="gu"), "અગિયાર દશાંશ એક એક")
        self.assertEqual(num2words(20.2, lang="gu"), "વીસ દશાંશ બે")
        self.assertEqual(num2words(99.99, lang="gu"), "નવ્વાણું દશાંશ નવ નવ")
        self.assertEqual(num2words(100.01, lang="gu"), "એક સો દશાંશ શૂન્ય એક")
        self.assertEqual(num2words(100.5, lang="gu"), "એક સો દશાંશ પાંચ")
        self.assertEqual(num2words(123.45, lang="gu"), "એક સો તેવીસ દશાંશ ચાર પાંચ")
        self.assertEqual(num2words(1000.5, lang="gu"), "એક હજાર દશાંશ પાંચ")
        self.assertEqual(
            num2words(1234.56, lang="gu"), "એક હજાર બે સો ચોત્રીસ દશાંશ પાંચ છ"
        )
        self.assertEqual(num2words(10000.01, lang="gu"), "દસ હજાર દશાંશ શૂન્ય એક")
        self.assertEqual(num2words(-0.5, lang="gu"), "ઋણ શૂન્ય દશાંશ પાંચ")
        self.assertEqual(num2words(-1.5, lang="gu"), "ઋણ એક દશાંશ પાંચ")
        self.assertEqual(num2words(-10.5, lang="gu"), "ઋણ દસ દશાંશ પાંચ")

    def test_ordinal(self):
        """Test ordinal numbers."""
        self.assertEqual(num2words(1, lang="gu", ordinal=True), "પહેલો")
        self.assertEqual(num2words(2, lang="gu", ordinal=True), "બીજો")
        self.assertEqual(num2words(3, lang="gu", ordinal=True), "ત્રીજો")
        self.assertEqual(num2words(4, lang="gu", ordinal=True), "ચોથો")
        self.assertEqual(num2words(5, lang="gu", ordinal=True), "પાંચમો")
        self.assertEqual(num2words(6, lang="gu", ordinal=True), "છઠ્ઠો")
        self.assertEqual(num2words(7, lang="gu", ordinal=True), "સાતમો")
        self.assertEqual(num2words(8, lang="gu", ordinal=True), "આઠમો")
        self.assertEqual(num2words(9, lang="gu", ordinal=True), "નવમો")
        self.assertEqual(num2words(10, lang="gu", ordinal=True), "દસમો")
        self.assertEqual(num2words(11, lang="gu", ordinal=True), "અગિયારમો")
        self.assertEqual(num2words(12, lang="gu", ordinal=True), "બારમો")
        self.assertEqual(num2words(13, lang="gu", ordinal=True), "તેરમો")
        self.assertEqual(num2words(14, lang="gu", ordinal=True), "ચૌદમો")
        self.assertEqual(num2words(15, lang="gu", ordinal=True), "પંદરમો")
        self.assertEqual(num2words(16, lang="gu", ordinal=True), "સોળમો")
        self.assertEqual(num2words(17, lang="gu", ordinal=True), "સત્તરમો")
        self.assertEqual(num2words(18, lang="gu", ordinal=True), "અઢારમો")
        self.assertEqual(num2words(19, lang="gu", ordinal=True), "ઓગણીસમો")
        self.assertEqual(num2words(20, lang="gu", ordinal=True), "વીસમો")
        self.assertEqual(num2words(21, lang="gu", ordinal=True), "એકવીસમો")
        self.assertEqual(num2words(22, lang="gu", ordinal=True), "બાવીસમો")
        self.assertEqual(num2words(25, lang="gu", ordinal=True), "પચ્ચીસમો")
        self.assertEqual(num2words(30, lang="gu", ordinal=True), "ત્રીસમો")
        self.assertEqual(num2words(40, lang="gu", ordinal=True), "ચાલીસમો")
        self.assertEqual(num2words(50, lang="gu", ordinal=True), "પચાસમો")
        self.assertEqual(num2words(60, lang="gu", ordinal=True), "સાઠમો")
        self.assertEqual(num2words(70, lang="gu", ordinal=True), "સિત્તેરમો")
        self.assertEqual(num2words(80, lang="gu", ordinal=True), "એંસીમો")
        self.assertEqual(num2words(90, lang="gu", ordinal=True), "નેવુંમો")
        self.assertEqual(num2words(100, lang="gu", ordinal=True), "એક સોમો")
        self.assertEqual(num2words(101, lang="gu", ordinal=True), "એક સો એકમો")
        self.assertEqual(num2words(200, lang="gu", ordinal=True), "બે સોમો")
        self.assertEqual(num2words(500, lang="gu", ordinal=True), "પાંચ સોમો")
        self.assertEqual(num2words(1000, lang="gu", ordinal=True), "એક હજારમો")
        self.assertEqual(num2words(1001, lang="gu", ordinal=True), "એક હજાર એકમો")
        self.assertEqual(num2words(10000, lang="gu", ordinal=True), "દસ હજારમો")

    def test_currency(self):
        """Test currency conversion."""
        self.assertEqual(
            num2words(0, lang="gu", to="currency", currency="INR"), "શૂન્ય રૂપિયા"
        )
        self.assertEqual(
            num2words(0.01, lang="gu", to="currency", currency="INR"),
            "શૂન્ય રૂપિયા અને એક પૈસો",
        )
        self.assertEqual(
            num2words(0.5, lang="gu", to="currency", currency="INR"),
            "શૂન્ય રૂપિયા અને પચાસ પૈસા",
        )
        self.assertEqual(
            num2words(1, lang="gu", to="currency", currency="INR"), "એક રૂપિયો"
        )
        self.assertEqual(
            num2words(1.5, lang="gu", to="currency", currency="INR"),
            "એક રૂપિયો અને પચાસ પૈસા",
        )
        self.assertEqual(
            num2words(0, lang="gu", to="currency", currency="USD"), "શૂન્ય ડોલર"
        )
        self.assertEqual(
            num2words(0.01, lang="gu", to="currency", currency="USD"),
            "શૂન્ય ડોલર અને એક સેન્ટ",
        )
        self.assertEqual(
            num2words(0.5, lang="gu", to="currency", currency="USD"),
            "શૂન્ય ડોલર અને પચાસ સેન્ટ",
        )
        self.assertEqual(
            num2words(1, lang="gu", to="currency", currency="USD"), "એક ડોલર"
        )
        self.assertEqual(
            num2words(1.5, lang="gu", to="currency", currency="USD"),
            "એક ડોલર અને પચાસ સેન્ટ",
        )
        self.assertEqual(
            num2words(0, lang="gu", to="currency", currency="EUR"), "શૂન્ય યૂરો"
        )
        self.assertEqual(
            num2words(0.01, lang="gu", to="currency", currency="EUR"),
            "શૂન્ય યૂરો અને એક સેન્ટ",
        )
        self.assertEqual(
            num2words(0.5, lang="gu", to="currency", currency="EUR"),
            "શૂન્ય યૂરો અને પચાસ સેન્ટ",
        )
        self.assertEqual(
            num2words(1, lang="gu", to="currency", currency="EUR"), "એક યૂરો"
        )
        self.assertEqual(
            num2words(1.5, lang="gu", to="currency", currency="EUR"),
            "એક યૂરો અને પચાસ સેન્ટ",
        )
        self.assertEqual(
            num2words(0, lang="gu", to="currency", currency="GBP"), "શૂન્ય પાઉન્ડ"
        )
        self.assertEqual(
            num2words(0.01, lang="gu", to="currency", currency="GBP"),
            "શૂન્ય પાઉન્ડ અને એક પેન્સ",
        )
        self.assertEqual(
            num2words(0.5, lang="gu", to="currency", currency="GBP"),
            "શૂન્ય પાઉન્ડ અને પચાસ પેન્સ",
        )
        self.assertEqual(
            num2words(1, lang="gu", to="currency", currency="GBP"), "એક પાઉન્ડ"
        )
        self.assertEqual(
            num2words(1.5, lang="gu", to="currency", currency="GBP"),
            "એક પાઉન્ડ અને પચાસ પેન્સ",
        )

    def test_year(self):
        """Test year conversion."""
        self.assertEqual(num2words(1000, lang="gu", to="year"), "સન એક હજાર")
        self.assertEqual(num2words(1066, lang="gu", to="year"), "સન એક હજાર છાસઠ")
        self.assertEqual(
            num2words(1492, lang="gu", to="year"), "સન એક હજાર ચાર સો બાણું"
        )
        self.assertEqual(
            num2words(1776, lang="gu", to="year"), "સન એક હજાર સાત સો છોતેર"
        )
        self.assertEqual(num2words(1800, lang="gu", to="year"), "સન એક હજાર આઠ સો")
        self.assertEqual(num2words(1900, lang="gu", to="year"), "સન એક હજાર નવ સો")
        self.assertEqual(
            num2words(1984, lang="gu", to="year"), "સન એક હજાર નવ સો ચોર્યાસી"
        )
        self.assertEqual(
            num2words(1999, lang="gu", to="year"), "સન એક હજાર નવ સો નવ્વાણું"
        )
        self.assertEqual(num2words(2000, lang="gu", to="year"), "સન બે હજાર")
        self.assertEqual(num2words(2001, lang="gu", to="year"), "સન બે હજાર એક")
        self.assertEqual(num2words(2010, lang="gu", to="year"), "સન બે હજાર દસ")
        self.assertEqual(num2words(2020, lang="gu", to="year"), "સન બે હજાર વીસ")
        self.assertEqual(num2words(2024, lang="gu", to="year"), "સન બે હજાર ચોવીસ")
        self.assertEqual(num2words(2100, lang="gu", to="year"), "સન બે હજાર એક સો")

    def test_string_input(self):
        """Test string input conversion."""
        self.assertEqual(num2words("0", lang="gu"), "શૂન્ય")
        self.assertEqual(num2words("1", lang="gu"), "એક")
        self.assertEqual(num2words("10", lang="gu"), "દસ")
        self.assertEqual(num2words("100", lang="gu"), "એક સો")
        self.assertEqual(num2words("1000", lang="gu"), "એક હજાર")
        self.assertEqual(num2words("10000", lang="gu"), "દસ હજાર")
        self.assertEqual(num2words("100000", lang="gu"), "એક લાખ")
        self.assertEqual(num2words("1000000", lang="gu"), "દસ લાખ")

    def test_edge_cases(self):
        """Test edge cases and special conditions."""
        # Test zero
        self.assertEqual(num2words(0, lang="gu"), "શૂન્ય")

        # Test that the converter handles various input types
        self.assertEqual(num2words(100, lang="gu"), num2words("100", lang="gu"))
        self.assertEqual(num2words(1000, lang="gu"), num2words("1000", lang="gu"))

