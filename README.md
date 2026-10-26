# CalcDex

## TL;DR

CalcDex is one free program for Windows that does almost every kind of
calculation in one place — like having a big box of hundreds of different
calculators (for money, health, school maths, building jobs, cooking,
sports, and much more) that all work with no internet, no ads, and no
sign-up. Pick a calculator, type your numbers, read the answer with the
working shown in plain words. It also works from the command line for
people who like typing commands.

Free forever. Optional donations are allowed but never required and
never unlock anything. No ads, no in-app purchases, no selling data.

## Who it is for

CalcDex is built for **students, teachers, professors, scientists
and engineers** — people who need a result they can check, not just a
number. That is why every calculator:

- shows its **formula, the substituted values, and the result**
  (worked steps you can follow or mark against),
- keeps **full precision** in results (rounded forms are labelled,
  never silently substituted),
- accepts **scientific notation** inputs (e.g. `6.022e23`, `1.5e-3`),
- names the **constants** it uses (g = 9.80665 m/s², R = 0.082057
  L·atm/(mol·K) = 8.314 J/(mol·K), Avogadro 6.02214076×10²³),
- uses correct **units and symbols**, and labels anything that is an
  approximation as an **ESTIMATE** (body-fat, BAC, race prediction,
  carbon footprint and similar),
- explains input errors in words that say *why* there is no answer
  (for example: log of a negative number has no real value).

**For students & teachers:** results read like worked solutions —
formula first, then the numbers put in, then the answer — so homework
and lab calculations can be verified line by line.

**For professionals:** the same registry runs from the command line,
so any calculation in the window can be scripted, repeated and logged
with identical output.

## What it does

CalcDex brings together the calculator sets of two big websites —
Omni Calculator (3,938 calculators) and Nutilz (637+ tools, calculator
subset only) — as one offline desktop program. Where those sites list
hundreds of near-identical pages (for example a separate page for every
unit pair), CalcDex uses one calculator where you pick the units —
see `docs/COVERAGE.md` for the honest mapping.

Categories: Math, Finance, Health, Physics, Chemistry, Construction,
Conversion, Statistics, Date & Time, Everyday, Sports, Food, Ecology,
Other.

Every calculator shows its formula and the steps in plain words, and
handles bad input (dividing by zero, square root of a negative, and so
on) with a clear message instead of crashing.

## The look

Dark "Terminal" theme, exactly SEAN's Purgeon Terminal colours:
near-black background `#0c0c0c`, panels `#121212`, phosphor-green
accent `#4ade80`, Cascadia Mono type, sharp corners everywhere.

## How to use it (window)

Since v0.2.0 the window uses the **Design 3** layout SEAN picked
from the eight research mockups (mode rail / centre work area /
history rail — the pattern Microsoft's Calculator app uses):

1. Run `CalcDex.exe`.
2. **Mode rail (far left):** the categories are *modes* — ALL,
   MATH, FINANCE, HEALTH, and so on. Click a mode to see its
   calculators, or hold **Alt** and press the key printed on the
   mode's row. The full working shortcut set is Alt+1 … Alt+9,
   Alt+0 for the first ten modes, then Alt+Q, Alt+W, Alt+E,
   Alt+R, Alt+T for the last five, in rail order. The **Search**
   box at the top of the rail filters every calculator by name.
3. **Centre:** click a calculator in the chip strip under the mode
   heading, then fill in the labelled boxes (list calculators take
   numbers separated by commas or spaces in one big box). Dates go
   in ONE box per date, in the format the hint under the box
   shows — MM/DD/YYYY, so 04/23/1988 means April 23, 1988. Press
   the green **Today?** button beside any date box to fill in
   today's date (the Age calculator's End Date and the Countdown's
   From Date already start filled with today).
4. Press **CALCULATE** (or Enter). The
   result appears in the **Display** at the top of the centre in
   large green type, with the full formula and worked steps in the
   footer block underneath — nothing is cut down to one line.
5. **History rail (right):** every result is saved there at once
   (see Calculation log below). RE-OPEN on an entry restores its
   calculator and inputs. On a narrow window the rail collapses —
   below 1100 points wide the header **LOG** button shows the same
   history in the centre instead.

