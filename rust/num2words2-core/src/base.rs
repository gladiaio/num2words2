//! Faithful port of num2words2's `base.py` engine.
//!
//! The Python original builds a nested list-of-tuples tree in `splitnum`,
//! then folds it with `clean`/`merge`. The tree shape is load-bearing:
//! `clean` branches on whether an element is a tuple or a list, so `Node`
//! mirrors that distinction exactly rather than flattening it.

use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::sync::OnceLock;

/// A grammatical-kwargs value (`case=`, `gender=`, `reading=`, ...).
///
/// Covers every type the library's converter signatures accept: str, bool,
/// int, list-of-str, and an explicit `None`. Python bools are ints, so the
/// binding must try Bool before Int when extracting.
#[derive(Debug, Clone, PartialEq)]
pub enum KwVal {
    Str(String),
    Bool(bool),
    Int(i64),
    List(Vec<String>),
    None,
}

/// The kwargs bag handed to the `*_kw` trait hooks. Order-preserving —
/// insertion order never matters to the converters, but it keeps error
/// messages deterministic.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Kwargs(pub Vec<(String, KwVal)>);

impl Kwargs {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn get(&self, key: &str) -> Option<&KwVal> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    pub fn str(&self, key: &str) -> Option<&str> {
        match self.get(key) {
            Some(KwVal::Str(s)) => Some(s),
            _ => Option::None,
        }
    }
    pub fn bool(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            Some(KwVal::Bool(b)) => Some(*b),
            _ => Option::None,
        }
    }
    pub fn int(&self, key: &str) -> Option<i64> {
        match self.get(key) {
            Some(KwVal::Int(i)) => Some(*i),
            _ => Option::None,
        }
    }
    pub fn list(&self, key: &str) -> Option<&[String]> {
        match self.get(key) {
            Some(KwVal::List(l)) => Some(l),
            _ => Option::None,
        }
    }
    /// True when every key is in `allowed`. A language's `*_kw` override
    /// starts with this guard: an unexpected key means the Python signature
    /// would raise TypeError, so the hook returns NotImplemented and the
    /// dispatcher falls back to Python, which raises the original error.
    pub fn only(&self, allowed: &[&str]) -> bool {
        self.0.iter().all(|(k, _)| allowed.contains(&k.as_str()))
    }
}

/// A node in the tree `splitnum` produces.
///
/// `Leaf` is Python's `(text, num)` tuple; `List` is a nested list. `clean`
/// distinguishes the two, so collapsing them would change the output.
#[derive(Debug, Clone)]
pub enum Node {
    Leaf(String, BigInt),
    List(Vec<Node>),
}

impl Node {
    fn leaf(text: impl Into<String>, num: BigInt) -> Node {
        Node::Leaf(text.into(), num)
    }
    fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf(..))
    }
}

/// The ordered card table: value -> word, in descending key order.
///
/// Python uses an `OrderedDict` whose insertion order (high, then mid, then
/// low) happens to be descending. `splitnum` iterates it in that order, so a
/// sorted `Vec` reproduces the semantics while allowing binary search.
#[derive(Debug, Clone, Default)]
pub struct Cards {
    entries: Vec<(BigInt, String)>,
}

impl Cards {
    pub fn new() -> Self {
        Cards { entries: Vec::new() }
    }

    pub fn insert(&mut self, key: BigInt, word: impl Into<String>) {
        let word = word.into();
        match self.entries.binary_search_by(|(k, _)| key.cmp(k)) {
            Ok(i) => self.entries[i] = (key, word),
            Err(i) => self.entries.insert(i, (key, word)),
        }
    }

    pub fn get(&self, key: &BigInt) -> Option<&str> {
        self.entries
            .binary_search_by(|(k, _)| key.cmp(k))
            .ok()
            .map(|i| self.entries[i].1.as_str())
    }

