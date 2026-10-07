# CLI Reference

The package installs a `num2words2` command.

```bash
num2words2 [options] <number>
```

## Examples

```bash
num2words2 10001
# ten thousand and one

num2words2 24120.10
# twenty-four thousand, one hundred and twenty point one zero

num2words2 24120.10 --lang es
# veinticuatro mil ciento veinte punto uno cero

num2words2 2.14 --lang es --to currency
# dos euros con catorce céntimos
```

The command also supports module execution:

```bash
python -m num2words2 42 --lang fr
```

## Options

| Option | Description |
|---|---|
| `<number>` | Number or string value to convert |
| `-l`, `--lang <code>` | Output language. Default: `en` |
| `--to <mode>` | Conversion mode. The legacy script entrypoint also accepts `-t` |
| `--list-languages` | Print supported language and locale codes |
| `--list-converters` | Print supported conversion modes |
| `-h`, `--help` | Show command help |
| `-v`, `--version` | Show script version when using the legacy script entrypoint |

## Conversion Modes

```bash
num2words2 --list-converters
```

Current modes:

```text
cardinal
cheque
currency
fraction
ordinal
ordinal_num
year
```

## Notes

- Quote shell-sensitive values such as fractions: `num2words2 '1/3'`.
- For locale codes with hyphens, quote only when your shell requires it: `num2words2 42 --lang pt-BR`.
- Use the Python API when you need keyword options such as `currency=`, `precision=`, `style=`, `gender=`, or `case=`.
