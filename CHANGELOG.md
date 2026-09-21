# Changelog

Release notes for tagged versions are generated on the
[GitHub Releases](https://github.com/gladiaio/num2words2/releases) page.
This file carries the human-written notes that the auto-generated ones do not:
in particular **credit for upstream work ported into this project**.

`num2words2` is not a divergent fork — it is a Rust port of the `num2words`
engine. Fixes accepted upstream at
[savoirfairelinux/num2words](https://github.com/savoirfairelinux/num2words)
are reviewed and ported here. When that happens the upstream PR and its author
are credited below, and in the PR title so the credit reaches the generated
release notes too.

## Unreleased

### Ported from upstream

- **Arabic and Hindi: reject negative and fractional ordinals.**
  Ports [savoirfairelinux/num2words#672][672] by [@santhreal][santhreal].
  `to_ordinal` in Arabic indexed its ordinal table with a negative and
  returned a silently wrong word (`-1` → "إحدى", `-100` → "مائة"); Hindi's
  `to_ordinal_num` mapped the value through a digit table and raised
  `KeyError: "'-'"` on the sign and `KeyError: "'.'"` on a decimal point.
  Both now call `verify_ordinal` first and raise `TypeError`, matching every
  other language. Hindi word ordinals still accept negatives
  (`to_ordinal(-1)` → "माइनस एकवाँ"), as upstream's own test requires.

[672]: https://github.com/savoirfairelinux/num2words/pull/672
[santhreal]: https://github.com/santhreal
