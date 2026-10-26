# CalcDex — Project Rules

Prompt in force: The Elite Software Master Prompt v1.1 · Annexes: A (desktop/Windows), B (Storytold orchestration), C (SEAN defaults)
Purpose in one line: The ultimate offline calculator — every kind of calculation from Omni Calculator + the Nutilz calculator subset, one registry behind a Terminal-themed Windows app.
Frozen foundation: Cargo workspace-less single crate; `src/lib.rs` owns the registry types + helpers; calculators live one module per category under `src/calcs/` (module layout = agent territories); `src/main.rs` is GUI + CLI shell ONLY (no calculation logic). Interface: `Calc` struct + `all_calcs()` / `run_calc(id, &[f64])` — frozen for v0.1; changes land alone first.
Build: `cargo build --release` · Full gate: `cargo fmt --check` (after `cargo fmt`), `cargo clippy --all-targets --locked`, `cargo test --locked` (ALL targets) · Test corpus: `tests/known_answers.rs` (known-answer vectors, grows per slice)
Budgets: startup < 1 s on a normal PC; a single calculation < 50 ms; registry build < 10 ms; installer + portable produced by CI-style local script/NSIS, binary PE-verified before release.
Coverage floors: Elite defaults — 85% core / 70% overall / 100% safety (error) paths; known-answer tests for ≥30 calculators across all categories in v0.1.
Special rules for this project: (1) CALCULATORS ONLY — no PDF/image/text/dev tools (SEAN's pick). (2) Terminal theme with the exact Purgeon palette, radius 0, Cascadia Mono first. (3) Every formula is a standard published formula, named in the calculator's description; estimates are labelled ESTIMATE. (4) NO winget, ever (no code-signing certificate). (5) Releases = exactly two assets: loose installer .exe + one portable .zip. (6) Free + optional donations; no ads, no in-app purchases, no selling data. (7) No placeholder screenshots.
Known limitations live at: docs/ROADMAP.md · Decision log: DECISIONS.md · QUALITY.md: QUALITY.md · Parity: docs/PARITY.md · Coverage: docs/COVERAGE.md
