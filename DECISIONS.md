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

## 2026-10-25 — v0.2.0: Design 3 built (layout decisions)

Design acceptance criteria for this build: instant visible
response to input, fewest-clicks flows, a stable predictable
layout, WCAG AA contrast with a full keyboard path, no-panic
teaching errors, an uncluttered window, and a dedicated polish
pass (aligned keypad grid, 4/8 pt spacing rhythm, designed empty
states). The on-Windows result is Unverified until a field run.

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

## 2026-10-25 — v0.3.0: Batch 1 format choices

- **Sig-figs display (#1):** rounded showing is ADDED, never
  substituted; where plain notation would make the count
  ambiguous (integer part longer than the digit count, or
  exponent < -4) the rounded value is shown in scientific form
  (1628.89 at 3 figures = `1.63e3`). Dates inside a headline are
  never rewritten.
- **Vary CLI order (#16):** `CalcDex <id> --vary <field> <start>
  <end> <step> <other numbers...>` — field numbered from 0 (the
  `--describe` numbering), then the sweep, then the remaining
  inputs in their normal order. Output is CSV (`value,result`).
- **Batch CSV (#31):** header row = the display field labels in
  order (date groups = their one merged label; cells may be
  MM/DD/YYYY). List calculators: header is the single list label
  and each row's cells are the list values. Output = input cells
  + headline result (first line), or `ERROR: <message>`. A small
  hand-written CSV parser (quotes/commas/CRLF) instead of a new
  dependency.
- **Regression inputs (#34):** the registry's pair convention —
  (x, y) pairs in one list — plus ONE optional trailing value, a
  measured y for calibration (a new `pair_tail_mode` registry
  flag; other pair calculators are unaffected).
- **Recent placement (#46):** Recent chips live in the FAVOURITES
  mode above the chip strip, not above every mode's strip — the
  centre strip stays about the current mode (uncluttered), and
  Favourites is one Alt+F away.
- **Settings file (#46/#48/#1):** one line-format text file,
  `%APPDATA%\CalcDex\settings.txt`, shared by favourites,
  recent, remembered inputs and sig-figs. The file is the shared
  source of truth for GUI and CLI (see the CLI-parity decision
  below); corrupt content falls back to defaults.
- **Defaults policy (#50):** a field's opening default is its
  teaching example unless `meta::DEFAULT_OVERRIDES` says
  otherwise — so defaults and examples can never drift apart, and
  every default is a value the audit has run. Gas-law defaults
  are STP (273.15 K, 1 atm, 22.414 L molar volume); gravity
  stays the named constant g = 9.80665 in the physics formulas
  (it is not an input anywhere).

## 2026-10-25 — Standing rule: tooltips for everything (applied)

Rule (applied from v0.3.0): every button, menu item and
interactive control must have a tooltip, no exceptions.
Applied to CalcDex from v0.3.0: all
buttons and selectable labels are constructed ONLY through the
`tip_btn` / `tip_btn_sized` / `tip_sel` / `tip_sel_value` /
`tip_chip` helpers in main.rs, each of which REQUIRES a tooltip
argument;
text edits and the combo box carry tooltips on their responses;
mode rows and history entries already had them. tests/tooltips.rs
scans src/main.rs and fails if a bare widget constructor appears
outside the helpers, so the rule is enforced by the gate, not by
memory.

## 2026-10-25 — Standing rule: full command-line support (applied)

Rule (applied from v0.3.0): every capability available in the
window must also be usable from the command line. CalcDex's
one-registry design already gave this for
calculations; v0.3.0 closes the state-feature gaps: `--favourite
<id>` / `--unfavourite <id>` / `--favourites` operate on the same
settings file the window uses, CLI runs write remembered inputs
and Recent into that file too, and `--describe` / `--example` /
`--vary` / `--batch` / `--sig-figs` mirror the window's panels.
docs/PARITY.md is maintained as the GUI-vs-CLI parity table;
there are no window-only capabilities — every capability in the
window has a CLI counterpart.

## 2026-10-25 — v0.3.0: on-screen keypad removed

The on-screen keypad (added in v0.2.0) is removed in v0.3.0: it
made the window contents uneven. The keypad UI, its focus
tracking, and the library pad code (`pad_apply` and friends, with
their tests) are deleted — no dead code. The centre column now
spans its full width uniformly and form fields widened (cap
380 → 480 pt). Keyboard flow is unchanged: typing in the fields,
Enter = Calculate, Alt+ mode keys, Ctrl+K search.