    /// Descending iteration, matching the Python insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &(BigInt, String)> {
        self.entries.iter()
    }

    /// Entries from the first card `<= value`, descending.
    ///
    /// Python does `for elem in self.cards: if elem > value: continue`, an
    /// O(n) scan that costs ~100 BigInt comparisons to reach "forty" from
    /// 10^303. Entries are sorted descending, so the first card `<= value`
    /// is a binary-search partition point — identical result, O(log n).
    pub fn iter_from(&self, value: &BigInt) -> impl Iterator<Item = &(BigInt, String)> {
        let start = self.entries.partition_point(|(k, _)| k > value);
        self.entries[start..].iter()
    }

    pub fn highest(&self) -> Option<&BigInt> {
        self.entries.first().map(|(k, _)| k)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Errors mirroring the exception types the Python original raises.
///
/// `Index`, `Key` and `Value` exist because several language modules crash
/// on edge inputs rather than raising deliberately — e.g. `lang_PL`'s
/// `to_ordinal(0)` pops its last fragment and then indexes an empty list
/// (IndexError), `to_ordinal(-1)` feeds "-" to `int()` (ValueError), and
/// `to_ordinal(10**12)` misses `prefixes_ordinal[4]` (KeyError).
///
/// Those are Python bugs, but they are observable behaviour that callers may
/// catch, so parity requires reproducing the exception *type*. Do not
/// "improve" them into a tidy TypeError.
#[derive(Debug)]
pub enum N2WError {
    Overflow(String),
    Type(String),
    /// Python raised NotImplementedError as a DELIBERATE result — the same
    /// exception the original produces, so it must propagate to the caller.
    /// `to_ordinal(>100)` in Welsh, an unknown Japanese counter, and the
    /// abstract `pluralize` all land here. Distinct from `Fallback`.
    NotImplemented(String),
    /// Not a Python exception: the Rust core declines this specific call and
    /// the binding re-runs the ORIGINAL Python converter, which owns it
    /// (an unported kwarg whose Python path raises the real TypeError; an
    /// Inf/NaN string a language handles in Python; a sentence sub-case).
    /// Kept separate from `NotImplemented` so a genuine NotImplementedError
    /// raise is served natively instead of being swallowed by the shim's
    /// fallback catch.
    Fallback(String),
    ZeroDivision(String),
    /// Python raised IndexError (usually an out-of-range list access).
    Index(String),
    /// Python raised KeyError (usually a missing dict entry).
    Key(String),
    /// Python raised ValueError (usually int()/str parsing of a bad token).
    Value(String),
    /// Python raised AttributeError. Usually the converter simply does not
    /// define the method or attribute at all — `Num2Word_RM` has no
    /// `to_year`/`to_ordinal_num`, and `Num2Word_IT` has no `errmsg_negord`
    /// — so the crash happens on attribute lookup, before any conversion.
    Attribute(String),
    /// Python raised AssertionError. `lang_AR` asserts its bounds instead of
    /// raising deliberately, so `to_ordinal(>=10**51)` and
    /// `to_cardinal(-(10**51))` surface as AssertionError, not OverflowError.
    Assertion(String),
    /// Not an error: Python's function fell off the end and returned `None`.
    ///
    /// `lang_VI` does this past 10^75 — no raise, no string, just an implicit
    /// `None` that propagates to the caller. `None` is a *success* value, so
    /// the binding turns this sentinel into Python `None` rather than an
    /// exception. Modelled as an Err variant purely to avoid rewriting
    /// `Result<String>` to `Result<Option<String>>` across 154 language files.
    ReturnsNone,
    /// Python raised an exception class defined in a language module rather
    /// than a builtin: `(module_path, class_name, message)`.
    ///
    /// `lang_BN.py` is the only case in the library — it defines
    /// `NumberTooLargeError(Exception)` and raises it past `MAX_NUMBER`
    /// (~10^306) instead of `OverflowError`. The binding imports the class
    /// and raises the real thing, so `except NumberTooLargeError` keeps
    /// working.
    Custom {
        module: &'static str,
        class: &'static str,
        msg: String,
    },
}

pub type Result<T> = std::result::Result<T, N2WError>;

/// The per-language behaviour that `base.py` leaves abstract or overridable.
///
/// Defaults mirror `Num2Word_Base`; each language overrides what it needs,
/// exactly as the Python subclasses do.
///
/// Two shapes exist in the wild, and both must be expressible:
///
/// 1. *Engine languages* supply `cards` + `merge` and let the default
///    `to_cardinal` drive `splitnum`/`clean` (as `Num2Word_Base` does).
/// 2. *Self-contained languages* override `to_cardinal` outright and never
///    touch cards or merge — 121 of the 156 Python modules do this.
///
/// So `cards`/`maxval`/`merge` carry defaults rather than being required;
/// a shape-2 language simply ignores them.
pub trait Lang {
    fn cards(&self) -> &Cards {
        static EMPTY: OnceLock<Cards> = OnceLock::new();
        EMPTY.get_or_init(Cards::new)
    }

    fn maxval(&self) -> &BigInt {
        static ZERO: OnceLock<BigInt> = OnceLock::new();
        ZERO.get_or_init(BigInt::zero)
    }

    fn negword(&self) -> &str {
        "(-) "
    }
    fn pointword(&self) -> &str {
        "(.)"
    }
    fn is_title(&self) -> bool {
        false
    }
    fn exclude_title(&self) -> &[String] {
        &[]
    }

    /// `merge` is abstract in Python (`raise NotImplementedError`).
    /// Languages that override `to_cardinal` never reach it.
    fn merge(&self, _l: (&str, &BigInt), _r: (&str, &BigInt)) -> (String, BigInt) {
        unimplemented!("merge not implemented for this language")
    }

    fn title(&self, value: &str) -> String {
        if !self.is_title() {
            return value.to_string();
        }
        let excl = self.exclude_title();
        value
            .split_whitespace()
            .map(|w| {
                if excl.iter().any(|e| e == w) {
                    w.to_string()
                } else {
                    // Python does word[0].upper() + word[1:] — uppercase the
                    // first *character*, not the first byte.
                    let mut c = w.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        None => String::new(),
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        default_to_cardinal(self, value)
    }

    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        Ok(value.to_string())
    }

    fn to_year(&self, value: &BigInt) -> Result<String> {
        self.to_cardinal(value)
    }

    // ---- currency ----------------------------------------------------
    //
    // Every hook Python's currency path can reach is exposed here with a
    // default. A language overrides only what its Python class overrides.

    /// Class name, for the `'X' not implemented for 'Num2Word_Y'` message.
    fn lang_name(&self) -> &str {
        "Num2Word_Base"
    }

    /// `CURRENCY_FORMS[code]`. `None` -> NotImplementedError, as in Python.
    fn currency_forms(&self, _code: &str) -> Option<&CurrencyForms> {
        None
    }

    /// `CURRENCY_ADJECTIVES[code]`.
    fn currency_adjective(&self, _code: &str) -> Option<&str> {
        None
    }

    /// `CURRENCY_PRECISION.get(code, 100)` — subunits per unit. 1000 for
    /// 3-decimal currencies (BHD/KWD/...), 1 for no-subunit currencies.
    fn currency_precision(&self, _code: &str) -> i64 {
        100
    }

    /// `pluralize(n, forms)`. Abstract in Python (`raise NotImplementedError`),
    /// so the default raises rather than guessing a plural rule.
    fn pluralize(&self, _n: &BigInt, _forms: &[String]) -> Result<String> {
        Err(N2WError::NotImplemented("pluralize".into()))
    }

    fn money_verbose(&self, number: &BigInt, _currency: &str) -> Result<String> {
        self.to_cardinal(number)
    }

    fn cents_verbose(&self, number: &BigInt, _currency: &str) -> Result<String> {
        self.to_cardinal(number)
    }

    fn cents_terse(&self, number: &BigInt, currency: &str) -> Result<String> {
        Ok(crate::currency::default_cents_terse(
            number,
            self.currency_precision(currency),
        ))
    }

    /// `Num2Word_Base.to_cardinal_float`. `precision_override` is the
    /// `precision=` kwarg (issue #580).
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        crate::floatpath::default_to_cardinal_float(self, value, precision_override)
    }

    /// Render a non-integral value, for the fractional-cents branch. Python
    /// reaches this as `self.to_cardinal(float(right))`, i.e. through the
    /// float path, so the default routes there rather than raising.
    fn cardinal_from_decimal(&self, value: &BigDecimal) -> Result<String> {
        crate::floatpath::cardinal_from_bigdecimal(self, value)
    }

    /// `separator: None` means the caller omitted the kwarg, so the
    /// language's own default applies. Python expresses this as a per-method
    /// default (`Num2Word_CA.to_currency(..., separator=" amb")`), which a
    /// plain `&str` cannot represent: once the shim substitutes Base's ",",
    /// "caller omitted it" and "caller passed a comma" are indistinguishable.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        crate::currency::default_to_currency(
            self,
            val,
            currency,
            cents,
            separator.unwrap_or(self.default_separator()),
            adjective,
        )
    }

    /// This language's own `separator=` default. Base's is ",".
    fn default_separator(&self) -> &str {
        ","
    }

    /// This language's own `to_currency(adjective=...)` default. Base's is
    /// False; only Mongolian defaults to True.
    fn default_adjective(&self) -> bool {
        false
    }

    /// This language's own `to_currency(currency=...)` default. Base's is
    /// "EUR", but only 44 of 156 languages actually use it — en_IN defaults
    /// to INR, id to IDR, af to ZAR, and so on. Hardcoding EUR in the
    /// dispatcher silently converts the wrong currency.
    fn default_currency(&self) -> &str {
        "EUR"
    }

    fn to_cheque(&self, val: &BigDecimal, currency: &str) -> Result<String> {
        crate::currency::default_to_cheque(self, val, currency)
    }

    // ---- fractions -----------------------------------------------------

    /// `to='fraction'` (issue #584). EN/DE/ES/FR/IT/PT/CA and the aero
    /// profiles override with idiomatic forms (half/quarter/Drittel); every
    /// other language raises NotImplementedError (#217).
    fn to_fraction(&self, numerator: &BigInt, denominator: &BigInt) -> Result<String> {
        default_to_fraction(self, numerator, denominator)
    }

    // ---- float/Decimal routing ------------------------------------------
    //
    // Python's dispatcher hands a float/Decimal straight to the converter
    // method; where it lands is the *language's* decision. Base semantics:
    // `assert int(value) == value` succeeds for a whole value -> integer
    // path ("one"); fails -> to_cardinal_float. Languages overriding
    // Python's to_cardinal (ru "пять целых ноль десятых", cs, be, ...) make
    // their own call and override these hooks.
    //
    // The ordinal / ordinal_num / year hooks below are no longer reached
    // from the dispatcher (gladiaio/num2words2#213/#214): it sends an
    // integral value to the integer modes and raises TypeError for any
    // other, before the language is called.

    /// `to_cardinal(float/Decimal)` — the full entry, whole values included.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
    ) -> Result<String> {
        if let Some(i) = value.as_whole_int() {
            return self.to_cardinal(&i);
        }
        self.to_cardinal_float(value, precision_override)
    }

    /// `to_ordinal(float/Decimal)`. Base's to_ordinal is to_cardinal, so the
    /// default routes identically; languages with verify_ordinal-style type
    /// checks (TypeError on non-int) override.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `to_ordinal_num(float/Decimal)`. Base returns the value unchanged and
    /// the dispatcher str()s it, so the default echoes the Python repr the
    /// binding computed.
    fn ordinal_num_float_entry(&self, _value: &FloatValue, repr_str: &str) -> Result<String> {
        Ok(repr_str.to_string())
    }

    /// `to_year(float/Decimal)`. Base's to_year is to_cardinal.
    fn year_float_entry(&self, value: &FloatValue) -> Result<String> {
        self.cardinal_float_entry(value, None)
    }

    /// `to_year(float/Decimal, **kwargs)`. Like the integer `*_kw` hooks the
    /// default serves only an empty bag and declines any actual kwarg.
    fn year_float_kw(&self, value: &FloatValue, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.year_float_entry(value);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    // ---- string inputs ---------------------------------------------------

    /// `converter.str_to_number`. Base is `Decimal(value)`; ES ("1ro" ->
    /// ordinal handshake), PT_BR ("1.50" -> 'ponto'), DV (TypeError instead
    /// of InvalidOperation), BN (abs()) and ID override.
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        python_decimal_parse(s)
    }

    /// The pt_BR 'ponto' rendering: `to_cardinal(value)` with the pointword
    /// swapped for this one call. Only reachable through
    /// `ParsedNumber::DecPoint`, so only PT_BR implements it.
    fn cardinal_with_pointword(
        &self,
        _value: &FloatValue,
        _pointword: &str,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        Err(N2WError::Fallback("cardinal_with_pointword".into()))
    }

    // ---- grammatical kwargs ----------------------------------------------
    //
    // The `*_kw` hooks receive the caller's kwargs bag. The default accepts
    // only an empty bag; any actual kwarg is NotImplemented, which the
    // binding surfaces as NotImplementedError so the dispatcher falls back
    // to Python — either the original handles the kwarg (port not landed) or
    // it raises the original TypeError (converter doesn't accept it).

    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_cardinal(value);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    fn to_ordinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_ordinal(value);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    fn to_ordinal_num_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_ordinal_num(value);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    fn to_year_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if kw.is_empty() {
            return self.to_year(value);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    #[allow(clippy::too_many_arguments)]
    fn to_currency_kw(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.to_currency(val, currency, cents, separator, adjective);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if kw.is_empty() {
            return self.cardinal_float_entry(value, precision_override);
        }
        Err(N2WError::Fallback("kwargs".into()))
    }

    /// Spanish "1ro" currency: does the stashed ordinal fire? `Num2Word_ES`
    /// apocopates 1 → "un euro" in `to_currency`, bypassing `to_cardinal`, so
    /// the stash never fires (false, the default). The regional `es_XX`
    /// classes inherit `Base.to_currency`, whose `_money_verbose` calls
    /// `to_cardinal(1)` → the stash fires → "primero córdoba" (true).
    fn es_currency_ordinal_fires(&self) -> bool {
        false
    }

    /// No longer called: the dispatcher reads every negative zero as zero,
    /// without "minus" (gladiaio/num2words2#237). Kept so the existing
    /// overrides still compile.
    ///
    /// Render `Decimal('-0.0')` for mode `to`, which BigDecimal cannot
    /// represent (it has no signed zero). `None` — the default — means the
    /// value coincides with float `-0.0`, so the binding's `Float{-0.0}`
    /// demotion is exact. The ~10 languages that render the Decimal form
    /// differently (ce/cy "dim"/"ноль" via the integer path, pl its float
    /// grammar with the negword, rm a TypeError) override this to serve it
    /// natively instead of leaning on a Python fallback.
    fn neg_zero_decimal(&self, _to: &str) -> Option<Result<String>> {
        None
    }

    /// `str_to_number` produced `Decimal('Infinity')` / `-Infinity`. Base's
    /// integer path does `int(Decimal('Infinity'))` → OverflowError. The many
    /// self-contained converters that feed the raw token to `int("Infinity")`
    /// get ValueError instead and override this. `to` is the requested mode.
    fn inf_result(&self, _negative: bool, _to: &str) -> Result<String> {
        Err(N2WError::Overflow(
            "cannot convert Infinity to integer".into(),
        ))
    }

    /// `str_to_number` produced `Decimal('NaN')`. Base's `int(NaN)` raises
    /// ValueError (caught into the float path, which raises ValueError again).
    /// Languages whose parse raises `decimal.InvalidOperation` first override.
    fn nan_result(&self, _to: &str) -> Result<String> {
        Err(N2WError::Value("cannot convert NaN to integer".into()))
    }

    // ---- introspection ---------------------------------------------------

    /// Python's `getattr(converter, "MAXVAL", None)` — the `maxval()` API.
    /// Engine languages get it from their card table (Base sets
    /// `MAXVAL = 1000 * top_card`); self-contained ones mostly lack the
    /// attribute (None). The zero sentinel in `maxval()` means "unset".
    fn python_maxval(&self) -> Option<BigInt> {
        let m = self.maxval();
        if m.is_zero() {
            None
        } else {
            Some(m.clone())
        }
    }
}

