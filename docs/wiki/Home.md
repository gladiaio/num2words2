# num2words2 Wiki

`num2words2` converts numbers into words for multilingual applications. It is a modern fork of `num2words` with broader locale coverage, active maintenance, sentence-level conversion helpers, currency handling, cheque formatting, fractions, and aviation/ICAO English phraseology.

The package is useful for text normalization, speech systems, LLM preprocessing, document generation, search indexing, and any workflow that needs stable human-readable numbers.

## Quick Example

```python
from num2words2 import num2words, num2words_sentence

num2words(42)
# 'forty-two'

num2words(42, lang="fr")
# 'quarante-deux'

num2words(42, to="ordinal")
# 'forty-second'

num2words(42.50, to="currency", currency="USD")
# 'forty-two dollars, fifty cents'

num2words("1/3")
# 'one third'

num2words_sentence("I bought 6 apples for $12.50.")
# 'I bought six apples for twelve dollars, fifty cents.'
```

## Documentation

- [Getting Started](Getting-Started)
- [API Reference](API-Reference)
- [CLI Reference](CLI-Reference)
- [Supported Languages and Locales](Supported-Languages-and-Locales)
- [Sentence Conversion](Sentence-Conversion)
- [Currency, Cheques, and Fractions](Currency-Cheques-and-Fractions)
- [Aviation and ICAO English](Aviation-ICAO-English)
- [Migration from num2words](Migration-from-num2words)
- [Development and Contributing](Development-and-Contributing)

## Project Links

- Repository: https://github.com/gladiaio/num2words2
- PyPI: https://pypi.org/project/num2words2/
- Issues: https://github.com/gladiaio/num2words2/issues
- Pull requests: https://github.com/gladiaio/num2words2/pulls

