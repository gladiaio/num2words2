num2words2 library - Convert numbers to words in multiple languages
===================================================================

.. image:: https://img.shields.io/pypi/v/num2words2.svg
   :target: https://pypi.python.org/pypi/num2words2
   :alt: PyPI version

.. image:: https://img.shields.io/pypi/pyversions/num2words2.svg
   :target: https://pypi.python.org/pypi/num2words2
   :alt: Python versions

.. image:: https://img.shields.io/pypi/l/num2words2.svg
   :target: https://pypi.python.org/pypi/num2words2
   :alt: License

.. image:: https://img.shields.io/pypi/dm/num2words2.svg
   :target: https://pypi.python.org/pypi/num2words2
   :alt: Monthly downloads

.. image:: https://img.shields.io/aur/version/python-num2words2.svg
   :target: https://aur.archlinux.org/packages/python-num2words2
   :alt: AUR version

.. image:: https://github.com/gladiaio/num2words2/actions/workflows/ci.yml/badge.svg?branch=main
   :target: https://github.com/gladiaio/num2words2/actions/workflows/ci.yml
   :alt: CI status

.. image:: https://github.com/gladiaio/num2words2/actions/workflows/lint.yml/badge.svg?branch=main
   :target: https://github.com/gladiaio/num2words2/actions/workflows/lint.yml
   :alt: Lint status

.. image:: https://coveralls.io/repos/github/gladiaio/num2words2/badge.svg?branch=main
   :target: https://coveralls.io/github/gladiaio/num2words2?branch=main
   :alt: Coverage


``num2words2`` converts numbers like ``42`` to words like ``forty-two`` across
120+ languages and 170+ locale codes (see ``REFERENCE.md`` for the full table),
producing cardinals, ordinals, currency, year, fractions, bank-cheque format, and
aviation/ICAO digit-by-digit phraseology.

It is a **Rust port of the num2words conversion engine with a thin Python binder**.
Every conversion runs in a compiled Rust core (PyO3/``abi3``); Python only normalises
the arguments and shapes the result. The output is byte-for-byte identical to the
original pure-Python library — validated against a frozen ~150,000-case corpus — while
running typically **4–12× faster**, and up to **26×** on some languages and **150×**
on string parsing (see `Performance`_).

The project is hosted on GitHub_, and the full documentation is available in
the Wiki_. Contributions are welcome.

.. _GitHub: https://github.com/gladiaio/num2words2
.. _Wiki: https://github.com/gladiaio/num2words2/wiki
.. _GitHub Releases: https://github.com/gladiaio/num2words2/releases

Performance
-----------

Because the conversion engine is compiled Rust, ``num2words2`` is several times
faster than the original pure-Python ``num2words``. Benchmarked against
``num2words`` 0.5.14, Apple M-series, nanoseconds per call:

.. list-table::
   :header-rows: 1
   :widths: 34 22 22 14

   * - Operation
     - original
     - num2words2
     - speedup
   * - int → cardinal (``8765``)
     - 24,384 ns
     - 2,099 ns
     - 11.6×
   * - float → cardinal (``1234.56``)
     - 33,107 ns
     - 3,367 ns
     - 9.8×
   * - ordinal
     - 24,969 ns
     - 2,407 ns
     - 10.4×
   * - currency
     - 32,518 ns
     - 3,345 ns
     - 9.7×
   * - year
     - 13,484 ns
     - 1,434 ns
     - 9.4×
   * - string input (``"1234"``)
     - 335,862 ns
     - 2,200 ns
     - 152.6×
   * - French cardinal
     - 61,046 ns
     - 2,359 ns
     - 25.9×
   * - German cardinal
     - 63,046 ns
     - 2,386 ns
     - 26.4×
   * - Russian cardinal
     - 4,781 ns
     - 1,001 ns
     - 4.8×

The speedup is largest where the original does heavy Python-level parsing (string
inputs) or deep recursion (French, German); it never comes at the cost of accuracy —
output is byte-for-byte identical to the original across a frozen ~150,000-case
corpus of cardinals, ordinals, currency, years, fractions, floats and strings.

Installation
------------

Install from PyPI (Python 3.10+) — the wheel bundles the Python binder and the
compiled Rust extension, built per platform::

    pip install num2words2

Prebuilt wheels are published for:

