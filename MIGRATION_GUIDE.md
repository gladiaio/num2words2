# Migration Guide: num2words → num2words2

Complete guide for migrating from the original `num2words` library to `num2words2`.

## Why Migrate?

- **Active maintenance** - Regular updates and bug fixes
- **AI/LLM optimization** - Better support for modern applications
- **Enhanced language support** - More languages and improved implementations
- **Better reliability** - Critical bug fixes for production use
- **Future-proof** - Aligned with evolving AI/ML ecosystem

## Quick Start (TL;DR)

```bash
# 1. Run migration script
curl -O https://raw.githubusercontent.com/jqueguiner/num2words/master/migrate_to_num2words2.py
python migrate_to_num2words2.py --dry-run .  # Preview changes
python migrate_to_num2words2.py .            # Apply changes

# 2. Install new package
pip install num2words2
pip uninstall num2words  # Optional: remove old package

# 3. Update requirements.txt
# Replace "num2words" with "num2words2"
```

## Detailed Migration Steps

### Step 1: Assessment

First, check what needs to be migrated in your project:

```bash
# Find all files with num2words imports
grep -r "num2words" --include="*.py" .

# Check requirements files
grep -r "num2words" requirements*.txt setup.py pyproject.toml
```

### Step 2: Automated Migration

Use our migration script for Python code:

```bash
# Download the script
curl -O https://raw.githubusercontent.com/jqueguiner/num2words/master/migrate_to_num2words2.py

# Preview changes (recommended)
python migrate_to_num2words2.py --dry-run /path/to/your/project

# Apply changes
python migrate_to_num2words2.py /path/to/your/project
```

The script handles these import patterns:

| Before | After |
|--------|-------|
| `from num2words import num2words` | `from num2words2 import num2words` |
| `import num2words` | `import num2words2 as num2words` |
| `from num2words.lang_en import Num2Word_EN` | *(left unchanged, with a warning — see Scenario 3)* |

### Step 3: Manual Updates

#### Requirements Files

Update your dependency declarations:

**requirements.txt:**
```diff
- num2words>=0.5.12
+ num2words2>=0.5.15
```

**pyproject.toml:**
```diff
dependencies = [
-    "num2words>=0.5.12",
+    "num2words2>=0.5.15",
]
```

**setup.py:**
```diff
install_requires=[
-    "num2words>=0.5.12",
+    "num2words2>=0.5.15",
]
```

#### Docker Files

Update Dockerfiles:

```diff
- RUN pip install num2words
+ RUN pip install num2words2
```

#### CI/CD Configurations

Update CI configuration files:

**.github/workflows/test.yml:**
```diff
- pip install num2words
+ pip install num2words2
```

**tox.ini:**
```diff
deps =
-    num2words
+    num2words2
```

### Step 4: Testing

Verify your migration works correctly:

```python
# test_migration.py
from num2words2 import num2words

def test_basic_functionality():
    """Test basic num2words2 functionality"""
    # Cardinal numbers
    assert num2words(42) == "forty-two"
    assert num2words(1000) == "one thousand"

    # Different languages
    assert num2words(42, lang='es') == "cuarenta y dos"
    assert num2words(42, lang='fr') == "quarante-deux"

    # Ordinal numbers
    assert num2words(42, to='ordinal') == "forty-second"

    # Currency
    result = num2words(42.50, to='currency', lang='en')
    assert 'forty' in result and 'fifty' in result

def test_edge_cases():
    """Test edge cases and bug fixes"""
    # Test negative numbers (fixed in num2words2)
    assert num2words(-1, lang='bn') == 'ঋণাত্মক এক'

    # Test float handling (improved in num2words2)
    assert num2words(1.5, lang='en') == 'one point five'

if __name__ == "__main__":
    test_basic_functionality()
    test_edge_cases()
    print("✅ All tests passed! Migration successful.")
```

Run the test:
```bash
python test_migration.py
```

## Common Migration Scenarios

### Scenario 1: Simple Import Replacement

**Before:**
```python
from num2words import num2words

def convert_number(n):
    return num2words(n)
```

**After:**
```python
from num2words2 import num2words

def convert_number(n):
    return num2words(n)
```

### Scenario 2: Direct Module Import

**Before:**
```python
import num2words

def convert_number(n, lang='en'):
    return num2words.num2words(n, lang=lang)
```

**After:**
```python
import num2words2 as num2words

def convert_number(n, lang='en'):
    return num2words.num2words(n, lang=lang)
```

### Scenario 3: Converter Classes (`num2words.lang_*`)

`num2words2` runs every conversion in its Rust core and does **not** ship the
`lang_*` modules or the `Num2Word_*` converter classes, and it has no
`CONVERTER_CLASSES` table. The migration script leaves these imports alone
and prints a warning for each one. Rewrite them to the public API:

**Before:**
```python
from num2words.lang_en import Num2Word_EN

converter = Num2Word_EN()
converter.to_ordinal(42)
```

