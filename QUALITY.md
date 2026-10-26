# QUALITY — CalcDex v0.3.0

Scored from evidence only (Elite §17). Gate numbers below are from the actual gate run
at release (2026-10-25, this machine).

| Dimension | Score /5 | Evidence |
|---|---|---|
| Correctness of formulas | 5 | v0.1.2 full audit (docs/AUDIT.md): every one of the 299 calculators now has at least one independent known-answer vector (was 67); 14 formula/edge-case bugs found and fixed, each pinned by a regression test. Expected values computed independently (Python/hand), never from program output. |
| Error handling | 4 | Divide-by-zero, negative sqrt/log, invalid dates, out-of-range unit numbers all return clear errors; `errors_are_clear_and_never_panic` test; no panics by construction (no unwrap on user input paths in registry dispatch). |
| Tests | 5 | 67 test groups green at v0.3.0 (60 from v0.2.0, minus 6 removed keypad tests, plus 12 in tests/v030.rs and 1 in tests/tooltips.rs): 300/300 calculators carry a known-answer vector AND a working example/default set (registry-wide tests); bug-pin tests cover every v0.1.2 fix. Coverage % not measured (no coverage tool in the offline toolchain) — the per-calculator vector count is the evidence instead. |
| Code structure | 4 | One module per category, registry pattern identical to DevTextTools; GUI/CLI share `run_calc`. Clippy: 0 warnings on --all-targets. |
| GUI / theme fidelity | 4 | Terminal palette from the exact Purgeon values. v0.1.1 FIELD-VERIFIED on SEAN's machine 2026-10-25: merged date fields + format hints + Today? buttons + pre-filled End Date render and work; LOG (1) view with RE-OPEN/CLEAR LOG works; Icon 1 shows in the title bar; his Age run 06/04/1977 → 10/26/2026 = 49y / 18,041d / 2,577w independently re-checked exactly correct. His captures are in docs/screenshots/. v0.1.2 readability + responsive changes are verified by construction, measured contrast ratios (docs/AUDIT.md) and code review — on-Windows resize/zoom is Partially Verified until SEAN's check. v0.2.0's Design 3 layout (mode rail, keypad, display, history rail) is likewise construction-verified with all new pairings measured at/above WCAG AA (docs/AUDIT.md addendum) — its on-Windows look and feel are Unverified until SEAN runs it, hence 4, not 5. |
| Audience fit (academic/professional, SEAN 2026-10-25) | 4 | Worked-step outputs, full precision, named constants, teaching error messages, ESTIMATE labels — implemented and visible in CLI output. v0.1.1 adds what the field test asked for: unambiguous date entry with format hints and teaching errors, plus a persistent calculation log for later reference (GUI + CLI). Not 5 until the v0.1.1 GUI additions are field-checked. |
| Batch-1 features (v0.3.0) | 4 | Items 1, 11, 16, 20, 31, 34, 46, 47, 48, 50 implemented as library engines (features.rs, meta.rs, settings.rs) with dedicated tests; CLI smoke verified live for --describe/--example/--vary/--batch/--sig-figs/--favourite(s) and linear-regression. Not 5: the new GUI panels (BATCH, VARY, FAVOURITES, sig-figs selector) are construction-verified only — a Windows field run is the remaining verdict. |
| GUI/CLI parity + tooltips (standing rules) | 5 | docs/PARITY.md carries the full parity table (zero window-only capabilities); tooltips enforced by construction (tip_* helpers) plus the tests/tooltips.rs source scan in the gate. |
| Docs | 4 | README TL;DR-first with SEAN's two real v0.1.1 captures (byte-identical, SHA-256 verified, honestly captioned as showing the old left column; captions updated at v0.2.0; fresh v0.2.0 main-window capture still owed and listed in the README). docs/AUDIT.md records the full v0.1.2 audit plus the v0.2.0 layout addendum: per-category tables, contrast ratios, layout mechanisms, remaining gaps. |
| Packaging | 4/5 — built and hash-verified locally (see gate transcript); not 5 until a release is downloaded back and re-verified by the integrator. | Two-asset rule: installer + portable zip, PE-verified, hashes recorded in the release report. |
| Performance | 4 | Pure arithmetic registry; no I/O, no network. Startup dominated by egui (same as DevTextTools, field-acceptable). Not benchmarked formally — 4 not 5. |

**Waivers:** none. **Fails:** none known at gate time.

## Gate transcript (summary)

- `cargo fmt` — applied; `--check` clean after.
- `cargo clippy --all-targets --locked` — 0 warnings, 0 errors.
- `cargo test --locked` (all targets) — 12 passed, 0 failed (12 known-answer/registry test groups in tests/known_answers.rs; lib/bin/doc targets 0 additional).
- Windows cross-build (x86_64-pc-windows-gnu) — success; PE32+ GUI subsystem 2 verified with file + objdump; system DLL imports only; exe 4,649,472 B, SHA-256 83a4fabeee2bef9211bb7c6b92ae69caec02fb65208ffbfaf954a74e11a8e9c8; strings check confirms the final sources (teaching error messages present).