## How to use it (command line)

Every calculator works the same from the command line — nothing is
window-only:

```
CalcDex --list
CalcDex --search loan
CalcDex compound-interest 1000 5 10
CalcDex bmi 70 175
CalcDex mean 1 2 3 4
CalcDex convert-length 1 7 3
CalcDex age 04/23/1988 10/25/2026
```

Dates on the command line are one argument in MM/DD/YYYY form, just
like the window (the old six-number form `1988 4 23 2026 10 25`
still works and gives the identical answer).

More command-line tools (v0.3.0 — everything in the window has a
command-line counterpart; see `docs/PARITY.md` for the full table):

```
CalcDex --describe bmi
CalcDex bmi --example
CalcDex compound-interest --vary 1 0 10 2.5 1000 10
CalcDex --batch compound-interest in.csv out.csv
CalcDex --sig-figs 3 compound-interest 1000 5 10
CalcDex --favourite bmi
CalcDex --favourites
```

Numbers go in the same order as the labelled boxes in the window.
For unit converters the two extra numbers pick the units from the
numbered list shown in that calculator's description.
`CalcDex --help` explains the rest.

## New in v0.3.0 — Batch 1 of the 50-enhancement program

Ten enhancements (program items 1, 11, 16, 20, 31, 34, 46, 47, 48,
50), every one of them in the window AND on the command line:

- **Significant-figures mode (1).** The header's SIG FIGS selector
  (Off / 2 / 3 / 4 / 5 / 6) adds a rounded showing of the result
  headline next to the full-precision answer — counted correctly
  (0.00456 to 3 figures stays 0.00456; 1628.89 becomes 1.63e3, so
  the count is never ambiguous). Display only: the stored result
  and the log always keep full precision. CLI: `--sig-figs 3`.
- **Try an example (11).** Every calculator has a TRY AN EXAMPLE
  button that fills in a worked teaching example and runs it at
  once. CLI: `CalcDex bmi --example`.
- **Vary one input (16).** The VARY section under the form sweeps
  one input across start/end/step and tables the results (200-row
  cap, stated in the panel). CLI:
  `CalcDex compound-interest --vary 1 0 10 2.5 1000 10` (the field
  is numbered from 0, as in `--describe`; the remaining numbers are
  the other inputs in order).
- **Hints under every input (20).** Every field shows what format
  and unit it wants — and what the codes mean where a field takes
  codes (sex codes, unit numbers, activity factors). CLI:
  `CalcDex --describe bmi` prints them too.
- **Batch from CSV (31).** The header BATCH panel (or
  `CalcDex --batch <id> <input.csv> <output.csv>`) runs one
  calculator over a whole file. The header row names the fields in
  order — for Compound Interest that is:

  ```
  Principal P,Annual rate %,Years
  1000,5,10
  2000,10,5
  ```

  Each output row is the inputs plus the result headline, or
  `ERROR: ...` — one bad row never stops the batch. (In the window
  the file paths are typed: this toolkit has no file-picker
  dialog.) For list calculators the header is the single list
  label and each row's cells are the list values.
- **Linear Regression & Calibration (34).** A new Statistics
  calculator: enter (x, y) pairs and get slope, intercept, r, R²,
  means, residuals, and the working (n, Σx, Σy, Σxy, Σx²). Add one
  extra value at the end — a measured y — and it predicts the x
  that produced it, x = (y − b) / m. Registry now 300 calculators.
- **Favourites + Recent (46).** The ★ FAVOURITE button on any
  calculator stars it into the FAVOURITES mode at the top of the
  rail (Alt+F), which also shows your last 8 calculators as Recent
  chips. CLI: `--favourite bmi`, `--unfavourite bmi`,
  `--favourites`.
- **Ctrl+K quick search with synonyms (47).** Ctrl+K (or `/`)
  jumps to search from anywhere; search understands everyday words
  — "home loan" finds the mortgage calculator, "bmi" finds Body
  Mass Index — and ranks exact names first. 40+ synonym pairs.
- **Remembered inputs (48).** Reopening a calculator restores the
  values you last used in it; RESET puts the smart defaults back.