/// Base's `errmsg_negord`: the `TypeError` most languages raise for a
/// negative ordinal. Shared so the languages that used to crash or wrap
/// around instead (et/ms/pl/hy/cy/..., gladiaio/num2words2#155/#158) raise
/// the same wording.
pub fn negord_error(value: impl std::fmt::Display) -> N2WError {
    N2WError::Type(format!("Cannot treat negative num {} as ordinal.", value))
}

/// Base's `errmsg_floatord`: the `TypeError` for a fractional ordinal.
pub fn floatord_error(value: impl std::fmt::Display) -> N2WError {
    N2WError::Type(format!("Cannot treat float {} as ordinal.", value))
}

/// `str(value)` of a float/Decimal, for error-message text only.
pub fn py_num_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{:?}", value),
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
    }
}

/// `value < 0` — strictly, so `-0.0` is not negative here.
pub fn strictly_negative(value: &FloatValue) -> bool {
    match value {
        FloatValue::Float { value, .. } => *value < 0.0,
        FloatValue::Decimal { value, .. } => value.is_negative(),
    }
}

/// Base's `verify_ordinal` for an integer: negatives raise `errmsg_negord`.
pub fn verify_ordinal(value: &BigInt) -> Result<()> {
    if value.is_negative() {
        return Err(negord_error(value));
    }
    Ok(())
}

