//! Port of `lang_UK.py` (Ukrainian). Registry key `"uk"` → `Num2Word_UK`
//! (verified in `num2words2/__init__.py`: `"uk": lang_UK.Num2Word_UK()`).
//!
//! Shape: **self-contained**. `Num2Word_UK` subclasses `Num2Word_Base` but
//! defines no `high_numwords`/`mid_numwords`/`low_numwords`, so Python never
//! populates `self.cards` and `MAXVAL` is never consulted. `to_cardinal` is
//! overridden outright and drives `_int2word` over 3-digit chunks, so
//! `cards`/`merge` stay at their trait defaults here. The `THOUSANDS` /
//! `prefixes_ordinal` tables (keys 1..=10) end at 10^30 "нонільйон"; past
//! that Python raised `KeyError`. Fixed (gladiaio/num2words2#159): `maxval()`
//! is 10^33 and values at or above it raise `OverflowError` (quirk 8).
//!
//! Inherited from `Num2Word_Base`, unchanged by UK:
//!   * `verify_ordinal(value)` — raises `TypeError` for negatives. UK's
//!     `to_ordinal` calls it *first*, so negative ordinals are `TypeError`
//!     here, **not** the `ValueError` that `lang_PL` produces (PL skips
//!     `verify_ordinal` and lets `int("-")` blow up instead). Confirmed
//!     against the corpus: `ordinal(-1)` … `ordinal(-1000000)` → `TypeError`.
//!   * `to_ordinal_num(value) -> value` → the trait default `value.to_string()`.
//!   * `to_year(value) -> self.to_cardinal(value)` → the trait default
//!     delegates through `&self` and picks up the `to_cardinal` override
//!     below. UK has no year-specific logic, so `to_year(1999)` is just the
//!     cardinal "одна тисяча дев'ятсот дев'яносто дев'ять", and `to_year(-500)`
//!     is "мінус п'ятсот".
//!   * `setup()` only assigns `negword`/`pointword`; there is **no cross-call
//!     mutable state** anywhere in this module.
//!
//! # Quirks reproduced verbatim
//!
//! This is a port, not a rewrite. All of the following were checked against
//! the interpreter and are exactly what Python emits:
//!
//! 1. `to_ordinal(0)` raises `IndexError`. `splitbyx("0", 3)` yields `[0]`;
//!    the `while last == 0` loop pops the only fragment and then evaluates
//!    `fragments[-1]` on an empty list.
//! 2. The level-suffix concatenation in `to_ordinal` has **no separator**:
//!    `to_ordinal(100000)` == "стотисячний", `to_ordinal(21000)` ==
//!    "двадцятиоднотисячний". For Ukrainian this is actually the correct
//!    morphology (the `[1]` table forms are bound stems), unlike the
//!    equivalent PL code path — but it is still literal concatenation.
//! 3. `if last == 1 and level > 0 and output != "": output += " "` inserts a
//!    *space* before the suffix in the one case where a `pre_part` exists and
//!    the surviving fragment is 1 — e.g. `to_ordinal(1001000)` ==
//!    "один мільйон тисячний" and `to_ordinal(1000001000)` ==
//!    "один мільярд тисячний". So the suffix is glued for every other input
//!    but spaced for these. Verified.
//! 4. `_int2word` line 652 carries a commented-out
//!    `# elif n1 > 0 and not (i > 0 and x == 1)` and uses a plain `elif n1 > 0`
//!    instead. The live code therefore never elides the leading "one" of a
//!    thousands chunk: `to_cardinal(1000)` == "одна тисяча" (not "тисяча")
//!    and `to_cardinal(1000000)` == "один мільйон". Corpus-confirmed.
//! 5. `THOUSANDS[1][1][0]` is "тисячи" — a Russianism; the Ukrainian genitive
//!    is "тисячі" (which the module does use at `[5][0]`). Preserved as-is.
//!    Unreachable in this scope: `morphological_case` is always 0 (see below).
//! 6. `TWENTIES_ORDINALS[9][1]` is "дев'яности"; the standard form is
//!    "дев'яноста". Preserved as-is, and it *is* reachable —
//!    `to_ordinal(90000)` == "дев'яноститисячний".
//! 7. `ONES_FEMININE` is selected by chunk index (`i == 1`, the thousands
//!    chunk) because "тисяча" is feminine — hence "одна тисяча" / "дві тисячі"
//!    but "один мільйон" / "два мільйони". Correct Ukrainian, noted because
//!    the condition looks like a gender bug at a glance.
//! 8. Fixed (gladiaio/num2words2#159): `THOUSANDS`/`prefixes_ordinal` stop
//!    at key 10, so Python's `to_cardinal(10**33)` and `to_ordinal(10**33)`
//!    raised `KeyError: 11`. `_int2word` and `to_ordinal` now check
//!    `abs(n) >= 10**33` first and raise `OverflowError`, and `maxval()`
//!    reports 10^33.
//!
//! # Kwargs
//!
//! Python's `to_cardinal(number, **kwargs)` accepts `gender` (feminine
//! aliases `{"f", "feminine", "ж", "жіночий", "женский"}`, compared after
//! `str.lower()`; anything else is masculine) and `case` (one of six
//! morphological cases, resolved via `[...].index(case)`).
//!
//! * `gender` is served by `to_cardinal_kw` / `to_cardinal_float_kw`
//!   (gladiaio/num2words2#145); for float/Decimal input it applies to both
//!   sides of the point, as in Python.
//! * `case` is not served yet: the trait hooks decline it, so
//!   `morphological_case` is always 0 (nominative). The full six-form tables
//!   are transcribed anyway so the data is complete and reviewable.
//!
//! Integer currency picks the gender per currency (`FEMININE_MONEY` /
//! `FEMININE_CENTS`) and takes no kwargs.
//!
//! # Currency surface
//!
//! `Num2Word_UK` subclasses `Num2Word_Base` **directly** — there is no
//! `lang_EU`/`lang_EUR` in the chain — so `CURRENCY_ADJECTIVES` and
//! `CURRENCY_PRECISION` are both the empty dicts from `Num2Word_Base` and are
//! never merged with anything. Consequences, all corpus-confirmed:
//!
//!   * `adjective=True` is a **no-op** (nothing to prefix).
//!   * The divisor is **always 100**. KWD/BHD are *not* treated as 3-decimal
//!     and JPY is *not* treated as 0-decimal, so `to_currency(12.34, "JPY")`
//!     renders a cents segment ("…тридцять чотири сен") and
//!     `to_cheque(1234.56, "JPY")` prints "56/100". Do not "correct" this.
//!
//! `to_currency` is overridden for the `isinstance(val, int)` case only; the
//! float case and `to_cheque` fall through to `Num2Word_Base`. See the
//! `to_currency` doc comment for the quirks the int path carries; its
//! number/gender agreement now matches the float path (#164).
//!
//! ## Fractional cents (out of scope, flagged)
//!
//! `Num2Word_Base.to_currency`'s fractional-cents branch is reachable for UK —
//! `to_currency(12.345, "EUR")` returns "дванадцять євро, тридцять чотири кома
//! п'ять центи" in Python, because `Num2Word_UK.to_cardinal` accepts a float
//! (`str(number)` contains a "."). That float path is a later phase, so
//! `cardinal_from_decimal` stays at the trait default and this input raises
//! `NotImplemented` instead. No corpus row exercises it.
//!
//! Two things must be fixed together when that phase lands, or the output will
//! be subtly wrong:
//!   1. Implement `cardinal_from_decimal` (the `"." in n` branch of
//!      `Num2Word_UK.to_cardinal`, including its leading-zero expansion:
//!      `(ZERO[0] + " ") * leading_zero_count`).
//!   2. `currency::default_to_currency` closes that branch with
//!      `pluralize(right_int, cr2)`, but Python uses `cr2[1]`
//!      **unconditionally** there — it skips `pluralize` entirely. The two
//!      diverge whenever `pluralize` would not pick form 1, e.g.
//!      `to_currency(12.015, "EUR")` → Python "один кома п'ять центи"
//!      (`cr2[1]`), while `pluralize(1, cr2)` would give "цент". Reproducing
//!      that needs the float path implemented inline here rather than
//!      delegating, or a fix in `currency.rs`.
//!
//! # Error variants
//!
//! `N2WError::Type` for negative ordinals (a deliberate `raise` in
//! `verify_ordinal`), `N2WError::Index` for `to_ordinal(0)`, and
//! `N2WError::Key` past 10^30. The latter two are Python *crashes* rather than
//! deliberate raises, but the exception type is observable behaviour a caller
//! may catch, so parity means reproducing the type rather than tidying it
//! into a `TypeError`.