**After:**
```python
from num2words2 import num2words

num2words(42, lang='en', to='ordinal')
```

Every `to_<mode>()` method maps to `num2words(..., to='<mode>')`. To list the
supported language codes, run `num2words2 --list-languages`.

### Scenario 4: Gradual Migration

For large codebases, you can migrate gradually:

```python
# compatibility.py - Create a compatibility layer
try:
    from num2words2 import num2words
    print("Using num2words2")
except ImportError:
    from num2words import num2words
    print("Fallback to num2words")

# Use this in your codebase
from .compatibility import num2words
```

### Scenario 5: Library/Framework Integration

**Flask Example:**
```python
# Before
from flask import Flask
from num2words import num2words

# After
from flask import Flask
from num2words2 import num2words

app = Flask(__name__)

@app.route('/convert/<int:number>')
def convert_endpoint(number):
    return num2words(number)
```

**FastAPI Example:**
```python
# Before
from fastapi import FastAPI
from num2words import num2words

# After
from fastapi import FastAPI
from num2words2 import num2words

app = FastAPI()

@app.get("/convert/{number}")
def convert_number(number: int):
    return {"result": num2words(number)}
```

## Rollback Instructions

If you need to rollback the migration:

### Automatic Rollback

The migration script creates backups:

```bash
# Find all backup files
find . -name "*.num2words_backup"

# Restore all files
for file in $(find . -name "*.num2words_backup"); do
    original="${file%.num2words_backup}"
    cp "$file" "$original"
    echo "Restored $original"
done

# Clean up backup files
find . -name "*.num2words_backup" -delete
```

### Manual Rollback

1. Revert imports in Python files
2. Update requirements files back to `num2words`
3. Reinstall original package: `pip install num2words`
4. Remove new package: `pip uninstall num2words2`

## Troubleshooting

### Common Issues

**Issue: ImportError after migration**
```
ImportError: No module named 'num2words2'
```
**Solution:** Install num2words2: `pip install num2words2`

**Issue: Different output than expected**
```
AssertionError: Expected 'forty-two' but got 'forty two'
```
**Solution:** Check if you're using a different language or locale setting.

**Issue: Migration script doesn't find files**
```
No Python files found to scan.
```
**Solution:** Ensure you're running the script in the correct directory and have Python files.

### Performance Comparison

num2words2 maintains the same performance characteristics as the original:

```python
import time
from num2words2 import num2words

# Benchmark
start = time.time()
for i in range(10000):
    num2words(i)
end = time.time()

print(f"Converted 10,000 numbers in {end - start:.2f} seconds")
```

### Getting Help

- **Documentation:** https://github.com/jqueguiner/num2words
- **Issues:** https://github.com/jqueguiner/num2words/issues
- **Discussions:** https://github.com/jqueguiner/num2words/discussions

## Migration Checklist

Use this checklist to ensure complete migration:

- [ ] **Code Assessment**
  - [ ] Located all num2words imports in Python files
  - [ ] Identified requirements files to update
  - [ ] Checked Docker/CI configurations

- [ ] **Automated Migration**
  - [ ] Downloaded migration script
  - [ ] Ran dry-run to preview changes
  - [ ] Applied migration script
  - [ ] Verified backups were created

- [ ] **Manual Updates**
  - [ ] Updated requirements.txt/pyproject.toml/setup.py
  - [ ] Updated Docker files
  - [ ] Updated CI/CD configurations
  - [ ] Updated documentation/README

- [ ] **Testing**
  - [ ] Installed num2words2
  - [ ] Ran existing test suite
  - [ ] Tested basic functionality
  - [ ] Tested edge cases
  - [ ] Verified no regressions

- [ ] **Cleanup**
  - [ ] Uninstalled original num2words (optional)
  - [ ] Removed backup files (after verification)
  - [ ] Updated team documentation

## FAQ

**Q: Is num2words2 compatible with the original num2words?**
A: Yes for the public API: `num2words2` is a drop-in replacement for the `num2words()` function. The internal converter classes are not available (see Scenario 3).

**Q: Will my existing code break?**
A: Not if you use the public functions (`num2words`, `num2words_sentence`, ...): only the import statements need to change. Code that imports the converter classes (`num2words.lang_*`, `CONVERTER_CLASSES`) has to switch to `num2words(..., lang=..., to=...)`; see Scenario 3.

**Q: Can I use both libraries simultaneously?**
A: Not recommended as they may conflict. Choose one for your project.

**Q: What about performance?**
A: Performance is equivalent or better than the original library.

**Q: Are all languages supported?**
A: Yes, all original languages plus additional languages (Armenian, Mongolian, Shona).

---

**Need more help?** Check our [migration examples](https://github.com/jqueguiner/num2words/tree/master/examples) or [open an issue](https://github.com/jqueguiner/num2words/issues).