/// Base's `verify_ordinal` for a float/Decimal: a fractional (or non-finite)
/// value raises `errmsg_floatord`, a negative one `errmsg_negord`. Returns
/// the whole value as an integer.
pub fn verify_ordinal_float(value: &FloatValue) -> Result<BigInt> {
    let i = value
        .as_whole_int()
        .ok_or_else(|| floatord_error(py_num_str(value)))?;
    if strictly_negative(value) {
        return Err(negord_error(py_num_str(value)));
    }
    Ok(i)
}

/// The `TypeError` for `to='year'` on a fractional value, in the wording
/// `lang_en` uses (issue #67).
pub fn year_float_error(value: &FloatValue) -> N2WError {
    N2WError::Type(format!(
        "to='year' expects an integer; got non-integer {} {}",
        match value {
            FloatValue::Float { .. } => "float",
            FloatValue::Decimal { .. } => "Decimal",
        },
        py_num_str(value)
    ))
}

/// `NotImplementedError` for a `to=` mode a language has no rules for. The
/// binder prefixes `lang='xx' ` (the core does not know the language key),
/// giving `lang='ru' does not support to='fraction'`.
pub fn unsupported_mode(to: &str) -> N2WError {
    N2WError::NotImplemented(format!("does not support to='{}'", to))
}

