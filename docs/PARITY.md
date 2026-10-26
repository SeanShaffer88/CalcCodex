# Parity — CalcDex v0.2.0

Scored ONLY from test/build evidence in this repo (Elite rule).
"Covered" means a registered calculator exists AND the category has
known-answer test coverage. Percentages are the share of the *source
sites' distinct operations* (after the parametric collapsing explained
in COVERAGE.md) judged covered by the v0.1 registry — an integrator
judgement, labelled as such, not a website page count.

| Capability area | Evidence | Score |
|---|---|---|
| Math operations | Registry (Math module) + math_known_answers tests | Covered — core set |
| Finance operations | Registry + finance_known_answers (compound interest 1628.89 vector etc.) | Covered — core set |
| Health estimates | Registry + health_known_answers (BMI, BMR, due date) | Covered — core set, estimates labelled |
| Physics | Registry + physics_known_answers | Covered — core set |
| Chemistry | Registry + chemistry_known_answers (pH, gas law, dilution, half-life) | Covered — core set |
| Construction (Nutilz strength) | Registry + construction_known_answers (concrete bags, paint, stairs) | Covered — core set |
| Conversion (all Omni pair pages) | Parametric converters + conversion_known_answers | Covered by collapsing (see COVERAGE.md) |
| Statistics | List/pair registry + stats_known_answers | Covered — core set (two-series stats = roadmap) |
| Date & Time | Real calendar algorithm + dates_known_answers (leap-year rejection asserted) + v011 tests (MM/DD/YYYY parse/format round-trip, teaching errors, CLI date-arg equivalence, merged date-group metadata on all 9 date calculators) | Covered — v0.1.1 merged date fields, Today? button, Birth/End Date wording |
| Sports/Food/Ecology/Other | Registry + sports_food_eco_known_answers | Covered — representative set (thinner than Math/Finance by design of v0.1) |
| Error handling (safety) | errors_are_clear_and_never_panic test | Covered for tested paths |
| GUI == CLI parity | Same `run_calc` dispatch in main.rs CLI and GUI calculate() | Covered by construction; v0.1.1 GUI FIELD-VERIFIED on SEAN's machine (2026-10-25 screenshots). v0.2.0 Design 3 layout (mode rail, keypad, display, docked history rail) is code-verified only — its on-Windows look is Unverified until SEAN's fresh captures |
| Calculation log | lib history functions + tests/v011 log round-trip/cap/restore tests + CLI --log/--clear-log in main.rs | Covered — FIELD-VERIFIED at v0.1.1; v0.2.0 docks the same log as the right history rail (collapses to the centre log view below 1100 pt), file format and location unchanged |
| On-screen keypad (Design 3) | Pure pad logic in lib (`pad_apply`/`pad_key_allowed`) + tests/v020.rs (7 tests: editing, per-kind key matrix, registry untouched at 299) | Covered by construction + unit tests; on-Windows feel Unverified |
| Live-data calculators | Not implemented (offline design) | Gap — ROADMAP |
| Matrix / graphing / two-series stats | Not implemented | Gap — ROADMAP |
| Jurisdiction tax tables | Not implemented (rate-as-input versions exist) | Gap — ROADMAP |

Audience fit (SEAN decision, 2026-10-25: students, teachers,
professors, scientists, engineers) | Formula + substituted values +
result in outputs; full-precision formatting; scientific-notation
inputs; constants named; estimates labelled | Implemented in the registry
output text and README — Verified by reading outputs in the CLI smoke
tests; v0.1.0 GUI presentation FIELD-VERIFIED 2026-10-25; v0.1.1 GUI
additions Unverified until SEAN's fresh run

No dimension is scored from intention: anything without a test or a
registry entry is listed as a Gap above.
