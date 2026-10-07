# API Reference

## Public Imports

```python
from num2words2 import (
    num2words,
    num2words_sentence,
    convert_sentence,
    sentence_to_words,
)
```

`convert_sentence` and `sentence_to_words` are aliases for `num2words_sentence`.

## `num2words`

```python
num2words(number, ordinal=False, lang="en", to="cardinal", **kwargs)
```

Arguments:

- `number`: `int`, `float`, `Decimal`, numeric string, or fraction string in strict `n/d` form.
- `ordinal`: backward-compatible shortcut. If true, it behaves like `to="ordinal"`.
- `lang`: language or locale code. Defaults to `en`.
- `to`: conversion mode. Defaults to `cardinal`.
- `**kwargs`: language-specific or mode-specific options.

## Conversion Modes

### `cardinal`

Plain number words.

```python
num2words(123)
# 'one hundred and twenty-three'

num2words(-3.14)
# 'minus three point one four'
```

### `ordinal`

Word-form ordinal numbers.

```python
num2words(1, to="ordinal")
# 'first'

num2words(101, to="ordinal")
# 'one hundred and first'
```

### `ordinal_num`

Digit plus ordinal suffix, where the language supports it.

```python
num2words(5, to="ordinal_num")
# '5th'

num2words(5, to="ordinal_num", lang="es")
# '5º'
```

### `year`

Year-style reading when the language has behavior distinct from cardinal numbers.

```python
num2words(1971, to="year")
# 'nineteen seventy-one'

num2words(2024, to="year")
# 'twenty twenty-four'
```

### `currency`

Spelled-out monetary values.

```python
num2words(1234.56, to="currency", currency="USD")
# 'one thousand, two hundred and thirty-four dollars, fifty-six cents'
```

See [Currency, Cheques, and Fractions](Currency-Cheques-and-Fractions).

### `cheque`

Bank-cheque style output with uppercase words and fractional digits.

```python
num2words(1234.56, to="cheque", currency="USD")
# 'ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND 56/100 DOLLARS'
```

### `fraction`

Fraction mode is usually selected automatically when `number` is a strict fraction string.

```python
num2words("1/2")
# 'one half'

num2words("2/3")
# 'two thirds'
```

## Common Keyword Options

### `currency=`

Selects the ISO-like currency code for `currency` or `cheque` output.

```python
num2words(42.50, to="currency", currency="EUR")
num2words(42.50, to="currency", currency="USD")
```

Supported codes vary by language. The shared base includes common currencies such as `EUR`, `USD`, `GBP`, `AUD`, `CAD`, `CHF`, `JPY`, `CNY`, `INR`, `KRW`, `BRL`, and 3-decimal currencies such as `BHD`, `KWD`, `OMR`, `JOD`, `TND`, `LYD`, and `IQD`.

### `cents=`

Controls subunit output for currency.

```python
num2words(5.99, to="currency", currency="USD", cents=True)
# verbose cents

num2words(5.99, to="currency", currency="USD", cents=False)
# terse digit cents in converters that support terse output
```

Current converters also recognize string forms such as `cents="verbose"`, `cents="terse"`, and `cents="omit"` where implemented.

### `separator=`

Overrides the separator between currency units and subunits.

```python
num2words(5.99, to="currency", currency="USD", separator=" and ")
```

### `adjective=`

Some language/currency combinations can prefix an adjective form of the currency.

```python
num2words(100, to="currency", currency="DEM", lang="de", adjective=True)
```

### `precision=`

Overrides the number of fractional digits read for decimal values.

```python
num2words(3.14159, precision=5)
# 'three point one four one five nine'
```

### `style=`

Some converters support named style variants.

```python
num2words(1234, lang="en", style="us")
# 'one thousand, two hundred thirty-four'

num2words(100, to="ordinal", lang="en", style="terse")
# 'hundredth'
```

### `gender=` and `case=`

Inflected languages may accept grammatical gender or case options.

```python
num2words(1, lang="ru", gender="f")
num2words(1, lang="ru", case="genitive")
```

Support is language-specific. If a converter does not implement an option, it may ignore it or raise an error.

## `num2words_sentence`

```python
num2words_sentence(sentence, lang="en", to="cardinal", **kwargs)
```

Converts numbers inside a sentence while preserving the surrounding text. It handles regular numbers, ordinals, dates, years, temperatures, and common currency symbols.

```python
num2words_sentence("I bought 6 apples on April 5, 2024 for $12.50.")
# 'I bought six apples on April fifth, twenty twenty-four for twelve dollars, fifty cents.'
```

See [Sentence Conversion](Sentence-Conversion).

## Runtime Discovery

```python
from num2words2 import CONVERTER_TYPES

sorted(CONVERTER_TYPES)  # every `to=` value
```

There is no public table of converter classes: every conversion runs in the
Rust core. The CLI lists the language codes and the `to=` values:

```bash
num2words2 --list-languages
num2words2 --list-converters
```

