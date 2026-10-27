# Coverage — CalcDex v0.1 vs the two source sites

Counts in this file are the **registered calculator count proven by
the registry test** (`tests/known_answers.rs::registry_is_big_and_unique`
and `cargo run -- --list | tail`), not page counts from the websites.
`REGISTRY_COUNT` and the per-category table are filled from the built
program's own `--list` output at release time (see QUALITY.md for the
gate transcript).

## Sources

- **Omni Calculator** — homepage states "Your life in 3938 free
  calculators", in 13 categories: Biology (not shown on every listing),
  Chemistry, Construction, Conversion, Ecology, Everyday life, Finance,
  Food, Health, Math, Physics, Sports, Statistics, Other.
- **Nutilz** — "637+ tools". SEAN's scope pick was calculators only,
  so the covered subset is Nutilz's calculator pages (construction
  calculators like concrete/paint/tile/mulch/gravel/flooring/fence/
  stair, plus percent-error, speed-distance-time, multiplication,
  loan/BMI-type calculators). Nutilz's PDF/image/text/dev/design/
  security/wellness *tools* are out of scope by SEAN's decision.

## How the mapping works (parametric collapsing)

Omni inflates its count with one page per *pair* or per *item*:

- ~Hundreds of unit pages ("inches to cm", "cm to inches", "kg to lb",
  ...) → CalcDex registers **one converter per dimension** (length,
  mass, area, volume, temperature, speed, time, data, pressure,
  energy, power, force, torque, density, frequency, flow,
  illuminance, fuel economy) with the unit pair as numbered inputs,
  the table printed in the calculator's description. Every Omni pair
  in those dimensions is computable.
- "Grams to cups" per ingredient → one calculator per ingredient
  (12: flour, sugars, butter, cocoa, rice, oats, honey, milk, water,
  oil...) with the published grams-per-cup density.
- Percentage family: Omni lists percentage, percentage increase,
  percentage decrease, percentage change, percentage difference as
  separate pages → CalcDex registers each distinct operation
  (percent-of, percent-is, percent-change, percent-difference,
  reverse-percent, percent-off-result).
- Solve-for variants: Omni has separate pages to solve speed,
  distance or time → CalcDex registers each as its own calculator
  (same for Ohm's law V/I/R, ideal gas P/V/n/T, dilution M2/V1).

So the honest comparison is *operations covered*, not page titles.

## Per-category coverage (v0.1)

| Omni/Nutilz category | CalcDex category | Status |
|---|---|---|
| Math | Math | Broad: percentages, fractions, powers/roots/logs, number theory, algebra, geometry areas/volumes, trig |
| Finance | Finance | Broad: interest, loans, mortgage, tax/VAT, margin/markup, salary, break-even, payoff, depreciation |
| Health | Health | Broad: BMI, BMR (4 variants), TDEE, body fat (Navy), heart-rate, pace, due date, ovulation, BAC, macros — all labelled estimates |
| Physics | Physics | Broad: mechanics, energy, electricity (Ohm's law set), waves, projectile, pendulum |
| Chemistry | Chemistry | Core: moles, molarity, dilution, pH/pOH, gas laws (5), composition, yield, half-life |
| Construction | Construction | Broad (Nutilz-heavy): concrete x3, paint, tile, mulch, gravel, flooring, fence, stairs, roofing, drywall, board feet, deck, brick, block, post holes |
| Conversion | Conversion | 18 dimension converters + temperature + fuel economy + 12 ingredient converters + shoe sizes |
| Statistics | Statistics | Core: list stats (mean/median/mode/SD/geometric/harmonic), z, p-value (z), probability, combinations/permutations, CI, MoE, sample size, EV, weighted average, percentiles |
| Everyday life | Everyday + Date & Time | Dates (between/plus/age/weekday/business days/countdown), pet ages, fuel, electricity, download, aspect ratio, PPI, GPA, grades, attendance, bill split |
| Sports | Sports | Pace, Riegel predictor, swim pace, cycling, VO2max (Cooper), 1RM (Epley), splits |
| Food | Food | Recipe scale, baker's %, oven temp/gas mark, alcohol units, coffee ratio, pizza dough, roast time |
| Ecology | Ecology | Driving CO2, solar estimate, rainwater, pool volume/chlorine, tree CO2, lighting cost |
| Other | Other | Working hours/pay, business gross margin, case unit price, shrinkage, circumference |

Gaps are listed, item by item, in `docs/ROADMAP.md` (live-data
calculators, jurisdiction tax tables, matrix/graphing, two-series
statistics, triangle solver, and the v0.2 formulaic backlog).

## Registry count

REGISTRY_COUNT = 299 calculators in 14 registry categories
(from the built binary's own `--list` output; asserted >= 180 by the
registry test, exact count 299 at v0.1.0).

Per-category counts from `--list`: Math 61, Finance 34, Physics 31,
Conversion 31, Construction 26, Health 24, Chemistry 23, Statistics 21,
Everyday 15, Sports 7, Food 7, Ecology 7, Date & Time 7, Other 5.


## v0.1.1 note (2026-10-25)

v0.1.1 changes no calculator count (299) and no formulas. It renames
the program (built under its previous name in v0.1.0),
merges every date calculator's year/month/day boxes into single
MM/DD/YYYY fields with named date groups (Birth Date, End Date, Start
Date, From Date, Target Date, Date, First Day of Last Period), adds
Today? buttons and CLI one-argument dates, and adds the shared
GUI+CLI calculation log.

## v0.8.0 note (2026-10-26)

Registry now 316 calculators in the same 14 categories (from
the built binary's own `--list` output). Per-category counts:
Math 62, Conversion 36, Physics 34, Finance 34, Construction
30, Chemistry 24, Statistics 24, Health 24, Everyday 15,
Sports 7, Food 7, Ecology 7, Date & Time 7, Other 5. The
v0.7.0 scientists batch added Replicates Summary and Outlier
Check (Statistics) and Serial Dilution (Chemistry); v0.8.0
adds the electronics pack to Physics (Resistor Colour Code,
Resistors in Series & Parallel, Voltage Divider) and the
mechanical pack to Construction (Tolerance Stack-Up, Safety
Factor & Margin of Safety, Bolt Torque, Beam — Stress &
Deflection). The materials table (#44) is reference data
served by `--materials` and the MATERIALS view, not a
calculator, so it does not change the count.
