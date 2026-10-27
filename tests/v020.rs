//! v0.2.0 (Design 3 layout) — registry pin.
//!
//! The layout change did not touch the registry: 299 calculators at
//! v0.2.0; v0.3.0 adds one — linear regression — for 300; v0.4.0
//! adds the expression calculator + five compound-unit converters
//! for 306; v0.7.0 adds replicates summary, outlier check and
//! serial dilution for 309. (The v0.2.0 on-screen keypad and its
//! pad tests were removed in v0.3.0 — the keypad made the window
//! contents uneven.)

use calcdex::all_calcs;

#[test]
fn registry_unchanged_by_layout() {
    // Was 299 at v0.2.0; v0.3.0 item #34 adds Linear Regression
    // (300); v0.4.0 adds #5 + #8 (306); v0.7.0 adds #33/#36/#35 (309);
    // v0.8.0 adds the engineering seven (316).
    assert_eq!(all_calcs().len(), 316);
}
