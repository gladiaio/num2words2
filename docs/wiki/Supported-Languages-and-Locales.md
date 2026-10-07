# Supported Languages and Locales

The current converter registry includes 172 language and locale codes. The exact runtime list is available with:

```bash
num2words2 --list-languages
```

## Locale Normalization

`num2words2` normalizes hyphens to underscores and tries regional fallbacks:

```python
num2words(42, lang="pt-BR")  # uses pt_BR
num2words(42, lang="fr-FR")  # falls back to fr
num2words(42, lang="en-GB")  # falls back to en
```

If no full code or base-language fallback is available, `NotImplementedError` is raised.

## Codes

```text
af am ar as az ba ban be bg bm bn bo br bs
ca ce ceb ckb cnh cn cs cy cz da de dk dv
el en en_AERO en_Aero_FAA en_Aero_ICAO en_Aero_NATO
en_Aero_USN en_Aero_US_Army en_Aero_US_Navy en_IN en_NE
en_NG en_NP en_aero_icao en_x_aero_icao eo es es_CO
es_CR es_GT es_HN es_NI es_VE et eu fa ff fi fil fo
fr fr_BE fr_CH fr_DZ gl gu ha haw he hi hmn hr ht hu
hy id is it ja jp jv jw ka ki kk km kn ko kok ksw ku
ky kz la lb lg lij ln lo lt lus lv mg mi miz mk ml mn
mr ms mt my nb ne nl nn no oc om or pa pap pl pli ps
pt pt_BR rm rm_puter rm_surmiran rm_sursilv rm_sutsilv
rm_vallader ro ru rw sa sd si sk sl sn so sq sr sr_Cyrl
sr_Latn su sv sw ta te tet tg th ti tk tl tr tt uk ur
uz uz_Cyrl uz_cyr vi wo xh yi yo zh zh_CN zh_HK zh_TW zu
```

## Common Regional Variants

| Family | Codes |
|---|---|
| English | `en`, `en_IN`, `en_NE`, `en_NG`, `en_NP` |
| Aviation English | `en_Aero_ICAO`, `en_Aero_FAA`, `en_Aero_NATO`, `en_Aero_USN`, `en_Aero_US_Navy`, `en_Aero_US_Army` |
| Spanish | `es`, `es_CO`, `es_CR`, `es_GT`, `es_HN`, `es_NI`, `es_VE` |
| French | `fr`, `fr_BE`, `fr_CH`, `fr_DZ` |
| Portuguese | `pt`, `pt_BR` |
| Romansh | `rm`, `rm_puter`, `rm_surmiran`, `rm_sursilv`, `rm_sutsilv`, `rm_vallader` |
| Serbian | `sr`, `sr_Cyrl`, `sr_Latn` |
| Uzbek | `uz`, `uz_Cyrl`, `uz_cyr` |
| Chinese | `zh`, `zh_CN`, `zh_HK`, `zh_TW` |

## Aliases

Some aliases are provided for compatibility or convenience:

| Alias | Target |
|---|---|
| `jp` | `ja` |
| `cn` | `zh_CN` |
| `cz` | `cs` |
| `dk` | `da` |
| `jv`, `jw` | Javanese converter aliases |
| `en_AERO`, `en_aero_icao`, `en_x_aero_icao` | ICAO aviation English |

Alias behavior can change as locale support grows, so prefer the canonical code shown in API examples when writing new code.

