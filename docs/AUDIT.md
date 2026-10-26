# CalcDex v0.1.2 — Full Audit

Date: 2026-10-25. Scope: SEAN's three directions from the v0.1.1 field
test — (1) left columns hard to read, (2) window contents must adjust
when resized, (3) check all calculations for usability, accuracy,
context, and accessibility.

Every calculator in the registry (299) was read against the standard
published formula for its name, its labels, its description, and its
edge-case behaviour. The test corpus was expanded from 67 calculators
with a known-answer vector to **all 299**.

## Formula bugs found and fixed

Each bug is pinned by a regression test in `tests/audit_*.rs` that
fails on the old behaviour and passes now.

| # | Calculator | Bug | Before | Correct now |
|---|---|---|---|---|
| 1 | Fraction Multiply (`fraction-multiply`) | The output printed the simplified **numerator** as a bogus "= value" | 2/3 × 3/4 showed `= 1` alongside the right decimal | Prints `Result: 1/2 (decimal 0.5)` |
| 2 | Average Velocity (`average-velocity`) | The divide-by-zero guard tested `t2 − x2` instead of `t2 − t1` (wrong input index) | Equal times (t1 = t2) slipped through and printed an **infinite velocity**; equal x2/t2 values triggered a **false error** on a valid problem | Guard tests `t2 − t1`; equal times give a teaching error |
| 3 | Ideal Weight, Devine (`ideal-weight-devine`) | Inches over 5 ft were clamped at 0, but the Devine formula is signed | Anyone under 5 ft silently got the 5 ft base weight (a 4′10″ man got 50.0 kg) | 4′10″ man now gets 45.4 kg (50 − 2.3 × 2) |
| 4 | Ratio (`ratio`) | Parts were cast to whole numbers before the GCD, truncating decimals | 1.5 : 2.5 "simplified" to 1 : 2 | Parts are scaled to whole numbers first: 1.5 : 2.5 = 3 : 5 |
| 5 | Fraction to Decimal (`fraction-to-decimal`) | A negative whole part added the fraction instead of subtracting it | −1 and 1/2 gave −0.5 | −1 and 1/2 gives −1.5 |
| 6 | Nth Root (`nth-root`) | Odd roots of negative numbers fell through to `powf`, which returns NaN | Cube root of −27 reported "too big or not a number" | Cube root of −27 = −3; even/non-whole roots of negatives give a teaching error |
| 7 | Double Discount (`double-discount`) | A starting price of 0 divided by the price | `Equivalent single discount: NaN%` | Explains there is no equivalent discount to compute at price 0 |
| 8 | Effective Rate (`effective-rate`) | Compounds-per-year n = 0 was not checked | NaN / infinity output | Teaching error: n must be at least 1 |
| 9 | Rule of 72 (`rule-of-72`) | A negative rate produced a negative "exact doubling time" | Rate −5% printed a negative doubling time | Teaching error: at 0% or less, money never doubles |
| 10 | Sample Size, Proportion (`sample-size-proportion`) | p = 0 or p = 1 was accepted | Answered "0 people needed" | Teaching error: p must be strictly between 0 and 1 (use 0.5 if unsure) |
| 11 | Dog Age (`dog-age`) | The size code was not validated; anything except 3 silently meant +5/yr | Size code 9 gave a confident answer | Teaching error: code must be 1, 2 or 3 |
| 12 | All unit converters (`convert-*`, `convert-temperature`) | Fractional unit numbers were silently truncated | Unit "1.9" quietly converted as unit 1 | Teaching error: unit numbers must be whole numbers from the listed table |
| 13 | Proportion (`proportion`) | The second output line was self-contradictory ("A/B = C/x form with C=A: x = B") — wording, not arithmetic | Confusing, wrong-looking second answer | Both standard forms are computed and labelled: form 1 `A/B = x/C → x = A·C/B`, form 2 `A/B = C/x → x = B·C/A` |
| 14 | Fraction results (multiply / divide / add / subtract / simplify) | A negative denominator kept its sign in printed results | Results like `3/-2` | Sign normalized: `−3/2` |

