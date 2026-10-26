//! v0.2.0 (Design 3 layout) — pure keypad-editing logic tests.
//!
//! The on-screen keypad writes into the focused field through the
//! library's `pad_apply`; these tests pin that behaviour, the
//! per-field-kind key matrix, and that the registry itself is
//! untouched by the layout change (still 299 calculators).

use calcdex::{all_calcs, pad_apply, pad_key_allowed, FieldKind, PadKey};

#[test]
fn pad_digits_append() {
    let mut s = String::new();
    for d in ['0', '6', '/', '0', '4'] {
        let k = if d == '/' {
            PadKey::Slash
        } else {
            PadKey::Digit(d)
        };
        s = pad_apply(&s, k);
    }
    assert_eq!(s, "06/04");
}

#[test]
fn pad_dot_only_once_per_number() {
    assert_eq!(pad_apply("1", PadKey::Dot), "1.");
    assert_eq!(pad_apply("1.5", PadKey::Dot), "1.5");
    // In a list, a new number after the separator may take its own dot.
    assert_eq!(pad_apply("1.5, 2", PadKey::Dot), "1.5, 2.");
    assert_eq!(pad_apply("1.5, 2.5", PadKey::Dot), "1.5, 2.5");
}

#[test]
fn pad_negate_toggles_sign() {
    assert_eq!(pad_apply("5", PadKey::Negate), "-5");
    assert_eq!(pad_apply("-5", PadKey::Negate), "5");
    assert_eq!(pad_apply("", PadKey::Negate), "-");
    assert_eq!(pad_apply("-", PadKey::Negate), "");
}

#[test]
fn pad_backspace_and_clear() {
    assert_eq!(pad_apply("12.5", PadKey::Backspace), "12.");
    assert_eq!(pad_apply("", PadKey::Backspace), "");
    assert_eq!(pad_apply("06/04/1977", PadKey::Clear), "");
}

#[test]
fn pad_slash_and_comma_are_well_behaved() {
    // No leading slash, no doubled slash, no trailing-comma spam.
    assert_eq!(pad_apply("", PadKey::Slash), "");
    assert_eq!(pad_apply("06/", PadKey::Slash), "06/");
    assert_eq!(pad_apply("06", PadKey::Slash), "06/");
    assert_eq!(pad_apply("", PadKey::Comma), "");
    assert_eq!(pad_apply("1, ", PadKey::Comma), "1, ");
    assert_eq!(pad_apply("1", PadKey::Comma), "1, ");
}

#[test]
fn pad_key_matrix_by_field_kind() {
    // Numeric: digits, dot, sign, editing — no slash, no comma.
    assert!(pad_key_allowed(FieldKind::Numeric, PadKey::Digit('3')));
    assert!(pad_key_allowed(FieldKind::Numeric, PadKey::Dot));
    assert!(pad_key_allowed(FieldKind::Numeric, PadKey::Negate));
    assert!(!pad_key_allowed(FieldKind::Numeric, PadKey::Slash));
    assert!(!pad_key_allowed(FieldKind::Numeric, PadKey::Comma));
    // Date: digits and slash only (plus editing).
    assert!(pad_key_allowed(FieldKind::Date, PadKey::Digit('3')));
    assert!(pad_key_allowed(FieldKind::Date, PadKey::Slash));
    assert!(!pad_key_allowed(FieldKind::Date, PadKey::Dot));
    assert!(!pad_key_allowed(FieldKind::Date, PadKey::Negate));
    assert!(!pad_key_allowed(FieldKind::Date, PadKey::Comma));
    // List: separators and per-number dots/signs.
    assert!(pad_key_allowed(FieldKind::List, PadKey::Comma));
    assert!(pad_key_allowed(FieldKind::List, PadKey::Dot));
    assert!(!pad_key_allowed(FieldKind::List, PadKey::Slash));
    // Editing keys are always allowed.
    for kind in [FieldKind::Numeric, FieldKind::Date, FieldKind::List] {
        assert!(pad_key_allowed(kind, PadKey::Backspace));
        assert!(pad_key_allowed(kind, PadKey::Clear));
    }
}

#[test]
fn registry_unchanged_by_layout() {
    assert_eq!(all_calcs().len(), 299);
}
