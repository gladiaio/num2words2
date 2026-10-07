# Add a new language

Every conversion runs in the Rust core (`rust/num2words2-core`). The Python
package in this directory is a thin binder over the compiled `_rust`
extension: it holds no per-language code, so a new language is a Rust module
plus Python tests.

Below, `xx` stands for the ISO 639-1 or ISO 639-3
[language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

## 1. Write the converter: `rust/num2words2-core/src/lang_xx.rs`

Create a struct and implement the `Lang` trait from `base.rs` for it.
Every trait method has a default, so implement what your language needs:

- `to_cardinal(&self, value: &BigInt) -> Result<String>` (needed by every
  language). Either override it outright (a *self-contained* language), or
  provide `cards()`, `maxval()` and `merge()` and keep the default, which
  drives the shared split-and-merge engine (an *engine* language).
- `to_ordinal`, `to_ordinal_num`, `to_year` (they default to the cardinal,
  or the plain digits for `to_ordinal_num`).
- `negword()`, `pointword()` for negative numbers and decimals.
- `lang_name()`, `currency_forms()`, `pluralize()` and the other currency
  hooks if the language supports `to='currency'`, and `to_fraction()` for
  idiomatic fractions.

Start the file with a `//!` doc comment that says how numerals work in the
language and lists any known limitation. Existing modules of both shapes
(for example `lang_cy.rs` for a self-contained language) are the best
templates.

Extra options such as gender or grammatical case reach the converter through
the `*_kw` trait methods (`to_cardinal_kw`, `to_ordinal_kw`, ...) as a
`Kwargs` value; see how `lang_ru.rs` or `lang_he.rs` read them.

## 2. Register it: `rust/num2words2-core/src/lib.rs`

- add `pub mod lang_xx;` and `pub use lang_xx::LangXx;`;
- in `get_lang_by_key`, add a `static XX: OnceLock<LangXx>` and a match arm,
  `"xx" => Some(XX.get_or_init(LangXx::new)),` (list any alias keys in the
  same arm, e.g. `"xx" | "xx_YY"`);
- add `"xx"` (and each alias) to the sorted list in `supported_lang_keys()`.
  That list is what `num2words2 --list-languages` prints.

Then document the code in `REFERENCE.md` (Locale codes).

## 3. Test it: `tests/lang/test_xx.py`

Add Python tests that call the public API, for example
`num2words(42, lang='xx')`, `num2words(42, lang='xx', to='ordinal')`, and the
currency and year modes the language supports. Add Rust unit tests at the
bottom of `lang_xx.rs` for internal helpers if useful.

## 4. Build and validate

```bash
pip install maturin
pip install -r tests/requirements-test.txt
maturin develop --no-default-features   # rebuild after every Rust change
python -m pytest tests/
(cd rust && cargo test)
```

`tests/test_invariants.py` runs cross-language checks on every registered
code (no unexpected exceptions, no stray whitespace, no English words leaking
into the output, a consistent `maxval()`). A new language must pass all of
them: do not add it to an `ALLOW` list.