Also fixed as context/wording (no arithmetic change): the ideal-gas
descriptions repeated the gas constant twice ("(= 8.314 J/(mol K))
(= 8.314 J/(mol K) in SI)"); the Power (physics) description mentioned
force × velocity in a way that implied it was an input; the Decimal →
Fraction description now states the 9-decimal-place precision limit.

## Usability / context fixes by category

- **Math:** proportion rewritten (bug 13); decimal-to-fraction
  precision limit documented.
- **Finance:** three silent-garbage edge cases now teach (bugs 7–9).
- **Health:** Karvonen's first input is now "Age (years)"; Devine
  description states the signed per-inch rule.
- **Physics:** Speed / Distance / Time calculators now say which
  units match ("metres & seconds gives m/s") in the description and
  on the input labels; Average Velocity labels name the unit
  relationship; Power description corrected.
- **Chemistry:** gas-constant duplication removed (×3).
- **Statistics:** sample-size proportion validates p (bug 10).
- **Everyday / Date & Time:** dog-age size code validated (bug 11);
  cat-age rejects non-positive ages like its neighbours.
- **Conversion:** whole-number unit validation everywhere (bug 12).
  All converter unit tables were re-read number by number: complete
  and correctly numbered in all 18 tables.
- **All categories:** every input label was checked for a stated or
  implied unit; results are spelled out in words (no result carries
  meaning by colour alone — errors are also prefixed in text, and the
  GUI's red error colour is a second signal, not the only one).
  Estimates still carry the ESTIMATE label; the health group's
  "not medical advice" wording stays where it already was — no
  per-output advice spam was added.

## Test corpus

Expected values were computed **independently** of the program
(Python/hand arithmetic from the published formulas; constants
g = 9.80665 m/s², R = 0.082057 L·atm/(mol·K), 1 hp = 745.699872 W,
1 acre = 43,560 sq ft, etc.). The program's own output was used only
to check display *formatting* (e.g. how many decimals a value prints
with), never to derive an expected value; where a first expectation
disagreed with the program, the case was re-derived by hand — two
were my own expectation errors (temperature unit codes are
0 = C, 1 = F, 2 = K; the stair calculator rounds step count to
nearest, a standard convention), the rest were the bugs above.

| Category | Calculators | With vector at v0.1.1 | With vector now |
|---|---|---|---|
| Math | 61 | 13 | 61 |
| Finance | 34 | 9 | 34 |
| Physics | 31 | 6 | 31 |
| Conversion | 31 | 5 | 31 |
| Construction | 26 | 5 | 26 |
| Health | 24 | 5 | 24 |
| Chemistry | 23 | 5 | 23 |
| Statistics | 21 | 8 | 21 |
| Everyday | 15 | 1 | 15 |
| Sports | 7 | 1 | 7 |
| Food | 7 | 2 | 7 |
| Ecology | 7 | 1 | 7 |
| Date & Time | 7 | 6 | 7 |
| Other | 5 | 0 | 5 |
| **Total** | **299** | **67** | **299** |

Full gate at v0.1.2: 53 test groups, all passing (19 pre-existing +
34 new audit groups). See QUALITY.md for the gate transcript.

## Workstream 1 — left-column readability

What changed in `src/main.rs`:

- List rows are now drawn by a dedicated `list_row` renderer: text in
  the main text colour **#d0d0d0 at 13 pt** (was egui's default
  selectable-label rendering, which SEAN's screenshots showed as dim
  grey and cramped), one row every 26 pt with consistent spacing.
- Category headers (SEARCH / CATEGORY / CALCULATORS) are now green,
  bold, 12 pt (were muted grey 11 pt — 4.36:1, below AA).
- Selected row: green-selection background **#163322** + green text +
  a **solid 3 pt marker bar on the left edge** — selection is visible
  by position and shape, not colour alone.
- Hover background is #262626 on the lists and #2a2a2a for widgets
  (v0.1.1 used #1a1a1a, barely distinguishable from the panel).
  Pressed widgets get the green-dim fill with green text.
- Long names stay on **one line**, truncated with an ellipsis, with
  the full name in a hover tooltip; the full name is always shown in
  the detail-pane header.
- Muted grey (#7a7a7a) is now used only for egui "weak" text —
  field hint/placeholder text — never for list content or labels.
  Input labels in the detail pane moved from muted 11 pt uppercase to
  main-colour 12 pt.

Measured WCAG contrast ratios (relative-luminance formula, computed
from the hex values):

| Pairing | Ratio | AA normal text (≥ 4.5:1) |
|---|---|---|
| List text #d0d0d0 on panel #121212 | **12.15 : 1** | Pass (also AAA) |
| List text #d0d0d0 on window #0c0c0c | **12.68 : 1** | Pass |
| List text #d0d0d0 on hover #262626 | **10.19 : 1** | Pass |
| Selected text #4ade80 on selection #163322 | **7.87 : 1** | Pass |
| Header green #4ade80 on panel #121212 | **10.75 : 1** | Pass |
| Old muted #7a7a7a on panel #121212 (v0.1.1 labels) | 4.36 : 1 | **Failed** — reason for the change |
| Error red #f26d6d on panel #121212 | 6.41 : 1 | Pass |

## Workstream 2 — responsive layout

Layout mechanisms (all in `src/main.rs`), and what each guarantees:

- **User-resizable left panel:** `SidePanel::left` with
  `.resizable(true)`, default width 250 pt, `.width_range(190..=400)`.
  The user can drag it; it can never crush the detail pane below
  190 pt of list or swallow the window beyond 400 pt.
- **Fluid detail pane:** the entire detail pane (header, description,
  inputs, Calculate, result) lives in a vertical `ScrollArea`. Below
  the minimum window size — or at high zoom — content scrolls into
  reach instead of clipping or overlapping.
- **Relative input rows:** label column = 38% of the pane's available
  width, clamped 130–300 pt; field width = the remaining width,
  clamped 150–380 pt; both recomputed every frame from
  `ui.available_width()`. Rows use wrapping layout
  (`horizontal_wrapped`): if a row cannot fit, the field wraps under
  the label instead of running off the right edge. Wide windows do
  not stretch fields absurdly (380 pt cap). Date rows behave the
  same, with the Today? button wrapping with the field.
- **Search box** takes the panel width left over after the Clear
  button (min 110 pt) — no fixed 150 pt width to clip.
- **Minimum window size:** 760 × 520 pt (was 880 × 580), set with
  `with_min_inner_size`. At that size the left panel sits at its
  minimum, the detail pane keeps priority, and everything remains
  reachable by scrolling.
- **Text scaling:** all sizes are egui points and the base text style
  was raised to 13 pt; at 150%/200% zoom (Ctrl + / −) every width
  above is measured in the same scaled points, so rows reflow and
  the scroll area absorbs the growth — no hardcoded pixel widths
  remain in the layout path except the clamps above, which are
  themselves in scaled points.
- **List text** truncates with an ellipsis (never wraps mid-word);
  **detail text** (description, results) wraps to pane width.

Verification level: the layout is guaranteed by construction and
code review; this machine cannot run the Windows GUI, so on-Windows
resize/zoom behaviour is **Partially Verified** until SEAN checks it
on his PC (v0.1.1 itself is field-verified — see CHANGELOG).

## README screenshots

SEAN's two real v0.1.1 captures are now in `docs/screenshots/`
(`age-calculator.png`, `calculation-log.png`), byte-identical to his
originals (SHA-256 verified: d9d4e2d1… and 357e4d2d…). They show the
OLD left column, and the README captions say so honestly; the
capture-needed list now asks for a fresh main-window shot after this
readability fix.

## Remaining gaps (honest)

- On-Windows resize / 150–200% zoom behaviour: Partially Verified
  (construction + code review only) until SEAN's check.
- Combinations/permutations and factorial print very large results
  in decimal notation (inherent to f64); exact big-integer output is
  a roadmap item, not an audit fix.
- Kitchen gram-per-cup weights are published-table estimates and
  vary a little between sources (e.g. cocoa 100 g/cup); the values
  used are the standard King Arthur / USDA-style table values.
- BMI imperial uses the standard published 703 factor (the exact
  constant is 703.0696); 703 is what textbooks print.
- The audit re-derived formulas from the calculator names and
  standard references; calculators whose "standard" is a named
  estimate (Riegel, Cooper VO2max, Navy body fat) are labelled
  ESTIMATE in the program and were checked against their named
  published formulas.
