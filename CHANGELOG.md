# Changelog — CalcDex

## 0.3.0 — 2026-10-25

**Batch 1 of the 50-enhancement program — items 1, 11, 16, 20,
31, 34, 46, 47, 48, 50: DONE.** (Program tracker: Batch 1 complete;
next is the rigour batch, items 2–10 + 37.) Also the first release
This release
also applies two standing rules in full — **tooltips for
everything** and **full command-line support** — both recorded in
DECISIONS.md.

- **#1 Significant-figures mode.** Header SIG FIGS selector
  (Off/2/3/4/5/6) and CLI `--sig-figs N`: the headline is ALSO
  shown rounded to N significant figures (pure `format_sig_figs`
  in the library, classic traps tested: 0, negatives, powers of
  ten, 999.9, 0.0000999). Display only — full precision is kept
  everywhere, including the log.
- **#11 Try an example.** Every one of the 300 calculators carries
  hand-written teaching example inputs (registry-wide test: each
  has one, and it runs). TRY AN EXAMPLE button fills + runs in one
  click; `CalcDex <id> --example` does the same on the CLI.
- **#16 Vary-one-input table.** GUI VARY section + CLI
  `<id> --vary <field> <start> <end> <step> <other numbers...>`
  (CSV-friendly output). Pure sweep engine in the library; 200-row
  cap with the cap stated; list calculators and bad sweeps get
  teaching messages.
- **#20 Hints under every input.** Format + unit + code meanings
  (hand-written where the label is not enough — sex/size/unit
  codes, activity factors); hints live in the registry metadata,
  so `--describe <id>` prints them too.
- **#31 Batch from CSV.** GUI BATCH panel (typed paths — no
  file-picker in this toolkit, stated honestly) + CLI `--batch`.
  Hand-written CSV parser (quoted commas supported; no new
  dependency). Bad rows are captured per-row; a header mismatch
  is refused with the expected header named.
- **#34 Linear Regression & Calibration.** New Statistics
  calculator (registry 299 → 300): slope, intercept, r, R², means,
  residuals, substituted sums in the steps, and calibration —
  append one measured y and it predicts x = (y − b)/m. Known
  answers from a textbook dataset (m = 0.6, b = 2.2, R² = 0.6).
- **#46 Favourites + Recent.** ★ toggle per calculator, FAVOURITES
  mode at the top of the rail (Alt+F) with the last 8 run as
  Recent chips.
- **#47 Ctrl+K quick search with synonyms.** Ctrl+K (or `/`)
  focuses search; 80+-entry synonym table (40+ pairs) with
  ranking exact name > synonym > description.
- **#48 Remembered inputs.** Reopening a calculator restores its
  last-used values; RESET restores the smart defaults.
- **#50 Smart defaults everywhere.** Fields open on visible,
  editable defaults (teaching examples; gas laws at STP —
  273.15 K / 1 atm / 22.414 L; dates default to today where they
  did before). AUDIT.md addendum lists the notable ones.
