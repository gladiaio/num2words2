# Migration from num2words

`num2words2` is designed as a drop-in replacement for the original `num2words` package in common usage.

## Minimal Change

```python
# Before
from num2words import num2words

# After
from num2words2 import num2words
```

Most calls keep the same shape:

```python
num2words(42)
num2words(42, lang="fr")
num2words(42, to="ordinal")
num2words(42.50, to="currency", currency="USD")
```

## Dependency Updates

`requirements.txt`:

```diff
- num2words
+ num2words2
```

`pyproject.toml`:

```diff
 dependencies = [
-    "num2words",
+    "num2words2",
 ]
```

`setup.py`:

```diff
 install_requires=[
-    "num2words",
+    "num2words2",
 ]
```

## Automated Migration Script

The repository includes a migration script:

```bash
curl -O https://raw.githubusercontent.com/gladiaio/num2words2/main/migration/migrate_to_num2words2.py

python migrate_to_num2words2.py --dry-run /path/to/project
python migrate_to_num2words2.py /path/to/project
```

Use `--dry-run` first so you can review what will change.

The script handles common import patterns:

| Before | After |
|---|---|
| `from num2words import num2words` | `from num2words2 import num2words` |
| `import num2words` | `import num2words2 as num2words` |
| `from num2words.lang_en import Num2Word_EN` | `from num2words2.lang_en import Num2Word_EN` |

## Gradual Migration

For applications that need to support both packages temporarily:

```python
try:
    from num2words2 import num2words
except ImportError:
    from num2words import num2words
```

## Behavior to Retest

Run tests around these areas after migration:

- decimal and float formatting
- negative values
- ordinal behavior in each language
- currency pluralization and subunits
- unsupported language fallback behavior
- any direct imports from language modules

`num2words2` adds behavior beyond the original package, including `cheque`, `fraction`, sentence conversion, expanded locale support, and aviation English.