* **Linux** — x86_64 and aarch64 (manylinux2014)
* **macOS** — Apple Silicon (arm64); Intel (x86_64) from a later release
* **Windows** — x86_64

On any other platform, pip falls back to the source distribution and builds the
extension locally — this needs a **stable Rust toolchain** (`rustup
<https://rustup.rs>`_).

Slim vs. full wheels
~~~~~~~~~~~~~~~~~~~~~~

``num2words2`` ships in two flavours:

* **slim** (the PyPI default, ~2 MB) — everything except native
  language-detection. ``num2words_sentence(lang=None)`` auto-detect is disabled;
  pass an explicit ``lang`` and every converter works.
* **full** (~140 MB) — embeds the lingua-rs models so
  ``num2words_sentence(lang=None)`` auto-detects the language natively. Full
  wheels are attached to each `GitHub Releases`_ entry; install one directly::

      pip install https://github.com/gladiaio/num2words2/releases/download/v1.0.18/num2words2-1.0.18-cp38-abi3-<platform>.whl

The compiled extension is distributed **only** through these wheels (it is not
committed to the repository).

To build from source you need a stable Rust toolchain and `maturin
<https://www.maturin.rs>`_::

    maturin build --release                        # full (bundles lingua models)
    maturin build --release --no-default-features  # slim (~2 MB, no models)



Development Setup
-----------------
The project uses pre-commit hooks to ensure code quality. To set up your development environment::

    # Install pre-commit
    pip install pre-commit

    # Install the git hook scripts
    pre-commit install

    # Run hooks on all files (optional, useful for initial setup)
    pre-commit run --all-files

This will automatically format and lint your code before each commit using:

* autopep8 - PEP 8 formatting
* autoflake - removes unused imports and variables
* isort - sorts imports
* flake8 - style and quality checks
* trailing-whitespace removal
* end-of-file fixing


Testing
-------

The library uses `pytest` for testing. First, install the development dependencies:

.. code-block:: bash

    make dev-install

Then, you can run the test suite using several methods:

*   **Run basic tests:** This runs tests with your current Python environment.

    .. code-block:: bash

        make test

*   **Run with Tox:** This runs tests against all supported Python versions, which is the standard for CI.

    .. code-block:: bash

        tox

Generating End-to-End Tests with LLMs
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The repository includes a powerful script to generate high-quality, realistic test cases using Large Language Models (LLMs). This helps ensure accuracy across multiple languages and complex scenarios.

**What it does:** The ``tests/scripts/generate_llm_tests.py`` script uses an LLM (like GPT-4o) to create sentences containing numbers, dates, and currencies, and then generates the expected word-for-word conversion.

**Requirements:**

*   An OpenAI API key. You must set it as an environment variable: ``export OPENAI_API_KEY='your-key-here'``

**How to Use:**

To generate 10 new test sentences for French and Spanish, you can run:

.. code-block:: bash

    python tests/scripts/generate_llm_tests.py --languages fr,es --samples 10

The new tests will be appended to ``tests/data/e2e_test_sentences.csv``.

**Key Options:**

*   ``--languages``: Comma-separated list of language codes (e.g., ``en_IN,de,it``).
*   ``--samples``: Number of samples to generate per language.
*   ``--mode``: Use ``sentences`` for full sentences or ``numbers`` for direct number-to-word conversions.
*   ``--model``: The OpenAI model to use (e.g., ``gpt-4o``, ``gpt-4o-mini``).
*   ``--output``: Specify a different output file.
*   ``--overwrite``: Overwrite the output file instead of appending.

This tool is essential for expanding test coverage and ensuring the library's robustness.


Usage
-----
Command line::

    $ num2words2 10001
    ten thousand and one
    $ num2words2 24,120.10
    twenty-four thousand, one hundred and twenty point one
    $ num2words2 24,120.10 -l es
    veinticuatro mil ciento veinte punto uno
    $ num2words2 2.14 -l es --to currency
    dos euros con catorce céntimos

In code there's only one function to use::

    >>> from num2words2 import num2words
    >>> num2words(42)
    forty-two
    >>> num2words(42, to='ordinal')
    forty-second
    >>> num2words(42, lang='fr')
    quarante-deux

Besides the numerical argument, there are two main optional arguments, ``to:`` and ``lang:``