- **Settings file** (shared by #1/#46/#48):
  `%APPDATA%\CalcDex\settings.txt`, a documented line format.
  A corrupt or missing file can never break startup.
- **Tooltips for everything (standing rule).** Every button,
  toggle, field, chip and history control carries a
  plain-words tooltip. All buttons/selectables are built ONLY
  through helper constructors that require a tooltip argument,
  and tests/tooltips.rs scans the GUI source to keep it so.
- **Full command-line support (standing rule).** CLI gains
  `--describe`, `--example`, `--vary`, `--batch`, `--sig-figs`,
  `--favourite` / `--unfavourite` / `--favourites`. CLI runs now
  also write remembered inputs + recent to the shared settings
  file. `docs/PARITY.md` is now a full GUI-vs-CLI parity table.
  One CLI behaviour change: `--search` is now the ranked synonym
  search (same engine as the window), so result order/content for
  a query follows the ranking instead of registry order.
- All v0.2.0 behaviours carry over (Design 3 layout, history
  rail, 299 known-answer vectors + the new regression vectors),
  except the keypad (next item).
- **Removed: the on-screen keypad.** It made the window contents
  uneven. The keypad UI, its focus tracking and its library code
  are gone; the centre column now spans its full width evenly
  (form fields widened). Keyboard flow is unchanged: type in the
  fields, Enter calculates, Alt+ switches modes, Ctrl+K searches. The new GUI panels are **Unverified** on a real
  Windows machine until a field run — fresh v0.3.0 captures are on
  the README list.


## 0.2.0 — 2026-10-25

The **Design 3** release — the layout SEAN picked from the eight
research mockups (Microsoft-Calculator-inspired), replacing the
v0.1.x two-column window. No calculator changed: the registry, the
CLI and every result are exactly as audited at 0.1.2.

- **Mode rail (far left).** The categories are modes — ALL, MATH,
  FINANCE, HEALTH, PHYSICS, CHEM, STATS, CONVERT, BUILD, DATE,
  DAILY, SPORTS, FOOD, ECOLOGY, OTHER — one click each, with the
  v0.1.2 readability styling (bright 13 pt text, green selection
  background + text + marker bar, ellipsis + full-name tooltip).
  Each row prints its Alt+ shortcut; the working set is Alt+1..9,
  Alt+0 for the first ten modes and Alt+Q/W/E/R/T for the last
  five, in rail order. Search stays at the top of the rail.
- **Centre work area.** A chip strip lists the active mode's
  calculators (search still filters across all); below it the
  selected calculator's form is unchanged — labelled inputs with
  units, merged MM/DD/YYYY date fields with format hints and
  Today? buttons, list-input format, teaching errors, pre-filled
  defaults (Age End Date / Countdown From Date = today).
- **Display + steps footer.** Results land in a prominent Display
  (large green type) at the top of the centre; the full result
  text — formula, substituted values, result — sits in the
  STEPS / FULL RESULT footer underneath, complete and scrollable.
- **On-screen keypad** (removed in v0.3.0 — it made the window
  contents uneven).
- **History rail (right).** The calculation log is docked as a
  right-hand rail — newest first, RE-OPEN restores calculator +
  inputs, CLEAR LOG, count in the header. Below a 1100 pt window
  width it collapses and the header LOG button opens the same
  history in the centre (the v0.1.x behaviour). LOG now starts
  shown, matching the mockup.
- Responsive rules kept: resizable rails, fluid detail rows,
  vertical scrolling, the 760 × 520 minimum and zoom reflow from
  0.1.2. The new layout's on-Windows look is **Unverified** until
  SEAN's run — fresh v0.2.0 captures are on the README list.
- Deviations from the mockup are recorded in DECISIONS.md (the
  mockup's consumer mode names and memory row do not exist in
  CalcDex; × ÷ operator keys are omitted on purpose).

## 0.1.2 — 2026-10-25

The readability + responsive + full-audit release, from SEAN's field
test of 0.1.1. (0.1.1 itself is FIELD-VERIFIED: SEAN's screenshots
show the merged date fields, format hints, Today? buttons, the
pre-filled End Date, the calculation log view with its LOG (1)
count, RE-OPEN and CLEAR LOG, and Icon 1 in the title bar; his Age
run 06/04/1977 → 10/26/2026 = 49 years / 18,041 days / 2,577 weeks
was independently re-checked and is exactly correct. His two
captures are now in `docs/screenshots/`.)

- **Left column readability:** list text is now the main text
  colour at 13 pt (measured 12.15:1 contrast on the panel — WCAG AA
  asks 4.5:1; the old muted-grey labels measured 4.36:1), category
  headers are green and bold, rows are taller, the selected row has
  a green background, green text AND a marker bar on its left edge
  (selection is never colour-alone), hover is clearly visible, and
  long names stay on one line with an ellipsis + full-name tooltip
  instead of wrapping mid-name.
- **Responsive window:** the left panel is user-resizable
  (190–400 pt); the detail pane scrolls vertically and its input
  rows reflow from the available width (labels 38%, fields take the
  rest, capped) so nothing clips off the right edge at the new
  760 × 520 minimum size or at 150–200% text zoom; the search box
  stretches with the panel. On-Windows resize behaviour is
  Partially Verified (by construction + code review) until SEAN's
  check — see `docs/AUDIT.md` for the layout mechanisms.
- **Full audit of all 299 calculators** (usability, accuracy,
  context, accessibility) — full report in `docs/AUDIT.md`. Known-
  answer test coverage went from 67 of 299 calculators to **299 of
  299** (expected values computed independently). Formula bugs
  fixed, each pinned by a regression test:
  - Fraction Multiply printed the numerator as a bogus "= value"
    (2/3 × 3/4 showed "= 1"; now "Result: 1/2 (decimal 0.5)").
  - Average Velocity's divide-by-zero guard tested the wrong
    inputs: equal times printed an infinite velocity, and some
    valid problems got a false error. Guard now tests t2 − t1.
  - Ideal Weight (Devine) clamped heights under 5 ft to the 5 ft
    weight (a 4′10″ man got 50.0 kg; the signed formula gives
    45.4 kg).
  - Ratio truncated decimal parts before simplifying (1.5 : 2.5
    became 1 : 2; now correctly 3 : 5).
  - Fraction to Decimal mishandled negative mixed numbers
    (−1 and 1/2 gave −0.5; now −1.5).
  - Nth Root rejected odd roots of negatives (cube root of −27
    now = −3, with a teaching error for even/non-whole roots).
  - Silent-garbage edge cases now teach instead: Double Discount
    at price 0 (was "NaN%"), Effective Rate at n = 0, Rule of 72
    at a negative rate, Sample Size at p = 0 or 1 (was "0 people
    needed"), Dog Age with an invalid size code, and fractional
    converter unit numbers (1.9 was silently read as unit 1).
  - Proportion's second output line was self-contradictory; both
    standard forms are now computed and labelled. Fraction results
    no longer print negative denominators (3/-2 → −3/2).
  - Wording/labels: Speed, Distance and Time calculators state
    their unit assumptions on the labels; Karvonen's "Age" is now
    "Age (years)"; the ideal-gas descriptions no longer repeat the
    gas constant twice; Decimal → Fraction documents its
    9-decimal-place precision limit.

## 0.1.1 — 2026-10-25

Renamed CalcCodex → **CalcDex** (program, project, and repo) and the
"smarter dates + calculation log" release, from SEAN's field test of
0.1.0 (main window and Age Calculator field-verified on his machine;
the Age result 04/23/1988 → 10/25/2026 = 38 years / 14,064 days was
independently re-checked and was exactly right).

- **Merged date fields:** every date calculator now takes one box per
  date in MM/DD/YYYY form instead of separate year/month/day boxes,
  with a plain-words format hint under the field ("for example
  04/23/1988 means April 23, 1988") and teaching errors for dates
  that are not real (Feb 29 2025 fails, Feb 29 2024 works).
- **Clear date labels:** Birth Date / End Date (the old "On year /
  On month / On day" wording is gone everywhere, including results),
  Start Date, From Date, Target Date, Date, and First Day of Last
  Period for the due-date and ovulation calculators.
- **Today? button** beside every date field fills it with today's
  date. The Age calculator's End Date and the Countdown's From Date
  start pre-filled with today.
- **Calculation log:** every successful calculation — in the window
  or on the command line — is saved automatically to
  %APPDATA%\CalcDex\history.txt (newest 500 kept). The LOG button
  in the header opens it (newest first, count shown); RE-OPEN loads
  an entry's calculator with its inputs restored; CLEAR LOG empties
  it. Command line: `CalcDex --log` and `CalcDex --clear-log`.
- **Icon (SEAN's pick, option 1):** phosphor-green calculator
  outline on near-black, redrawn cleanly from the concept sheet
  (its bottom-row overlap glitch fixed), wired into the window,
  the .exe itself, and the installer.
- **CLI dates:** a date is one argument — `CalcDex age 04/23/1988
  10/25/2026` gives exactly the same answer as the old six-number
  form, which still works.
- Field wording in the registry now uses "end date" throughout.


## 0.1.0 — 2026-10-25

First release. Calculators only (SEAN's scope pick), covering the
Omni Calculator categories and the Nutilz calculator subset as one
offline Windows program in the dark Terminal theme.
(0.1.0 was built and field-tested locally under the name CalcCodex;
the program, project, and repo were renamed to CalcDex before any
GitHub publication, so the first published release is 0.1.1.)

- One registry of calculators (exact count in docs/COVERAGE.md and
  proven by a registry test), used identically by the window and the
  command line — no window-only features.
- Categories: Math, Finance, Health, Physics, Chemistry,
  Construction, Conversion, Statistics, Date & Time, Everyday,
  Sports, Food, Ecology, Other.
- Every result shows the standard formula and the steps in plain
  words; estimates are labelled ESTIMATE; bad input gives a clear
  error, never a crash.
- Search with Clear button, category list with counts, labelled
  inputs, CALCULATE, result panel.
- Known-answer tests across every category (tests/known_answers.rs).
- Release shape (SEAN's rule): loose NSIS installer .exe + one
  portable .zip (exe + LICENSE + PORTABLE.txt). Unsigned; no winget.
