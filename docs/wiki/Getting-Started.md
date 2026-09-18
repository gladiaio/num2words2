# Getting Started

## Requirements

`num2words2` targets modern Python. The current package metadata requires Python 3.10 or newer and advertises support for Python 3.10 through 3.15.

## Installation

Install from PyPI:

```bash
python -m pip install num2words2
```

Install from a local checkout for development:

```bash
git clone https://github.com/gladiaio/num2words2.git
cd num2words2
python -m pip install -e .
```

## Basic Usage

The main API is `num2words(value, lang="en", to="cardinal", **kwargs)`.

```python
from num2words2 import num2words

num2words(42)
# 'forty-two'

num2words(42, lang="es")
# 'cuarenta y dos'

num2words(42, lang="fr")
# 'quarante-deux'
```

Use `to=` to select a conversion mode:

```python
num2words(42, to="ordinal")
# 'forty-second'

num2words(5, to="ordinal_num")
# '5th'

num2words(2024, to="year")
# 'twenty twenty-four'

num2words(42.50, to="currency", currency="USD")
# 'forty-two dollars, fifty cents'

num2words(1234.56, to="cheque", currency="USD")
# 'ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND 56/100 DOLLARS'

num2words("3/4")
# 'three quarters'
```

## Language Codes

Pass a locale code with `lang=`:

```python
num2words(42, lang="pt_BR")
# 'quarenta e dois'

num2words(42, lang="pt-BR")
# 'quarenta e dois'
```

Hyphenated locale tags are normalized to underscores. If a full regional code is not implemented, `num2words2` tries the base language before raising `NotImplementedError`.

```python
num2words(42, lang="en-GB")
# falls back to 'en'
```

See [Supported Languages and Locales](Supported-Languages-and-Locales) for the full list and runtime discovery commands.

## Error Handling

Unsupported languages and conversion modes raise `NotImplementedError`.

```python
from num2words2 import num2words

def safe_num2words(value, lang):
    try:
        return num2words(value, lang=lang)
    except NotImplementedError:
        return num2words(value, lang="en")
```

Some language converters may raise `TypeError`, `ValueError`, or `ZeroDivisionError` for invalid inputs such as negative ordinals, non-integer ordinals, malformed fraction strings, or division by zero.

## Next Steps

- Use [API Reference](API-Reference) for conversion modes and keyword options.
- Use [CLI Reference](CLI-Reference) for shell usage.
- Use [Sentence Conversion](Sentence-Conversion) to convert numbers inside larger strings.

