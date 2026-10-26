//! Tooltip rule (SEAN standing rule, applied v0.3.0): every button,
//! menu item and interactive control in the window has a tooltip —
//! no exceptions. Enforcement is by construction + this scan:
//! buttons/selectables in src/main.rs may ONLY be built through the
//! tip_* helpers (each requires a tooltip argument). If a bare
//! widget constructor appears
//! outside those helpers, this test fails.

#[test]
fn every_button_goes_through_a_tooltip_helper() {
    let path = format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(&path).expect("read src/main.rs");
    // Strip comment lines (they mention the patterns in prose).
    let code: String = src
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    // The helpers exist and are the only construction sites.
    for helper in [
        "fn tip_btn(",
        "fn tip_btn_sized(",
        "fn tip_sel(",
        "fn tip_sel_value(",
        "fn tip_chip(",
    ] {
        assert!(code.contains(helper), "missing helper {helper}");
    }
    assert_eq!(
        code.matches("ui.button(").count(),
        0,
        "a bare ui.button( bypasses the tooltip helpers"
    );
    assert_eq!(
        code.matches("ui.selectable_label(").count(),
        1,
        "selectable labels must be built only inside tip_sel"
    );
    assert_eq!(
        code.matches(".selectable_value(").count(),
        1,
        "selectable values must be built only inside tip_sel_value"
    );
    assert_eq!(
        code.matches("egui::Button::new").count(),
        3,
        "buttons must be built only in tip_btn, tip_btn_sized and tip_chip"
    );
    // Plenty of direct response tooltips for text edits and the rest.
    assert!(
        code.matches("on_hover_text").count() >= 15,
        "only {} on_hover_text sites — the sweep regressed",
        code.matches("on_hover_text").count()
    );
    // Sample tooltip texts are present and plain.
    assert!(code.contains("Fill in today's date"));
    assert!(code.contains("Clear the search text"));
}