/// `to_fraction` for a language without its own fraction rules.
///
/// Python's `Num2Word_Base.to_fraction` read "n ordinal" and pluralised with
/// an English "s" in every language ("два третийs", "三 第四s"; #217). A
/// fraction reading is language grammar (Russian "две третьих", Dutch "twee
/// derde"), not something a generic rule can build from the ordinal, so a
/// language without rules raises — as rm already did — rather than invent a
/// plural. A zero denominator still raises ZeroDivisionError first.
pub fn default_to_fraction<L: Lang + ?Sized>(
    _lang: &L,
    _numerator: &BigInt,
    denominator: &BigInt,
) -> Result<String> {
    if denominator.is_zero() {
        return Err(N2WError::ZeroDivision(
            "denominator must not be zero".into(),
        ));
    }
    Err(unsupported_mode("fraction"))
}

/// Python's `splitnum`. Returns `None` where Python falls off the loop and
/// implicitly returns `None` (value larger than every card).
pub fn splitnum<L: Lang + ?Sized>(lang: &L, value: &BigInt) -> Option<Vec<Node>> {
    let cards = lang.cards();
    for (elem, word) in cards.iter_from(value) {
        let mut out: Vec<Node> = Vec::new();
        let (div, mod_) = if value.is_zero() {
            (BigInt::one(), BigInt::zero())
        } else {
            value.div_mod_floor(elem)
        };

        if div.is_one() {
            let one = BigInt::one();
            let w = cards.get(&one).unwrap_or("").to_string();
            out.push(Node::leaf(w, one));
        } else {
            if &div == value {
                // Tally systems (e.g. Roman numerals): repeat the word `div`
                // times rather than recursing, matching Python's `div * word`.
                let reps = div.to_string().parse::<usize>().unwrap_or(0);
                return Some(vec![Node::leaf(word.repeat(reps), &div * elem)]);
            }
            out.push(Node::List(splitnum(lang, &div)?));
        }

        out.push(Node::leaf(word.clone(), elem.clone()));

        if !mod_.is_zero() {
            out.push(Node::List(splitnum(lang, &mod_)?));
        }

        return Some(out);
    }
    None
}

