# QUALITY — CalcDex v0.1.2

Scored from evidence only (Elite §17). Gate numbers below are from the actual gate run
at release (2026-10-25, this machine).

| Dimension | Score /5 | Evidence |
|---|---|---|
| Correctness of formulas | 5 | v0.1.2 full audit (docs/AUDIT.md): every one of the 299 calculators now has at least one independent known-answer vector (was 67); 14 formula/edge-case bugs found and fixed, each pinned by a regression test. Expected values computed independently (Python/hand), never from program output. |
| Error handling | 4 | Divide-by-zero, negative sqrt/log, invalid dates, out-of-range unit numbers all return clear errors; `errors_are_clear_and_never_panic` test; no panics by construction (no unwrap on user input paths in registry dispatch). |
| Tests | 5 | 53 test groups green at v0.1.2 (19 pre-existing + 34 new audit groups in tests/audit_*.rs): 299/299 calculators carry a known-answer vector; bug-pin tests cover every v0.1.2 fix. Coverage % not measured (no coverage tool in the offline toolchain) — the per-calculator vector count is the evidence instead. |
| Code structure | 4 | One module per category, registry pattern identical to DevTextTools; GUI/CLI share `run_calc`. Clippy: 0 warnings on --all-targets. |
| GUI / theme fidelity | 4 | Terminal palette from the exact Purgeon values. v0.1.1 FIELD-VERIFIED on SEAN's machine 2026-10-25: merged date fields + format hints + Today? buttons + pre-filled End Date render and work; LOG (1) view with RE-OPEN/CLEAR LOG works; Icon 1 shows in the title bar; his Age run 06/04/1977 → 10/26/2026 = 49y / 18,041d / 2,577w independently re-checked exactly correct. His captures are in docs/screenshots/. v0.1.2 readability + responsive changes are verified by construction, measured contrast ratios (docs/AUDIT.md) and code review — on-Windows resize/zoom is Partially Verified until SEAN's check, hence 4, not 5. |
| Audience fit (academic/professional, SEAN 2026-10-25) | 4 | Worked-step outputs, full precision, named constants, teaching error messages, ESTIMATE labels — implemented and visible in CLI output. v0.1.1 adds what the field test asked for: unambiguous date entry with format hints and teaching errors, plus a persistent calculation log for later reference (GUI + CLI). Not 5 until the v0.1.1 GUI additions are field-checked. |
| Docs | 4 | README TL;DR-first with SEAN's two real v0.1.1 captures (byte-identical, SHA-256 verified, honestly captioned as showing the old left column; fresh v0.1.2 main-window capture still owed). docs/AUDIT.md records the full v0.1.2 audit: per-category tables, contrast ratios, layout mechanisms, remaining gaps. |
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