- **Smart defaults (50).** Every field opens on a realistic,
  visible, editable default (the teaching examples double as the
  defaults): today in date fields, standard temperature/pressure
  in the gas laws (273.15 K, 1 atm, 22.414 L), g = 9.80665 m/s²
  inside the physics formulas, sane 0%/100% bounds elsewhere. The
  notable ones are listed in `docs/AUDIT.md`.

**Tooltips:** every button, toggle, field and key in the window
has a plain-words tooltip saying what it does — no exceptions
(enforced by
construction in the code and checked by a test).

**Settings file:** favourites, recent, remembered inputs and the
sig-figs choice live in `%APPDATA%\CalcDex\settings.txt` (plain
text, format documented in the file's code). The window and the
command line read and write the SAME file — it is the shared
source of truth, so a calculator you star or run on the command
line is starred and remembered in the window too.

## Calculation log

Every successful calculation is saved automatically — whether you
ran it in the window or on the command line — so classwork, lab
results and estimates (SEAN's original request was for age
calculations) can be found again later:

- The saved entries live in the **History rail** docked on the
  right of the window, newest first (header **LOG** shows/hides
  it; on narrow windows the same history opens in the centre).
  **RE-OPEN** on an entry loads that calculator with its inputs
  restored; **CLEAR LOG** empties the log.
- Or from the command line: `CalcDex --log` prints the log and
  `CalcDex --clear-log` empties it.
- Entries are kept in `%APPDATA%\CalcDex\history.txt` (plain text:
  time, calculator, inputs — dates in MM/DD/YYYY form — and result).
  Only the newest 500 entries are kept; log timestamps are UTC.
  Failed calculations are never logged.

## Requirements & building

- Windows 10/11 for the shipped program (it is a standard Windows
  .exe; it also builds and runs on Linux for development).
- To build from source: a recent Rust toolchain, then:
  `cargo build --release` — and the full check is
  `cargo fmt + cargo clippy --all-targets + cargo test --locked`.

## Install

Releases carry exactly two files, as always:

1. `CalcDex-Setup-0.3.0.exe` — the installer (per-user, no admin).
2. `CalcDex-0.3.0-portable.zip` — the portable build: just the .exe
   plus the license and a short note. Unzip and run.

The program is unsigned (no code-signing certificate) — Windows may
warn on first run; choose "More info", then "Run anyway", if you trust
this build. Check the SHA-256 in the release notes to be sure.

## Screenshots

Real captures from SEAN's machine, taken during the v0.1.1 field
test (2026-10-25). They show the OLD left column — v0.1.2 makes the
lists brighter, larger, and single-line; a fresh main-window capture
after the readability fix is still needed.

![Age Calculator in v0.1.1 — merged Birth Date and End Date fields with format hints and Today? buttons](docs/screenshots/age-calculator.png)

*Age Calculator (v0.1.1 capture): one MM/DD/YYYY box per date, the
format hint under each field, and a Today? button beside each date.
The result shown — 06/04/1977 to 10/26/2026 = 49 years / 18,041
days / 2,577 weeks — was independently re-checked and is exactly
right.*

![Calculation Log in v0.1.1](docs/screenshots/calculation-log.png)

*Calculation Log (v0.1.1 capture): every calculation is saved
automatically; RE-OPEN restores an entry's calculator and inputs,
CLEAR LOG empties the log. Timestamps are UTC.*

**Captures still needed (real ones only, no placeholders):** a fresh
main-window shot of **v0.3.0** showing the Design 3 layout with the
Batch 1 features (mode rail, display, history rail, hints
under the fields), ideally one at full width and one narrow enough
that the history rail has collapsed.

## Limitations (honest)

- Formula calculators only: the very specialised Omni pages that need
  live data (currency exchange rates, weather, stock prices), big
  lookup tables (tax brackets by country/year), medical diagnosis, or
  drawing graphs are not in v0.1 — they are listed in
  `docs/ROADMAP.md`, not silently dropped.
- Health and building results are estimates from the standard
  published formulas named in each calculator. They are not medical
  or engineering advice.
- See `docs/PARITY.md` for category-by-category scoring from test
  evidence.

## License

MIT — see `LICENSE`. Free software; use it, share it, change it.