## v0.1.1 gate transcript (2026-10-25, this machine)

- `cargo fmt` — applied; `--check` clean after.
- `cargo clippy --all-targets --locked` — 0 warnings, 0 errors.
- `cargo test --locked` (all targets) — 19 passed, 0 failed (12 known-answer/registry + 7 v0.1.1 groups; lib/bin/doc targets 0 additional).
- Live CLI demo (APPDATA pointed at a scratch folder): `CalcDex age 04/23/1988 10/25/2026` → "Age: 38 years (birthday already reached in the end year). Total days: 14064. 2009 weeks approx." — identical for the old six-number form; `--log` then showed both runs with inputs "04/23/1988, 10/25/2026"; `age 02/29/2025 ...` → teaching error naming the leap-year reason, exit 2.
- Windows cross-build (x86_64-pc-windows-gnu) — success; PE32+ GUI subsystem 2 verified with file + objdump; system DLL imports only; exe 4,732,416 B, SHA-256 9c4a45d0b75648b29c0556f1800e79bab29e8ced41f216c40c7087b925e7c3a5. The icon resource is embedded (`.rsrc` section present, ~14.8 KB). Strings check: "CalcDex", "MM/DD/YYYY", "Today?", "end year", "CALCULATION LOG" all present; ZERO strings of the old name in the exe (build-path residue from the moved target cache was purged and the affected crates rebuilt).
- Assets: CalcDex-Setup-0.1.1.exe 2,005,472 B, SHA-256 21f13f88206746ebb58bf93e879b12581db056fadcca5b788c552e06f0064b66; CalcDex-0.1.1-portable.zip 2,359,583 B, SHA-256 b1d7fc88e9e4d0f80e91f28174df0e48a37a65f0589eac81674a537c10f4258a (CalcDex.exe + LICENSE + PORTABLE.txt). Log timestamps are UTC (no timezone database is bundled) — stated in the README.

## v0.1.2 gate transcript (2026-10-25, this machine)

