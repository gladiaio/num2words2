//! Issue #204: no public float/currency entry point may panic, whatever the
//! input. NaN, ±inf and a Decimal with more fractional digits than
//! `format!`'s width limit (65535) must come back as a typed `N2WError` or a
//! string, in every language.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::str::FromStr;
use std::sync::Mutex;

use bigdecimal::BigDecimal;
use num2words2_core::{get_lang_by_key, supported_lang_keys, CurrencyValue, FloatValue, Lang};

/// `Decimal('1E-70000')`: 70000 fractional digits, past the 65535 limit.
const HUGE_SCALE: u32 = 70_000;

fn float_inputs() -> Vec<(&'static str, FloatValue)> {
    let tiny = BigDecimal::from_str(&format!("1E-{HUGE_SCALE}")).unwrap();
    let long = BigDecimal::from_str(&format!("0.{}", "7".repeat(HUGE_SCALE as usize))).unwrap();
    vec![
        ("nan", FloatValue::Float { value: f64::NAN, precision: 0 }),
        ("nan/1", FloatValue::Float { value: f64::NAN, precision: 1 }),
        ("inf", FloatValue::Float { value: f64::INFINITY, precision: 0 }),
        ("-inf", FloatValue::Float { value: f64::NEG_INFINITY, precision: 0 }),
        ("inf/1", FloatValue::Float { value: f64::INFINITY, precision: 1 }),
        ("1E-70000", FloatValue::Decimal { value: tiny, precision: HUGE_SCALE }),
        ("0.77…7", FloatValue::Decimal { value: long, precision: HUGE_SCALE }),
    ]
}

fn currency_inputs() -> Vec<(&'static str, CurrencyValue)> {
    let tiny = BigDecimal::from_str(&format!("1E-{HUGE_SCALE}")).unwrap();
    let long = BigDecimal::from_str(&format!("1.{}", "7".repeat(HUGE_SCALE as usize))).unwrap();
    vec![
        ("1E-70000", CurrencyValue::Decimal { value: tiny, has_decimal: true, is_float: false }),
        ("1.77…7", CurrencyValue::Decimal { value: long, has_decimal: true, is_float: false }),
    ]
}

/// Every entry point × input for one language; returns the calls that panicked.
fn panics_for(key: &str, lang: &(dyn Lang + Sync)) -> Vec<String> {
    let mut failures = Vec::new();
    let mut check = |what: String, call: &dyn Fn()| {
        if catch_unwind(AssertUnwindSafe(call)).is_err() {
            failures.push(format!("{key}: {what}"));
        }
    };
    for (label, v) in float_inputs() {
        check(format!("cardinal_float_entry({label})"), &|| drop(lang.cardinal_float_entry(&v, None)));
        check(format!("ordinal_float_entry({label})"), &|| drop(lang.ordinal_float_entry(&v)));
        check(format!("year_float_entry({label})"), &|| drop(lang.year_float_entry(&v)));
        check(format!("ordinal_num_float_entry({label})"), &|| {
            drop(lang.ordinal_num_float_entry(&v, "x"))
        });
        check(format!("to_cardinal_float({label})"), &|| drop(lang.to_cardinal_float(&v, None)));
    }
    let currency = lang.default_currency();
    for (label, v) in currency_inputs() {
        for cents in [true, false] {
            check(format!("to_currency({label}, cents={cents})"), &|| {
                drop(lang.to_currency(&v, currency, cents, None, false))
            });
        }
    }
    failures
}

#[test]
fn float_and_currency_entry_points_never_panic() {
    // Keep the output readable: the failures are collected and reported once.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    // One thread per language: the 70000-digit inputs are slow in debug.
    let failures = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for key in supported_lang_keys() {
            let failures = &failures;
            // Named, so a stack overflow (which aborts, see lang_ha) says where.
            std::thread::Builder::new()
                .name(key.to_string())
                .spawn_scoped(s, move || {
                    let lang = get_lang_by_key(key).unwrap();
                    let found = panics_for(key, lang);
                    failures.lock().unwrap().extend(found);
                })
                .unwrap();
        }
    });

    std::panic::set_hook(prev_hook);
    let mut failures = failures.into_inner().unwrap();
    failures.sort();
    assert!(failures.is_empty(), "{} panics:\n{}", failures.len(), failures.join("\n"));
}
