#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
Test the num2words_sentence function against the e2e CSV test file.
"""

import csv
import os
import sys

from num2words2 import num2words_sentence


def _force_utf8_console():
    """Make print() survive non-ASCII output on a Windows console.

    Only for running this file as a script. Under pytest, sys.stdout is
    pytest's capture stream: wrapping its buffer in a new TextIOWrapper
    hands ownership of pytest's temp file to the wrapper, which closes it
    when garbage-collected, and pytest then crashes at teardown with
    "ValueError: I/O operation on closed file" (every Windows job of the
    scheduled workflow). reconfigure() changes the encoding in place.
    """
    if sys.platform != "win32":
        return
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8")
    os.environ["PYTHONIOENCODING"] = "utf-8"


def normalize_text(text):
    """Normalize text for comparison."""
    import re

    # Remove extra spaces and normalize
    text = re.sub(r"\s+", " ", text.strip())
    # Normalize quotes and punctuation
    text = text.replace("«", '"').replace("»", '"')
    return text.lower()


def calculate_similarity(expected, actual):
    """Calculate word-level similarity between two strings."""
    expected_words = set(normalize_text(expected).split())
    actual_words = set(normalize_text(actual).split())

    if not expected_words or not actual_words:
        return 0.0

    intersection = expected_words & actual_words
    union = expected_words | actual_words
    return len(intersection) / len(union) if union else 0.0


def run_csv_file(csv_path):
    """Test the sentence function with the CSV file."""

    results = {"total": 0, "passed": 0, "failed": 0, "by_language": {}}

    failures = []

    print("=" * 80)
    print("E2E TEST: num2words_sentence function")
    print("=" * 80)

    with open(csv_path, "r", encoding="utf-8") as f:
        reader = csv.DictReader(f)

        for row_num, row in enumerate(reader, 2):
            if not row.get("language_code"):
                continue

            lang_code = row["language_code"]
            lang_name = row["language_name"]
            original = row["original_sentence"]
            # Try new column name first, fallback to old for compatibility
            expected = row.get("expected_output") or row.get("full_text_conversion", "")

            results["total"] += 1

            # Initialize language stats
            if lang_code not in results["by_language"]:
                results["by_language"][lang_code] = {
                    "name": lang_name,
                    "total": 0,
                    "passed": 0,
                    "failed": 0,
                }

            results["by_language"][lang_code]["total"] += 1

            try:
                # Use our new function
                actual = num2words_sentence(original, lang=lang_code)

                # Compare results
                expected_norm = normalize_text(expected)
                actual_norm = normalize_text(actual)

                # Calculate similarity
                similarity = calculate_similarity(expected, actual)

                # Check for exact match or high similarity
                if expected_norm == actual_norm:
                    results["passed"] += 1
                    results["by_language"][lang_code]["passed"] += 1
                    status = "[PASS] EXACT"
                elif similarity >= 0.95:
                    results["passed"] += 1
                    results["by_language"][lang_code]["passed"] += 1
                    status = f"[PASS] CLOSE ({similarity:.1%})"
                elif similarity >= 0.8:
                    results["failed"] += 1
                    results["by_language"][lang_code]["failed"] += 1
                    status = f"[WARN] PARTIAL ({similarity:.1%})"
                    failures.append(
                        {
                            "row": row_num,
                            "lang": lang_code,
                            "original": original,
                            "expected": expected,
                            "actual": actual,
                            "similarity": similarity,
                        }
                    )
                else:
                    results["failed"] += 1
                    results["by_language"][lang_code]["failed"] += 1
                    status = f"[FAIL] FAIL ({similarity:.1%})"
                    failures.append(
                        {
                            "row": row_num,
                            "lang": lang_code,
                            "original": original,
                            "expected": expected,
                            "actual": actual,
                            "similarity": similarity,
                        }
                    )

                print(f"Row {row_num:3} [{lang_code}]: {status}")

                # Show details for failures
                if similarity < 0.95:
                    print(f"  Original:  {original[:60]}...")
                    print(f"  Expected:  {expected[:60]}...")
                    print(f"  Got:       {actual[:60]}...")

            except Exception as e:
                results["failed"] += 1
                results["by_language"][lang_code]["failed"] += 1
                print(f"Row {row_num:3} [{lang_code}]: [ERROR] - {e}")
                failures.append(
                    {
                        "row": row_num,
                        "lang": lang_code,
                        "original": original,
                        "error": str(e),
                    }
                )

    # Print summary
    print("\n" + "=" * 80)
    print("SUMMARY")
    print("=" * 80)

    if results["total"] > 0:
        pass_rate = (results["passed"] / results["total"]) * 100
        print(f"Total Tests:  {results['total']}")
        print(f"Passed:       {results['passed']} ({pass_rate:.1f}%)")
        print(f"Failed:       {results['failed']} ({100-pass_rate:.1f}%)")

        print("\nBy Language:")
        print("-" * 40)
        for lang, stats in sorted(results["by_language"].items()):
            lang_pass_rate = (
                (stats["passed"] / stats["total"] * 100) if stats["total"] > 0 else 0
            )
            status = (
                "[GOOD]"
                if lang_pass_rate >= 90
                else "[WARN]"
                if lang_pass_rate >= 70
                else "[FAIL]"
            )
            print(
                f"{status} {stats['name']:10} ({lang:5}): {stats['passed']}/{stats['total']} ({lang_pass_rate:.1f}%)"
            )

    # Show worst failures
    if failures:
        print("\n" + "=" * 80)
        print("TOP FAILURES (Lowest Similarity)")
        print("=" * 80)

        # Sort by similarity
        worst = sorted(
            [f for f in failures if "similarity" in f], key=lambda x: x["similarity"]
        )[:5]

        for failure in worst:
            print(
                f"\nRow {failure['row']} [{failure['lang']}] - Similarity: {failure['similarity']:.1%}"
            )
            print(f"  Original:  {failure['original'][:80]}")
            print(f"  Expected:  {failure['expected'][:80]}")
            print(f"  Got:       {failure.get('actual', 'ERROR')[:80]}")

    return results


def main():
    _force_utf8_console()
    csv_path = "tests/data/e2e_test_sentences.csv"

    print(f"Testing with: {csv_path}\n")

    results = run_csv_file(csv_path)

    # Exit code based on results
    if results["total"] > 0:
        pass_rate = (results["passed"] / results["total"]) * 100
        if pass_rate >= 95:
            print("\n[EXCELLENT] Over 95% pass rate!")
            sys.exit(0)
        elif pass_rate >= 80:
            print("\n[SUCCESS] Good! Over 80% pass rate.")
            sys.exit(0)
        else:
            print("\n[WARNING] Needs improvement. Less than 80% pass rate.")
            sys.exit(1)


if __name__ == "__main__":
    main()
