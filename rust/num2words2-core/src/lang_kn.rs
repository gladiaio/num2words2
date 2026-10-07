//! Port of `lang_KN.py` (Kannada), via its `lang_EUR` -> `Num2Word_Base` ancestry.
//!
//! Shape: **engine**. `Num2Word_KN` defines `low_numwords`/`mid_numwords`/
//! `high_numwords` plus `merge`, and lets `Num2Word_Base.to_cardinal` drive
//! `splitnum`/`clean`. So this file builds a `Cards` table in `new()`,
//! implements `merge`, and leaves `to_cardinal` at the trait default.
//!
//! # Inheritance notes
//!
//! `Num2Word_KN.setup()` does **not** call `super().setup()`, so none of
//! `Num2Word_EUR`'s Latin-prefix illion machinery (`gen_high_numwords`, the
//! `cent`/`m`/`b`/`tr` stems) is reachable. KN also overrides
//! `set_high_numwords` to a completely different contract:
//!
//! ```python
//! def set_high_numwords(self, high):
//!     for n, word in self.high_numwords:
//!         self.cards[10**n] = word
//! ```
//!
//! i.e. `high_numwords` holds `(exponent, word)` pairs -- `[(7, crore),
//! (5, lakh), (3, thousand)]` -- rather than EUR's bare word list zipped
//! against a computed exponent range. Note it ignores its `high` parameter and
//! reads `self.high_numwords` directly; same object, so no behavioural
//! difference. The result is the Indian numbering system: crore (10^7), lakh
//! (10^5), thousand (10^3), hundred (10^2), then 0..=99 as literal cards.
//!
//! Inherited unchanged from `Num2Word_Base` (trait defaults are correct):
//!   * `negword` = `"(-) "` -- KN never overrides it, so negative cardinals
//!     read "(-) <word>" (base does `"%s " % negword.strip()`).
//!   * `is_title` = False -> `title()` is the identity.
//!   * `to_year(value)` -> `to_cardinal(value)`, including for negative years;
//!     there is no BC/CE suffix, so `to_year(-500) == to_cardinal(-500)`.
//!
//! # MAXVAL
//!
//! `Num2Word_Base.__init__` sets `MAXVAL = 1000 * list(self.cards.keys())[0]`.
//! The OrderedDict is filled high -> mid -> low, so key[0] is 10^7 and
//! **MAXVAL == 10^10**. Anything `>= 10^10` raises `OverflowError`. Values are
//! therefore bounded in practice, but `BigInt` is kept throughout because the
//! overflow check runs against the caller's arbitrary-precision input.
//!
//! # Unicode: the source is NOT NFC-normalized
//!
//! This is the single biggest hazard in this port. `lang_KN.py` ships a **mix**
//! of precomposed and decomposed Kannada vowel signs, and the decomposed spellings
//! are load-bearing for byte-for-byte output parity. Eight card words are
//! decomposed in the source: 11, 15, 17, 41, 45, 81, 90, and -- most visibly --
//! 10^7 (crore), whose second vowel is stored as U+0CCA U+0CD5 rather than the
//! precomposed U+0CCB. Crore appears in every 10^7+ result, so normalizing it
//! would silently break a large slice of the corpus while still *rendering*
//! identically on screen.
//!
//! Every Kannada string below is therefore written as explicit `\u{...}`
//! escapes, generated mechanically from the Python source. Escapes are pure
//! ASCII and survive editors/tools that would otherwise silently NFC-normalize
//! literal Kannada text (which is exactly what happened on the first draft of
//! this file). The human-readable spelling of each entry lives in the trailing
//! comment, where it is inert. **Do not "clean up" these escapes into literals.**
//!
//! Reassuringly, NFC does not change the *final* character of any card word, so
//! the `merge`/`to_ordinal` modifier tests below behave the same either way --
//! only the emitted bytes differ.
//!
//! # Faithfully reproduced Python quirks
//!
//! This is a port, not a rewrite. All of the following are exactly what Python
//! emits, verified against the interpreter over every card boundary in 0..10^10:
//!
//! 1. **`modifiers[3]` is dead code.** The list has 15 entries, but entry 3 is
//!    the *decomposed* two-codepoint sequence U+0CBF U+0CD5 rather than the
//!    precomposed U+0CC0 that entry 4 already holds. Both `merge` and
//!    `to_ordinal` test membership with a **single character**
//!    (`ltext[-1] in self.modifiers` / `outwords[-1] in ...`), and a one-char
//!    string can never equal a two-char one, so entry 3 can never match. It is
//!    preserved verbatim anyway: dropping it would be behaviourally identical,
//!    but keeping it documents the source. This is why [`MODIFIERS`] is
//!    `[&str; 15]` and not `[char; 15]`.
//!
//! 2. **`to_ordinal` truncates a real letter, not just a vowel sign.** Stripping
//!    the trailing modifier from the cardinal for 10^7 removes the final vowel
//!    of "koti", yielding "...kota-ne" instead of "...koti-ne". Same for every
//!    crore value. Wrong-looking, but it is the spec.
//!
//! 3. **`to_ordinal_num` prepended the digits to the full ordinal *words*
//!    (fixed, #224)**: Python's `"%s%s" % (value, self.to_ordinal(value))`
//!    gave "1ಒಂದನೇ". The port writes the numeral plus the ordinal suffix
//!    `to_ordinal` uses: "1ನೇ", "100ನೇ".
//!
//! 4. **Typos and inconsistent spacing in `low_numwords` (fixed,
//!    gladiaio/num2words2#247).** Python's table wrote 21..=44 and the 3s/4s
//!    of each later decade as two words or with a stray virama ("ಇಪ್ಪತ್ತ್
//!    ಮೂರು" for 23, "ಮೂವತ್ಎಂಟು" for 38, missing a ತ). Kannada writes each of
//!    them as one word: the ten drops its final -ು and a vowel-initial unit
//!    fuses as a vowel sign (ಇಪ್ಪತ್ತೊಂದು, ಇಪ್ಪತ್ತೈದು, ಮೂವತ್ತೆಂಟು); a
//!    consonant-initial one follows -ತ್ತ (ಇಪ್ಪತ್ತಮೂರು, ನಲವತ್ತನಾಲ್ಕು), the
//!    pattern the table already used for 53 (ಐವತ್ತಮೂರು). The 30
//!    corrected entries are marked `(#247)` and written NFC; the other
//!    entries, decomposed ones included, are untouched. (ಇಪ್ಪತ್ಮೂರು-style
//!    contractions are also attested; Omniglot/preply/multibhashi differ.)
//! 5. **The `"%s-%s"` hyphen branch of `merge` is unreachable.** Every value
//!    below 100 has its own card, so `splitnum` always resolves it with `div == 1`
//!    and `merge` takes the first branch instead. The branch is ported anyway to
//!    keep the arm structure identical to Python.
//!
//! 6. **KN's currency table is whatever `lang_EN` last mutated it into.**
//!    See "Currency" below. This is cross-*language* state leakage in the
//!    Python original and it is observable, so it is reproduced.
//!
//! 7. **EN's currency *precisions* do NOT leak, but its *forms* do.** The two
//!    sit in adjacent lines of the same `__init__` and behave differently. See
//!    "Currency" below.
//!
//! # Currency
//!
//! `Num2Word_KN` defines nothing currency-related. It inherits `pluralize` from
//! `Num2Word_EUR` and the whole `to_currency`/`to_cheque` machinery from
//! `Num2Word_Base`. The tables, however, are not what `lang_EUR.py` reads like.
//!
//! `Num2Word_EN.__init__` runs this:
//!
//! ```python
//! self.CURRENCY_FORMS["GBP"] = (("pound", "pounds"), ("penny", "pence"))
//! self.CURRENCY_FORMS["CHF"] = (("franc", "francs"), ("rappen", "rappen"))
//! ...
//! self.CURRENCY_PRECISION = {"BHD": 1000, "KWD": 1000, ...}
//! ```
//!
//! `Num2Word_EN` declares no `CURRENCY_FORMS` of its own, so `self.CURRENCY_FORMS`
//! resolves up the MRO to **`Num2Word_EUR`'s dict object** and `__setitem__`
//! mutates it **in place**. Every `Num2Word_EUR` subclass -- KN included --
//! shares that one dict, so EN's edits are permanently visible to KN. Because
//! `num2words2/__init__.py` instantiates every converter at import time,
//! `Num2Word_EN()` has always run before any KN call, and the table is fully
//! mutated by the time `to_currency` reads it. Net effect for KN:
//!
//!   * 39 codes instead of EUR's 23 (CHF/CNY/KWD/BHD/AED/... arrive from EN).
//!   * `EUR` is `("euro", "euros")`, not EUR's own `("euro", "euro")`.
//!   * `GBP` is `("pound", "pounds")`, not `("pound sterling", "pounds sterling")`.
//!   * `SAR` is `("riyal", "riyals")`, not `("saudi riyal", "saudi riyals")`.
//!
//! The **last** line is different in kind: `self.CURRENCY_PRECISION = {...}` is
//! an *assignment*, not a mutation, so it binds an instance attribute on the EN
//! object alone and never touches the class. KN therefore keeps
//! `Num2Word_Base.CURRENCY_PRECISION == {}` and gets divisor 100 for every
//! code -- including the 3-decimal (KWD/BHD) and 0-decimal (JPY) currencies.
//! Both halves of this asymmetry are confirmed against the frozen corpus.
//!
//! **Replaced by native nouns (gladiaio/num2words2#222).** That table is
//! English — even INR read "rupees"/"paise" — so [`CURRENCY_FORMS`] now holds
//! Kannada nouns for the codes with a sourced unit and subunit (EUR, GBP, INR,
//! JPY, USD; singular after a numeral, as Kannada usually writes "೧೦೦
//! ರೂಪಾಯಿ") and every other code raises NotImplementedError. The rest of this
//! section describes Python's table.
//!
//! [`CURRENCY_ADJECTIVES`] below was dumped from a live interpreter after a
//! full `import num2words2` (it happens to be unmutated). Its demonyms are
//! still English ("Indian"); that is open.
//!
//! # Errors
//!
//! KN raises only what the base class raises: `OverflowError` for
//! `abs(value) >= 10^10` (from `to_cardinal`), and `TypeError` from
//! `verify_ordinal` for negative ordinals. There are no crash-style
//! Index/Key/Value paths in this module.

