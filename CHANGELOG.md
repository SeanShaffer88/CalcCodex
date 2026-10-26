# Changelog — CalcCodex

## 0.1.0 — 2026-10-25

First release. Calculators only (SEAN's scope pick), covering the
Omni Calculator categories and the Nutilz calculator subset as one
offline Windows program in the dark Terminal theme.

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
