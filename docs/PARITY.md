# Parity — CalcCodex v0.1

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
| Date & Time | Real calendar algorithm + dates_known_answers (leap-year rejection asserted) | Covered |
| Sports/Food/Ecology/Other | Registry + sports_food_eco_known_answers | Covered — representative set (thinner than Math/Finance by design of v0.1) |
| Error handling (safety) | errors_are_clear_and_never_panic test | Covered for tested paths |
| GUI == CLI parity | Same `run_calc` dispatch in main.rs CLI and GUI calculate() | Covered by construction; GUI look Unverified until SEAN's run |
| Live-data calculators | Not implemented (offline design) | Gap — ROADMAP |
| Matrix / graphing / two-series stats | Not implemented | Gap — ROADMAP |
| Jurisdiction tax tables | Not implemented (rate-as-input versions exist) | Gap — ROADMAP |

Audience fit (SEAN decision, 2026-10-25: students, teachers,
professors, scientists, engineers) | Formula + substituted values +
result in outputs; full-precision formatting; scientific-notation
inputs; constants named; estimates labelled | Implemented in the
registry output text and README — Verified by reading outputs in the
CLI smoke tests; GUI presentation Unverified until SEAN's run

No dimension is scored from intention: anything without a test or a
registry entry is listed as a Gap above.
