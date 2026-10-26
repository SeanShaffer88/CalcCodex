# Decisions — CalcCodex

## 2026-10-25 — Audience: academic + professional first
SEAN: "keep in mind the kind of people who will be using this program
— students, teachers, professors, scientists, engineers, etc."
Applied across v0.1: precise (not cutesy) wording; every result shows
formula + substituted values + result; full precision kept (general
number formatting carries 10 decimal places, nothing rounded away
silently); scientific-notation inputs accepted everywhere (Rust f64
parsing, documented in README + `--help`); physical constants named
with values in descriptions/outputs (g = 9.80665, R in both unit
systems, Avogadro); estimates labelled ESTIMATE; error messages teach
the reason (domain errors stated mathematically). Category and
calculator names use the vocabulary of the fields (Statistics, Gas
Laws, Thermodynamics-adjacent naming where honest).

## 2026-10-25 — Scope: calculators only
SEAN was offered three scopes (phased / calculators-only / one giant
build) and picked **calculators only**: skip Nutilz's PDF, image, text
and developer tools. Sources: Nutilz calculator subset + all of Omni
Calculator. SEAN added: "should be the ultimate tool for any and all
kinds of calculations" — maximise calculator breadth honestly.

## 2026-10-25 — Stack: Rust + egui/eframe 0.33
Same stack and versions as DevTextTools (SEAN's proven pattern, crates
already cached): pure functions in one registry dispatched identically
by GUI and CLI; fully offline; no extra dependencies at all in v0.1
(no network, no date crate — calendar maths is implemented from
Howard Hinnant's published days-from-civil algorithm and tested).

## 2026-10-25 — Parametric collapsing (counting honesty)
Omni lists a separate page per unit pair and per ingredient; Nutilz
lists a separate tool per image/PDF format pair (out of scope here).
CalcCodex registers ONE converter per dimension with the units as
numbered inputs, and one grams↔cups calculator per ingredient with
its published density. The registry count is the count of these real
registered calculators — no inflated per-pair counting in either
direction. Documented in docs/COVERAGE.md.

## 2026-10-25 — Registry signature
`CalcFn = Fn(&[f64]) -> Result<String, String>`. Fixed calculators
validate argument count in `run_calc`; list calculators take any
count; pair calculators take repeating pairs. Output text always
includes the result AND the formula/steps in plain words, so GUI,
CLI and tests see identical output.

## 2026-10-25 — Theme
Terminal theme with Purgeon's exact `:root` values (fetched from
SeanShaffer88/Purgeon src/styles.css in earlier work, recorded in
memory): bg #0c0c0c, panel #121212, raised #181818, border #262626,
green #4ade80, selection #163322, text #d0d0d0, muted #7a7a7a, amber
#e5c07b, cyan #56b6c2, violet #b294f0, red #f26d6d; radius 0; UI font
forced to the Monospace family and Cascadia Mono/Code font *files*
are loaded when present on the machine, matching DevTextTools'
Segoe-font loading pattern.

## 2026-10-25 — Compound interest signature
`compound-interest` is (principal, rate %, years) with YEARLY
compounding, so the headline example `compound-interest 1000 5 10`
= 1628.89 matches Omni's common default. Monthly compounding lives
in `compound-interest-monthly-deposit`; no silent assumption.
