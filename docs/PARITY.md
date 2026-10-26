# Parity — CalcDex v0.3.0

Scored ONLY from test/build evidence in this repo (Elite rule).
"Covered" means a registered calculator exists AND the category has
known-answer test coverage. Percentages are the share of the *source
sites' distinct operations* (after the parametric collapsing explained
in COVERAGE.md) judged covered by the registry — an integrator
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
| Statistics | List/pair registry + stats_known_answers + v030 regression vectors (textbook dataset m=0.6, b=2.2, R²=0.6) | Covered — core set, incl. Linear Regression & Calibration (v0.3.0 #34); two-series stats beyond regression = roadmap |
| Date & Time | Real calendar algorithm + dates_known_answers + v011 tests (MM/DD/YYYY round-trip, CLI date-arg equivalence) | Covered — merged date fields, Today? button, Birth/End Date wording |
| Sports/Food/Ecology/Other | Registry + sports_food_eco_known_answers | Covered — representative set |
| Error handling (safety) | errors_are_clear_and_never_panic test + v030 teaching-error tests (vary/batch) | Covered for tested paths |
| Batch-1 features (#1/#11/#16/#20/#31/#46/#47/#48/#50) | features.rs / meta.rs / settings.rs + tests/v030.rs (11 tests incl. registry-wide example/default/hint coverage over all 300 calculators) + CLI smoke | Covered by construction + tests; new GUI panels Unverified on Windows until a field run |
| Tooltips (standing rule) | tip_* helper constructors in main.rs + tests/tooltips.rs source scan | Covered by construction + gate test |
| Calculation log | lib history functions + tests/v011 + CLI --log/--clear-log | Covered — FIELD-VERIFIED at v0.1.1 |
| Live-data calculators | Not implemented (offline design) | Gap — ROADMAP |
| Matrix / graphing | Not implemented | Gap — ROADMAP |
| Jurisdiction tax tables | Not implemented (rate-as-input versions exist) | Gap — ROADMAP |

## GUI vs CLI parity (standing rule: full command-line support)

Every capability in the window has a command-line counterpart
below. The settings file (%APPDATA%\CalcDex\settings.txt) and the
history file are shared by both — they are the single source of
truth, not two copies.

| Window capability | Command line |
|---|---|
| Pick a mode (category) and a calculator | `CalcDex --list`, `CalcDex --search <words>`, then the calculator id |
| Search with synonyms + ranking | `--search` (same `search_ranked` engine; Ctrl+K/`/` only move focus) |
| Fill fields and press CALCULATE | `CalcDex <id> <numbers...>` (same registry dispatch, identical output) |
| Today? button beside a date field | Pass today's date as one MM/DD/YYYY argument |
| TRY AN EXAMPLE | `CalcDex <id> --example` |
| RESET (smart defaults) | The defaults ARE the example values — `CalcDex <id> --example` runs them; `--describe` shows them |
| Hints under fields | `CalcDex --describe <id>` prints every field + hint |
| SIG FIGS selector | `--sig-figs N` (same annotation; log keeps full precision both ways) |
| VARY ONE INPUT panel | `CalcDex <id> --vary <field> <start> <end> <step> <others...>` |
| BATCH panel | `CalcDex --batch <id> <in.csv> <out.csv>` (same engine; the panel only adds typed path fields) |
| Display + steps footer | The full result text is printed verbatim |
| History rail, RE-OPEN | `--log` prints entries with their inputs — re-running with those inputs is the restore |
| CLEAR LOG | `--clear-log` |
| ★ FAVOURITE toggle + FAVOURITES mode | `--favourite <id>` / `--unfavourite <id>` / `--favourites` |
| RECENT chips | `--favourites` prints the recent list (CLI runs update it too) |
| Remembered inputs | The shared settings file: CLI runs write them, the window reads them (and vice versa); `--favourites` reports how many calculators have saved inputs |
| Mode rail + Alt+ shortcuts | Navigation only — the CLI names the calculator directly, which is the same capability in one step |

Window-only capabilities: **none**.

The v0.2.0 on-screen keypad was removed in v0.3.0 (it made the
window contents uneven); keyboard entry covers all input, in the
window and on the command line alike.

Audience fit (audience: students, teachers,
professors, scientists, engineers): formula + substituted values +
result in outputs; full precision; scientific-notation inputs;
constants named; estimates labelled. Implemented in the registry
output text and README — Verified by CLI output; GUI presentation
of the v0.3.0 panels is Unverified until a fresh field run.
