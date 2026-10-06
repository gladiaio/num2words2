# -*- coding: utf-8 -*-
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
"""migration/migrate_to_num2words2.py rewrites only the public API.

num2words2 has no ``lang_*`` modules or ``CONVERTER_CLASSES``, so the script
must leave those imports alone and warn about them instead of rewriting them
to a module that does not exist (gladiaio/num2words2#239).
"""

import importlib.util
import os
import subprocess
import sys

import pytest

SCRIPT = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "migration",
    "migrate_to_num2words2.py",
)


@pytest.fixture(scope="module")
def migrate():
    spec = importlib.util.spec_from_file_location("migrate_to_num2words2", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


@pytest.mark.parametrize(
    "before, after",
    [
        ("from num2words import num2words\n", "from num2words2 import num2words\n"),
        ("import num2words\n", "import num2words2 as num2words\n"),
        ("import num2words as n2w\n", "import num2words2 as n2w\n"),
        ("    import num2words\n", "    import num2words2 as num2words\n"),
    ],
)
def test_public_api_imports_are_rewritten(migrate, before, after):
    new, changes, warnings = migrate.migrate_file_content(before)
    assert new == after
    assert changes and not warnings


@pytest.mark.parametrize(
    "line",
    [
        "from num2words2 import num2words\n",
        "import num2words2\n",
        "import num2words2 as num2words\n",
    ],
)
def test_already_migrated_code_is_left_alone(migrate, line):
    assert migrate.migrate_file_content(line) == (line, [], [])


@pytest.mark.parametrize(
    "line",
    [
        "from num2words.lang_en import Num2Word_EN\n",
        "import num2words.lang_fr\n",
        "from num2words import CONVERTER_CLASSES\n",
    ],
)
def test_converter_class_imports_warn(migrate, line):
    new, _, warnings = migrate.migrate_file_content(line)
    assert "num2words2.lang_" not in new
    assert len(warnings) == 1 and "line 1" in warnings[0]


def test_warning_only_file_is_not_rewritten(migrate, tmp_path):
    source = tmp_path / "legacy.py"
    source.write_text("from num2words.lang_en import Num2Word_EN\n")
    result = subprocess.run(
        [sys.executable, SCRIPT, str(tmp_path)],
        capture_output=True,
        text=True,
        encoding="utf-8",
        env=dict(os.environ, PYTHONIOENCODING="utf-8"),
        check=True,
    )
    assert "Num2Word_EN" in result.stdout
    assert source.read_text() == "from num2words.lang_en import Num2Word_EN\n"
    assert not (tmp_path / "legacy.py.num2words_backup").exists()


def test_file_is_migrated_with_backup(migrate, tmp_path):
    source = tmp_path / "app.py"
    source.write_text("from num2words import num2words\n")
    result = migrate.migrate_file(str(source))
    assert result["backup"] == str(source) + ".num2words_backup"
    assert source.read_text() == "from num2words2 import num2words\n"