/// Python's `clean`: fold the tree down to a single `(text, num)` leaf.
///
/// Mirrors the original's quirk where the tail `val[2:]` is appended as a
/// *nested list* rather than spliced, which changes how the next iteration
/// sees it.
pub fn clean<L: Lang + ?Sized>(lang: &L, val: Vec<Node>) -> Node {
    let mut val = val;
    while val.len() != 1 {
        let mut out: Vec<Node> = Vec::new();
        if val.len() >= 2 && val[0].is_leaf() && val[1].is_leaf() {
            let (lt, ln) = match &val[0] {
                Node::Leaf(t, n) => (t.clone(), n.clone()),
                _ => unreachable!(),
            };
            let (rt, rn) = match &val[1] {
                Node::Leaf(t, n) => (t.clone(), n.clone()),
                _ => unreachable!(),
            };
            let (mt, mn) = lang.merge((&lt, &ln), (&rt, &rn));
            out.push(Node::Leaf(mt, mn));
            if val.len() > 2 {
                out.push(Node::List(val[2..].to_vec()));
            }
        } else {
            for elem in val.into_iter() {
                match elem {
                    Node::List(inner) => {
                        if inner.len() == 1 {
                            out.push(inner.into_iter().next().unwrap());
                        } else {
                            out.push(clean(lang, inner));
                        }
                    }
                    leaf => out.push(leaf),
                }
            }
        }
        val = out;
    }
    val.into_iter().next().unwrap()
}

