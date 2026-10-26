# Changelog — CalcDex

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
