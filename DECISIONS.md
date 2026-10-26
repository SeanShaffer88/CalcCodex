# Decisions — CalcDex

## 2026-10-25 — Renamed CalcCodex → CalcDex
SEAN's order: "rename repo project and program to CalcDex". The rename
applies everywhere the program/project is named: the directory
(~/workspace/calcdex), the Cargo package (calcdex), the binary
(CalcDex.exe), the window title/header (CALCDEX), the CLI name and help
text, the installer, and all docs. Calculator ids (e.g.
compound-interest) and formulas do not change. v0.1.0 was built and
field-tested locally under the old name CalcCodex and renamed before
any GitHub publication; the first published release is CalcDex
v0.1.1. Historical mentions of the old name survive only in this note
and the CHANGELOG rename note.

## 2026-10-25 — Icon: option 1 (calculator outline)
SEAN was shown a six-option icon contact sheet for CalcDex. He first
tapped Icon 2, then immediately sent "1" as his final word — the FINAL
pick is **Icon 1**, superseding 2. Icon 1 is the calculator-outline
design: phosphor green (#4ade80) calculator outline with a dark-green
display screen and a 3x3 button grid plus a wide bottom bar, on
near-black (#0c0c0c), sharp corners. The production artwork
(assets/icon.ico, 16–256 px, plus assets/icon-64.rgba for the window)
was redrawn cleanly from the concept — the concept sheet's overlap
glitch, where the wide bottom row crossed the third button row, was
fixed with even spacing. The icon is wired into the window/taskbar
(eframe with_icon), the .exe resource itself (build.rs + windres,
.rsrc section), and the NSIS installer/uninstaller.

## 2026-10-25 — Smarter dates + calculation log (v0.1.1)
From SEAN's field test of v0.1.0: date entry as separate year/month/
day boxes with "On year" wording was odd. Decided: one MM/DD/YYYY box
per date (explicit date-group metadata on the registry, internal
numeric triples unchanged), a plain-words format hint, a "Today?"
button beside every date field (Age End Date and Countdown From Date
pre-filled), the CLI accepts a date as one MM/DD/YYYY argument (the
six-number form still works identically), and a persistent
calculation log shared by GUI and CLI (%APPDATA%\CalcDex\history.txt,
500-entry cap, --log / --clear-log, click-to-restore in the GUI).


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
CalcDex registers ONE converter per dimension with the units as
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

## 2026-10-25 — Future layout: Design 3 picked
SEAN picked **Design 3** from the 8 research mockups as CalcDex's
future layout (2026-10-25). The redesign is its own slice AFTER
v0.1.2 — v0.1.2 deliberately keeps the current layout and only
carries the readability / contrast / responsive work, which is
written generally so it carries forward into Design 3.

## 2026-10-25 — v0.2.0: Design 3 built (layout decisions + brief)

SEAN's design brief for this build, verbatim (2026-10-25):
"CalcDex should feel interactive, intuitive, well-organized,
accessible, easy, robust, not cumbersome — and very polished."
The eight words were treated as acceptance criteria; the CHANGELOG /
README "Feel" section records how each was honoured and what is
still Unverified (SEAN's on-Windows run is the verdict).

Layout decisions taken while building Design 3:

- **Modes are the categories.** The mockup's rail lists a consumer
  calculator's six modes (Standard, Scientific, Graphing,
  Programmer, Date Calculation, Converters). CalcDex has no such
  modes — its organisation IS its 14 categories — so the rail lists
  All + the 14 categories as modes in a curated order (not
  alphabetical), with short labels (CHEM, STATS, CONVERT, BUILD,
  DATE, DAILY…) and the full name + count in the row tooltip.
- **Shortcut set:** Alt+1..Alt+9, Alt+0 for the first ten modes,
  Alt+Q/W/E/R/T for the remaining five, in rail order. Only these
  are printed (on the rail rows and in the README), and only these
  are implemented. Mode rows are click targets; the keyboard path
  to a mode is its Alt+ key (everything after that — chips, fields,
  CALCULATE, RE-OPEN — is Tab-reachable egui widgets).
- **The mockup's memory row (MC/MR/M+/M-/MS) is omitted.**
  CalcDex's memory is the persistent calculation log / history
  rail: results are saved automatically and re-open with inputs
  restored, which does the job a register memory does on the
  reference app, in the way this program already worked.
- **Operator keys × ÷ are omitted from the keypad on purpose.**
  CalcDex fields take single values, not expressions; a × or ÷
  keypress would put a character in the field that cannot parse
  and would only produce an error. Sign is the +/− key; `/` appears
  for date fields and `,` for list fields (the separator each
  field actually uses); `=` is Calculate.
- **The mockup shows a category list under the modes in the rail.**
  Ours are the same list (modes ARE categories), so there is no
  second list — one navigation system, nothing duplicated.
- **Click counts did not grow vs v0.1.2.** Pick a result:
  mode click (or Alt+key, or none — All is default) → calculator
  click → type/pad the fields → CALCULATE. Identical count to
  v0.1.2's category → calculator → CALCULATE, and the keypad adds
  a mouse-only path that did not exist before. Search is the same
  box, still filtering as you type.
- **History collapse threshold: 1100 pt window width** (content
  rect). At/above it, LOG docks/undocks the right rail; below it,
  LOG switches the centre to the log view (the v0.1.x behaviour),
  so history is reachable at every size including the 760 pt
  minimum. LOG starts shown, as in the mockup.
- **Keypad target safety:** the pad edits only the string of the
  field last clicked/focused on the currently selected calculator
  (falling back to that calculator's first field). Fields are
  per-selection state and are reset when a calculator is picked,
  so pad input can never land in another calculator's saved or
  logged state; the log only ever records Calculate runs.
- **Display headline:** the display shows the result's first line
  at 24 pt; the complete output (formula, substituted values,
  result) is duplicated verbatim into the STEPS / FULL RESULT
  footer — the design's "steps footer" — so nothing is truncated.
