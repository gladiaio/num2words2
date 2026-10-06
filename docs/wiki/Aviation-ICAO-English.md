# Aviation and ICAO English

`num2words2` includes aviation English language codes that read numbers digit by digit with the ICAO radiotelephony digits.

## Recommended Codes

| Code | Profile |
|---|---|
| `en_Aero_ICAO` | ICAO |
| `en_Aero_FAA` | FAA |
| `en_Aero_NATO` | NATO STANAG 1059 |
| `en_Aero_USN` | US Navy |
| `en_Aero_US_Navy` | US Navy alias |
| `en_Aero_US_Army` | US Army |

All six codes produce identical output today. Compatibility aliases include `en_AERO`, `en_aero_icao`, and `en_x_aero_icao`.

## Basic Conversion

```python
from num2words2 import num2words

num2words(5739, lang="en_Aero_ICAO")
# 'fife seven tree niner'
```

In aviation profiles, cardinal values are typically read digit by digit. Ordinals, fractions, currency, and cheque modes fall back to compatible English behavior where available.

## Phraseology

The aviation support is these language codes only. There are no phraseology
helpers (altitude, flight level, heading, squawk, runway, frequency) and no
public converter classes; compose such phrases from `num2words()` calls:

```python
"squawk " + num2words(7700, lang="en_Aero_ICAO")
# 'squawk seven seven zero zero'

num2words(121.5, lang="en_Aero_ICAO")
# 'wun too wun decimal fife'
```