**to:** The converter to use. Supported values are:

* ``cardinal`` (default)
* ``ordinal``
* ``ordinal_num``
* ``year``
* ``currency``

**lang:** The language in which to convert the number. Supported values are:

* ``en`` (English, default)
* ``am`` (Amharic)
* ``ar`` (Arabic)
* ``az`` (Azerbaijani)
* ``be`` (Belarusian)
* ``bn`` (Bangladeshi)
* ``ca`` (Catalan)
* ``ce`` (Chechen)
* ``cs`` (Czech)
* ``cy`` (Welsh)
* ``da`` (Danish)
* ``de`` (German)
* ``en_GB`` (English - Great Britain)
* ``en_IN`` (English - India)
* ``en_NG`` (English - Nigeria)
* ``es`` (Spanish)
* ``es_CO`` (Spanish - Colombia)
* ``es_CR`` (Spanish - Costa Rica)
* ``es_GT`` (Spanish - Guatemala)
* ``es_VE`` (Spanish - Venezuela)
* ``eu`` (EURO)
* ``fa`` (Farsi)
* ``fi`` (Finnish)
* ``fr`` (French)
* ``fr_BE`` (French - Belgium)
* ``fr_CH`` (French - Switzerland)
* ``fr_DZ`` (French - Algeria)
* ``he`` (Hebrew)
* ``hi`` (Hindi)
* ``hu`` (Hungarian)
* ``hy`` (Armenian)
* ``id`` (Indonesian)
* ``is`` (Icelandic)
* ``it`` (Italian)
* ``ja`` (Japanese)
* ``kn`` (Kannada)
* ``ko`` (Korean)
* ``kz`` (Kazakh)
* ``mn`` (Mongolian)
* ``lt`` (Lithuanian)
* ``lv`` (Latvian)
* ``nl`` (Dutch)
* ``no`` (Norwegian)
* ``pl`` (Polish)
* ``pt`` (Portuguese)
* ``pt_BR`` (Portuguese - Brazilian)
* ``ro`` (Romanian)
* ``ru`` (Russian)
* ``sl`` (Slovene)
* ``sk`` (Slovak)
* ``sr`` (Serbian)
* ``sv`` (Swedish)
* ``te`` (Telugu)
* ``tet`` (Tetum)
* ``tg`` (Tajik)
* ``tr`` (Turkish)
* ``th`` (Thai)
* ``uk`` (Ukrainian)
* ``vi`` (Vietnamese)
* ``zh`` (Chinese - Traditional)
* ``zh_CN`` (Chinese - Simplified / Mainland China)
* ``zh_TW`` (Chinese - Traditional / Taiwan)
* ``zh_HK`` (Chinese - Traditional / Hong Kong)

You can supply values like ``fr_FR``; if the country doesn't exist but the
language does, the code will fall back to the base language (i.e. ``fr``). If
you supply an unsupported language, ``NotImplementedError`` is raised.
Therefore, if you want to call ``num2words`` with a fallback, you can do::

    try:
        return num2words(42, lang=mylang)
    except NotImplementedError:
        return num2words(42, lang='en')

Additionally, some converters and languages support other optional arguments
that are needed to make the converter useful in practice.

Wiki
----
For the full documentation, including installation, API details, CLI usage,
supported locales, sentence conversion, currency handling, and migration
guidance, please check the Wiki_. Feel free to propose wiki enhancements.

History
-------

``num2words`` is based on an old library, ``pynum2word``, created by Taro Ogawa
in 2003. Unfortunately, the library stopped being maintained and the author
can't be reached. There was another developer, Marius Grigaitis, who in 2011
added Lithuanian support, but didn't take over maintenance of the project.

Virgil Dupras from Savoir-faire Linux based himself on Marius Grigaitis' improvements
and re-published ``pynum2word`` as ``num2words``.

Relationship to ``num2words``
-----------------------------

``num2words2`` is **not a divergent fork**. It is a Rust port of the ``num2words``
conversion engine with a Python binder, kept output-compatible with the original.

The original project's issues and pull requests are actively monitored. Valid,
relevant fixes and features are reviewed and then ported into this project's Rust
core, with byte-parity re-verified against the frozen corpus. ``num2words2`` therefore
tracks upstream improvements while delivering them natively — and several times faster.

Jean-Louis Queguiner
