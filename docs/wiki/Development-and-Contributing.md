# Development and Contributing

## Repository

Development happens at:

https://github.com/gladiaio/num2words2

Open issues and pull requests against `gladiaio/num2words2`.

## Local Setup

```bash
git clone https://github.com/gladiaio/num2words2.git
cd num2words2

python -m pip install -e .
python -m pip install -r requirements-test.txt
```

The Makefile provides common workflows:

```bash
make help
make dev-install
make test
make test-coverage
make lint
make format
```

## Testing

Run the standard test suite:

```bash
make test
```

Run tox across configured Python versions:

```bash
tox
```

Run a quick stop-on-first-failure pass:

```bash
make test-quick
```

## Adding a Language

Typical steps:

1. Add a `num2words2/lang_XX.py` converter.
2. Register it in `CONVERTER_CLASSES` in `num2words2/__init__.py`.
3. Add tests under `tests/lang/test_xx.py`.
4. Verify cardinal behavior first, then ordinals, currency, years, and special options as applicable.
5. Run focused tests, then the full suite.

Keep behavior compatible with existing converter conventions. Raise `NotImplementedError` when a mode or currency code is not implemented.

## Adding or Fixing Currency

Currency behavior is language-sensitive. Update the converter's `CURRENCY_FORMS` or language-specific currency methods, then add tests for:

- singular and plural major units
- zero, one, and multiple subunits
- negative values
- fractional cents or 3-decimal currencies if relevant
- unsupported currency codes

## Pull Request Checklist

Before opening a PR:

- The branch merges cleanly.
- Unit tests pass.
- New behavior has focused tests.
- Formatting and import ordering match the project tooling.
- User-facing changes are reflected in README, `REFERENCE.md`, or the wiki when relevant.

## Release Readiness

The Makefile includes release-oriented targets:

```bash
make clean
make build
make check-build
make release-check
```

Only run release commands when preparing an actual package release.

