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

from __future__ import division, print_function, unicode_literals

from unittest import TestCase

from num2words2 import num2words


def n2zh(*args, **kwargs):
    return num2words(*args, lang="zh", **kwargs)


class Num2WordsZHTest(TestCase):
    def test_low(self):
        self.assertEqual(n2zh(0), "零")
        self.assertEqual(n2zh(0, reading="capital"), "零")
        self.assertEqual(n2zh(1), "一")
        self.assertEqual(n2zh(1, reading="capital"), "壹")
        self.assertEqual(n2zh(2), "二")
        self.assertEqual(n2zh(2, reading="capital"), "贰")
        self.assertEqual(n2zh(3), "三")
        self.assertEqual(n2zh(3, reading="capital"), "叁")
        self.assertEqual(n2zh(4), "四")
        self.assertEqual(n2zh(4, reading="capital"), "肆")
        self.assertEqual(n2zh(5), "五")
        self.assertEqual(n2zh(5, reading="capital"), "伍")
        self.assertEqual(n2zh(6), "六")
        self.assertEqual(n2zh(6, reading="capital"), "陆")
        self.assertEqual(n2zh(7), "七")
        self.assertEqual(n2zh(7, reading="capital"), "柒")
        self.assertEqual(n2zh(8), "八")
        self.assertEqual(n2zh(8, reading="capital"), "捌")
        self.assertEqual(n2zh(9), "九")
        self.assertEqual(n2zh(9, reading="capital"), "玖")
        self.assertEqual(n2zh(10), "十")
        self.assertEqual(n2zh(10, reading="capital"), "壹拾")
        self.assertEqual(n2zh(11), "十一")
        self.assertEqual(n2zh(11, reading="capital"), "壹拾壹")
        self.assertEqual(n2zh(12), "十二")
        self.assertEqual(n2zh(12, reading="capital"), "壹拾贰")
        self.assertEqual(n2zh(13), "十三")
        self.assertEqual(n2zh(13, reading="capital"), "壹拾叁")
        self.assertEqual(n2zh(14), "十四")
        self.assertEqual(n2zh(14, reading="capital"), "壹拾肆")
        self.assertEqual(n2zh(15), "十五")
        self.assertEqual(n2zh(15, reading="capital"), "壹拾伍")
        self.assertEqual(n2zh(16), "十六")
        self.assertEqual(n2zh(16, reading="capital"), "壹拾陆")
        self.assertEqual(n2zh(17), "十七")
        self.assertEqual(n2zh(17, reading="capital"), "壹拾柒")
        self.assertEqual(n2zh(18), "十八")
        self.assertEqual(n2zh(18, reading="capital"), "壹拾捌")
        self.assertEqual(n2zh(19), "十九")
        self.assertEqual(n2zh(19, reading="capital"), "壹拾玖")
        self.assertEqual(n2zh(20), "二十")
        self.assertEqual(n2zh(20, reading="capital"), "贰拾")

    def test_mid(self):
        self.assertEqual(n2zh(100), "一百")
        self.assertEqual(n2zh(100, reading="capital"), "壹佰")
        self.assertEqual(n2zh(-123), "负一百二十三")
        self.assertEqual(n2zh(123, reading="capital"), "壹佰贰拾叁")
        self.assertEqual(n2zh(300), "三百")
        self.assertEqual(n2zh("300", reading="capital"), "叁佰")
        self.assertEqual(n2zh(1000), "一千")
        self.assertEqual(n2zh(-1000, reading="capital"), "负壹仟")
        self.assertEqual(n2zh("8000", reading="capital"), "捌仟")

    def test_high(self):
        self.assertEqual(n2zh(10000), "一万")
        self.assertEqual(n2zh("10000", reading="capital"), "壹万")
        self.assertEqual(n2zh(12345), "一万二千三百四十五")
        self.assertEqual(n2zh("12345", reading="capital"), "壹万贰仟叁佰肆拾伍")
        self.assertEqual(n2zh(10**8), "一亿")
        self.assertEqual(n2zh(10**8, reading="capital"), "壹亿")
        self.assertEqual(n2zh(1234567890), "十二亿三千四百五十六万七千八百九十")
        self.assertEqual(n2zh(1234567890, reading="capital"), "壹拾贰亿叁仟肆佰伍拾陆万柒仟捌佰玖拾")
        self.assertEqual(
            n2zh(12345678901234567890),
            "一千二百三十四京五千六百七十八兆九千零一十二亿三千四百五十六万七千八百九十",
        )
        self.assertEqual(n2zh(120078900500090), "一百二十兆零七百八十九亿零五十万零九十")
        with self.assertRaises(OverflowError):
            n2zh(10**100)

    def test_stuff_zero(self):
        self.assertEqual(n2zh(1203405, stuff_zero=1), "一百二十万零三千四百零五")
        self.assertEqual(n2zh(1203405, stuff_zero=2), "一百二十万三千四百零五")
        self.assertEqual(n2zh(1203405, stuff_zero=3), "一百二十万三千四百五")
        self.assertEqual(n2zh(908070605, stuff_zero=1), "九亿零八百零七万零六百零五")
        self.assertEqual(n2zh(908070605, stuff_zero=2), "九亿零八百零七万零六百零五")
        self.assertEqual(n2zh(908070605, stuff_zero=3), "九亿八百七万六百五")
        self.assertEqual(n2zh(1200034005, stuff_zero=1), "十二亿零三万四千零五")
        self.assertEqual(n2zh(1200034005, stuff_zero=2), "十二亿零三万四千零五")
        self.assertEqual(n2zh(1200034005, stuff_zero=3), "十二亿三万四千五")
        self.assertEqual(n2zh(5000006, stuff_zero=1), "五百万零六")
        self.assertEqual(n2zh(5000006, stuff_zero=2), "五百万零六")
        self.assertEqual(n2zh(5000006, stuff_zero=3), "五百万六")
        self.assertEqual(n2zh(102003040000000, stuff_zero=1), "一百零二兆零三十亿零四千万")
        self.assertEqual(n2zh(102003040000000, stuff_zero=2), "一百零二兆零三十亿四千万")
        self.assertEqual(n2zh(102003040000000, stuff_zero=3), "一百二兆三十亿四千万")

    def test_cardinal_float(self):
        self.assertEqual(n2zh(0.123456789), "零点一二三四五六七八九")
        # self.assertEqual(n2zh('10.012345678901234567890123456789'),"一十点零一二三四五六七八九零一二三四五六七八九零一二三四五六七八九")
        self.assertEqual(n2zh(10**8 + 0.01), "一亿点零一")
        self.assertEqual(n2zh(10**8 + 0.01, reading="capital"), "壹亿点零壹")

    def test_ordinal(self):
        self.assertEqual(n2zh(0, to="ordinal"), "第零")
        self.assertEqual(n2zh(2, to="ordinal"), "第二")
        self.assertEqual(n2zh(10, to="ordinal"), "第十")
        self.assertEqual(n2zh(11, to="ordinal"), "第十一")
        self.assertEqual(n2zh("19", to="ordinal"), "第十九")
        self.assertEqual(n2zh(109, to="ordinal"), "第一百零九")
        self.assertEqual(n2zh(2, to="ordinal", counter="名"), "第二名")
        self.assertEqual(n2zh(3, to="ordinal", counter="位"), "第三位")

    def test_ordinal_num(self):
        self.assertEqual(n2zh(1.5, to="ordinal_num"), "第1.5")
        self.assertEqual(n2zh(120, to="ordinal_num"), "第120")

    def test_currency(self):
        self.assertEqual(n2zh("0", to="currency", reading="capital"), "零圆正")
        self.assertEqual(n2zh(5.00, to="currency", reading="capital"), "伍圆正")
        self.assertEqual(n2zh("0", to="currency"), "零元")
        self.assertEqual(n2zh(5.00, to="currency"), "五元")
        self.assertEqual(n2zh(10.05, to="currency", reading="capital"), "壹拾圆伍分")
        self.assertEqual(n2zh(10.05, to="currency"), "十元零五分")
        self.assertEqual(n2zh(12.12, to="currency", reading="capital"), "壹拾贰圆壹角贰分")
        self.assertEqual(
            n2zh(1235678, to="currency", reading="capital"),
            "壹佰贰拾叁万伍仟陆佰柒拾捌圆正",
        )
        self.assertEqual(
            n2zh("1234567890.123", to="currency", reading="capital"),
            "壹拾贰亿叁仟肆佰伍拾陆万柒仟捌佰玖拾圆壹角贰分",
        )
        self.assertEqual(n2zh(67890.126, to="currency"), "六万七千八百九十元一角三分")
        self.assertEqual(
            n2zh(987654.3, to="currency", currency="USD", reading="capital"),
            "美元玖拾捌万柒仟陆佰伍拾肆圆叁角",
        )
        self.assertEqual(
            n2zh(987654.3, to="currency", currency="USD"),
            "美元九十八万七千六百五十四元三角",
        )
        self.assertEqual(n2zh(135.79, to="currency", currency="EUR"), "欧元一百三十五元七角九分")
        # ABC currency test removed - handled by base class now

    def test_year(self):
        self.assertEqual(n2zh(2020, to="year", prefer=["〇"]), "二〇二〇年")
        self.assertEqual(n2zh(2020, to="year"), "二零二零年")
        self.assertEqual(n2zh(2020.0, to="year"), "二零二零年")
        self.assertEqual(n2zh(2020, to="year", reading="capital"), "公元二零二零年")
        self.assertEqual(
            n2zh(2020, to="year", reading="capital", prefer=["西元"]), "西元二零二零年"
        )
        self.assertEqual(n2zh(-1, to="year"), "公元前一年")
        self.assertEqual(n2zh(-1, to="year", prefer=["西元"]), "西元前一年")
        with self.assertRaises(TypeError):
            n2zh(2020.1, to="year")

    def test_negative_decimals(self):
        # Comprehensive test for negative decimals including -0.4
        self.assertEqual(num2words(-0.4, lang="zh"), "负零点四")
        self.assertEqual(num2words(-0.5, lang="zh"), "负零点五")
        self.assertEqual(num2words(-1.4, lang="zh"), "负一点四")
        self.assertEqual(num2words(-10.25, lang="zh"), "负一十点二五")
