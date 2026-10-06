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
python -m pip install -r tests/requirements-test.txt
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

1. Add a `rust/num2words2-core/src/lang_xx.rs` module implementing the `Lang` trait (`base.rs`).
2. Register it in `rust/num2words2-core/src/lib.rs`: `pub mod`/`pub use`, a match arm in `get_lang_by_key`, and the code in `supported_lang_keys()`.
3. Add tests under `tests/lang/test_xx.py`, then rebuild with `maturin develop --no-default-features`.
4. Verify cardinal behavior first, then ordinals, currency, years, and special options as applicable.
5. Run focused tests, then the full suite.

Keep behavior compatible with existing converter conventions. Raise `NotImplementedError` when a mode or currency code is not implemented.

## Adding or Fixing Currency

Currency behavior is language-sensitive. Update the language's `currency_forms()` table or its currency hooks in `lang_xx.rs`, then add tests for:

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