- `cargo fmt --check` — clean.
- `cargo clippy --locked --all-targets` — 0 warnings, 0 errors.
- `cargo test --locked` (all targets) — **53 passed, 0 failed** (19 pre-existing + 34 new audit groups). Known-answer coverage: 299/299 calculators (was 67/299 at v0.1.1).
- Windows cross-build (x86_64-pc-windows-gnu) — success; PE32+ GUI (subsystem 2) verified with file + objdump; system DLL imports only; `.rsrc` icon section present (~14.8 KB); exe 4,753,920 B, SHA-256 75f00933249d1ef431e349e698adbef9c6d4b55b47c2f4cc83e266fff24e8785; mtime/size freshness checked and v0.1.2 fix strings verified present in the binary ("must differ", "Size code must be 1", "whole numbers from the list", "never doubles", "Form 2").
- Assets: CalcDex-Setup-0.1.2.exe 2,014,013 B, SHA-256 4a9f5df0d2a05e1aabaead5f28438541fb031c947baec38e09088a7da0fea663; CalcDex-0.1.2-portable.zip 2,370,415 B, SHA-256 8289e0b5540d45a0d8e498517c1fb49d060098ee3d4edc5c0b2da8b785d3387f (CalcDex.exe + LICENSE + PORTABLE.txt; extracted exe hash matches the built exe).
- Screenshots: SEAN's two v0.1.1 captures copied byte-identically into docs/screenshots/ (SHA-256 d9d4e2d150c9c19acc03915ab517ddf6f3870dc02566913ca87fe0c13d4391f7 and 357e4d2de6bf76ee1d8914edf069c484bd8cdf87343376f0e58ae528878d6db2 — identical to his originals).
- On-Windows GUI behaviour of the v0.1.2 layout changes: **Partially Verified** (construction + code review + measured contrast ratios; SEAN's run is the remaining verdict). v0.1.1 features: **FIELD-VERIFIED** (see GUI row above).

## v0.2.0 gate transcript (2026-10-25, this machine)

- `cargo fmt --check` — clean.
- `cargo clippy --locked --all-targets` — 0 warnings, 0 errors.
- `cargo test --locked` (all targets) — **60 passed, 0 failed** (53 from v0.1.2 + 7 new in tests/v020.rs: keypad editing behaviour, per-field-kind key matrix, registry unchanged at 299).
- CLI smoke — `age 06/04/1977 10/26/2026` → "Age: 49 years (birthday already reached in the end year). Total days: 18041. 2577 weeks approx." and `compound-interest 1000 5 10` → 1628.89 — byte-identical to v0.1.2 behaviour; `--search loan` unchanged. Log file format/location unchanged.
- Windows cross-build (x86_64-pc-windows-gnu) — success; PE32+ GUI (subsystem 2) verified with file + objdump; system DLL imports only; `.rsrc` icon section present (0x39c8 ≈ 14.8 KB); exe 4,786,688 B, SHA-256 9c0c7f9bd80d05fcadb08aace29e434b9c150d2ac35bb4c326ed6bdc6dde6328; freshness strings verified ("KEYPAD", "STEPS / FULL RESULT", "PICK A CALCULATOR", "Enter = Calculate" present; ZERO old-name strings).
- Assets: CalcDex-Setup-0.2.0.exe 2,027,407 B, SHA-256 279c372893ceb0378319415f1314d93f53d06d972878965253e370f2f47f8e5a; CalcDex-0.2.0-portable.zip 2,387,138 B, SHA-256 e9bc2cc391a4126621edfde34d8d696e5848ad2cfa12ff106e296dba1d9eff14 (CalcDex.exe + LICENSE + PORTABLE.txt; extracted exe hash matches the built exe 9c0c7f9b…).
- Polish pass (SEAN's "very polished" criterion): dedicated review of the whole window by construction at minimum (760×520), default (1180×760) and large sizes — fixed: a narrow-width header hint label that crowded the header at 760 pt (removed), a display-frame width call that could misalign the frame's right edge (removed), stray 6/10 pt gaps pulled onto the 4/8 pt spacing rhythm, and the no-calculator empty state rebuilt as a designed framed panel. Verified by construction in the same pass: the keypad grid is exact at any width (four columns computed from available width, 48 pt rows), rail/centre/history edges share panel boundaries, and all three empty states (no calculator, no search matches, empty history) render designed panels.
- On-Windows look/feel of the Design 3 layout: **Unverified** — SEAN's run is the verdict.

## v0.3.0 gate transcript (2026-10-25, this machine)

- `cargo fmt --check` — clean.
- `cargo clippy --locked --all-targets` — 0 warnings, 0 errors.
- `cargo test --locked` (all targets) — **67 passed, 0 failed** (60 at v0.2.0, −6 keypad tests removed with the keypad, +12 in tests/v030.rs covering items 1/11/16/20/31/34/46/47/48/50 and the CLI/settings integration, +1 in tests/tooltips.rs enforcing the tooltip construction rule). Registry: 300 calculators, 300/300 known-answer + example coverage.
- CLI smoke (APPDATA pointed at a scratch folder): `--describe bmi` prints fields + hints + example; `compound-interest --example` = 1628.89; `--sig-figs 3 compound-interest 1000 5 10` adds "[To 3 significant figures: Future value: 1.63e3. Interest earned: 629.]" while the stored result keeps full precision; `--vary 1 0 10 2.5` sweeps 5 rows as CSV; `linear-regression 1 2 2 4 3 5 4 4 5 5 5` → slope 0.6, intercept 2.2, R² 0.6, calibration x = 4.666667; `--batch` over a 2-row CSV with one bad row → "2 rows: 1 ok, 1 errors", error captured per-row; `--favourite bmi` then a `bmi 70 175` run leaves `favourite=bmi` + `inputs.bmi=70;;175` + `recent=bmi` in the shared settings file.
- Keypad removal verified in the binary: `strings` finds ZERO "KEYPAD" occurrences; freshness strings present ("TRY AN EXAMPLE", "RUN BATCH", "Linear Regression & Calibration", settings-format strings, FAVOURITES).
- Windows cross-build (x86_64-pc-windows-gnu) — success; PE32+ GUI (subsystem 2) verified with file + objdump; system DLL imports only; `.rsrc` icon section present (0x39c8 ≈ 14.8 KB); exe 4,972,544 B, SHA-256 ccd5ecb839c45111ecd6bee8cfe55c1e9903fbd24a0d385ab121e0cd91e7199f.
- Assets: CalcDex-Setup-0.3.0.exe 2,084,318 B, SHA-256 784e2d624cde459182ab019dd3234781cdb48b538c3b25491e1c9457de0192e8; CalcDex-0.3.0-portable.zip 2,463,167 B, SHA-256 93b062eb8a1e4dfb252504bb013a7f2c1dc4d8dad8b23ec641ddab314b4495c5 (CalcDex.exe + LICENSE + PORTABLE.txt; extracted exe hash matches the built exe ccd5ecb8…).
- Centre rebalance after the keypad removal (construction check at 760×520 / 1180×760 / large): every centre block (display, form, steps footer, VARY) now spans the full centre width — the removed keypad was the only width-capped (520 pt) block, which is what made the contents uneven; form field cap raised 380 → 480 pt so fields line up better against the full-width display.
- On-Windows look/feel of the v0.3.0 panels and the rebalanced centre: **Unverified** — a field run is the verdict.
