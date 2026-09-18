# num2words2 — API Reference

A single-page reference for everything `num2words2` exposes beyond the basic
`num2words(value)` call: every conversion mode, per-call option, locale alias,
utility function, and the aviation/ICAO subsystem.

> **TL;DR for users coming from upstream `num2words`**: the public API is
> compatible — the only required change is `from num2words2 import num2words`.
> This document covers the additions specific to the fork.

## Contents

- [Quick start](#quick-start)
- [Conversion modes (`to=`)](#conversion-modes-to)
  - [`cardinal`](#cardinal-default) (default)
  - [`ordinal`](#ordinal) and [`ordinal_num`](#ordinal_num)
  - [`year`](#year)
  - [`currency`](#currency) — including 3-decimal currencies
  - [`cheque`](#cheque)
  - [`fraction`](#fraction)
- [Per-call options](#per-call-options)
  - [`style=`](#style--ordinal--us-english-conventions)
  - [`precision=`](#precision--floating-point-precision-override)
  - [`cents=`](#cents--currency-subunit-control)
  - [`spaced=` / `decimal_word=`](#spaced--decimal_word--turkish)
  - [`gender=`](#gender--grammatical-gender)
  - [`case=`](#case--grammatical-case)
- [Aviation / ICAO English](#aviation--icao-english)
  - [Service profiles](#service-profiles)
  - [Phraseology methods](#phraseology-methods)
- [Utility functions](#utility-functions)
  - [`maxval(lang)`](#maxvallang)
  - [`group_digits(value, locale=)`](#group_digitsvalue-locale)
  - [`num2words_sentence(text, lang=)`](#num2words_sentencetext-lang)
- [Locale codes](#locale-codes)
- [String input semantics](#string-input-semantics)
- [Migrating per-feature from `num2words`](#migrating-per-feature-from-num2words)

---

## Quick start

```python
from num2words2 import num2words

num2words(42)                                  # 'forty-two'
num2words(42, lang='es')                       # 'cuarenta y dos'
num2words(42, to='ordinal')                    # 'forty-second'
num2words(2024, to='year')                     # 'twenty twenty-four'
num2words(42.50, to='currency', currency='USD')  # 'forty-two dollars, fifty cents'
num2words('1/3')                               # 'one third'
num2words(1234.56, to='cheque', currency='USD')
# 'ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND 56/100 DOLLARS'
```

---

## Conversion modes (`to=`)

### `cardinal` (default)

Plain spelled-out numbers.

```python
num2words(123)                  # 'one hundred and twenty-three'
num2words(123, lang='fr')       # 'cent vingt-trois'
num2words(-3.14)                # 'minus three point one four'
```

### `ordinal`

Word-form ordinals.

```python
num2words(1, to='ordinal')                  # 'first'
num2words(101, to='ordinal')                # 'one hundred and first'
num2words(1, to='ordinal', lang='es')       # 'primero'
num2words(1, to='ordinal', lang='fr')       # 'premier'
```

### `ordinal_num`

Numeric ordinals (digit + suffix).

```python
num2words(5, to='ordinal_num')               # '5th'
num2words(5, to='ordinal_num', lang='es')    # '5º'
num2words(5, to='ordinal_num', lang='fr')    # '5me'
num2words(5, to='ordinal_num', lang='en_Aero_ICAO')  # '5'  (no suffix in aviation)
num2words(2, to='ordinal_num', lang='tr')    # "2'nci"  (TDK convention)
num2words(6, to='ordinal_num', lang='tr')    # "6'ncı"  (vowel-harmony)
```

### `year`

Year-style reading, when distinct from cardinal in the language.

```python
num2words(1971, to='year')              # 'nineteen seventy-one'
num2words(2024, to='year')              # 'twenty twenty-four'
num2words(-44, to='year')               # 'forty-four BC'
num2words(2026, to='year', lang='nl')   # 'twintig zesentwintig'
```

### `currency`

Spelled-out monetary amounts. Supports 2-decimal default plus 3-decimal
"mil" currencies (BHD, KWD, OMR, JOD, TND, LYD, IQD).

```python
num2words(1234.56, to='currency', currency='USD')
# 'one thousand, two hundred and thirty-four dollars, fifty-six cents'

num2words(5.123, to='currency', currency='BHD')
# 'five dinars, one hundred and twenty-three fils'

num2words(0.001, to='currency', currency='TND')
# 'zero dinars, one millime'
```

Built-in currencies (`currency=`): EUR, USD, GBP, AUD, CAD, NZD, HKD, SGD,
CHF, AED, JPY, CNY, INR, KRW, MXN, BRL, ZAR, SAR, QAR, KWD, NGN, BHD, OMR,
JOD, TND, LYD, IQD. Other languages add their own (RUB, UAH, BGN, ...).

Optional `adjective=True` prefixes the currency adjective for languages that
have one (German "DM" → "Deutsche Mark", etc.):

```python
num2words(100, to='currency', currency='DEM', lang='de', adjective=True)
# 'einhundert Deutsche Mark'
```

### `cheque`

Bank-style cheque format — integer part as words, fractional part as
digits, currency name pluralised, whole result upper-cased.

```python
num2words(1234.56, to='cheque', currency='USD')
# 'ONE THOUSAND, TWO HUNDRED AND THIRTY-FOUR AND 56/100 DOLLARS'

num2words(1.00, to='cheque', currency='USD')
# 'ONE AND 00/100 DOLLARS'

num2words(1.234, to='cheque', currency='BHD')   # composes with 3-decimal
# 'ONE AND 234/1000 DINARS'
```

### `fraction`

Pass `'n/d'` strings to produce a spoken fraction. Idiomatic forms for
common denominators are implemented per language; everything else uses
the ordinal-as-noun pattern with the language's natural plural rule.

```python
num2words('1/3')                # 'one third'
num2words('1/2', lang='fr')     # 'un demi'
num2words('2/3', lang='it')     # 'due terzi'
num2words('3/4', lang='de')     # 'drei Viertel'
num2words('1/100', lang='de')   # 'ein Hundertstel'
num2words('-3/4')               # 'minus three quarters'
```

| Language | Idiomatic forms | Plural rule |
|---|---|---|
| `en` | half/quarter | -s |
| `fr` | demi / tiers (invariant) / quart | -s |
| `es` | medio / tercio / cuarto | -s; "un" for 1 |
| `it` | mezzo | -o → -i |
| `pt`, `pt_BR` | meio / terço | -s |
| `de` | halb + Drittel/Viertel/...el | invariant; "ein" for 1 |
| others | ordinal + "s" (fallback) | — |

Edge cases: `'0/3'` → "zero", `'1/1'` → "one", `'1/0'` → `ZeroDivisionError`,
`'1/-3'` and `'-1/3'` both → "minus one third".

---

## Per-call options

Every option below is passed as a keyword argument on `num2words(...)`.

### `style=` — ordinal / US-English conventions

Two named styles affect string shape:

```python
# style='terse' on ordinals drops the leading 'one' / 'un' / 'uno'.
num2words(100, to='ordinal', lang='en', style='terse')   # 'hundredth' (default: 'one hundredth')
num2words(1000, to='ordinal', lang='en', style='terse')  # 'thousandth'

# style='us' on English drops the conjunction 'and'.
num2words(1234, lang='en')                  # 'one thousand, two hundred and thirty-four'
num2words(1234, lang='en', style='us')      # 'one thousand, two hundred thirty-four'
```

### `precision=` — floating-point precision override

Override how many fractional digits to read. Useful when the language's
default doesn't match your input:

```python
num2words(3.14159, lang='en', precision=5)
# 'three point one four one five nine'

num2words(3.14159, lang='tr', precision=5)
# 'üçvirgülondörtbinyüzellidokuz'
```

### `cents=` — currency subunit control

```python
num2words(5, to='currency', currency='USD', cents='omit')
# 'five dollars'                                  (drops the cents segment)

num2words(5.99, to='currency', currency='USD', cents='terse')
# 'five dollars, 99 cents'                        (digits, not words)

num2words(5.99, to='currency', currency='USD', cents='verbose')
# 'five dollars, ninety-nine cents'               (default)
```

The legacy `cents=False` is kept; it means "use digits not words" (same as
`cents='terse'`). `cents=True` means "use words" (same as `cents='verbose'`).

### `spaced=` / `decimal_word=` — Turkish

Turkish-specific kwargs. `spaced=True` re-tokenises the concatenated
output with spaces; `decimal_word=` replaces the `virgül` separator.

```python
num2words(1234, lang='tr')                          # 'binikiyüzotuzdört'
num2words(1234, lang='tr', spaced=True)             # 'bin iki yüz otuz dört'

num2words(1.5, lang='tr', decimal_word='nokta')     # 'birnoktabeş'
```

### `gender=` — grammatical gender

For languages with grammatical gender (Hebrew, Russian, German), pass
`'m'` / `'f'` / `'n'` to select the form.

```python
num2words(1, lang='he', gender='m')   # 'אחד'
num2words(1, lang='he', gender='f')   # 'אחת'
num2words(1, lang='ru', gender='m')   # 'один'
num2words(1, lang='ru', gender='f')   # 'одна'
num2words(1, lang='ru', gender='n')   # 'одно'
```

### `case=` — grammatical case

Russian, Arabic, and other inflected languages accept a `case=` kwarg.

```python
num2words(1, lang='ru', case='nominative')      # 'один'
num2words(1, lang='ru', case='genitive')        # 'одного'
num2words(1, lang='ru', case='dative')          # 'одному'
num2words(1, lang='ru', case='accusative')      # 'один' / 'одного' (animacy-dependent)
num2words(1, lang='ru', case='instrumental')    # 'одним'
num2words(1, lang='ru', case='prepositional')   # 'одном'

num2words(200, lang='ar', case='nominative')    # 'مئتان'
num2words(200, lang='ar', case='accusative')    # 'مئتين'
```

---

## Aviation / ICAO English

A digit-by-digit reading mode following ICAO Annex 10 vol II, FAA AIM 4-2-9,
and SKYbrary phraseology. Use it for voice-radio applications, air-traffic
control, or any context where unambiguous transmission over a noisy
channel matters.

```python
num2words(5739, lang='en_Aero_ICAO')        # 'fife seven tree niner'
num2words(127.5, lang='en_Aero_ICAO')       # 'wun too seven decimal fife'
num2words(1971, lang='en_Aero_ICAO', to='year')  # 'wun niner seven wun'
```

The strict ICAO digit table:

| Digit | Spoken |
|---|---|
| 0 | zero |
| 1 | wun |
| 2 | too |
| 3 | tree |
| 4 | fower |
| 5 | fife |
| 6 | six |
| 7 | seven |
| 8 | ait |
| 9 | niner |

Ordinals, fractions, currency, and cheque modes on `en_Aero_ICAO`
delegate to plain English so output stays readable (`"one third"`, not
`"wun treeth"`).

### Service profiles

| Locale code | Class | Profile |
|---|---|---|
| `en_Aero_ICAO` | `Num2Word_EN_AERO` | ICAO |
| `en_Aero_FAA` | `Num2Word_EN_AERO_FAA` | FAA |
| `en_Aero_USN` | `Num2Word_EN_AERO_USN` | US Navy |
| `en_Aero_US_Navy` | (alias for USN) | US Navy |
| `en_Aero_US_Army` | `Num2Word_EN_AERO_US_Army` | US Army |
| `en_Aero_NATO` | `Num2Word_EN_AERO_NATO` | NATO STANAG 1059 |

Modern services have all converged on the ICAO digit table for joint
operations, so today **all six produce identical output**. They exist as
separate named entry points so callers can document *which* standard
they're targeting, and so future divergent variants (historical WW2-era
US Navy, ITU/IMO maritime, tactical service deviations) can be added
without breaking back-compat.

Programmatic profile selection:

```python
from num2words2.lang_EN_AERO import Num2Word_EN_AERO
Num2Word_EN_AERO(profile='FAA').to_cardinal(5739)   # 'fife seven tree niner'
Num2Word_EN_AERO(profile='Klingon')                 # ValueError
```

Back-compat aliases from v1.0.14: `en_AERO`, `en_aero`, `en-AERO`,
`en-aero`, `en-x-aero-icao` (BCP 47 private-use form).

### Phraseology methods

These are aviation-only methods on the `Num2Word_EN_AERO` family, called
directly via the converter instance (they don't fit the language-agnostic
`to=` contract because their inputs and validation rules are
domain-specific):

```python
from num2words2 import CONVERTER_CLASSES
aero = CONVERTER_CLASSES['en_Aero_ICAO']
```

| Method | Example | Output |
|---|---|---|
| `to_altitude(value, unit='feet')` | `aero.to_altitude(5500)` | `'fife thousand fife hundred feet'` |
| `to_altitude(value)` (≥ 10 000 ft) | `aero.to_altitude(12500)` | `'wun too thousand fife hundred feet'` |
| `to_flight_level(value)` | `aero.to_flight_level(230)` | `'flight level too tree zero'` |
| `to_heading(value)` | `aero.to_heading(30)` | `'heading zero tree zero'` |
| `to_squawk(value)` | `aero.to_squawk(7700)` | `'squawk seven seven zero zero'` |
| `to_runway(value)` | `aero.to_runway('27R')` | `'runway too seven right'` |
| `to_runway(value)` | `aero.to_runway('09L')` | `'runway zero niner left'` |
| `to_frequency(value)` | `aero.to_frequency(121.5)` | `'wun too wun decimal fife'` |
| `to_frequency(value)` | `aero.to_frequency(118.025)` | `'wun wun ait decimal zero too fife'` |

Conventions implemented:

- **Altitude.** Below 10 000 ft, thousands portion is a single ICAO digit
  + "thousand"; at 10 000 and above, thousands are read digit-by-digit
  (FAA AIM 4-2-9). Hundreds are always single digit + "hundred".
- **Flight level / heading / squawk.** Always three or four zero-padded
  digits read individually.
- **Heading 0** maps to **360** (north convention).
- **Squawk** validates the 4-digit octal range (0–7777).
- **Runway suffixes** L / R / C are spoken as "left" / "right" / "center"
  (case-insensitive).
- **Frequency** uses "decimal" for the radio mark; pass a string to
  preserve trailing zeros that float literals lose.

---

## Utility functions

### `maxval(lang)`

Maximum integer the given language's converter can handle. Useful for
input validation before calling `num2words()`.

```python
from num2words2 import maxval

maxval('en')   # 10**3003 - 1
maxval('tr')   # 10**21 - 1
maxval('fr')   # 10**18 - 1
```

Accepts the same locale-fallback rules as `num2words` (case
normalisation, hyphen-to-underscore, two-letter prefix). Raises
`NotImplementedError` if no converter matches.

### `group_digits(value, locale=)`

Format a number with locale-appropriate digit grouping.

```python
from num2words2 import group_digits

group_digits(1234567)                       # '1,234,567'  (Western, default)
group_digits(1234567, locale='indian')      # '12,34,567'  (Indian #,##,##,###)
group_digits(1234567, locale='chinese')     # '123,4567'   (4-digit Chinese)
group_digits(1234567, locale='western', separator=' ')  # '1 234 567'
```

### `num2words_sentence(text, lang=)`

Convert all numeric tokens inside a sentence in place, leaving non-numeric
text untouched.

```python
from num2words2 import num2words_sentence

num2words_sentence("I bought 6 apples for $4.50")
# 'I bought six apples for four dollars, fifty cents'

num2words_sentence("Prend la 3e à droite", lang='fr')
# 'Prend la troisième à droite'
```

This is also what plain `num2words("text 6")` falls back to when the
input has digits mixed with non-numeric characters.

---

## Locale codes

`num2words2` registers **170+ locale keys** including aliases. Pass any of
them as `lang=`. Unknown locales fall back to the two-letter ISO 639-1
prefix where possible.

### Major languages

`af` Afrikaans, `am` Amharic, `ar` Arabic, `as` Assamese, `az` Azerbaijani,
`be` Belarusian, `bg` Bulgarian, `bn` Bengali, `bs` Bosnian, `ca` Catalan,
`ce` Chechen, `cs` Czech, `cy` Welsh, `da` Danish, `de` German, `dv` Divehi,
`el` Greek, `en` English, `eo` Esperanto, `es` Spanish, `et` Estonian,
`eu` Basque, `fa` Persian, `fi` Finnish, `fil` Filipino, `fr` French,
`gl` Galician, `gu` Gujarati, `ha` Hausa, `he` Hebrew, `hi` Hindi,
`hr` Croatian, `hu` Hungarian, `hy` Armenian, `id` Indonesian, `is` Icelandic,
`it` Italian, `ja` Japanese, `ka` Georgian, `kk` Kazakh, `km` Khmer,
`kn` Kannada, `ko` Korean, `ku` Kurdish, `ky` Kyrgyz, `la` Latin, `lb` Luxembourgish,
`lo` Lao, `lt` Lithuanian, `lv` Latvian, `mg` Malagasy, `mi` Maori,
`mk` Macedonian, `ml` Malayalam, `mn` Mongolian, `mr` Marathi, `ms` Malay,
`mt` Maltese, `my` Burmese, `ne` Nepali, `nl` Dutch, `no` Norwegian,
`om` Oromo, `or` Odia, `pa` Punjabi, `pl` Polish, `ps` Pashto, `pt` Portuguese,
`rm` Romansh, `ro` Romanian, `ru` Russian, `rw` Kinyarwanda, `sa` Sanskrit,
`sd` Sindhi, `si` Sinhala, `sk` Slovak, `sl` Slovenian, `sn` Shona,
`so` Somali, `sq` Albanian, `sr` Serbian, `sv` Swedish, `sw` Swahili,
`ta` Tamil, `te` Telugu, `tet` Tetum, `tg` Tajik, `th` Thai, `ti` Tigrinya,
`tk` Turkmen, `tl` Tagalog, `tr` Turkish, `tt` Tatar, `uk` Ukrainian,
`ur` Urdu, `uz` Uzbek, `vi` Vietnamese, `wo` Wolof, `xh` Xhosa, `yi` Yiddish,
`yo` Yoruba, `zh` Chinese, `zu` Zulu.

### Regional variants

| Code | Notes |
|---|---|
| `en_IN` | Indian English (lakh / crore numbering) |
| `en_NE`, `en_NG`, `en_NP` | Nepalese / Nigerian / Nepali English |
| `en_Aero_*` | Aviation/ICAO family — see [Aviation section](#aviation--icao-english) |
| `es_CO`, `es_CR`, `es_GT`, `es_HN`, `es_NI`, `es_VE` | Spanish regional (Latin America) |
| `fr_BE`, `fr_CH`, `fr_DZ` | Belgian / Swiss / Algerian French |
| `pt_BR` | Brazilian Portuguese |
| `rm_puter`, `rm_surmiran`, `rm_sursilv`, `rm_sutsilv`, `rm_vallader` | Romansh idioms |
| `sr_Cyrl`, `sr_Latn` | Serbian Cyrillic / Latin (Gaj's transliteration) |
| `uz_Cyrl` | Uzbek Cyrillic |
| `zh_CN`, `zh_HK`, `zh_TW` | Mainland / Hong Kong / Taiwan Chinese |

### Aliases

The dispatcher normalises hyphens to underscores and tries case-folded
fallbacks. These explicit aliases are also registered:

| Alias | Canonical | Reason |
|---|---|---|
| `cz` | `cs` | Pre-ISO-639 country code for Czech |
| `dk` | `da` | Pre-ISO-639 country code for Danish |
| `cn` | `zh_CN` | Country-code shorthand |
| `jp` | `ja` | Country-code shorthand |
| `jw` | `jv` | Old code for Javanese |
| `miz` | `lus` | Old code for Mizo |
| `nb` | `no` | Norwegian Bokmål → Norwegian |
| `uz_cyr` | `uz_Cyrl` | Lowercase variant |
| `en_AERO`, `en_aero_icao`, `en_x_aero_icao` | `en_Aero_ICAO` | Aviation aliases |

Run `num2words2 --list-languages` from the CLI for the live list.

---

## String input semantics

`num2words` accepts string input in addition to numeric. The dispatcher
applies the following rules in order:

1. **Fraction pattern (`'n/d'`)** — routed directly to `to_fraction(n, d)`.
   Whitespace and signs on either side are tolerated.
   ```python
   num2words('1/3')        # 'one third'
   num2words('-3/4')       # 'minus three quarters'
   num2words(' 1 / 3 ')    # 'one third'
   ```

2. **Numeric string** — parsed to `Decimal` via `str_to_number()` so trailing
   zeros and trillion-scale precision survive the round-trip:
   ```python
   num2words('98746251323029.99')      # exact, no IEEE 754 loss
   num2words('1.50', lang='tr')        # 'birvirgülelli' (preserves trailing zero)
   num2words('0.50')                   # 'zero point five zero'
   ```

3. **Mixed text** (any non-numeric character + at least one digit) — routed
   to `num2words_sentence()` which converts numerals in place:
   ```python
   num2words('I have 6 apples')
   # 'I have six apples'
   ```

4. **Pure non-numeric** — raises `decimal.InvalidOperation`.

---

## Migrating per-feature from `num2words`

Most of the additions are non-breaking, but two are worth flagging:

- **String-fraction routing** — passing `'1/3'` to upstream `num2words`
  raises `decimal.InvalidOperation`; in num2words2 it returns "one third".
  If your code intentionally relied on the exception to validate input,
  add a fraction guard before the call.

- **Turkish natural-precision floats (v1.0.15+)** — `num2words(0.1, lang='tr')`
  used to return `'sıfırvirgülon'` ("zero point ten") and now returns
  `'sıfırvirgülbir'` ("zero point one") to match how Turkish speakers
  actually read short decimals. To get the v1.0.14 padded behaviour
  back, pass `precision=2` explicitly or use a string with the trailing
  zero (`'0.10'`).

Everything else is additive: new modes (`cheque`, `fraction`), new
locales (`en_Aero_*`), new kwargs (`style=`, `precision=`, `cents=`,
`spaced=`, `decimal_word=`, `gender=`, `case=`), new utility functions
(`maxval`, `group_digits`, `num2words_sentence`).

---

*Last updated for v1.0.17. Open an issue at
[gladiaio/num2words2](https://github.com/gladiaio/num2words2/issues)
if anything's missing or wrong.*
