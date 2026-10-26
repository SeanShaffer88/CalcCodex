# QUALITY — CalcDex v0.1.1

Scored from evidence only (Elite §17). Gate numbers below are from the actual gate run
at release (2026-10-25, this machine).

| Dimension | Score /5 | Evidence |
|---|---|---|
| Correctness of formulas | 4 | Standard published formulas named per calculator; known-answer tests across all categories pass (12/12 test groups green; registry count 299 asserted). Remaining risk: long-tail calculators asserted by registry presence + spot vectors, not an individual vector each — hence 4, not 5. |
| Error handling | 4 | Divide-by-zero, negative sqrt/log, invalid dates, out-of-range unit numbers all return clear errors; `errors_are_clear_and_never_panic` test; no panics by construction (no unwrap on user input paths in registry dispatch). |
| Tests | 4 | 19 tests green at v0.1.1: the 12 v0.1 known-answer/registry groups plus 7 new v0.1.1 groups (date parse/format round-trip + teaching errors, civil_from_days inverse sweep, CLI date-arg equivalence on SEAN's field-test values, date-group metadata on all 9 date calculators, log append/read/500-cap/restore round-trip, end-date wording). Coverage % not measured (no coverage tool in the offline toolchain). |
| Code structure | 4 | One module per category, registry pattern identical to DevTextTools; GUI/CLI share `run_calc`. Clippy: 0 warnings on --all-targets. |
| GUI / theme fidelity | 4 | Terminal palette applied in code from the exact Purgeon values; Cascadia font files loaded when present. FIELD-VERIFIED for v0.1.0 on SEAN's Windows machine 2026-10-25 (his screenshots: main window renders in the Terminal theme, categories/search/counters correct; the Age result was exactly right). The v0.1.1 GUI additions (merged date fields, Today? button, log view) are code- and test-verified but await his fresh screenshots — hence 4, not 5. |
| Audience fit (academic/professional, SEAN 2026-10-25) | 4 | Worked-step outputs, full precision, named constants, teaching error messages, ESTIMATE labels — implemented and visible in CLI output. v0.1.1 adds what the field test asked for: unambiguous date entry with format hints and teaching errors, plus a persistent calculation log for later reference (GUI + CLI). Not 5 until the v0.1.1 GUI additions are field-checked. |
| Docs | 4 | README TL;DR-first, COVERAGE/PARITY/ROADMAP honest-gap docs, PROJECT_RULES, DECISIONS, CHANGELOG. Screenshots "To be confirmed" per the no-placeholder rule. |
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