use std::sync::OnceLock;
use crate::base::{check_maxval, pow10_big, KwVal, Kwargs, Lang, N2WError, Result};
use crate::currency::{CurrencyForms, CurrencyValue};
use crate::floatpath::FloatValue;
use crate::strnum::{python_decimal_parse, python_decimal_str, ParsedNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::collections::HashMap;

const ZERO: &str = "нуль";
const NEGWORD: &str = "мінус";
const POINTWORD: &str = "кома";

/// `ONES_FEMININE`, keys 1..=9 × 6 morphological cases.
/// Index 0 is absent in Python (guarded by `n1 > 0`).
static ONES_FEMININE: [[&str; 6]; 10] = [
    ["", "", "", "", "", ""], // absent in Python
    ["одна", "однієї", "одній", "одну", "однією", "одній"],
    ["дві", "двох", "двом", "дві", "двома", "двох"],
    ["три", "трьох", "трьом", "три", "трьома", "трьох"],
    ["чотири", "чотирьох", "чотирьом", "чотири", "чотирма", "чотирьох"],
    ["п'ять", "п'яти", "п'яти", "п'ять", "п'ятьма", "п'яти"],
    ["шість", "шести", "шести", "шість", "шістьма", "шести"],
    ["сім", "семи", "семи", "сім", "сьома", "семи"],
    ["вісім", "восьми", "восьми", "вісім", "вісьма", "восьми"],
    ["дев'ять", "дев'яти", "дев'яти", "дев'ять", "дев'ятьма", "дев'яти"],
];

/// `ONES`, keys 1..=9 × 6 cases. Differs from `ONES_FEMININE` only at key 1.
static ONES: [[&str; 6]; 10] = [
    ["", "", "", "", "", ""], // absent in Python
    ["один", "одного", "одному", "один", "одним", "одному"],
    ["два", "двох", "двом", "два", "двома", "двох"],
    ["три", "трьох", "трьом", "три", "трьома", "трьох"],
    ["чотири", "чотирьох", "чотирьом", "чотири", "чотирма", "чотирьох"],
    ["п'ять", "п'яти", "п'яти", "п'ять", "п'ятьма", "п'яти"],
    ["шість", "шести", "шести", "шість", "шістьма", "шести"],
    ["сім", "семи", "семи", "сім", "сьома", "семи"],
    ["вісім", "восьми", "восьми", "вісім", "вісьма", "восьми"],
    ["дев'ять", "дев'яти", "дев'яти", "дев'ять", "дев'ятьма", "дев'яти"],
];

/// `ONES_ORDINALS`, keys 1..=19 × 2 forms (`[0]` free / `[1]` bound stem).
/// Index 0 is absent in Python and unreachable (`last_two == 0` is handled
/// by the `HUNDREDS_ORDINALS` arm before this table is touched).
static ONES_ORDINALS: [[&str; 2]; 20] = [
    ["", ""], // absent in Python
    ["перший", "одно"],
    ["другий", "двох"],
    ["третій", "трьох"],
    ["четвертий", "чотирьох"],
    ["п'ятий", "п'яти"],
    ["шостий", "шести"],
    ["сьомий", "семи"],
    ["восьмий", "восьми"],
    ["дев'ятий", "дев'яти"],
    ["десятий", "десяти"],
    ["одинадцятий", "одинадцяти"],
    ["дванадцятий", "дванадцяти"],
    ["тринадцятий", "тринадцяти"],
    ["чотирнадцятий", "чотирнадцяти"],
    ["п'ятнадцятий", "п'ятнадцяти"],
    ["шістнадцятий", "шістнадцяти"],
    ["сімнадцятий", "сімнадцяти"],
    ["вісімнадцятий", "вісімнадцяти"],
    ["дев'ятнадцятий", "дев'ятнадцяти"],
];

/// `TENS`, keys 0..=9 × 6 cases — the 10..19 teens, keyed by the *units*
/// digit (`TENS[n1]` when `n2 == 1`).
static TENS: [[&str; 6]; 10] = [
    ["десять", "десяти", "десяти", "десять", "десятьма", "десяти"],
    [
        "одинадцять",
        "одинадцяти",
        "одинадцяти",
        "одинадцять",
        "одинадцятьма",
        "одинадцяти",
    ],
    [
        "дванадцять",
        "дванадцяти",
        "дванадцяти",
        "дванадцять",
        "дванадцятьма",
        "дванадцяти",
    ],
    [
        "тринадцять",
        "тринадцяти",
        "тринадцяти",
        "тринадцять",
        "тринадцятьма",
        "тринадцяти",
    ],
    [
        "чотирнадцять",
        "чотирнадцяти",
        "чотирнадцяти",
        "чотирнадцять",
        "чотирнадцятьма",
        "чотирнадцяти",
    ],
    [
        "п'ятнадцять",
        "п'ятнадцяти",
        "п'ятнадцяти",
        "п'ятнадцять",
        "п'ятнадцятьма",
        "п'ятнадцяти",
    ],
    [
        "шістнадцять",
        "шістнадцяти",
        "шістнадцяти",
        "шістнадцять",
        "шістнадцятьма",
        "шістнадцяти",
    ],
    [
        "сімнадцять",
        "сімнадцяти",
        "сімнадцяти",
        "сімнадцять",
        "сімнадцятьма",
        "сімнадцяти",
    ],
    [
        "вісімнадцять",
        "вісімнадцяти",
        "вісімнадцяти",
        "вісімнадцять",
        "вісімнадцятьма",
        "вісімнадцяти",
    ],
    [
        "дев'ятнадцять",
        "дев'ятнадцяти",
        "дев'ятнадцяти",
        "дев'ятнадцять",
        "дев'ятнадцятьма",
        "дев'ятнадцяти",
    ],
];

/// `TWENTIES`, keys 2..=9 × 6 cases. Indices 0/1 are absent in Python
/// (guarded by `n2 > 1`).
static TWENTIES: [[&str; 6]; 10] = [
    ["", "", "", "", "", ""], // absent in Python
    ["", "", "", "", "", ""], // absent in Python
    [
        "двадцять",
        "двадцяти",
        "двадцяти",
        "двадцять",
        "двадцятьма",
        "двадцяти",
    ],
    [
        "тридцять",
        "тридцяти",
        "тридцяти",
        "тридцять",
        "тридцятьма",
        "тридцяти",
    ],
    ["сорок", "сорока", "сорока", "сорок", "сорока", "сорока"],
    [
        "п'ятдесят",
        "п'ятдесяти",
        "п'ятдесяти",
        "п'ятдесят",
        "п'ятдесятьма",
        "п'ятдесяти",
    ],
    [
        "шістдесят",
        "шістдесяти",
        "шістдесяти",
        "шістдесят",
        "шістдесятьма",
        "шістдесяти",
    ],
    [
        "сімдесят",
        "сімдесяти",
        "сімдесяти",
        "сімдесят",
        "сімдесятьма",
        "сімдесяти",
    ],
    [
        "вісімдесят",
        "вісімдесяти",
        "вісімдесяти",
        "вісімдесят",
        "вісімдесятьма",
        "вісімдесяти",
    ],
    [
        "дев'яносто",
        "дев'яноста",
        "дев'яноста",
        "дев'яносто",
        "дев'яностами",
        "дев'яноста",
    ],
];

/// `TWENTIES_ORDINALS`, keys 2..=9 × 2 forms.
/// Key 9's bound stem is "дев'яности" (standard would be "дев'яноста") — sic.
static TWENTIES_ORDINALS: [[&str; 2]; 10] = [
    ["", ""], // absent in Python
    ["", ""], // absent in Python
    ["двадцятий", "двадцяти"],
    ["тридцятий", "тридцяти"],
    ["сороковий", "сорока"],
    ["п'ятдесятий", "п'ятдесяти"],
    ["шістдесятий", "шістдесяти"],
    ["сімдесятий", "сімдесяти"],
    ["вісімдесятий", "вісімдесяти"],
    ["дев'яностий", "дев'яности"], // sic — see module docs, quirk 6
];

/// `HUNDREDS`, keys 1..=9 × 6 cases. Index 0 absent (guarded by `n3 > 0`).
static HUNDREDS: [[&str; 6]; 10] = [
    ["", "", "", "", "", ""], // absent in Python
    ["сто", "ста", "ста", "сто", "стами", "стах"],
    ["двісті", "двохста", "двомстам", "двісті", "двомастами", "двохстах"],
    [
        "триста",
        "трьохста",
        "трьомстам",
        "триста",
        "трьомастами",
        "трьохстах",
    ],
    [
        "чотириста",
        "чотирьохста",
        "чотирьомстам",
        "чотириста",
        "чотирмастами",
        "чотирьохстах",
    ],
    [
        "п'ятсот",
        "п'ятиста",
        "п'ятистам",
        "п'ятсот",
        "п'ятьмастами",
        "п'ятистах",
    ],
    [
        "шістсот",
        "шестиста",
        "шестистам",
        "шістсот",
        "шістьмастами",
        "шестистах",
    ],
    ["сімсот", "семиста", "семистам", "сімсот", "сьомастами", "семистах"],
    [
        "вісімсот",
        "восьмиста",
        "восьмистам",
        "вісімсот",
        "восьмастами",
        "восьмистах",
    ],
    [
        "дев'ятсот",
        "дев'ятиста",
        "дев'ятистам",
        "дев'ятсот",
        "дев'ятьмастами",
        "дев'ятистах",
    ],
];

/// `HUNDREDS_ORDINALS`, keys 1..=9 × 2 forms.
///
/// Note: unlike `lang_PL`'s one-form `HUNDREDS`, these tuples genuinely have
/// two entries, so `last_fragment_to_ordinal`'s `[1]` indexing is safe here —
/// UK has no counterpart to PL's `IndexError` bug.
static HUNDREDS_ORDINALS: [[&str; 2]; 10] = [
    ["", ""], // absent in Python
    ["сотий", "сто"],
    ["двохсотий", "двохсот"],
    ["трьохсотий", "трьохсот"],
    ["чотирьохсотий", "чотирьохсот"],
    ["п'ятисотий", "п'ятсот"],
    ["шестисотий", "шістсот"],
    ["семисотий", "сімсот"],
    ["восьмисотий", "вісімсот"],
    ["дев'ятисотий", "дев'ятсот"],
];

/// `THOUSANDS`: chunk index 1..=10 (10^3 … 10^30) × 6 cases × 3 plural forms.
/// Index 0 is absent in Python and unreachable (guarded by `i > 0`).
/// A chunk index of 11 or more would be a `KeyError`; the 10^33 MAXVAL
/// check (#159) keeps it unreachable.
static THOUSANDS: [[[&str; 3]; 6]; 11] = [
    // 0 — absent in Python
    [
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
    ],
    // 10^3
    [
        ["тисяча", "тисячі", "тисяч"],
        ["тисячи", "тисяч", "тисяч"], // "тисячи" sic — see module docs, quirk 5
        ["тисячі", "тисячам", "тисячам"],
        ["тисячу", "тисячі", "тисяч"],
        ["тисячею", "тисячами", "тисячами"],
        ["тисячі", "тисячах", "тисячах"],
    ],
    // 10^6
    [
        ["мільйон", "мільйони", "мільйонів"],
        ["мільйона", "мільйонів", "мільйонів"],
        ["мільйону", "мільйонам", "мільйонам"],
        ["мільйон", "мільйони", "мільйонів"],
        ["мільйоном", "мільйонами", "мільйонів"],
        ["мільйоні", "мільйонах", "мільйонах"],
    ],
    // 10^9
    [
        ["мільярд", "мільярди", "мільярдів"],
        ["мільярда", "мільярдів", "мільярдів"],
        ["мільярду", "мільярдам", "мільярдам"],
        ["мільярд", "мільярди", "мільярдів"],
        ["мільярдом", "мільярдами", "мільярдів"],
        ["мільярді", "мільярдах", "мільярдах"],
    ],
    // 10^12
    [
        ["трильйон", "трильйони", "трильйонів"],
        ["трильйона", "трильйонів", "трильйонів"],
        ["трильйону", "трильйонам", "трильйонам"],
        ["трильйон", "трильйони", "трильйонів"],
        ["трильйоном", "трильйонами", "трильйонів"],
        ["трильйоні", "трильйонах", "трильйонах"],
    ],
    // 10^15
    [
        ["квадрильйон", "квадрильйони", "квадрильйонів"],
        ["квадрильйона", "квадрильйонів", "квадрильйонів"],
        ["квадрильйону", "квадрильйонам", "квадрильйонам"],
        ["квадрильйон", "квадрильйони", "квадрильйонів"],
        ["квадрильйоном", "квадрильйонами", "квадрильйонів"],
        ["квадрильйоні", "квадрильйонах", "квадрильйонах"],
    ],
    // 10^18
    [
        ["квінтильйон", "квінтильйони", "квінтильйонів"],
        ["квінтильйона", "квінтильйонів", "квінтильйонів"],
        ["квінтильйону", "квінтильйонам", "квінтильйонам"],
        ["квінтильйон", "квінтильйони", "квінтильйонів"],
        ["квінтильйоном", "квінтильйонами", "квінтильйонів"],
        ["квінтильйоні", "квінтильйонах", "квінтильйонах"],
    ],
    // 10^21
    [
        ["секстильйон", "секстильйони", "секстильйонів"],
        ["секстильйона", "секстильйонів", "секстильйонів"],
        ["секстильйону", "секстильйонам", "секстильйонам"],
        ["секстильйон", "секстильйони", "секстильйонів"],
        ["секстильйоном", "секстильйонами", "секстильйонів"],
        ["секстильйоні", "секстильйонах", "секстильйонах"],
    ],
    // 10^24
    [
        ["септильйон", "септильйони", "септильйонів"],
        ["септильйона", "септильйонів", "септильйонів"],
        ["септильйону", "септильйонам", "септильйонам"],
        ["септильйон", "септильйони", "септильйонів"],
        ["септильйоном", "септильйонами", "септильйонів"],
        ["септильйоні", "септильйонах", "септильйонах"],
    ],
    // 10^27
    [
        ["октильйон", "октильйони", "октильйонів"],
        ["октильйона", "октильйонів", "октильйонів"],
        ["октильйону", "октильйонам", "октильйонам"],
        ["октильйон", "октильйони", "октильйонів"],
        ["октильйоном", "октильйонами", "октильйонів"],
        ["октильйоні", "октильйонах", "октильйонах"],
    ],
    // 10^30
    [
        ["нонільйон", "нонільйони", "нонільйонів"],
        ["нонільйона", "нонільйонів", "нонільйонів"],
        ["нонільйону", "нонільйонам", "нонільйонам"],
        ["нонільйон", "нонільйони", "нонільйонів"],
        ["нонільйоном", "нонільйонами", "нонільйонів"],
        ["нонільйоні", "нонільйонах", "нонільйонах"],
    ],
];

/// `prefixes_ordinal`, keys 1..=10. Index 0 is absent in Python and
/// unreachable (guarded by `level > 0`); index >= 11 is a `KeyError`.
static PREFIXES_ORDINAL: [&str; 11] = [
    "", // absent in Python
    "тисячний",
    "мільйонний",
    "мільярдний",
    "трильйонний",
    "квадрильйонний",
    "квінтильйонний",
    "секстильйонний",
    "септильйонний",
    "октильйонний",
    "нонільйонний",
];

// --- Currency data --------------------------------------------------------

/// `GENERIC_DOLLARS` / `GENERIC_CENTS` — shared 3-form tuples that many
/// `CURRENCY_FORMS` entries alias in the Python source.
const GENERIC_DOLLARS: &[&str] = &["долар", "долари", "доларів"];
const GENERIC_CENTS: &[&str] = &["цент", "центи", "центів"];

/// `FEMININE_MONEY` — currencies whose *unit* noun is feminine, so
/// `_money_verbose` renders "одна"/"дві" instead of "один"/"два".
static FEMININE_MONEY: [&str; 33] = [
    "AOA", "BAM", "BDT", "BWP", "CZK", "DKK", "ERN", "HNL", "HRK", "IDR", "INR", "ISK", "JPY",
    "KPW", "KRW", "LKR", "MOP", "MRU", "MUR", "MVR", "MWK", "NGN", "NIO", "NOK", "NPR", "PKR",
    "SCR", "SEK", "STN", "TRY", "WST", "UAH", "ZMW",
];

/// `FEMININE_CENTS` — currencies whose *subunit* noun is feminine.
///
/// "OMR" appears twice in the Python tuple. `in` on a tuple is a linear scan,
/// so the duplicate is inert; transcribed verbatim rather than de-duplicated.
static FEMININE_CENTS: [&str; 15] = [
    "ALL", "BDT", "BGN", "BYN", "GHS", "HRK", "ILS", "INR", "NPR", "OMR", "OMR", "PKR", "RSD",
    "RUB", "UAH",
];

/// `Num2Word_UK.CURRENCY_FORMS`, 150 codes, every one a `(3-form, 3-form)`
/// pair. `pluralize` indexes 0..=2, so all three forms are load-bearing.
///
/// Transcribed mechanically from the Python dict (dict order is irrelevant —
/// only keyed lookup is ever performed).
static CURRENCY_FORMS_DATA: &[(&str, &[&str], &[&str])] = &[
    ("AED", &["дирхам", "дирхами", "дирхамів"], &["філс", "філси", "філсів"]),
    ("AFN", &["афгані", "афгані", "афгані"], &["пул", "пули", "пулів"]),
    ("ALL", &["лек", "леки", "леків"], &["кіндарка", "кіндарки", "кіндарок"]),
    ("AMD", &["драм", "драми", "драмів"], &["лум", "лум", "лум"]),
    ("ANG", &["гульден", "гульдени", "гульденів"], GENERIC_CENTS),
    ("AOA", &["кванза", "кванзи", "кванз"], &["сентимо", "сентимо", "сентимо"]),
    ("ARS", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("AUD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("AWG", &["флорин", "флорини", "флоринів"], GENERIC_CENTS),
    ("AZN", &["манат", "манати", "манатів"], &["гяпік", "гяпіки", "гяпіків"]),
    ("BAM", &["марка", "марки", "марок"], &["фенінг", "фенінги", "фенінгів"]),
    ("BBD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("BDT", &["така", "таки", "так"], &["пойша", "пойші", "пойш"]),
    ("BGN", &["лев", "леви", "левів"], &["стотинка", "стотинки", "стотинок"]),
    ("BHD", &["динар", "динари", "динарів"], &["філс", "філси", "філсів"]),
    ("BIF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("BMD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("BND", GENERIC_DOLLARS, GENERIC_CENTS),
    ("BOB", &["болівіано", "болівіано", "болівіано"], &["сентаво", "сентаво", "сентаво"]),
    ("BRL", &["реал", "реали", "реалів"], &["сентаво", "сентаво", "сентаво"]),
    ("BSD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("BTN", &["нгултрум", "нгултруми", "нгултрумів"], &["четрум", "четруми", "четрумів"]),
    ("BWP", &["пула", "пули", "пул"], &["тхебе", "тхебе", "тхебе"]),
    ("BYN", &["рубель", "рублі", "рублів"], &["копійка", "копійки", "копійок"]),
    ("BZD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("CAD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("CDF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("CHF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("CLP", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("CNY", &["юань", "юані", "юанів"], &["финь", "фині", "финів"]),
    ("COP", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("CRC", &["колон", "колони", "колонів"], &["сентімо", "сентімо", "сентімо"]),
    ("CUC", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("CUP", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("CVE", &["ескудо", "ескудо", "ескудо"], &["сентаво", "сентаво", "сентаво"]),
    ("CZK", &["крона", "крони", "крон"], &["гелер", "гелери", "гелерів"]),
    ("DJF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("DKK", &["крона", "крони", "крон"], &["ере", "ере", "ере"]),
    ("DOP", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("DZD", &["динар", "динари", "динарів"], &["сантим", "сантими", "сантимів"]),
    ("EGP", &["фунт", "фунти", "фунтів"], &["піастр", "піастри", "піастрів"]),
    ("ERN", &["накфа", "накфи", "накф"], GENERIC_CENTS),
    ("ETB", &["бир", "бири", "бирів"], GENERIC_CENTS),
    ("EUR", &["євро", "євро", "євро"], GENERIC_CENTS),
    ("FJD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("FKP", &["фунт", "фунти", "фунтів"], &["пенс", "пенси", "пенсів"]),
    ("GBP", &["фунт", "фунти", "фунтів"], &["пенс", "пенси", "пенсів"]),
    ("GEL", &["ларі", "ларі", "ларі"], &["тетрі", "тетрі", "тетрі"]),
    ("GHS", &["седі", "седі", "седі"], &["песева", "песеви", "песев"]),
    ("GIP", &["фунт", "фунти", "фунтів"], &["пенс", "пенси", "пенсів"]),
    ("GMD", &["даласі", "даласі", "даласі"], &["бутут", "бутути", "бутутів"]),
    ("GNF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("GTQ", &["кетсаль", "кетсалі", "кетсалів"], &["сентаво", "сентаво", "сентаво"]),
    ("GYD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("HKD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("HNL", &["лемпіра", "лемпіри", "лемпір"], &["сентаво", "сентаво", "сентаво"]),
    ("HRK", &["куна", "куни", "кун"], &["ліпа", "ліпи", "ліп"]),
    ("HTG", &["гурд", "гурди", "гурдів"], &["сантим", "сантими", "сантимів"]),
    ("HUF", &["форинт", "форинти", "форинтів"], &["філлер", "філлери", "філлерів"]),
    ("IDR", &["рупія", "рупії", "рупій"], GENERIC_CENTS),
    ("ILS", &["шекель", "шекелі", "шекелів"], &["агора", "агори", "агор"]),
    ("INR", &["рупія", "рупії", "рупій"], &["пайса", "пайси", "пайс"]),
    ("IQD", &["динар", "динари", "динарів"], &["філс", "філси", "філсів"]),
    ("IRR", &["ріал", "ріали", "ріалів"], &["динар", "динари", "динарів"]),
    ("ISK", &["крона", "крони", "крон"], &["ейре", "ейре", "ейре"]),
    ("JMD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("JOD", &["динар", "динари", "динарів"], &["філс", "філси", "філсів"]),
    ("JPY", &["єна", "єни", "єн"], &["сен", "сен", "сен"]),
    ("KES", &["шилінг", "шилінги", "шилінгів"], GENERIC_CENTS),
    ("KGS", &["сом", "соми", "сомів"], &["тиїн", "тиїни", "тиїнів"]),
    ("KHR", &["рієль", "рієлі", "рієлів"], &["су", "су", "су"]),
    ("KMF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("KPW", &["вона", "вони", "вон"], &["чон", "чони", "чонів"]),
    ("KRW", &["вона", "вони", "вон"], &["джеон", "джеони", "джеонів"]),
    ("KWD", &["динар", "динари", "динарів"], &["філс", "філси", "філсів"]),
    ("KYD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("KZT", &["теньге", "теньге", "теньге"], &["тиїн", "тиїни", "тиїнів"]),
    ("LAK", &["кіп", "кіпи", "кіпів"], &["ат", "ати", "атів"]),
    ("LBP", &["фунт", "фунти", "фунтів"], &["піастр", "піастри", "піастрів"]),
    ("LKR", &["рупія", "рупії", "рупій"], GENERIC_CENTS),
    ("LRD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("LSL", &["лоті", "малоті", "малоті"], &["сенте", "лісенте", "лісенте"]),
    ("LYD", &["динар", "динари", "динарів"], &["дирхам", "дирхами", "дирхамів"]),
    ("MAD", &["дирхам", "дирхами", "дирхамів"], &["сантим", "сантими", "сантимів"]),
    ("MDL", &["лей", "леї", "леї"], &["бан", "бані", "бані"]),
    ("MGA", &["аріарі", "аріарі", "аріарі"], &["іраймбіланья", "іраймбіланья", "іраймбіланья"]),
    ("MKD", &["денар", "денари", "денарів"], &["дені", "дені", "дені"]),
    ("MMK", &["к'ят", "к'ят", "к'ят"], &["п'я", "п'я", "п'я"]),
    ("MNT", &["тугрик", "тугрики", "тугриків"], &["мунгу", "мунгу", "мунгу"]),
    ("MOP", &["патака", "патакі", "патак"], &["аво", "аво", "аво"]),
    ("MRU", &["угія", "угії", "угій"], &["хумс", "хумс", "хумс"]),
    ("MUR", &["рупія", "рупії", "рупій"], GENERIC_CENTS),
    ("MVR", &["руфія", "руфії", "руфій"], &["ларі", "ларі", "ларі"]),
    ("MWK", &["квача", "квачі", "квач"], &["тамбала", "тамбала", "тамбала"]),
    ("MXN", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("MYR", &["рингіт", "рингіти", "рингітів"], GENERIC_CENTS),
    ("MZN", &["метікал", "метікали", "метікалів"], &["сентаво", "сентаво", "сентаво"]),
    ("NAD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("NGN", &["найра", "найри", "найр"], &["кобо", "кобо", "кобо"]),
    ("NIO", &["кордоба", "кордоби", "кордоб"], &["сентаво", "сентаво", "сентаво"]),
    ("NOK", &["крона", "крони", "крон"], &["ере", "ере", "ере"]),
    ("NPR", &["рупія", "рупії", "рупій"], &["пайса", "пайси", "пайс"]),
    ("NZD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("OMR", &["ріал", "ріали", "ріалів"], &["байза", "байзи", "байз"]),
    ("PAB", &["бальбоа", "бальбоа", "бальбоа"], &["сентесімо", "сентесімо", "сентесімо"]),
    ("PEN", &["соль", "соль", "соль"], &["сентімо", "сентімо", "сентімо"]),
    ("PGK", &["кіна", "кіна", "кіна"], &["тойя", "тойя", "тойя"]),
    ("PHP", &["песо", "песо", "песо"], &["сентаво", "сентаво", "сентаво"]),
    ("PKR", &["рупія", "рупії", "рупій"], &["пайса", "пайси", "пайс"]),
    ("PLN", &["злотий", "злоті", "злотих"], &["грош", "гроші", "грошів"]),
    ("PYG", &["гуарані", "гуарані", "гуарані"], &["сентімо", "сентімо", "сентімо"]),
    ("QAR", &["ріал", "ріали", "ріалів"], &["дирхам", "дирхами", "дирхамів"]),
    ("RON", &["лей", "леї", "леї"], &["бан", "бані", "бані"]),
    ("RSD", &["динар", "динари", "динарів"], &["пара", "пари", "пар"]),
    ("RUB", &["рубль", "рублі", "рублів"], &["копійка", "копійки", "копійок"]),
    ("RWF", &["франк", "франки", "франків"], &["сантим", "сантими", "сантимів"]),
    ("SAR", &["ріал", "ріали", "ріалів"], &["халал", "халали", "халалів"]),
    ("SBD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("SCR", &["рупія", "рупії", "рупій"], GENERIC_CENTS),
    ("SDG", &["фунт", "фунти", "фунтів"], &["піастр", "піастри", "піастрів"]),
    ("SEK", &["крона", "крони", "крон"], &["ере", "ере", "ере"]),
    ("SGD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("SHP", &["фунт", "фунти", "фунтів"], &["пенс", "пенси", "пенсів"]),
    ("SLL", &["леоне", "леоне", "леоне"], GENERIC_CENTS),
    ("SOS", &["шилінг", "шилінги", "шилінгів"], GENERIC_CENTS),
    ("SRD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("SSP", &["фунт", "фунти", "фунтів"], &["піастр", "піастри", "піастрів"]),
    ("STN", &["добра", "добри", "добр"], &["сентімо", "сентімо", "сентімо"]),
    ("SYP", &["фунт", "фунти", "фунтів"], &["піастр", "піастри", "піастрів"]),
    ("SZL", &["ліланґені", "ліланґені", "ліланґені"], GENERIC_CENTS),
    ("THB", &["бат", "бати", "батів"], &["сатанг", "сатанги", "сатангів"]),
    ("TJS", &["сомоні", "сомоні", "сомоні"], &["дірам", "дірами", "дірамів"]),
    ("TMT", &["манат", "манати", "манатів"], &["тенге", "тенге", "тенге"]),
    ("TND", &["динар", "динари", "динарів"], &["міллім", "мілліми", "міллімів"]),
    ("TOP", &["паанга", "паанга", "паанга"], &["сеніті", "сеніті", "сеніті"]),
    ("TRY", &["ліра", "ліри", "лір"], &["куруш", "куруші", "курушів"]),
    ("TTD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("TWD", &["новий долар", "нові долари", "нових доларів"], GENERIC_CENTS),
    ("TZS", &["шилінг", "шилінги", "шилінгів"], GENERIC_CENTS),
    ("UAH", &["гривня", "гривні", "гривень"], &["копійка", "копійки", "копійок"]),
    ("UGX", &["шилінг", "шилінги", "шилінгів"], GENERIC_CENTS),
    ("USD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("UYU", &["песо", "песо", "песо"], &["сентесімо", "сентесімо", "сентесімо"]),
    ("UZS", &["сум", "суми", "сумів"], &["тиїн", "тиїни", "тиїнів"]),
    ("VND", &["донг", "донги", "донгів"], &["су", "су", "су"]),
    ("WST", &["тала", "тали", "тал"], &["сене", "сене", "сене"]),
    ("XCD", GENERIC_DOLLARS, GENERIC_CENTS),
    ("YER", &["ріал", "ріали", "ріалів"], &["філс", "філси", "філсів"]),
    ("ZAR", &["ранд", "ранди", "рандів"], GENERIC_CENTS),
    ("ZMW", &["квача", "квачі", "квач"], &["нгве", "нгве", "нгве"]),
];

/// Built **once** in [`LangUk::new`] and stored on the struct. Rebuilding 150
/// `CurrencyForms` per call is what made an earlier revision of this port an
/// order of magnitude slower than the Python it replaces.
fn build_currency_forms() -> HashMap<&'static str, CurrencyForms> {
    CURRENCY_FORMS_DATA
        .iter()
        .map(|(code, unit, sub)| (*code, CurrencyForms::new(unit, sub)))
        .collect()
}

fn is_feminine_money(currency: &str) -> bool {
    FEMININE_MONEY.iter().any(|c| *c == currency)
}

fn is_feminine_cents(currency: &str) -> bool {
    FEMININE_CENTS.iter().any(|c| *c == currency)
}

// --- Python exception encoding -------------------------------------------
//
// `Index` and `Key` mirror *crashes* in lang_UK.py, not deliberate raises.
// The exception type is observable behaviour a caller may catch, so parity
// requires reproducing it rather than tidying it into a TypeError.

/// The feminine `gender=` aliases `Num2Word_UK.to_cardinal` accepts — the
/// same set as `lang_RU`: short code, English name, and the Ukrainian /
/// Russian Cyrillic names. Compared after `str.lower()`.
const FEMININE_ALIASES: [&str; 5] = ["f", "feminine", "ж", "жіночий", "женский"];

/// `isinstance(gender_kw, str) and gender_kw.lower() in feminine_aliases`.
/// Anything else — an absent key, a masculine/neuter name, a non-string —
/// is masculine; Python never raises on `gender=`.
fn feminine_from_kwarg(kv: Option<&KwVal>) -> bool {
    match kv {
        Some(KwVal::Str(g)) => FEMININE_ALIASES.contains(&g.to_lowercase().as_str()),
        _ => false,
    }
}

/// The kwargs `Num2Word_UK.to_cardinal(number, **kwargs)` reads. It swallows
/// any other key silently, but an unknown key still declines to the
/// dispatcher (like `lang_RU`) rather than guessing.
const CARDINAL_KWARGS: [&str; 1] = ["gender"];

fn index_error(msg: &str) -> N2WError {
    N2WError::Index(msg.to_string())
}

fn key_error(key: String) -> N2WError {
    N2WError::Key(key)
}

fn value_error(msg: String) -> N2WError {
    N2WError::Value(msg)
}

/// Python's `int(s)` for the digit-string chunks `splitbyx` produces.
///
/// Every reachable caller passes a pure digit string: `_int2word` strips the
/// sign by recursing on `abs(n)` before stringifying, and `to_ordinal` runs
/// `verify_ordinal` (which rejects negatives) first. So unlike `lang_PL`,
/// neither `int("-")` (ValueError) nor a negative chunk is reachable here.
/// The `ValueError` below therefore models `int("-")` for completeness only.
///
/// Chunks are provably `<= 999`: `splitbyx`'s head chunk is `length % 3`
/// digits (1 or 2, since a 0-length head is skipped), every other chunk is
/// exactly 3, and the short-input branch yields at most 3. Hence `u32`.
fn parse_chunk(s: &str) -> Result<u32> {
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) {
        // <= 3 ASCII digits, so this cannot overflow u32.
        Ok(s.parse::<u32>().unwrap_or(0))
    } else {
        Err(value_error(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        )))
    }
}

/// Python's `int(s)` for an arbitrary-length digit string — the `int(left)` /
/// `int(right)` calls in the float branch of `Num2Word_UK.to_cardinal`.
///
/// Every reachable caller passes a pure digit string (formatted from an f64 or
/// reconstructed from a `BigDecimal`), so the `ValueError` arm models
/// Python's `int("...")` on a non-digit token for completeness only. Leading
/// zeros are inert, exactly as `int("005") == 5`. Returns `BigInt` because
/// `left` can be trillion-scale (`98746251323029`) and `right` unbounded.
fn parse_int(s: &str) -> Result<BigInt> {
    if !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()) {
        s.parse::<BigInt>().map_err(|_| {
            value_error(format!("invalid literal for int() with base 10: '{}'", s))
        })
    } else {
        Err(value_error(format!(
            "invalid literal for int() with base 10: '{}'",
            s
        )))
    }
}

/// Whether Python's `repr(float)` for this finite value shows a decimal
/// point. CPython's float repr switches to exponent notation exactly when
/// the decimal exponent is `< -4` or `>= 16`, i.e. for non-zero values
/// outside `[1e-4, 1e16)`; every other finite float prints as "d.d…".
/// Zero (either sign) prints "0.0"/"-0.0".
fn float_repr_has_point(v: f64) -> bool {
    v == 0.0 || (v.abs() >= 1e-4 && v.abs() < 1e16)
}

/// Python's `int(s)` acceptance for the strings `Num2Word_UK.to_cardinal`
/// feeds it: an optional sign followed by ASCII digits. Anything else — the
/// exponent-form reprs "1e+16"/"1E+2" — is `None`, which the caller turns
/// into the `ValueError` Python raises.
fn parse_signed_int(s: &str) -> Option<BigInt> {
    let t = s.strip_prefix('-').unwrap_or(s);
    if t.is_empty() || !t.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse::<BigInt>().ok()
}

/// Python's `str(number)` for the two `FloatValue` arms, used by the ordinal
/// TypeError messages (the corpus compares exception *types* only, so the
/// interpolated value is best-effort for the float arm).
fn py_value_str(value: &FloatValue) -> String {
    match value {
        FloatValue::Float { value, .. } => format!("{}", value),
        FloatValue::Decimal { value, .. } => python_decimal_str(value),
    }
}

/// Port of `utils.splitbyx(n, x)` with `format_int=True`.
fn splitbyx(n: &str, x: usize) -> Result<Vec<u32>> {
    let chars: Vec<char> = n.chars().collect();
    let length = chars.len();
    let slice = |i: usize, j: usize| -> String { chars[i..j.min(length)].iter().collect() };

    let mut out: Vec<u32> = Vec::new();
    if length > x {
        let start = length % x;
        if start > 0 {
            out.push(parse_chunk(&slice(0, start))?);
        }
        let mut i = start;
        while i < length {
            out.push(parse_chunk(&slice(i, i + x))?);
            i += x;
        }
    } else {
        out.push(parse_chunk(n)?);
    }
    Ok(out)
}

/// Port of `utils.get_digits(n)`:
/// `[int(x) for x in reversed(list(("%03d" % n)[-3:]))]` → `(n1, n2, n3)`
/// (units, tens, hundreds).
///
/// `n` is always a `splitbyx` chunk in `0..=999`, for which `"%03d" % n` is
/// exactly three digits and the `[-3:]` slice is the whole string — so plain
/// arithmetic is an exact match. (The negative-input hazard that breaks
/// `lang_PL` here is unreachable in UK; see [`parse_chunk`].)
fn get_digits(n: u32) -> (usize, usize, usize) {
    (
        (n % 10) as usize,
        ((n / 10) % 10) as usize,
        ((n / 100) % 10) as usize,
    )
}

/// `THOUSANDS[i][morphological_case]`, raising `KeyError` past key 10.
fn thousands_at(i: usize, mcase: usize) -> Result<&'static [&'static str; 3]> {
    if i == 0 || i >= THOUSANDS.len() {
        return Err(key_error(i.to_string()));
    }
    THOUSANDS[i]
        .get(mcase)
        .ok_or_else(|| index_error("tuple index out of range"))
}

/// `prefixes_ordinal[level]`, raising `KeyError` past key 10.
fn prefixes_ordinal_at(level: usize) -> Result<&'static str> {
    if level == 0 || level >= PREFIXES_ORDINAL.len() {
        return Err(key_error(level.to_string()));
    }
    Ok(PREFIXES_ORDINAL[level])
}

/// Port of `Num2Word_UK.pluralize`.
///
/// ```python
/// if n % 100 < 10 or n % 100 > 20:
///     if n % 10 == 1:      form = 0
///     elif 5 > n % 10 > 1: form = 1
///     else:                form = 2
/// else:
///     form = 2
/// ```
/// `n` is a chunk in `0..=999`, so Rust's `%` matches Python's exactly.
fn pluralize(n: u32, forms: &'static [&'static str; 3]) -> &'static str {
    let form = if n % 100 < 10 || n % 100 > 20 {
        if n % 10 == 1 {
            0
        } else if n % 10 > 1 && n % 10 < 5 {
            1
        } else {
            2
        }
    } else {
        2
    };
    forms[form]
}

/// Port of the `Num2Word_UK.last_fragment_to_ordinal` staticmethod.
///
/// `level` is 0 or 1 — `to_ordinal` clamps it via `0 if level == 0 else 1`.
/// Level 0 emits free-standing words (appended to `words`, later joined with
/// spaces); level 1 concatenates bound stems into a *single* word, which is
/// what produces "стотисячний" / "двадцятиоднотисячний".
fn last_fragment_to_ordinal(last: u32, words: &mut Vec<String>, level: usize) -> Result<()> {
    let (n1, n2, n3) = get_digits(last);
    let last_two = n2 * 10 + n1;

    if last_two == 0 {
        // last != 0 is guaranteed by to_ordinal's pop loop, so n3 >= 1 here.
        words.push(HUNDREDS_ORDINALS[n3][level].to_string());
    } else if level == 1 && last == 1 {
        return Ok(());
    } else if last_two < 20 {
        if level == 0 {
            if n3 > 0 {
                words.push(HUNDREDS[n3][0].to_string());
            }
            words.push(ONES_ORDINALS[last_two][0].to_string());
        } else {
            let mut s = String::new();
            if n3 > 0 {
                s.push_str(HUNDREDS_ORDINALS[n3][1]);
            }
            s.push_str(ONES_ORDINALS[last_two][1]);
            words.push(s);
        }
    } else if last_two % 10 == 0 {
        // last_two >= 20 here, so n2 is in 2..=9.
        if level == 0 {
            if n3 > 0 {
                words.push(HUNDREDS[n3][0].to_string());
            }
            words.push(TWENTIES_ORDINALS[n2][0].to_string());
        } else {
            let mut s = String::new();
            if n3 > 0 {
                s.push_str(HUNDREDS_ORDINALS[n3][1]);
            }
            s.push_str(TWENTIES_ORDINALS[n2][1]);
            words.push(s);
        }
    } else {
        // last_two >= 21 and not a round ten, so n2 in 2..=9 and n1 in 1..=9.
        if level == 0 {
            if n3 > 0 {
                words.push(HUNDREDS[n3][0].to_string());
            }
            words.push(TWENTIES[n2][0].to_string());
            words.push(ONES_ORDINALS[n1][0].to_string());
        } else {
            let mut s = String::new();
            if n3 > 0 {
                s.push_str(HUNDREDS_ORDINALS[n3][1]);
            }
            s.push_str(TWENTIES_ORDINALS[n2][1]);
            s.push_str(ONES_ORDINALS[n1][1]);
            words.push(s);
        }
    }
    Ok(())
}

pub struct LangUk {
    /// `CURRENCY_FORMS`, built once at construction. The registry holds a
    /// single `OnceLock<LangUk>` per process, so this table is built at most
    /// once for the whole program.
    currency_forms: HashMap<&'static str, CurrencyForms>,
}

impl Default for LangUk {
    fn default() -> Self {
        Self::new()
    }
}

impl LangUk {
    pub fn new() -> Self {
        LangUk {
            currency_forms: build_currency_forms(),
        }
    }

    /// Port of `Num2Word_Base.verify_ordinal`. The float check
    /// (`errmsg_floatord`) is unreachable for `BigInt` input.
    fn verify_ordinal(&self, value: &BigInt) -> Result<()> {
        if value.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                value
            )));
        }
        Ok(())
    }

    /// Port of `Num2Word_UK._int2word`.
    ///
    /// `feminine` comes from the `gender=` kwarg (or the currency's gender);
    /// `morphological_case` is the `case=` index, always 0 for now.
    fn int2word(&self, n: &BigInt, feminine: bool, mcase: usize) -> Result<String> {
        check_maxval(n, maxval_ceiling())?;
        if n.is_negative() {
            let n_value = self.int2word(&n.abs(), feminine, mcase)?;
            return Ok(format!("{} {}", NEGWORD, n_value));
        }

        if n.is_zero() {
            return Ok(ZERO.to_string());
        }

        let mut words: Vec<String> = Vec::new();
        let chunks = splitbyx(&n.to_string(), 3)?;
        let mut i = chunks.len();
        for x in chunks.iter().copied() {
            i -= 1;

            if x == 0 {
                continue;
            }

            let (n1, n2, n3) = get_digits(x);

            if n3 > 0 {
                words.push(HUNDREDS[n3][mcase].to_string());
            }

            if n2 > 1 {
                words.push(TWENTIES[n2][mcase].to_string());
            }

            if n2 == 1 {
                words.push(TENS[n1][mcase].to_string());
            } else if n1 > 0 {
                // Python: `ONES_FEMININE if i == 1 or feminine and i == 0 else ONES`
                // — `and` binds tighter than `or`, so this is
                // `i == 1 or (feminine and i == 0)`. The `i == 1` arm is the
                // thousands chunk, agreeing with feminine "тисяча".
                //
                // NB: the source's commented-out guard
                // `# elif n1 > 0 and not (i > 0 and x == 1)` is *not* live, so
                // 1000 keeps its "одна" → "одна тисяча". See module docs.
                let ones = if i == 1 || (feminine && i == 0) {
                    &ONES_FEMININE
                } else {
                    &ONES
                };
                words.push(ones[n1][mcase].to_string());
            }

            if i > 0 {
                let thousands_val = thousands_at(i, mcase)?;
                words.push(pluralize(x, thousands_val).to_string());
            }
        }

        Ok(words.join(" "))
    }

    /// [`cardinal_float_entry`](Lang::cardinal_float_entry) with the
    /// resolved `gender`/`case` kwargs threaded through.
    fn cardinal_entry_with(&self, value: &FloatValue, feminine: bool, mcase: usize) -> Result<String> {
        let has_point = match value {
            FloatValue::Float { value, .. } => float_repr_has_point(*value),
            FloatValue::Decimal { value, .. } => python_decimal_str(value).contains('.'),
        };
        if has_point {
            return self.cardinal_float_with(value, feminine, mcase);
        }
        // No '.' in str(number) → `self._int2word(int(n), ...)`.
        match value {
            // A finite float without a point reprs in exponent form
            // ("1e+16"), which int() rejects.
            FloatValue::Float { value, .. } => Err(value_error(format!(
                "invalid literal for int() with base 10: '{}'",
                value
            ))),
            FloatValue::Decimal { value, .. } => {
                let s = python_decimal_str(value);
                match parse_signed_int(&s) {
                    Some(n) => self.int2word(&n, feminine, mcase),
                    // "1E+2"-style scientific strings → ValueError.
                    None => Err(value_error(format!(
                        "invalid literal for int() with base 10: '{}'",
                        s
                    ))),
                }
            }
        }
    }

    /// The `"." in n` branch of `Num2Word_UK.to_cardinal` — see
    /// [`to_cardinal_float`](Lang::to_cardinal_float) for the full port
    /// notes. Python hands the resolved `gender`/`case` to *both* sides of
    /// the point, so `to_cardinal(1.1, gender="f")` is "одна кома одна".
    fn cardinal_float_with(&self, value: &FloatValue, feminine: bool, mcase: usize) -> Result<String> {
        // Reconstruct (sign, integer part, fractional-digit string). `right`
        // keeps its leading zeros and is exactly `precision` chars wide (empty
        // only when precision == 0, i.e. str(number) had no ".").
        let (is_negative, left_int, right): (bool, BigInt, String) = match value {
            FloatValue::Float { value, precision } => {
                let precision = *precision as usize;
                // Shortest-round-trip repr, reproduced from the raw f64.
                let s = format!("{value:.precision$}");
                let is_negative = s.starts_with('-');
                let abs_s = s.strip_prefix('-').unwrap_or(&s);
                match abs_s.split_once('.') {
                    Some((l, r)) => (is_negative, parse_int(l)?, r.to_string()),
                    None => (is_negative, parse_int(abs_s)?, String::new()),
                }
            }
            FloatValue::Decimal { value, precision } => {
                let precision = *precision;
                let is_negative = value.is_negative();
                let abs_v = value.abs();
                // mantissa = abs_v * 10**precision, exact (abs_v's own scale is
                // precision, so no rounding). Mirrors floatpath's with_scale(0).
                let divisor = BigInt::from(10).pow(precision);
                let mantissa = (&abs_v * BigDecimal::from(divisor.clone()))
                    .with_scale(0)
                    .as_bigint_and_exponent()
                    .0;
                let left_int = &mantissa / &divisor;
                let right_int = &mantissa % &divisor;
                let right = if precision == 0 {
                    String::new()
                } else {
                    // Zero-pad to exactly `precision` digits, matching the
                    // fractional part str(Decimal) prints.
                    let digits = right_int.to_string();
                    let pad = (precision as usize).saturating_sub(digits.len());
                    format!("{}{}", "0".repeat(pad), digits)
                };
                (is_negative, left_int, right)
            }
        };

        // precision == 0 → str(number) carried no ".", i.e. the integer `else`
        // branch of Num2Word_UK.to_cardinal. Unreachable from the corpus (every
        // float repr and every Decimal row here has a fractional part), but
        // kept faithful — int2word applies the sign via its own negword arm.
        if right.is_empty() {
            let n = if is_negative { -left_int } else { left_int };
            return self.int2word(&n, feminine, mcase);
        }

        // leading_zero_count = len(right) - len(right.lstrip("0")); `right` is
        // all ASCII digits, so byte length equals char count.
        let leading_zero_count = right.len() - right.trim_start_matches('0').len();
        // int(right) — leading zeros are inert, "00" parses to 0.
        let right_int = parse_int(&right)?;

        let left_word = self.int2word(&left_int, feminine, mcase)?;
        let right_side = self.int2word(&right_int, feminine, mcase)?;
        // (ZERO[0] + " ") * leading_zero_count + right_side
        let decimal_part = format!(
            "{}{}",
            format!("{} ", ZERO).repeat(leading_zero_count),
            right_side
        );

        // "%s %s %s" % (left, self.pointword, decimal_part) — pointword is used
        // raw (UK never title-cases), and negword is prepended with one space.
        let mut result = format!("{} {} {}", left_word, POINTWORD, decimal_part);
        if is_negative {
            result = format!("{} {}", NEGWORD, result);
        }
        Ok(result)
    }
}

/// The exclusive ceiling (gladiaio/num2words2#159): the scale-word table
/// ends at 10^30, so 10^33 and above raise `OverflowError` instead of
/// reaching the missing table key.
fn maxval_ceiling() -> &'static BigInt {
    static M: OnceLock<BigInt> = OnceLock::new();
    M.get_or_init(|| pow10_big(33))
}

impl Lang for LangUk {
    fn maxval(&self) -> &BigInt {
        maxval_ceiling()
    }

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

    fn negword(&self) -> &str {
        NEGWORD
    }

    fn pointword(&self) -> &str {
        "кома"
    }

    /// Port of `Num2Word_UK.to_cardinal`, integer path only.
    ///
    /// Python does `n = str(number).replace(",", ".")` and branches on `"."`;
    /// `str(int)` never contains one, so integers always take the `else`
    /// branch → `self._int2word(int(n), gender, morphological_case)`. With no
    /// kwargs, `gender` is `False` and `morphological_case` is `0`; see
    /// `to_cardinal_kw` for the kwargs. The float
    /// branch (pointword, leading-zero expansion) is ported in
    /// [`to_cardinal_float`](Self::to_cardinal_float) below.
    ///
    /// There is no `MAXVAL`/`OverflowError` check: `Num2Word_UK` overrides
    /// `to_cardinal` outright and never reaches `Num2Word_Base`'s guard.
    fn to_cardinal(&self, value: &BigInt) -> Result<String> {
        self.int2word(value, false, 0)
    }

    /// `to_cardinal(number, gender=...)` (gladiaio/num2words2#145). `gender`
    /// selects `ONES_FEMININE` for the units chunk ("двадцять одна"); the
    /// thousands chunk is feminine regardless, agreeing with "тисяча".
    fn to_cardinal_kw(&self, value: &BigInt, kw: &Kwargs) -> Result<String> {
        if !kw.only(&CARDINAL_KWARGS) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.int2word(value, feminine_from_kwarg(kw.get("gender")), 0)
    }

    /// `to_cardinal(float/Decimal, gender=...)`: the "." branch applies the
    /// gender to both sides of the point. `precision=` is ignored, as in
    /// [`cardinal_float_entry`](Self::cardinal_float_entry).
    fn to_cardinal_float_kw(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
        kw: &Kwargs,
    ) -> Result<String> {
        if !kw.only(&CARDINAL_KWARGS) {
            return Err(N2WError::Fallback("kwargs".into()));
        }
        self.cardinal_entry_with(value, feminine_from_kwarg(kw.get("gender")), 0)
    }

    /// Port of the `"." in n` branch of `Num2Word_UK.to_cardinal`.
    ///
    /// `Num2Word_UK` does **not** override `Num2Word_Base.to_cardinal_float`;
    /// its `to_cardinal` intercepts non-integers inline and never touches
    /// `base.float2tuple`. So this port reproduces that *string-based* branch,
    /// not the binary `float2tuple` heuristic that the trait default
    /// (`floatpath::default_to_cardinal_float`) implements. The f64 artefact
    /// that heuristic exists to rescue is therefore irrelevant here: UK reads
    /// `str(number)` directly, so `2.675` → "два кома шістсот сімдесят п'ять"
    /// (the repr digits "675"), *not* the `674.999…`→675 artefact path.
    ///
    /// ```python
    /// n = str(number).replace(",", ".")
    /// if "." in n:
    ///     is_negative = n.startswith("-")
    ///     abs_n = n[1:] if is_negative else n
    ///     left, right = abs_n.split(".")
    ///     leading_zero_count = len(right) - len(right.lstrip("0"))
    ///     right_side = self._int2word(int(right), gender, morphological_case)
    ///     decimal_part = (ZERO[0] + " ") * leading_zero_count + right_side
    ///     result = "%s %s %s" % (self._int2word(int(left), ...),
    ///                            self.pointword, decimal_part)
    ///     if is_negative:
    ///         result = self.negword + " " + result
    ///     return result
    /// ```
    ///
    /// `str(number)` is reconstructed per arm:
    ///   * **Float** — `format!("{:.precision$}", value)` reproduces Python's
    ///     `repr`/`str(float)`. `precision` is the fractional-digit count of
    ///     that shortest round-trip repr, so formatting the raw f64 to exactly
    ///     that many places yields the identical digits (checked for every
    ///     corpus float, incl. `1.005`→"1.005" and `2.675`→"2.675"). No
    ///     `round_ties_even` is needed — the arithmetic never leaves the string.
    ///   * **Decimal** — exact arbitrary-precision digits, so the trailing zero
    ///     of `Decimal("1.10")` survives as "…кома десять" (10), never the
    ///     "…кома один" a `float(1.10)` cast would give (issue #603), and the
    ///     trillion-scale `98746251323029.99` keeps every digit.
    ///
    /// Here `gender`/`morphological_case` are the defaults (masculine /
    /// nominative); `to_cardinal_float_kw` passes the kwargs through
    /// `cardinal_float_with`. `precision=` is ignored — matching the live
    /// interpreter, where `num2words(0.5, lang="uk", precision=3)` is still
    /// "нуль кома п'ять".
    fn to_cardinal_float(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_float_with(value, false, 0)
    }

    /// `to_cardinal(float/Decimal)` — the FULL routing, whole values included.
    ///
    /// `Num2Word_UK.to_cardinal` is *string-driven*: `n = str(number)`, then
    /// `"." in n` decides. A whole float still reprs with a point ("5.0"), so
    /// it takes the float branch and renders "п'ять кома нуль нуль" — never
    /// the bare integer path the trait default would pick. Values whose
    /// `str()` carries no point (`Decimal("5")`, but also the exponent-form
    /// reprs "1e+16" / "1E+2") fall into `int(n)`, which raises `ValueError`
    /// for the exponent forms. All corpus-pinned.
    ///
    /// `precision_override` is ignored, as Python's `to_cardinal(number,
    /// **kwargs)` ignores a `precision=` kwarg.
    fn cardinal_float_entry(
        &self,
        value: &FloatValue,
        _precision_override: Option<u32>,
    ) -> Result<String> {
        self.cardinal_entry_with(value, false, 0)
    }

    /// `to_ordinal(float/Decimal)`.
    ///
    /// Python first runs `verify_ordinal` — TypeError for a fractional value
    /// (`errmsg_floatord`), TypeError for a strictly negative one
    /// (`errmsg_negord`; `-0.0` passes because `abs(-0.0) == -0.0`) — then
    /// builds `splitbyx(str(number), 3)`, where any '.'/'E' in the string
    /// dies in `int()` with ValueError. Only a plain digit `str()` (i.e. a
    /// whole `Decimal` in fixed notation) reaches the real ordinal path.
    fn ordinal_float_entry(&self, value: &FloatValue) -> Result<String> {
        let whole = match value.as_whole_int() {
            Some(i) => i,
            None => {
                return Err(N2WError::Type(format!(
                    "Cannot treat float {} as ordinal.",
                    py_value_str(value)
                )))
            }
        };
        if whole.is_negative() {
            return Err(N2WError::Type(format!(
                "Cannot treat negative num {} as ordinal.",
                py_value_str(value)
            )));
        }
        match value {
            // repr(float) always shows '.' or exponent form → int() fails.
            FloatValue::Float { value, .. } => Err(value_error(format!(
                "invalid literal for int() with base 10: '{}'",
                value
            ))),
            FloatValue::Decimal { value, .. } => {
                let s = python_decimal_str(value);
                if parse_signed_int(&s).is_some() {
                    self.to_ordinal(&whole)
                } else {
                    Err(value_error(format!(
                        "invalid literal for int() with base 10: '{}'",
                        s
                    )))
                }
            }
        }
    }

    // year_float_entry: `Num2Word_UK` inherits `Num2Word_Base.to_year`
    // (`return self.to_cardinal(value)`), and the trait default routes back
    // through `cardinal_float_entry` above — so "5.0" years also read
    // "п'ять кома нуль нуль", exactly as Python's do.

    /// `converter.str_to_number` — Base's `Decimal(value)`. `Decimal("Infinity")`
    /// parses fine in Python; the failure happens *next*, inside UK's own
    /// `to_cardinal`: `str(number)` == "Infinity" has no '.', so `int("Infinity")`
    /// raises **ValueError**. The binding otherwise maps `ParsedNumber::Inf` to
    /// the base integer path's OverflowError, so the ValueError must be raised
    /// here. (NaN needs no interception: the binding's ValueError already
    /// matches `int("NaN")`'s type.)
    fn str_to_number(&self, s: &str) -> Result<ParsedNumber> {
        match python_decimal_parse(s)? {
            ParsedNumber::Inf { negative } => Err(value_error(format!(
                "invalid literal for int() with base 10: '{}Infinity'",
                if negative { "-" } else { "" }
            ))),
            other => Ok(other),
        }
    }

    /// Port of `Num2Word_UK.to_ordinal`.
    ///
    /// Raises `TypeError` for negatives (via `verify_ordinal`), `IndexError`
    /// for 0, and `KeyError` for level >= 11 (>= 10^33).
    fn to_ordinal(&self, value: &BigInt) -> Result<String> {
        self.verify_ordinal(value)?;

        // PR savoirfairelinux/num2words#668: splitbyx("0") yields a single
        // zero fragment; the pop loop below empties the list and then indexes
        // it (IndexError). Handle zero explicitly.
        if value.is_zero() {
            return Ok("нульовий".to_string());
        }
        check_maxval(value, maxval_ceiling())?;

        let mut words: Vec<String> = Vec::new();
        let mut fragments = splitbyx(&value.to_string(), 3)?;

        let mut level: usize = 0;
        // `fragments[-1]` on an empty list → IndexError. splitbyx always
        // yields at least one element, so only the pop loop can empty it —
        // which is exactly what to_ordinal(0) does.
        let mut last = *fragments
            .last()
            .ok_or_else(|| index_error("list index out of range"))?;
        while last == 0 {
            level += 1;
            fragments.pop();
            last = *fragments
                .last()
                .ok_or_else(|| index_error("list index out of range"))?;
        }

        if fragments.len() > 1 {
            // Python: self._int2word(number - (last * 1000**level)) — called
            // with the defaults, i.e. masculine / nominative.
            let pow = BigInt::from(1000u32).pow(level as u32);
            let pre_part = self.int2word(&(value - BigInt::from(last) * pow), false, 0)?;
            words.push(pre_part);
        }

        last_fragment_to_ordinal(last, &mut words, if level == 0 { 0 } else { 1 })?;

        let mut output = words.join(" ");
        // Only fires when a pre_part exists and the surviving fragment is 1,
        // e.g. 1001000 → "один мільйон тисячний". Every other level > 0 input
        // glues the suffix on with no separator.
        if last == 1 && level > 0 && !output.is_empty() {
            output.push(' ');
        }
        if level > 0 {
            output.push_str(prefixes_ordinal_at(level)?);
        }
        Ok(output)
    }

    // to_ordinal_num: UK does not override Num2Word_Base.to_ordinal_num,
    // which returns the value unchanged → the trait default is correct.
    //
    // to_year: UK does not override Num2Word_Base.to_year, which delegates to
    // to_cardinal → the trait default is correct.

    // ---- currency --------------------------------------------------------

    fn lang_name(&self) -> &str {
        "Num2Word_UK"
    }

    fn currency_forms(&self, code: &str) -> Option<&CurrencyForms> {
        self.currency_forms.get(code)
    }

    // currency_adjective: `Num2Word_UK` does not define CURRENCY_ADJECTIVES
    // and inherits `Num2Word_Base.CURRENCY_ADJECTIVES = {}`, so
    // `currency in self.CURRENCY_ADJECTIVES` is always False and the
    // `adjective=True` flag is a no-op. The trait default returns None →
    // `prefix_currency` is never applied. Verified: `to_currency(12.34,
    // "EUR", adjective=True)` == the adjective=False output.
    //
    // currency_precision: likewise inherits `CURRENCY_PRECISION = {}`, so
    // `.get(currency, 100)` is **always 100** — UK has no 3-decimal or
    // 0-decimal currencies. This is load-bearing and counter-intuitive:
    //   * KWD/BHD use divisor 100, not 1000 →
    //     `to_currency(12.34, "KWD")` == "дванадцять динарів, тридцять чотири
    //     філси" (34 fils, not 340).
    //   * JPY uses divisor 100, not 1, so the zero-decimal short-circuit in
    //     `Num2Word_Base.to_currency` never fires and JPY *does* render a
    //     cents segment → "дванадцять єн, тридцять чотири сен".
    // Both corpus-confirmed. The trait default of 100 is therefore exactly
    // right and must NOT be "fixed".

    /// Port of `Num2Word_UK.pluralize`.
    ///
    /// ```python
    /// if n % 100 < 10 or n % 100 > 20:
    ///     if n % 10 == 1:      form = 0
    ///     elif 5 > n % 10 > 1: form = 1
    ///     else:                form = 2
    /// else:
    ///     form = 2
    /// return forms[form]
    /// ```
    ///
    /// `mod_floor` rather than `%`: Python's `%` floors on negatives. Every
    /// reachable caller passes a non-negative `n` (`parse_currency_parts`
    /// returns `abs`), so the two agree today, but the port matches the
    /// source's semantics rather than relying on that.
    ///
    /// Note this is a *different* function from the module-level `pluralize`
    /// used by `int2word`, which pluralizes the THOUSANDS scale words. Same
    /// rule, different call sites; Python has one method serving both.
    fn pluralize(&self, n: &BigInt, forms: &[String]) -> Result<String> {
        let ten = BigInt::from(10);
        let m100 = n.mod_floor(&BigInt::from(100));
        let m10 = n.mod_floor(&ten);

        let form = if m100 < ten || m100 > BigInt::from(20) {
            if m10.is_one() {
                0
            } else if m10 > BigInt::one() && m10 < BigInt::from(5) {
                1
            } else {
                2
            }
        } else {
            2
        };

        // `forms[form]` → IndexError if the tuple is short. Every UK entry has
        // exactly 3 forms, so this is unreachable; modelled for fidelity.
        forms
            .get(form)
            .cloned()
            .ok_or_else(|| index_error("tuple index out of range"))
    }

    /// Port of `Num2Word_UK._money_verbose`:
    /// `self._int2word(number, currency in FEMININE_MONEY)`.
    fn money_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        self.int2word(number, is_feminine_money(currency), 0)
    }

    /// Port of `Num2Word_UK._cents_verbose`:
    /// `self._int2word(number, currency in FEMININE_CENTS)`.
    fn cents_verbose(&self, number: &BigInt, currency: &str) -> Result<String> {
        self.int2word(number, is_feminine_cents(currency), 0)
    }

    // cents_terse: UK does not override `Num2Word_Base._cents_terse`. With
    // CURRENCY_PRECISION empty the divisor is 100 → width 2, which is what the
    // trait default computes via `default_cents_terse(n, 100)`.
    // Verified: `to_currency(0.05, "EUR", cents=False)` == "нуль євро, 05 центів".

    /// Port of `Num2Word_UK.to_currency`.
    ///
    /// ```python
    /// def to_currency(self, val, currency="EUR", cents=True,
    ///                 separator=",", adjective=False):
    ///     if isinstance(val, int):
    ///         try:
    ///             cr1, cr2 = self.CURRENCY_FORMS[currency]
    ///         except (KeyError, AttributeError):
    ///             return super().to_currency(val, ...)   # -> NotImplementedError
    ///         minus_str = self.negword if val < 0 else ""
    ///         abs_val = abs(val)
    ///         money_str = self.to_cardinal(abs_val)
    ///         if abs_val == 1: currency_str = cr1[0]
    ///         else:            currency_str = cr1[1]
    ///         return ("%s %s %s" % (minus_str, money_str, currency_str)).strip()
    ///     return super().to_currency(val, ...)
    /// ```
    ///
    /// # Fixed: number and gender agreement on the int path (#164)
    ///
    /// Python hard-codes `cr1[0]` for `abs == 1` and `cr1[1]` otherwise and
    /// calls `to_cardinal` instead of `_money_verbose`, so it emits
    /// "п'ять долари", "нуль долари", "один гривня" and
    /// "двадцять один гривні". The port instead uses the same `pluralize` +
    /// `_money_verbose` pair as the float path, so an int always matches the
    /// unit half of the equivalent float: `to_currency(5, "USD")` ==
    /// "п'ять доларів", `to_currency(21, "UAH")` == "двадцять одна гривня",
    /// `to_currency(1, "JPY")` == "одна єна".
    ///
    /// # Quirks still reproduced (all interpreter-verified)
    ///
    /// 1. **`minus_str` is `self.negword`, unstripped and unpadded** — unlike
    ///    `Num2Word_Base`, which builds `"%s " % self.negword.strip()`. For UK
    ///    `negword` is "мінус" with no surrounding space, so the `" ".join`-ish
    ///    `"%s %s %s"` template plus the trailing `.strip()` yields the same
    ///    result either way. Ported literally regardless.
    /// 2. **`cents`, `separator` and `adjective` are ignored on the int path.**
    ///    No cents segment is emitted, so `separator` has nothing to join and
    ///    `adjective` nothing to prefix (`CURRENCY_ADJECTIVES` is empty).
    ///
    /// The unknown-currency fallback delegates to the base implementation,
    /// which re-raises the `KeyError` as
    /// `NotImplementedError('Currency code "XXX" not implemented for
    /// "Num2Word_UK"')` — `N2WError::NotImplemented`. The `AttributeError` arm
    /// of Python's `except (KeyError, AttributeError)` guards against a class
    /// with no `CURRENCY_FORMS` at all; `Num2Word_UK` defines one, so only the
    /// `KeyError` arm is reachable and both arms lead to the same base call.
    fn to_currency(
        &self,
        val: &CurrencyValue,
        currency: &str,
        cents: bool,
        separator: Option<&str>,
        adjective: bool,
    ) -> Result<String> {
        // Trait now hands us None when the caller omitted separator=;
        // resolve it to this language's own default before the ported body.
        let separator = separator.unwrap_or(self.default_separator());
        if let CurrencyValue::Int(v) = val {
            // `self.CURRENCY_FORMS[currency]` — miss falls through to the base
            // implementation, which raises NotImplementedError.
            if let Some(forms) = self.currency_forms.get(currency) {
                let cr1 = &forms.unit;

                let minus_str = if v.is_negative() { NEGWORD } else { "" };
                let abs_val = v.abs();
                // #164: Python calls `to_cardinal` and picks `cr1[0]` for 1,
                // else `cr1[1]`, so "п'ять долари" / "двадцять один гривні".
                // Reuse the float path's agreement instead: `_money_verbose`
                // for the numeral's gender and `pluralize` for the noun form.
                let money_str = self.money_verbose(&abs_val, currency)?;
                let currency_str = self.pluralize(&abs_val, cr1)?;

                // ("%s %s %s" % (...)).strip() — for a positive value
                // minus_str is empty, so the template leaves a leading space
                // that .strip() removes.
                return Ok(format!("{} {} {}", minus_str, money_str, currency_str)
                    .trim()
                    .to_string());
            }
        }

        // Floats (and unknown-currency ints) take `Num2Word_Base.to_currency`.
        crate::currency::default_to_currency(self, val, currency, cents, separator, adjective)
    }

    // to_cheque: UK does not override `Num2Word_Base.to_cheque`, and the trait
    // default is a faithful port of it. It routes through the hooks above —
    // `currency_forms` (miss → NotImplementedError), `currency_precision`
    // (always 100 → "NN/100"), `money_verbose` (feminine-aware) — and takes
    // `cr1[-1]`, the genitive plural, unconditionally.
    // Verified: `to_cheque(1234.56, "JPY")` ==
    // "ОДНА ТИСЯЧА ДВІСТІ ТРИДЦЯТЬ ЧОТИРИ AND 56/100 ЄН" — note "56/100" even
    // for JPY, and the feminine "ОДНА" from `_money_verbose`.
    //
    // cardinal_from_decimal: deliberately left at the trait default (raises
    // NotImplemented). See the module-level "Fractional cents" note.
}

#[cfg(test)]
mod float_tests {
    use super::*;
    use std::str::FromStr;

    /// The float path receives `precision` from the Python binding, computed
    /// as `abs(Decimal(str(value)).as_tuple().exponent)` — the fractional-digit
    /// count of Python's `repr(float)`. That is passed explicitly here because
    /// Rust's own `{}` Display drops the `.0` of a whole float (so `1.0`→"1",
    /// precision 0), which would *not* reproduce Python's `str(1.0)`=="1.0".
    /// The precisions below are the exact values Python computes.
    fn card_float(f: f64, precision: u32) -> String {
        let l = LangUk::new();
        l.to_cardinal_float(
            &FloatValue::Float {
                value: f,
                precision,
            },
            None,
        )
        .unwrap()
    }

    fn card_dec(s: &str) -> String {
        let l = LangUk::new();
        let value = BigDecimal::from_str(s).unwrap();
        // precision = abs(exponent) = number of fractional digits in the string.
        let precision = s.split_once('.').map(|(_, f)| f.len() as u32).unwrap_or(0);
        l.to_cardinal_float(&FloatValue::Decimal { value, precision }, None)
            .unwrap()
    }

    #[test]
    fn corpus_floats() {
        // Every "lang":"uk","to":"cardinal" corpus row whose arg has a dot.
        // (value, precision) — precision is Python's repr fractional-digit count.
        assert_eq!(card_float(0.0, 1), "нуль кома нуль нуль");
        assert_eq!(card_float(0.5, 1), "нуль кома п'ять");
        assert_eq!(card_float(1.0, 1), "один кома нуль нуль");
        assert_eq!(card_float(1.5, 1), "один кома п'ять");
        assert_eq!(card_float(2.25, 2), "два кома двадцять п'ять");
        assert_eq!(card_float(3.14, 2), "три кома чотирнадцять");
        assert_eq!(card_float(0.01, 2), "нуль кома нуль один");
        assert_eq!(card_float(0.1, 1), "нуль кома один");
        assert_eq!(card_float(0.99, 2), "нуль кома дев'яносто дев'ять");
        assert_eq!(card_float(1.01, 2), "один кома нуль один");
        assert_eq!(card_float(12.34, 2), "дванадцять кома тридцять чотири");
        assert_eq!(card_float(99.99, 2), "дев'яносто дев'ять кома дев'яносто дев'ять");
        assert_eq!(card_float(100.5, 1), "сто кома п'ять");
        assert_eq!(
            card_float(1234.56, 2),
            "одна тисяча двісті тридцять чотири кома п'ятдесят шість"
        );
        assert_eq!(card_float(-0.5, 1), "мінус нуль кома п'ять");
        assert_eq!(card_float(-1.5, 1), "мінус один кома п'ять");
        assert_eq!(card_float(-12.34, 2), "мінус дванадцять кома тридцять чотири");
        // f64-artefact cases: str(f) gives the repr digits, no float2tuple.
        assert_eq!(card_float(1.005, 3), "один кома нуль нуль п'ять");
        assert_eq!(card_float(2.675, 3), "два кома шістсот сімдесят п'ять");
    }

    #[test]
    fn corpus_decimals() {
        // Every "lang":"uk","to":"cardinal_dec" corpus row.
        assert_eq!(card_dec("0.01"), "нуль кома нуль один");
        assert_eq!(card_dec("1.10"), "один кома десять"); // trailing zero → 10, not 1
        assert_eq!(card_dec("12.345"), "дванадцять кома триста сорок п'ять");
        assert_eq!(
            card_dec("98746251323029.99"),
            "дев'яносто вісім трильйонів сімсот сорок шість мільярдів двісті п'ятдесят один \
             мільйон триста двадцять три тисячі двадцять дев'ять кома дев'яносто дев'ять"
        );
        assert_eq!(card_dec("0.001"), "нуль кома нуль нуль один");
    }
}

#[cfg(test)]
mod kwargs_tests {
    use super::*;

    fn kw(items: &[(&str, &str)]) -> Kwargs {
        Kwargs(
            items
                .iter()
                .map(|(k, v)| (k.to_string(), KwVal::Str(v.to_string())))
                .collect(),
        )
    }

    fn card(n: i64, items: &[(&str, &str)]) -> String {
        LangUk::new()
            .to_cardinal_kw(&BigInt::from(n), &kw(items))
            .unwrap()
    }

    /// gladiaio/num2words2#145.
    #[test]
    fn gender() {
        assert_eq!(card(21, &[("gender", "f")]), "двадцять одна");
        assert_eq!(card(21, &[("gender", "Feminine")]), "двадцять одна");
        assert_eq!(card(2, &[("gender", "ж")]), "дві");
        assert_eq!(card(21, &[("gender", "m")]), "двадцять один");
        assert_eq!(card(21, &[]), "двадцять один");
        let f = LangUk::new()
            .to_cardinal_float_kw(
                &FloatValue::Float { value: 1.1, precision: 1 },
                None,
                &kw(&[("gender", "f")]),
            )
            .unwrap();
        assert_eq!(f, "одна кома одна");
        // Unknown kwargs still decline to the dispatcher.
        assert!(matches!(
            LangUk::new().to_cardinal_kw(&BigInt::from(1), &kw(&[("foo", "x")])),
            Err(N2WError::Fallback(_))
        ));
    }
}
