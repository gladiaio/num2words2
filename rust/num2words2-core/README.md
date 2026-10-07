# num2words2-core

Number-to-words conversion for 120+ languages (170+ locale codes), in Rust.
This is the engine behind the [`num2words2`](https://pypi.org/project/num2words2/)
Python package, a port of [`num2words`](https://github.com/savoirfairelinux/num2words)
with byte-for-byte identical output.

```toml
[dependencies]
num2words2-core = "0.2"
num-bigint = "0.4"
```

```rust
use num2words2_core::{get_lang_by_key, to_words};
use num_bigint::BigInt;

let en = get_lang_by_key("en").expect("English is supported");
assert_eq!(en.to_cardinal(&BigInt::from(42)).unwrap(), "forty-two");
assert_eq!(en.to_ordinal(&BigInt::from(3)).unwrap(), "third");
assert_eq!(to_words(1999, "fr").unwrap(), "mille neuf cent quatre-vingt-dix-neuf");
```

Language keys are the Python `lang=` codes (`supported_lang_keys()` lists
them). Each language implements the `Lang` trait: `to_cardinal`,
`to_ordinal`, `to_ordinal_num`, `to_year`, `to_currency`, and the float
paths. API docs: <https://docs.rs/num2words2-core>.

## License

LGPL-2.1-only, like upstream num2words; see [`COPYING`](COPYING).