/// `10**exp` as a `BigInt` (callers cache it in a `OnceLock`).
pub fn pow10_big(exp: u32) -> BigInt {
    BigInt::from(10u8).pow(exp)
}

/// The `abs(v) must be less than MAXVAL.` guard for self-contained languages
/// whose scale-word tables run out (gladiaio/num2words2#159). Call it at the
/// top of the routine that receives the whole integer, and return the same
/// `maxval` from `Lang::maxval` so `maxval(lang)` reports the real ceiling.
pub fn check_maxval(value: &BigInt, maxval: &BigInt) -> Result<()> {
    let v = value.abs();
    if &v >= maxval {
        return Err(N2WError::Overflow(format!(
            "abs({}) must be less than {}.",
            v, maxval
        )));
    }
    Ok(())
}

/// Python's `Num2Word_Base.to_cardinal` for integral input.
pub fn default_to_cardinal<L: Lang + ?Sized>(lang: &L, value: &BigInt) -> Result<String> {
    let mut out = String::new();
    let mut v = value.clone();
    if v.is_negative() {
        v = v.abs();
        out = format!("{} ", lang.negword().trim());
    }

    if &v >= lang.maxval() {
        return Err(N2WError::Overflow(format!(
            "abs({}) must be less than {}.",
            v,
            lang.maxval()
        )));
    }

    let tree = splitnum(lang, &v).ok_or_else(|| {
        N2WError::Overflow(format!("abs({}) must be less than {}.", v, lang.maxval()))
    })?;
    let node = clean(lang, tree);
    let words = match node {
        Node::Leaf(t, _) => t,
        Node::List(_) => return Err(N2WError::Type("clean did not reduce".into())),
    };
    Ok(lang.title(&format!("{}{}", out, words)))
}

/// Python's `set_low_numwords`: words map to descending values ending at 0.
pub fn set_low_numwords(cards: &mut Cards, numwords: &[&str]) {
    let n = numwords.len();
    for (i, word) in numwords.iter().enumerate() {
        cards.insert(BigInt::from(n - 1 - i), *word);
    }
}

pub fn set_mid_numwords(cards: &mut Cards, mid: &[(i64, &str)]) {
    for (k, v) in mid {
        cards.insert(BigInt::from(*k), *v);
    }
}
