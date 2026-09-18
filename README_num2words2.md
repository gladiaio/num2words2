# num2words2

[![PyPI version](https://img.shields.io/pypi/v/num2words2.svg)](https://pypi.org/project/num2words2/)
[![Python Versions](https://img.shields.io/pypi/pyversions/num2words2.svg)](https://pypi.org/project/num2words2/)
[![License](https://img.shields.io/pypi/l/num2words2.svg)](https://pypi.org/project/num2words2/)
[![Monthly downloads](https://img.shields.io/pypi/dm/num2words2.svg)](https://pypi.org/project/num2words2/)
[![AUR version](https://img.shields.io/aur/version/python-num2words2.svg)](https://aur.archlinux.org/packages/python-num2words2)
[![CI](https://github.com/gladiaio/num2words2/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/gladiaio/num2words2/actions/workflows/ci.yml)
[![Lint](https://github.com/gladiaio/num2words2/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/gladiaio/num2words2/actions/workflows/lint.yml)
[![Coverage](https://coveralls.io/repos/github/gladiaio/num2words2/badge.svg?branch=main)](https://coveralls.io/github/gladiaio/num2words2?branch=main)

`num2words2` is a **Rust port of the `num2words` conversion engine with a Python binder**, optimized for LLM/AI/speech applications. It converts numbers like `42` to words like `forty-two` across **120+ languages and 170+ locale codes** (including regional variants like `pt_BR`, `fr_BE`, `sr_Latn`, and aviation/ICAO). Every conversion runs in a compiled Rust core; Python only normalises arguments and shapes the result. Output is byte-for-byte identical to the original pure-Python library (validated against a frozen ~150,000-case corpus) while running typically 4–12× faster (up to 26× on some languages, 150× on string parsing). The original project's issues and PRs are actively monitored, and valid requests are ported here after review.

> **Documentation:** see the [GitHub Wiki](https://github.com/gladiaio/num2words2/wiki) for installation, API, CLI, supported locales, sentence conversion, currency handling, and migration guidance. For the full in-repo API reference, see [`REFERENCE.md`](REFERENCE.md).

## Install

Python: `pip install num2words2`. Rust: the compiled core is published on
crates.io as [`num2words2-core`](https://crates.io/crates/num2words2-core):

```bash
cargo add num2words2-core
```

## Why num2words2?

The original `num2words` library by Savoir-faire Linux became unmaintained and couldn't keep up with the rapid pace of innovation in the AI/LLM/speech technology space. Modern applications like:

- **Large Language Models (LLMs)** requiring robust text preprocessing
- **Speech synthesis systems** needing accurate number-to-word conversion
- **Voice assistants and chatbots** processing numerical data
- **AI training pipelines** requiring consistent text normalization
- **Multilingual AI applications** spanning 120+ languages and 170+ locale codes

...needed a more actively maintained solution with faster bug fixes, enhanced language support, and compatibility with modern Python ecosystems.

## Features

- Convert numbers to words in **120+ languages** (170+ locale codes)
- Conversion modes: `cardinal`, `ordinal`, `ordinal_num`, `year`, `currency`, `cheque`, `fraction`
- Per-call customization: `style=`, `precision=`, `cents=`, `gender=`, `case=`, `spaced=`, `decimal_word=`
- 3-decimal currencies (BHD, KWD, OMR, JOD, TND, LYD, IQD)
- Bank-cheque format (`to='cheque'`)
- Fractions: `num2words('1/3')` → `'one third'` (idiomatic forms in `en`/`fr`/`es`/`it`/`pt`/`de`)
- Aviation/ICAO English (`en_Aero_ICAO` and friends), with FAA/USN/US_Army/NATO profiles plus per-context phraseology (altitude, flight level, heading, squawk, runway, frequency)
- Utility helpers: `maxval(lang)`, `group_digits()`, `num2words_sentence()`
- Critical bug fixes for decimal handling, negative numbers, float conversions, Decimal/string precision at trillion scale
- Optimized for modern AI/ML/speech applications
- Actively maintained with regular updates and community contributions
- Drop-in replacement for the original num2words
- Modern CI/CD pipeline with comprehensive testing — auto-publishes to PyPI and AUR on every tag

## Installation

```bash
pip install num2words2
```

## Usage

### Basic Usage

```python
from num2words2 import num2words

# Cardinal numbers
print(num2words(42))  # forty-two
print(num2words(42, lang='es'))  # cuarenta y dos
print(num2words(42, lang='fr'))  # quarante-deux

# Ordinal numbers
print(num2words(42, to='ordinal'))  # forty-second
print(num2words(42, to='ordinal', lang='es'))  # cuadragésimo segundo

# Currency
print(num2words(42.50, to='currency'))  # forty-two euro, fifty cents
print(num2words(42.50, to='currency', lang='es'))  # cuarenta y dos euros con cincuenta céntimos

# Year
print(num2words(2024, to='year'))  # two thousand and twenty-four

# Fractions (new in v1.0.13)
print(num2words('1/3'))                   # one third
print(num2words('3/4', lang='fr'))        # trois quarts
print(num2words('1/100', lang='de'))      # ein Hundertstel

# Bank cheque format (new in v1.0.12)
print(num2words(1234.56, to='cheque', currency='USD'))
# ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND 56/100 DOLLARS

# 3-decimal currencies (new in v1.0.12)
print(num2words(5.123, to='currency', currency='BHD'))
# five dinars, one hundred and twenty-three fils

# Aviation/ICAO digit-by-digit (new in v1.0.14, refined through v1.0.17)
print(num2words(5739, lang='en_Aero_ICAO'))   # fife seven tree niner
from num2words2 import CONVERTER_CLASSES
aero = CONVERTER_CLASSES['en_Aero_ICAO']
print(aero.to_altitude(12500))   # wun too thousand fife hundred feet
print(aero.to_squawk(7700))      # squawk seven seven zero zero
print(aero.to_runway('27R'))     # runway too seven right

# Per-call options
print(num2words(1234, lang='en', style='us'))    # one thousand, two hundred thirty-four (no 'and')
print(num2words(1, lang='ru', case='genitive')) # одного
print(num2words(1, lang='he', gender='f'))       # אחת
```

For the full feature reference (every mode, kwarg, language, and aviation method), see [REFERENCE.md](REFERENCE.md).

### Command Line Interface

```bash
$ num2words2 10001
ten thousand and one

$ num2words2 24120.10
twenty-four thousand, one hundred and twenty point one

$ num2words2 24120.10 -l es
veinticuatro mil ciento veinte punto uno

$ num2words2 2.14 -l es --to currency
dos euros con catorce céntimos

# List all supported languages
$ num2words2 --list-languages

# List all converters
$ num2words2 --list-converters
```

## Supported Languages

`num2words2` supports **120+ languages** (170+ locale codes including aliases and regional variants). The full list lives in [REFERENCE.md → Locale codes](REFERENCE.md#locale-codes); the highlights below give a sense of breadth.

**European**: `en` English, `fr` French (`fr_BE`, `fr_CH`, `fr_DZ`), `es` Spanish (`es_CO`, `es_CR`, `es_GT`, `es_HN`, `es_NI`, `es_VE`), `de` German, `it` Italian, `pt` Portuguese (`pt_BR`), `nl` Dutch, `ru` Russian, `pl` Polish, `cs` Czech, `sk` Slovak, `sl` Slovenian, `hu` Hungarian, `ro` Romanian, `el` Greek, `bg` Bulgarian, `uk` Ukrainian, `be` Belarusian, `hr` Croatian, `sr` Serbian (`sr_Cyrl`, `sr_Latn`), `bs` Bosnian, `mk` Macedonian, `sq` Albanian, `sv` Swedish, `da` Danish, `no` Norwegian, `nn` Nynorsk, `nb` Bokmål, `fi` Finnish, `et` Estonian, `lv` Latvian, `lt` Lithuanian, `is` Icelandic, `fo` Faroese, `ga` Irish, `cy` Welsh, `eu` Basque, `ca` Catalan, `gl` Galician, `oc` Occitan, `mt` Maltese, `lb` Luxembourgish, `rm` Romansh (`rm_puter`, `rm_surmiran`, `rm_sursilv`, `rm_sutsilv`, `rm_vallader`), `eo` Esperanto, `lij` Ligurian, `br` Breton.

**Asian**: `zh` Chinese (`zh_CN`, `zh_HK`, `zh_TW`), `ja` Japanese, `ko` Korean, `hi` Hindi, `bn` Bengali, `ta` Tamil, `te` Telugu, `kn` Kannada, `ml` Malayalam, `mr` Marathi, `gu` Gujarati, `pa` Punjabi, `or` Odia, `as` Assamese, `sa` Sanskrit, `ne` Nepali, `si` Sinhala, `ur` Urdu, `fa` Persian, `ps` Pashto, `sd` Sindhi, `dv` Divehi, `bo` Tibetan, `my` Burmese, `th` Thai, `lo` Lao, `km` Khmer, `vi` Vietnamese, `id` Indonesian, `ms` Malay, `tl` Tagalog, `fil` Filipino, `jv` Javanese, `su` Sundanese, `ksw` S'gaw Karen, `cnh` Hakha Chin, `lus` Mizo, `mn` Mongolian.

**Middle Eastern & Caucasian**: `ar` Arabic, `he` Hebrew, `tr` Turkish, `az` Azerbaijani, `ku` Kurdish, `ckb` Central Kurdish, `hy` Armenian, `ka` Georgian, `ce` Chechen, `ba` Bashkir, `tt` Tatar.

**Central Asian**: `kk` Kazakh (`kz`), `ky` Kyrgyz, `uz` Uzbek (`uz_Cyrl`), `tg` Tajik, `tk` Turkmen.

**African**: `am` Amharic, `ti` Tigrinya, `so` Somali, `om` Oromo, `ha` Hausa, `yo` Yoruba, `ig` Igbo, `sw` Swahili, `xh` Xhosa, `zu` Zulu, `sn` Shona, `lg` Luganda, `rw` Kinyarwanda, `ki` Kikuyu, `ln` Lingala, `wo` Wolof, `ff` Fula, `bm` Bambara, `mg` Malagasy.

**Pacific**: `mi` Māori, `haw` Hawaiian, `tet` Tetum.

**Caribbean & American**: `ht` Haitian Creole, `pap` Papiamentu.

**Other**: `la` Latin, `pli` Pali, `ban` Balinese, `kok` Konkani, `hmn` Hmong, `yi` Yiddish, `ceb` Cebuano.

**Specialized**: `en_Aero_ICAO` (and `en_Aero_FAA`, `en_Aero_USN`, `en_Aero_US_Navy`, `en_Aero_US_Army`, `en_Aero_NATO`) — aviation/voice-radio digit-by-digit reading per ICAO Annex 10 vol II + FAA AIM 4-2-9. See [REFERENCE.md → Aviation](REFERENCE.md#aviation--icao-english).

Run `num2words2 --list-languages` for the live list.

And many regional variations like es_CO (Colombian Spanish), pt_BR (Brazilian Portuguese), etc.

## Improvements over Original num2words

### Maintenance & Community
- **Active Maintenance**: Regular updates aligned with AI/ML ecosystem needs
- **Rapid Bug Resolution**: Issues are addressed promptly, not left open for months/years
- **Modern Development**: Updated CI/CD, comprehensive testing, automated dependency management
- **Community-Driven**: Open to contributions and feature requests from the AI/speech community

### Technical Improvements
- **Critical Bug Fixes**: Resolved issues with negative decimal handling, float comparisons, and type conversions affecting ML pipelines
- **Enhanced Language Support**: Added Armenian (hy), Mongolian (mn), Shona (sn) and improved existing language modules
- **LLM Optimization**: Better handling of edge cases common in large-scale text processing
- **Type Safety**: Improved type handling and error messages for better integration with ML frameworks
- **Performance**: Optimizations for batch processing scenarios common in AI applications

### AI/ML Specific Features
- **Consistent Output**: More predictable text generation for training datasets
- **Edge Case Handling**: Better support for unusual number formats found in real-world data
- **Multilingual Robustness**: Enhanced support for code-switching and multilingual AI models

## Migration from num2words

`num2words2` is designed as a drop-in replacement for `num2words` with full backward compatibility.

### 🤖 Automated Migration (Recommended)

We provide a migration script to automatically update your codebase:

```bash
# Download and run the migration script
curl -O https://raw.githubusercontent.com/gladiaio/num2words2/main/migrate_to_num2words2.py
python migrate_to_num2words2.py /path/to/your/project

# Or just scan current directory
python migrate_to_num2words2.py .

# Dry run to see what would change (recommended first step)
python migrate_to_num2words2.py --dry-run .
```

The script will:
- 🔍 Find all Python files with `num2words` imports
- 💾 Create backups of original files
- 🔄 Update imports to use `num2words2`
- 📝 Provide a detailed summary of changes

### 📝 Manual Migration

If you prefer manual migration, simply update your imports:

```python
# Before
from num2words import num2words

# After
from num2words2 import num2words
```

For backward compatibility during transition:

```python
try:
    from num2words2 import num2words
except ImportError:
    from num2words import num2words
```

### 📦 Update Dependencies

Update your dependency files:

```bash
# requirements.txt
- num2words>=0.5.12
+ num2words2>=0.5.15

# pyproject.toml
dependencies = [
-    "num2words>=0.5.12",
+    "num2words2>=0.5.15",
]

# setup.py
install_requires=[
-    "num2words>=0.5.12",
+    "num2words2>=0.5.15",
],
```

### 🧪 Testing Your Migration

After migration, test your application:

```python
# Test basic functionality
from num2words2 import num2words

# These should work exactly as before
assert num2words(42) == "forty-two"
assert num2words(42, lang='es') == "cuarenta y dos"
assert num2words(42, to='ordinal') == "forty-second"
```

### 🚀 Installation

```bash
# Install num2words2
pip install num2words2

# If you were using num2words, you can remove it
pip uninstall num2words
```

### ⚠️ Breaking Changes

`num2words2` maintains full backward compatibility. However, if you experience any issues:

1. **Check version compatibility** - Ensure you're using `num2words2>=0.5.15`
2. **Report issues** - Create an issue at https://github.com/gladiaio/num2words2/issues
3. **Rollback if needed** - The migration script creates backups for easy rollback

### 🔄 Rollback Migration

If you need to rollback:

```bash
# The migration script creates .num2words_backup files
# Restore them if needed
for file in $(find . -name "*.num2words_backup"); do
    original="${file%.num2words_backup}"
    cp "$file" "$original"
    echo "Restored $original"
done
```

## Development

### Running Tests

```bash
# Install development dependencies
pip install -r requirements-test.txt

# Run tests
python -m pytest tests/

# Run tests with coverage
python -m pytest tests/ --cov=num2words2
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the GNU Lesser General Public License v2.1 - see the LICENSE file for details.

## Credits

`num2words2` is based on the original [num2words](https://github.com/savoirfairelinux/num2words) project by Savoir-faire Linux inc.

**Original Library History:**
- **pynum2word** (2003) - Created by Taro Ogawa
- **Lithuanian support** (2011) - Added by Marius Grigaitis
- **num2words** - Re-published by Virgil Dupras, Savoir-faire Linux
- **num2words2** (2025) - Rust port (with a Python binder) by Jean-Louis Queguiner for AI/ML applications; tracks the original's issues/PRs, ported after review

## Author

Maintained by Jean-Louis Queguiner

## Links

- [PyPI Package](https://pypi.org/project/num2words2/)
- [GitHub Repository](https://github.com/gladiaio/num2words2)
- [Issue Tracker](https://github.com/gladiaio/num2words2/issues)