use crate::base::{set_low_numwords, set_mid_numwords, Cards, Lang, N2WError, Result};
use crate::currency::CurrencyForms;
use crate::floatpath::FloatValue;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

// --- Data tables ---------------------------------------------------------
//
// Generated mechanically from lang_KN.py. Escapes, not literals -- see the
// "Unicode" section in the module docs above before touching these.

const LOW_NUMWORDS: [&str; 100] = [
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 99 = ತೊಂಬತ್ತೊಂಬತ್ತು
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 98 = ತೊಂಬತ್ತೆಂಟು
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 97 = ತೊಂಬತ್ತೇಳು
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 96 = ತೊಂಬತ್ತಾರು
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 95 = ತೊಂಬತ್ತೈದು
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 94 = ತೊಂಬತ್ತನಾಲ್ಕು (#247)
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 93 = ತೊಂಬತ್ತಮೂರು (#247)
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 92 = ತೊಂಬತ್ತೆರಡು
    "\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0ca6}\u{0cc1}", // 91 = ತೊಂಬತ್ತೊಂದು
    "\u{0ca4}\u{0cc6}\u{0cc2}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 90 = ತೊಂಬತ್ತು (decomposed)
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 89 = ಎಂಬತ್ತೊಂಬತ್ತು
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 88 = ಎಂಬತ್ತೆಂಟು
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 87 = ಎಂಬತ್ತೇಳು
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 86 = ಎಂಬತ್ತಾರು
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 85 = ಎಂಬತ್ತೈದು
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 84 = ಎಂಬತ್ತನಾಲ್ಕು (#247)
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 83 = ಎಂಬತ್ತಮೂರು (#247)
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 82 = ಎಂಬತ್ತೆರಡು
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cc2}\u{0c82}\u{0ca6}\u{0cc1}", // 81 = ಎಂಬತ್ತೊಂದು (decomposed)
    "\u{0c8e}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 80 = ಎಂಬತ್ತು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 79 = ಎಪ್ಪತ್ತೊಂಬತ್ತು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 78 = ಎಪ್ಪತ್ತೆಂಟು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 77 = ಎಪ್ಪತ್ತೇಳು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 76 = ಎಪ್ಪತ್ತಾರು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 75 = ಎಪ್ಪತ್ತೈದು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 74 = ಎಪ್ಪತ್ತನಾಲ್ಕು (#247)
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 73 = ಎಪ್ಪತ್ತಮೂರು (#247)
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 72 = ಎಪ್ಪತ್ತೆರಡು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0ca6}\u{0cc1}", // 71 = ಎಪ್ಪತ್ತೊಂದು
    "\u{0c8e}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 70 = ಎಪ್ಪತ್ತು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 69 = ಅರವತ್ತೊಂಬತ್ತು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 68 = ಅರವತ್ತೆಂಟು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 67 = ಅರವತ್ತೇಳು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 66 = ಅರವತ್ತಾರು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 65 = ಅರವತ್ತೈದು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 64 = ಅರವತ್ತನಾಲ್ಕು (#247)
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 63 = ಅರವತ್ತಮೂರು (#247)
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 62 = ಅರವತ್ತೆರಡು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0ca6}\u{0cc1}", // 61 = ಅರವತ್ತೊಂದು
    "\u{0c85}\u{0cb0}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 60 = ಅರವತ್ತು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 59 = ಐವತ್ತೊಂಬತ್ತು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 58 = ಐವತ್ತೆಂಟು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 57 = ಐವತ್ತೇಳು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 56 = ಐವತ್ತಾರು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 55 = ಐವತ್ತೈದು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 54 = ಐವತ್ತನಾಲ್ಕು (#247)
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 53 = ಐವತ್ತಮೂರು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 52 = ಐವತ್ತೆರಡು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0ca6}\u{0cc1}", // 51 = ಐವತ್ತೊಂದು
    "\u{0c90}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 50 = ಐವತ್ತು
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 49 = ನಲವತ್ತೊಂಬತ್ತು
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 48 = ನಲವತ್ತೆಂಟು
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 47 = ನಲವತ್ತೇಳು
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 46 = ನಲವತ್ತಾರು
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cd6}\u{0ca6}\u{0cc1}", // 45 = ನಲವತ್ತೈದು (decomposed)
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 44 = ನಲವತ್ತನಾಲ್ಕು (#247)
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 43 = ನಲವತ್ತಮೂರು (#247)
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 42 = ನಲವತ್ತೆರಡು (#247)
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cc2}\u{0c82}\u{0ca6}\u{0cc1}", // 41 = ನಲವತ್ತೊಂದು (decomposed)
    "\u{0ca8}\u{0cb2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 40 = ನಲವತ್ತು
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 39 = ಮೂವತ್ತೊಂಬತ್ತು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 38 = ಮೂವತ್ತೆಂಟು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 37 = ಮೂವತ್ತೇಳು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 36 = ಮೂವತ್ತಾರು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 35 = ಮೂವತ್ತೈದು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 34 = ಮೂವತ್ತನಾಲ್ಕು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 33 = ಮೂವತ್ತಮೂರು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 32 = ಮೂವತ್ತೆರಡು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0ca6}\u{0cc1}", // 31 = ಮೂವತ್ತೊಂದು (#247)
    "\u{0cae}\u{0cc2}\u{0cb5}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 30 = ಮೂವತ್ತು
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 29 = ಇಪ್ಪತ್ತೊಂಬತ್ತು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 28 = ಇಪ್ಪತ್ತೆಂಟು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc7}\u{0cb3}\u{0cc1}", // 27 = ಇಪ್ಪತ್ತೇಳು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cbe}\u{0cb0}\u{0cc1}", // 26 = ಇಪ್ಪತ್ತಾರು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc8}\u{0ca6}\u{0cc1}", // 25 = ಇಪ್ಪತ್ತೈದು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 24 = ಇಪ್ಪತ್ತನಾಲ್ಕು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 23 = ಇಪ್ಪತ್ತಮೂರು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 22 = ಇಪ್ಪತ್ತೆರಡು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0ca6}\u{0cc1}", // 21 = ಇಪ್ಪತ್ತೊಂದು (#247)
    "\u{0c87}\u{0caa}\u{0ccd}\u{0caa}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 20 = ಇಪ್ಪತ್ತು
    "\u{0cb9}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cca}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 19 = ಹತ್ತೊಂಬತ್ತು
    "\u{0cb9}\u{0ca6}\u{0cbf}\u{0ca8}\u{0cc6}\u{0c82}\u{0c9f}\u{0cc1}", // 18 = ಹದಿನೆಂಟು
    "\u{0cb9}\u{0ca6}\u{0cbf}\u{0ca8}\u{0cc6}\u{0cd5}\u{0cb3}\u{0cc1}", // 17 = ಹದಿನೇಳು (decomposed)
    "\u{0cb9}\u{0ca6}\u{0cbf}\u{0ca8}\u{0cbe}\u{0cb0}\u{0cc1}", // 16 = ಹದಿನಾರು
    "\u{0cb9}\u{0ca6}\u{0cbf}\u{0ca8}\u{0cc6}\u{0cd6}\u{0ca6}\u{0cc1}", // 15 = ಹದಿನೈದು (decomposed)
    "\u{0cb9}\u{0ca6}\u{0cbf}\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 14 = ಹದಿನಾಲ್ಕು
    "\u{0cb9}\u{0ca6}\u{0cbf}\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 13 = ಹದಿಮೂರು
    "\u{0cb9}\u{0ca8}\u{0ccd}\u{0ca8}\u{0cc6}\u{0cb0}\u{0ca1}\u{0cc1}", // 12 = ಹನ್ನೆರಡು
    "\u{0cb9}\u{0ca8}\u{0ccd}\u{0ca8}\u{0cc6}\u{0cc2}\u{0c82}\u{0ca6}\u{0cc1}", // 11 = ಹನ್ನೊಂದು (decomposed)
    "\u{0cb9}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 10 = ಹತ್ತು
    "\u{0c92}\u{0c82}\u{0cac}\u{0ca4}\u{0ccd}\u{0ca4}\u{0cc1}", // 9 = ಒಂಬತ್ತು
    "\u{0c8e}\u{0c82}\u{0c9f}\u{0cc1}", // 8 = ಎಂಟು
    "\u{0c8f}\u{0cb3}\u{0cc1}", // 7 = ಏಳು
    "\u{0c86}\u{0cb0}\u{0cc1}", // 6 = ಆರು
    "\u{0c90}\u{0ca6}\u{0cc1}", // 5 = ಐದು
    "\u{0ca8}\u{0cbe}\u{0cb2}\u{0ccd}\u{0c95}\u{0cc1}", // 4 = ನಾಲ್ಕು
    "\u{0cae}\u{0cc2}\u{0cb0}\u{0cc1}", // 3 = ಮೂರು
    "\u{0c8e}\u{0cb0}\u{0ca1}\u{0cc1}", // 2 = ಎರಡು
    "\u{0c92}\u{0c82}\u{0ca6}\u{0cc1}", // 1 = ಒಂದು
    "\u{0cb8}\u{0cca}\u{0ca8}\u{0ccd}\u{0ca8}\u{0cc6}", // 0 = ಸೊನ್ನೆ
];

const MODIFIERS: [&str; 15] = [
    "\u{0ccd}", // [0] ್
    "\u{0cbe}", // [1] ಾ
    "\u{0cbf}", // [2] ಿ
    "\u{0cbf}\u{0cd5}", // [3] ೀ  <- two codepoints: never matches a single-char test
    "\u{0cc0}", // [4] ೀ
    "\u{0cc1}", // [5] ು
    "\u{0cc2}", // [6] ೂ
    "\u{0cc3}", // [7] ೃ
    "\u{0cc6}", // [8] ೆ
    "\u{0cc7}", // [9] ೇ
    "\u{0cc8}", // [10] ೈ
    "\u{0cca}", // [11] ೊ
    "\u{0ccb}", // [12] ೋ
    "\u{0ccc}", // [13] ೌ
    "\u{0cd5}", // [14] ೕ
];

const ORDINAL_SUFFIX: &str = "\u{0ca8}\u{0cc7}"; // ನೇ
const GENITIVE: &str = "\u{0ca6}"; // ದ
#[allow(dead_code)] // kept from the port; currently unreferenced (#246)
const POINTWORD: &str = "\u{0cac}\u{0cbf}\u{0c82}\u{0ca6}\u{0cc1}"; // ಬಿಂದು
const HUNDRED: &str = "\u{0ca8}\u{0cc2}\u{0cb0}\u{0cc1}"; // ನೂರು
const THOUSAND: &str = "\u{0cb8}\u{0cbe}\u{0cb5}\u{0cbf}\u{0cb0}"; // ಸಾವಿರ
const LAKH: &str = "\u{0cb2}\u{0c95}\u{0ccd}\u{0cb7}"; // ಲಕ್ಷ
const CRORE: &str = "\u{0c95}\u{0cca}\u{0cd5}\u{0c9f}\u{0cbf}"; // ಕೋಟಿ (decomposed)

// --- Currency ------------------------------------------------------------
//
// See the "Currency" section of the module docs: Python read `Num2Word_EUR`'s
// CURRENCY_FORMS as mutated in place by `Num2Word_EN.__init__` (English); the
// forms below are Kannada (#222).
//
// Non-ASCII is escaped for the same reason the card tables are (see "Unicode"
// above): the escapes survive tools that would NFC-normalize literals.

/// `(code, unit_forms, subunit_forms)` in Kannada (#222). The noun stays
/// singular after a numeral, so both slots usually hold the same word.
const CURRENCY_FORMS: [(&str, &[&str], &[&str]); 5] = [
    ("EUR", &["\u{0caf}\u{0cc1}\u{0cb0}\u{0ccb}", "\u{0caf}\u{0cc1}\u{0cb0}\u{0ccb}"], &["\u{0cb8}\u{0cc6}\u{0c82}\u{0c9f}\u{0ccd}", "\u{0cb8}\u{0cc6}\u{0c82}\u{0c9f}\u{0ccd}"]),
    ("GBP", &["\u{0caa}\u{0ccc}\u{0c82}\u{0ca1}\u{0ccd}", "\u{0caa}\u{0ccc}\u{0c82}\u{0ca1}\u{0ccd}"], &["\u{0caa}\u{0cc6}\u{0ca8}\u{0ccd}\u{0ca8}\u{0cbf}", "\u{0caa}\u{0cc6}\u{0ca8}\u{0ccd}\u{0cb8}\u{0ccd}"]),
    ("INR", &["\u{0cb0}\u{0cc2}\u{0caa}\u{0cbe}\u{0caf}\u{0cbf}", "\u{0cb0}\u{0cc2}\u{0caa}\u{0cbe}\u{0caf}\u{0cbf}"], &["\u{0caa}\u{0cc8}\u{0cb8}\u{0cc6}", "\u{0caa}\u{0cc8}\u{0cb8}\u{0cc6}"]),
    ("JPY", &["\u{0caf}\u{0cc6}\u{0ca8}\u{0ccd}", "\u{0caf}\u{0cc6}\u{0ca8}\u{0ccd}"], &["\u{0cb8}\u{0cc6}\u{0ca8}\u{0ccd}", "\u{0cb8}\u{0cc6}\u{0ca8}\u{0ccd}"]),
    ("USD", &["\u{0ca1}\u{0cbe}\u{0cb2}\u{0cb0}\u{0ccd}", "\u{0ca1}\u{0cbe}\u{0cb2}\u{0cb0}\u{0ccd}"], &["\u{0cb8}\u{0cc6}\u{0c82}\u{0c9f}\u{0ccd}", "\u{0cb8}\u{0cc6}\u{0c82}\u{0c9f}\u{0ccd}"]),
];

/// `Num2Word_EUR.CURRENCY_ADJECTIVES` -- 16 entries, **not** mutated by anyone
/// (verified against the live interpreter: it is still EUR's dict, unchanged).
///
/// A `const` slice rather than a map: it is consulted only when
/// `adjective=true`, and 16 entries make a linear scan cheaper than hashing.
/// Being `const` it also costs nothing to construct.
const CURRENCY_ADJECTIVES: [(&str, &str); 16] = [
    ("AUD", "Australian"),
    ("BYN", "Belarusian"),
    ("CAD", "Canadian"),
    ("EEK", "Estonian"),
    ("HUF", "Hungarian"),
    ("INR", "Indian"),
    ("ISK", "\u{00ed}slenskar"), // islenskar
    ("JPY", "Japanese"),
    ("KRW", "Korean"),
    ("MXN", "Mexican"),
    ("NOK", "Norwegian"),
    ("RON", "Romanian"),
    ("RUB", "Russian"),
    ("SAR", "Saudi"),
    ("USD", "US"),
    ("UZS", "Uzbekistan"),
];

// --- Helpers -------------------------------------------------------------

/// Python's `x[-1] in self.modifiers`.
///
/// Python indexes by *character*, so this takes the final `char` and tests it
/// against the table as a one-character string. An empty `s` would be an
/// `IndexError` in Python; neither call site can produce one (every card word is
/// non-empty and `merge` only ever concatenates non-empty text), so `false` here
/// is unreachable rather than a papered-over divergence.
fn ends_with_modifier(s: &str) -> bool {
    match s.chars().next_back() {
        Some(c) => {
            let mut buf = [0u8; 4];
            let last: &str = c.encode_utf8(&mut buf);
            MODIFIERS.contains(&last)
        }
        None => false,
    }
}

/// Python's `x[:-1]` -- drop the final *character*, not the final byte.
fn strip_last_char(s: &str) -> String {
    let mut it = s.chars();
    it.next_back();
    it.as_str().to_string()
}

// --- Language ------------------------------------------------------------

pub struct LangKn {
    cards: Cards,
    maxval: BigInt,
    /// Built once in `new()`. Python resolves `self.CURRENCY_FORMS[code]` via a
    /// dict lookup on a table that already exists; rebuilding this per call
    /// would be pure overhead (and made an earlier revision of this port an
    /// order of magnitude slower than the Python it replaces).
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangKn {
    fn default() -> Self {
        Self::new()
    }
}

impl LangKn {
    pub fn new() -> Self {
        let mut cards = Cards::new();

        // KN's set_high_numwords:
        //     for n, word in self.high_numwords: self.cards[10**n] = word
        // with high_numwords = [(7, crore), (5, lakh), (3, thousand)].
        for (n, word) in [(7u32, CRORE), (5, LAKH), (3, THOUSAND)] {
            cards.insert(BigInt::from(10u8).pow(n), word);
        }

        set_mid_numwords(&mut cards, &[(100, HUNDRED)]);
        set_low_numwords(&mut cards, &LOW_NUMWORDS);

        // Num2Word_Base.__init__: MAXVAL = 1000 * list(self.cards.keys())[0].
        // The OrderedDict is filled high -> mid -> low, so key[0] is the highest
        // card (10^7) and MAXVAL lands on 10^10.
        let maxval = cards.highest().cloned().unwrap_or_else(BigInt::zero) * BigInt::from(1000);

        let currency_forms = CURRENCY_FORMS
            .iter()
            .map(|(code, unit, subunit)| (*code, CurrencyForms::new(unit, subunit)))
            .collect();

        LangKn {
            cards,
            maxval,
            currency_forms,
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`.
    ///
    /// The float check (`errmsg_floatord`) cannot fire on integer input, so only
    /// the negative check is modelled.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }
}

impl Lang for LangKn {
    /// This language's own `to_currency(currency=...)` default,
    /// read from the live Python signature. Only 44 of 156 use EUR.
    fn default_currency(&self) -> &str {
        "EUR"
    }

    /// This language's own `to_currency(separator=...)` default,
    /// read from the live Python signature. Base's is ",", but only
    /// 36 of 149 languages actually use it — most default to " " or a
    /// conjunction, so inheriting Base's comma silently corrupts them.
    fn default_separator(&self) -> &str {
        ","
    }

    fn cards(&self) -> &Cards {
        &self.cards
    }

    fn maxval(&self) -> &BigInt {
        &self.maxval
    }

    // negword: KN does not override Num2Word_Base's "(-) " -> trait default.
    // is_title / exclude_title: KN does not override -> trait defaults.

    fn pointword(&self) -> &str {
        "ಬಿಂದು"
    }

    /// Port of `Num2Word_KN.merge`.
    ///
    /// ```python
    /// if lnum == 1 and rnum < 100:  return (rtext, rnum)
    /// elif 100 > lnum > rnum:       return ("%s-%s" % (ltext, rtext), lnum + rnum)
    /// elif lnum >= 100 > rnum:
    ///     if ltext[-1] in self.modifiers:
    ///         return ("%s %s" % (ltext[:-1], rtext), lnum + rnum)
    ///     else:
    ///         return ("%s %s" % (ltext + GENITIVE, rtext), lnum + rnum)
    /// elif rnum > lnum:             return ("%s %s" % (ltext, rtext), lnum * rnum)
    /// return ("%s %s" % (ltext, rtext), lnum + rnum)
    /// ```
    ///
    /// The third arm drives the sandhi: a left word ending in a vowel sign loses
    /// it (100 + 1 -> "ondu nura ondu"), otherwise the genitive "da" is suffixed
    /// (2000 + 1 -> "eradu savirada ondu").
    ///
    /// Python's `100 > lnum > rnum` and `lnum >= 100 > rnum` are chained
    /// comparisons; they are expanded here into explicit conjunctions.
    fn merge(&self, l: (&str, &BigInt), r: (&str, &BigInt)) -> (String, BigInt) {
        let (ltext, lnum) = l;
        let (rtext, rnum) = r;
        let hundred = BigInt::from(100);

        if lnum.is_one() && rnum < &hundred {
            (rtext.to_string(), rnum.clone())
        } else if &hundred > lnum && lnum > rnum {
            // Unreachable for KN (every value < 100 is its own card), but ported
            // to keep the arm structure identical. See quirk 5 in the module docs.
            (format!("{}-{}", ltext, rtext), lnum + rnum)
        } else if lnum >= &hundred && &hundred > rnum {
            if ends_with_modifier(ltext) {
                (format!("{} {}", strip_last_char(ltext), rtext), lnum + rnum)
            } else {
                (format!("{}{} {}", ltext, GENITIVE, rtext), lnum + rnum)
            }
        } else if rnum > lnum {
            (format!("{} {}", ltext, rtext), lnum * rnum)
        } else {
            (format!("{} {}", ltext, rtext), lnum + rnum)
        }
    }

    /// Port of `Num2Word_KN.to_ordinal`.
    ///
    /// ```python
    /// self.verify_ordinal(value)
    /// outwords = self.to_cardinal(value)
    /// if outwords[-1] in self.modifiers:
    ///     outwords = outwords[:-1]
    /// return outwords + ORDINAL_SUFFIX
    /// ```
    ///
    /// Strips at most **one** character, unconditionally, whenever the cardinal
    /// ends in a vowel sign -- which is why the crore ordinal is mangled (quirk 2).
    /// Cardinals ending in a bare consonant (2000, 10^5) are left alone and simply
    /// take the suffix.
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        let outwords = self.to_cardinal(value)?;
        let stem = if ends_with_modifier(&outwords) {
            strip_last_char(&outwords)
        } else {
            outwords
        };
        Ok(format!("{}{}", stem, ORDINAL_SUFFIX))
    }

    /// `to_ordinal_num`: the numeral plus the ordinal suffix `to_ordinal`
    /// uses, "1ನೇ", "100ನೇ" (quirk 3, fixed). Negatives raise like
    /// `to_ordinal`.
    fn to_ordinal_num(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;
        Ok(format!("{}{}", value, ORDINAL_SUFFIX))
    }

    // to_year: KN does not override Num2Word_Base.to_year, which delegates to
    // to_cardinal -> the trait default is correct. There is no BC/CE handling.

    /// `to_ordinal(float/Decimal)` — `verify_ordinal` gates the float domain:
    /// a fractional value raises TypeError ("Cannot treat float %s as
    /// ordinal.") before the negative check ("Cannot treat negative num %s
    /// as ordinal."); -0.0 passes both (`abs(-0.0) == -0.0`). A surviving
    /// whole value takes the integer ordinal path — base `to_cardinal`
    /// routes a whole float through `int(value)` — so `5.00` is "ಐದನೇ" and
    /// `1e+12` still overflows inside `to_cardinal` (MAXVAL is 10**10).
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let i = verify_ordinal_float(value, &ordinal_float_repr(value))?;
        self.to_ordinal(&i)
    }

    /// `to_ordinal_num(float/Decimal)` — the same gate, then the repr plus
    /// the ordinal suffix ("5.0ನೇ"), like the integer path.
    fn ordinal_num_float_entry(&self, value: &FloatValue, repr_str: &str) -> Result<String> {
        verify_ordinal_float(value, repr_str)?;
        Ok(format!("{}{}", repr_str, ORDINAL_SUFFIX))
    }

    // ---- currency --------------------------------------------------------
    //
    // `Num2Word_KN` defines *nothing* currency-related itself. Everything is
    // inherited: the tables from `Num2Word_EUR` (as mutated at import time --
    // see the module docs), `pluralize` from `Num2Word_EUR`, and
    // `to_currency`/`to_cheque`/`_money_verbose`/`_cents_verbose`/
    // `_cents_terse` from `Num2Word_Base`. So only the three data hooks plus
    // `pluralize` and `lang_name` are overridden here; the rest of the trait
    // defaults already mirror `Num2Word_Base` exactly.

    /// Drives the NotImplementedError text:
    /// `Currency code "XXX" not implemented for "Num2Word_KN"`.
    fn lang_name(&self) -> &str {
        "Num2Word_KN"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    fn currency_adjective(&self, code: &str) -> Option<&str> {
        CURRENCY_ADJECTIVES
            .iter()
            .find(|(k, _)| *k == code)
            .map(|(_, v)| *v)
    }

    // currency_precision: deliberately NOT overridden. `Num2Word_KN` inherits
    // `Num2Word_Base.CURRENCY_PRECISION`, which is `{}` -- so
    // `CURRENCY_PRECISION.get(code, 100)` is 100 for *every* code and the trait
    // default (100) is already exact.
    //
    // This looks wrong for KWD/BHD (3-decimal) and JPY (0-decimal), and it is
    // -- but it is what Python does, and the corpus agrees. The reason is quirk
    // 7 in the module docs: `Num2Word_EN.__init__` *assigns*
    // `self.CURRENCY_PRECISION = {...}`, which binds an INSTANCE attribute on
    // the EN object only, whereas its sibling `self.CURRENCY_FORMS[k] = v`
    // lines MUTATE the shared class dict. So EN's forms leak into KN but EN's
    // precisions do not. Consequently KN renders KWD 12.34 as
    // "<12> dinars, <34> fils" (divisor 100, not 1000) and JPY 12.34 with a
    // sen segment instead of rounding to a whole yen. Verified against the
    // frozen corpus for all of JPY/KWD/BHD.

    /// Port of `Num2Word_EUR.pluralize`.
    ///
    /// ```python
    /// def pluralize(self, n, forms):
    ///     form = 0 if n == 1 else 1
    ///     return forms[form]
    /// ```
    ///
    /// Index 2 is never reached, so PLN's "zlotu" and RON's "de lei"/"de bani"
    /// are dead weight for KN -- but the arity is kept (see [`CURRENCY_FORMS`]).
    ///
    /// The out-of-range arm mirrors Python's `IndexError`, message included --
    /// `forms` is a tuple in Python, so the text is exactly
    /// "tuple index out of range". It is unreachable with the tables above
    /// (every entry has >= 2 forms), but `forms` is a caller-supplied slice, so
    /// the failure mode is modelled rather than papered over with a panic or a
    /// silent fallback.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let form = if n.is_one() { 0 } else { 1 };
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| N2WError::Index("tuple index out of range".into()))
    }

    // money_verbose / cents_verbose: Num2Word_Base's versions just delegate to
    // to_cardinal, which is what the trait defaults do. Not overridden.
    //
    // cents_terse: Num2Word_Base._cents_terse zero-pads to
    // `len(str(divisor)) - 1` digits; with divisor 100 that is "%02d". The
    // trait default calls default_cents_terse(n, self.currency_precision(c)),
    // which is the same function. Not overridden.
    //
    // cardinal_from_decimal: left at the default (raises NotImplemented). It is
    // reached only for fractional cents -- e.g. to_currency(12.345, "EUR"),
    // where Python's base falls through to to_cardinal_float(34.5) and yields
    // "<34> <pointword> <5> cents". That is the float/Decimal cardinal path,
    // which PORTING_CURRENCY.md defers to a later phase; no corpus row reaches
    // it (no corpus currency value has a third decimal). Flagged in the report.
}

/// Python's `Num2Word_Base.verify_ordinal` over the float/Decimal domain:
///
/// ```python
/// if not value == int(value):
///     raise TypeError(self.errmsg_floatord % value)
/// if not abs(value) == value:
///     raise TypeError(self.errmsg_negord % value)
/// ```
///
/// The float check runs first, so `-1.5` reports the *float* message; the
/// negative check compares numerically, so `-0.0` (== `abs(-0.0)`) passes.
/// `repr` is Python's `str(value)`, interpolated verbatim into the message.
fn verify_ordinal_float(value: &FloatValue, repr: &str) -> Result<BigInt> {
    match value.as_whole_int() {
        None => Err(N2WError::Type(format!(
            "Cannot treat float {} as ordinal.",
            repr
        ))),
        Some(i) if i.is_negative() => Err(N2WError::Type(format!(
            "Cannot treat negative num {} as ordinal.",
            repr
        ))),
        Some(i) => Ok(i),
    }
}

/// Best-effort `str(value)` for the TypeError messages raised by
/// [`verify_ordinal_float`] when no repr was handed in (the `to_ordinal`
/// entry). Exact for every reachable message: `str(Decimal)` is the spec
/// transcription, a fractional float's shortest round-trip matches Rust's
/// `Display` in the non-exponent range, and a whole float re-gains its
/// Python ".0" tail. (A whole float >= 1e16 would diverge — Python uses
/// exponent form — but such values pass verification and never reach a
/// message.)
fn ordinal_float_repr(value: &FloatValue) -> String {
    match value {
        FloatValue::Decimal { value, .. } => crate::strnum::python_decimal_str(value),
        FloatValue::Float { value, .. } => {
            if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e16 {
                format!("{:.1}", value)
            } else {
                value.to_string()
            }
        }
    }
}
