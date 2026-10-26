//! Registry metadata added at v0.3.0 (Batch 1 of the 50-enhancement
//! program): a hand-written teaching example for every calculator
//! (program items #11 / #50), plus hand-written input hints for the
//! fields whose label alone is not enough (item #20).
//!
//! Examples reuse the v0.1.2 audit known-answer vectors where they
//! fit (they are realistic, independently computed values); the
//! rest were written for this table. `tests/v030.rs` proves every
//! calculator has an example and that it runs without error, and
//! that the smart defaults (item #50) are the example values
//! unless a DEFAULTS override below says otherwise.

/// (calculator id, example inputs in registry order).
pub static EXAMPLES: &[(&str, &[f64])] = &[
    ("percent-of", &[25.0, 200.0]),
    ("percent-is", &[50.0, 200.0]),
    ("percent-change", &[100.0, 125.0]),
    ("percent-difference", &[80.0, 100.0]),
    ("reverse-percent", &[20.0, 120.0]),
    ("percent-off-result", &[-10.0, 200.0]),
    ("fraction-simplify", &[6.0, 8.0]),
    ("fraction-add", &[1.0, 2.0, 1.0, 3.0]),
    ("fraction-subtract", &[3.0, 4.0, 1.0, 4.0]),
    ("fraction-multiply", &[2.0, 3.0, 3.0, 4.0]),
    ("fraction-divide", &[1.0, 2.0, 3.0, 4.0]),
    ("fraction-to-decimal", &[1.0, 1.0, 2.0]),
    ("decimal-to-fraction", &[0.75]),
    ("exponent", &[2.0, 10.0]),
    ("square-cube", &[5.0]),
    ("square-root", &[144.0]),
    ("nth-root", &[27.0, 3.0]),
    ("log", &[1000.0, 10.0]),
    ("ln-log10", &[100.0]),
    ("antilog", &[2.0]),
    ("factorial", &[5.0]),
    ("scientific-notation", &[12345.0]),
    ("from-scientific", &[1.5, 4.0]),
    ("sig-figs", &[123.456, 3.0]),
    ("round-decimals", &[123.4567, 2.0]),
    ("gcd", &[48.0, 36.0]),
    ("lcm", &[4.0, 6.0]),
    ("prime-check", &[7.0]),
    ("ratio", &[4.0, 6.0, 10.0]),
    ("proportion", &[2.0, 3.0, 9.0]),
    ("rule-of-three", &[5.0, 100.0, 8.0]),
    ("quadratic", &[1.0, -3.0, 2.0]),
    ("linear-equation", &[2.0, 3.0, 11.0]),
    ("slope", &[0.0, 0.0, 2.0, 4.0]),
    ("distance-points", &[0.0, 0.0, 6.0, 8.0]),
    ("pythagorean", &[3.0, 4.0]),
    ("pythagorean-leg", &[13.0, 5.0]),
    ("arithmetic-sequence", &[3.0, 2.0, 5.0]),
    ("geometric-sequence", &[2.0, 3.0, 4.0]),
    ("circle", &[1.0]),
    ("circle-from-area", &[78.5398]),
    ("rectangle", &[5.0, 3.0]),
    ("square-shape", &[4.0]),
    ("triangle-area", &[10.0, 4.0]),
    ("triangle-heron", &[3.0, 4.0, 5.0]),
    ("trapezoid", &[4.0, 6.0, 3.0, 2.0, 2.0]),
    ("parallelogram", &[5.0, 3.0, 4.0]),
    ("rhombus", &[6.0, 8.0]),
    ("ellipse", &[2.0, 1.0]),
    ("regular-polygon", &[6.0, 1.0]),
    ("sector", &[2.0, 90.0]),
    ("cube-box", &[2.0, 3.0, 4.0]),
    ("cylinder", &[1.0, 2.0]),
    ("sphere", &[1.0]),
    ("cone", &[3.0, 4.0]),
    ("pyramid", &[2.0, 3.0]),
    ("frustum", &[2.0, 1.0, 3.0]),
    ("trig-deg", &[30.0]),
    ("trig-inverse", &[0.5]),
    ("angle-convert", &[180.0, 0.0]),
    ("average", &[10.0, 20.0]),
    ("compound-interest", &[1000.0, 5.0, 10.0]),
    (
        "compound-interest-monthly-deposit",
        &[1000.0, 5.0, 10.0, 100.0],
    ),
    ("simple-interest", &[1000.0, 5.0, 10.0]),
    ("loan-payment", &[10000.0, 0.0, 1.0]),
    (
        "mortgage-payment",
        &[300000.0, 60000.0, 6.5, 30.0, 4200.0, 1800.0, 0.0],
    ),
    ("auto-loan", &[30000.0, 5000.0, 2000.0, 8.0, 6.0, 5.0]),
    ("loan-affordability", &[500.0, 6.0, 10.0]),
    ("present-value", &[1100.0, 10.0, 1.0]),
    ("future-value", &[1000.0, 10.0, 2.0]),
    ("annuity-pv", &[1000.0, 10.0, 2.0]),
    ("annuity-fv", &[1000.0, 10.0, 2.0]),
    ("cagr", &[100.0, 121.0, 2.0]),
    ("roi", &[100.0, 150.0]),
    ("profit-margin", &[1000.0, 600.0]),
    ("price-from-margin", &[80.0, 20.0]),
    ("price-from-markup", &[50.0, 50.0]),
    ("discount", &[100.0, 20.0]),
    ("double-discount", &[100.0, 10.0, 10.0]),
    ("sales-tax-add", &[100.0, 8.0]),
    ("sales-tax-extract", &[108.0, 8.0]),
    ("tip", &[80.0, 15.0]),
    ("tip-split", &[100.0, 20.0, 4.0]),
    ("salary-hourly", &[52000.0, 2080.0]),
    ("hourly-salary", &[20.0, 40.0, 5.0]),
    ("savings-goal", &[1200.0, 0.0, 12.0]),
    ("break-even", &[1000.0, 25.0, 15.0]),
    ("effective-rate", &[12.0, 12.0]),
    ("rule-of-72", &[8.0]),
    ("credit-card-payoff", &[1200.0, 12.0, 100.0]),
    ("inflation", &[100.0, 3.0, 10.0]),
    ("net-margin", &[1000.0, 700.0]),
    ("payback-period", &[10000.0, 2500.0]),
    ("depreciation-straight", &[20000.0, 2000.0, 5.0, 2.0]),
    ("rent-vs-buy-simple", &[1500.0, 300000.0, 6.0, 5.0, 5000.0]),
    ("bmi", &[70.0, 175.0]),
    ("bmi-imperial", &[154.0, 68.0]),
    ("bmr-male", &[70.0, 175.0, 30.0]),
    ("bmr-female", &[60.0, 165.0, 30.0]),
    ("bmr-harris-male", &[70.0, 175.0, 30.0]),
    ("bmr-harris-female", &[60.0, 165.0, 30.0]),
    ("tdee", &[1600.0, 1.55]),
    ("calorie-target", &[2500.0, -0.5, 35.0]),
    ("body-fat-navy-male", &[86.0, 38.0, 178.0]),
    ("body-fat-navy-female", &[70.0, 95.0, 34.0, 165.0]),
    ("ideal-weight-devine", &[1.0, 170.18]),
    ("water-intake", &[70.0]),
    ("target-heart-rate", &[40.0]),
    ("karvonen", &[40.0, 60.0, 70.0]),
    ("pace", &[10.0, 50.0]),
    ("calories-burned-walk", &[70.0, 60.0]),
    ("calories-burned-run", &[70.0, 10.0]),
    ("steps-distance", &[10000.0, 70.0, 0.762]),
    ("due-date", &[2025.0, 1.0, 1.0]),
    ("ovulation", &[2026.0, 1.0, 1.0, 28.0]),
    ("bac-widmark", &[40.0, 80.0, 1.0, 0.0]),
    ("sleep-cycles", &[8.0]),
    ("macro-split", &[2000.0, 30.0, 40.0, 30.0]),
    ("protein-needs", &[70.0, 1.6]),
    ("speed-from-distance-time", &[60.0, 5.0]),
    ("distance-from-speed-time", &[12.0, 5.0]),
    ("time-from-speed-distance", &[100.0, 20.0]),
    ("average-velocity", &[0.0, 50.0, 0.0, 10.0]),
    ("acceleration", &[0.0, 20.0, 4.0]),
    ("final-velocity", &[5.0, 2.0, 3.0]),
    ("force", &[10.0, 2.0]),
    ("weight-from-mass", &[1.0]),
    ("mass-from-weight", &[98.0665]),
    ("kinetic-energy", &[2.0, 3.0]),
    ("potential-energy", &[2.0, 10.0]),
    ("mechanical-energy", &[2.0, 3.0, 10.0]),
    ("power-physics", &[1000.0, 20.0]),
    ("work", &[50.0, 3.0]),
    ("density-physics", &[100.0, 20.0]),
    ("pressure", &[100.0, 2.0]),
    ("ohms-law", &[12.0, 4.0]),
    ("ohms-solve-i", &[12.0, 3.0]),
    ("ohms-solve-v", &[3.0, 4.0]),
    ("electrical-power", &[230.0, 2.0, 3.0]),
    ("resistors-parallel", &[6.0, 3.0]),
    ("projectile", &[20.0, 45.0]),
    ("pendulum", &[1.0]),
    ("wave", &[10.0, 2.0]),
    ("torque", &[50.0, 0.4]),
    ("momentum", &[2.0, 3.0]),
    ("friction", &[0.3, 10.0]),
    ("hooke", &[200.0, 0.1]),
    ("gravity-force", &[1.0, 1.0, 1.0]),
    ("centripetal", &[2.0, 3.0, 1.5]),
    ("free-fall", &[2.0]),
    ("moles-from-mass", &[36.0, 18.0]),
    ("mass-from-moles", &[2.0, 18.0]),
    ("molecules", &[1.0]),
    ("molarity", &[0.5, 0.25]),
    ("molarity-from-mass", &[58.5, 58.5, 1.0]),
    ("dilution", &[10.0, 10.0, 100.0]),
    ("dilution-volume", &[10.0, 1.0, 100.0]),
    ("ph", &[0.01]),
    ("ph-inverse", &[7.0]),
    ("poh", &[0.001]),
    ("ideal-gas-pressure", &[1.0, 273.15, 22.414]),
    ("ideal-gas-volume", &[1.0, 273.15, 1.0]),
    ("ideal-gas-moles", &[1.0, 22.414, 273.15]),
    ("ideal-gas-temp", &[1.0, 22.414, 1.0]),
    ("boyle", &[2.0, 3.0, 6.0]),
    ("charles", &[3.0, 300.0, 600.0]),
    ("gay-lussac", &[2.0, 300.0, 450.0]),
    ("combined-gas", &[1.0, 2.0, 300.0, 1.0, 600.0]),
    ("percent-composition", &[30.0, 70.0]),
    ("solution-percent", &[10.0, 200.0]),
    ("empirical-ratio", &[0.3, 0.1]),
    ("reaction-yield", &[45.0, 50.0]),
    ("half-life", &[100.0, 10.0, 10.0]),
    ("square-footage-rect", &[10.0, 12.0]),
    ("square-footage-circle", &[10.0]),
    ("square-footage-triangle", &[10.0, 4.0]),
    ("concrete-slab", &[10.0, 10.0, 4.0, 0.0]),
    ("concrete-column", &[2.0, 1.0, 1.0, 0.0]),
    ("concrete-stairs", &[3.0, 6.0, 9.0, 0.0]),
    ("paint", &[10.0, 10.0, 8.0, 0.0, 0.0, 1.0, 350.0]),
    ("tile", &[10.0, 10.0, 12.0, 12.0, 10.0, 0.0]),
    ("mulch", &[100.0, 3.0, 50.0]),
    ("gravel", &[10.0, 10.0, 3.0, 1.4]),
    ("flooring", &[10.0, 12.0, 20.0, 10.0, 30.0]),
    ("fence", &[100.0, 8.0, 2.0, 5.5, 0.5]),
    ("stairs", &[84.0, 7.5, 10.0]),
    ("roofing", &[20.0, 10.0, 6.0, 0.0]),
    ("drywall", &[320.0, 32.0]),
    ("board-feet", &[2.0, 4.0, 8.0, 1.0, 0.0]),
    ("deck-boards", &[12.0, 10.0, 5.5, 0.5]),
    ("brick", &[10.0, 8.0, 0.0]),
    ("wallpaper", &[300.0, 30.0]),
    ("insulation-bags", &[1000.0, 10.0, 25.0]),
    ("asphalt-driveway", &[20.0, 10.0, 4.0]),
    ("room-area-walls", &[10.0, 12.0, 8.0]),
    ("cubic-yards", &[9.0, 9.0, 12.0]),
    ("angle-iron-weight", &[12.0, 12.0, 0.5]),
    ("concrete-blocks", &[10.0, 8.0, 0.0]),
    ("post-holes-concrete", &[12.0, 24.0, 4.0, 10.0]),
    ("convert-length", &[1.0, 7.0, 3.0]),
    ("convert-mass", &[1.0, 2.0, 5.0]),
    ("convert-area", &[1.0, 7.0, 5.0]),
    ("convert-volume", &[1.0, 10.0, 0.0]),
    ("convert-speed", &[100.0, 1.0, 0.0]),
    ("convert-time", &[2.0, 3.0, 2.0]),
    ("convert-data", &[1.0, 7.0, 6.0]),
    ("convert-pressure", &[1.0, 3.0, 4.0]),
    ("convert-energy", &[1.0, 5.0, 6.0]),
    ("convert-power-units", &[1.0, 2.0, 0.0]),
    ("convert-force-units", &[10.0, 3.0, 0.0]),
    ("convert-torque-units", &[10.0, 1.0, 0.0]),
    ("convert-density-units", &[1.0, 1.0, 0.0]),
    ("convert-frequency", &[60.0, 4.0, 0.0]),
    ("convert-flow", &[60.0, 1.0, 0.0]),
    ("convert-illuminance", &[1.0, 1.0, 0.0]),
    ("convert-temperature", &[32.0, 1.0, 0.0]),
    ("convert-fuel-economy", &[30.0, 0.0]),
    ("grams-cups-flour", &[250.0, 0.0]),
    ("grams-cups-sugar", &[0.0, 2.0]),
    ("grams-cups-brown-sugar", &[0.0, 2.0]),
    ("grams-cups-butter", &[0.0, 2.0]),
    ("grams-cups-cocoa", &[0.0, 2.0]),
    ("grams-cups-rice", &[0.0, 2.0]),
    ("grams-cups-oats", &[0.0, 2.0]),
    ("grams-cups-honey", &[0.0, 2.0]),
    ("grams-cups-milk", &[0.0, 2.0]),
    ("grams-cups-water", &[0.0, 2.0]),
    ("grams-cups-oil", &[0.0, 2.0]),
    ("grams-cups-powdered-sugar", &[0.0, 2.0]),
    ("shoe-size-men", &[9.5]),
    ("mean", &[1.0, 2.0, 3.0, 4.0]),
    ("median", &[1.0, 2.0, 3.0]),
    ("mode-range", &[1.0, 2.0, 2.0, 3.0, 4.0]),
    ("sum-product", &[1.0, 2.0, 3.0, 4.0]),
    ("std-population", &[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]),
    ("std-sample", &[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]),
    ("geometric-mean", &[2.0, 8.0]),
    ("harmonic-mean", &[10.0, 20.0]),
    ("z-score", &[85.0, 70.0, 15.0]),
    ("percentile-from-z", &[1.96]),
    ("p-value-z", &[2.0]),
    ("probability", &[1.0, 6.0]),
    ("combinations", &[5.0, 2.0]),
    ("permutations", &[5.0, 2.0]),
    ("confidence-interval-mean", &[100.0, 15.0, 25.0, 1.96]),
    ("margin-of-error", &[0.5, 1000.0, 1.96]),
    ("sample-size-proportion", &[0.03, 0.5, 1.96]),
    ("expected-value", &[10.0, 0.5, 20.0, 0.5]),
    ("dice-probability", &[7.0]),
    ("percentile-rank", &[85.0, 70.0, 80.0, 85.0, 90.0, 95.0]),
    ("weighted-average", &[80.0, 0.3, 90.0, 0.7]),
    (
        "linear-regression",
        &[1.0, 2.0, 2.0, 4.0, 3.0, 5.0, 4.0, 4.0, 5.0, 5.0, 5.0],
    ),
    ("days-between-dates", &[2025.0, 1.0, 1.0, 2025.0, 1.0, 10.0]),
    ("date-plus-days", &[2025.0, 1.0, 1.0, 30.0]),
    ("age", &[1988.0, 4.0, 23.0, 2026.0, 10.0, 25.0]),
    ("weekday", &[2025.0, 10.0, 25.0]),
    ("time-duration", &[9.0, 30.0, 17.0, 45.0]),
    ("countdown-days", &[2026.0, 10.0, 25.0, 2026.0, 12.0, 25.0]),
    ("business-days", &[2025.0, 1.0, 6.0, 2025.0, 1.0, 10.0]),
    ("dog-age", &[3.0, 3.0]),
    ("cat-age", &[5.0]),
    ("fuel-cost-trip", &[300.0, 30.0, 3.5]),
    ("mpg-trip", &[300.0, 10.0]),
    ("electricity-cost", &[100.0, 5.0, 0.2]),
    ("download-time", &[800.0, 100.0]),
    ("aspect-ratio", &[1920.0, 1080.0, 1280.0]),
    ("screen-ppi", &[1920.0, 1080.0, 15.6]),
    ("gpa", &[4.0, 3.0, 3.0, 4.0]),
    ("test-grade", &[45.0, 50.0, 90.0]),
    ("attendance", &[45.0, 50.0, 90.0]),
    ("age-in-days", &[2000.0, 1.0, 1.0, 2000.0, 1.0, 2.0]),
    ("split-bill", &[100.0, 10.0, 20.0, 4.0]),
    ("unit-price", &[10.0, 5.0, 9.0, 3.0]),
    ("overtime-pay", &[20.0, 40.0, 5.0]),
    ("race-pace", &[6.0, 10.0]),
    ("race-predictor-riegel", &[60.0, 10.0, 20.0]),
    ("swim-pace", &[30.0, 1500.0]),
    ("cycling-speed", &[40.0, 2.0]),
    ("vo2max-estimate", &[2800.0]),
    ("one-rep-max", &[100.0, 10.0]),
    ("marathon-splits", &[240.0, 42.195]),
    ("recipe-scale", &[200.0, 4.0, 8.0]),
    ("bakers-percent", &[500.0, 300.0, 60.0]),
    ("oven-temp", &[200.0]),
    ("alcohol-units", &[500.0, 5.0]),
    ("coffee-water-ratio", &[30.0, 16.0]),
    ("pizza-dough", &[300.0, 60.0, 1.0]),
    ("meat-cooking-time", &[5.0, 20.0]),
    ("carbon-driving", &[200.0, 12.5, 2.31]),
    ("solar-output", &[4000.0, 5.0, 0.3]),
    ("rainwater", &[100.0, 10.0]),
    ("pool-volume", &[10.0, 5.0, 1.5]),
    ("pool-chlorine", &[20000.0, 2.0]),
    ("tree-oxygen-co2", &[10.0, 5.0]),
    ("lighting-cost-year", &[10.0, 5.0, 4.0, 0.2]),
    ("working-hours-pay", &[9.0, 0.0, 17.0, 30.0, 30.0]),
    ("gross-profit-business", &[5000.0, 3000.0]),
    ("price-per-unit-other", &[24.0, 12.0, 25.0]),
    ("shrinkage", &[10000.0, 9500.0]),
    ("circumference", &[10.0]),
];

/// Smart-default overrides (item #50): where the teaching example is
/// a stress case rather than the value a field should open with, the
/// opening default lives here. Everywhere else the default IS the
/// example value — visible and editable, never hidden magic.
pub static DEFAULT_OVERRIDES: &[(&str, &[f64])] = &[
    // The audit example exercises a negative percent change; a field
    // should open with the ordinary case.
    ("percent-off-result", &[20.0, 200.0]),
    // STP-flavoured openings for the gas laws (273.15 K, 1 atm,
    // 22.414 L molar volume) — the discipline standard, stated here.
    ("ideal-gas-volume", &[1.0, 273.15, 1.0]),
    ("ideal-gas-moles", &[1.0, 22.414, 273.15]),
    ("ideal-gas-temp", &[1.0, 22.414, 1.0]),
];

/// Hand-written hints for fields whose label alone is not enough
/// (item #20) — mostly code fields. Aligned with `Calc::inputs`;
/// an empty string means "use the generated hint".
pub static HINTS: &[(&str, &[&str])] = &[
    (
        "convert-temperature",
        &[
            "A number, e.g. 100",
            "From unit code: 0 = Celsius, 1 = Fahrenheit, 2 = Kelvin",
            "To unit code: 0 = Celsius, 1 = Fahrenheit, 2 = Kelvin",
        ],
    ),
    (
        "convert-fuel-economy",
        &[
            "US miles per gallon — fill this OR the next field and leave the other 0",
            "Litres per 100 km — fill this OR the previous field and leave the other 0",
        ],
    ),
    (
        "angle-convert",
        &[
            "A number, e.g. 180",
            "Unit code: 0 = degrees, 1 = radians, 2 = gradians, 3 = turns",
        ],
    ),
    (
        "ideal-weight-devine",
        &["Sex code: 1 = male, 2 = female", "Height in centimetres, e.g. 175"],
    ),
    (
        "bac-widmark",
        &[
            "Pure alcohol in grams, e.g. 40",
            "Body weight in kilograms, e.g. 80",
            "Sex code: 1 = male, 2 = female",
            "Hours since drinking started, e.g. 1",
        ],
    ),
    (
        "dog-age",
        &[
            "The dog's age in years, e.g. 3",
            "Size code: 1 = small, 2 = medium, 3 = large",
        ],
    ),
    (
        "tdee",
        &[
            "Basal metabolic rate in cal/day, e.g. 1600",
            "Activity factor: 1.2 = sedentary, 1.375 = light, 1.55 = moderate, 1.725 = very active, 1.9 = athlete",
        ],
    ),
    (
        "shoe-size-men",
        &["US men's size, whole or half sizes 6 to 14, e.g. 9.5"],
    ),
    (
        "dice-probability",
        &["A whole number from 2 to 12, e.g. 7"],
    ),
];

/// The teaching example for a calculator (empty if none — the
/// registry-wide test in tests/v030.rs forbids that).
pub fn example_for(c: &crate::Calc) -> &'static [f64] {
    for (id, ex) in EXAMPLES {
        if *id == c.id {
            return ex;
        }
    }
    &[]
}

/// The smart defaults a calculator's fields open with (item #50):
/// the DEFAULT_OVERRIDES value when present, else the example.
pub fn defaults_for(c: &crate::Calc) -> &'static [f64] {
    for (id, ex) in DEFAULT_OVERRIDES {
        if *id == c.id {
            return ex;
        }
    }
    example_for(c)
}

/// The hint shown under one input field (item #20). Date fields use
/// the shared DATE_HINT; hand-written HINTS win next; everything else
/// gets a generated hint from its label + example value, so EVERY
/// field has one. Returns "" only for the hidden inner fields of a
/// date group (never rendered).
pub fn hint_for(c: &crate::Calc, i: usize) -> String {
    if crate::date_group_at(c, i).is_some() {
        return crate::DATE_HINT.to_string();
    }
    if c.date_groups.iter().any(|g| i > g.start && i < g.start + 3) {
        return String::new();
    }
    for (id, hints) in HINTS {
        if *id == c.id {
            if let Some(h) = hints.get(i) {
                if !h.is_empty() {
                    return h.to_string();
                }
            }
        }
    }
    let ex = example_for(c);
    let eg = ex.get(i).map(|v| crate::n(*v)).unwrap_or_default();
    if c.list {
        let joined = ex
            .iter()
            .map(|v| crate::n(*v))
            .collect::<Vec<_>>()
            .join(", ");
        return format!("Numbers separated by commas or spaces — e.g. {joined}");
    }
    let label = c.inputs.get(i).copied().unwrap_or("");
    if label.contains("number from list") {
        return format!(
            "A whole unit number from the numbered list in the description above — e.g. {eg}"
        );
    }
    if let (Some(open), Some(close)) = (label.find('('), label.rfind(')')) {
        let inside = &label[open + 1..close];
        // A short parenthesis with no digits/conditions reads as a unit.
        if inside.len() <= 12 && !inside.chars().any(|ch| ch.is_ascii_digit()) {
            return format!("A number in {inside} — e.g. {eg}");
        }
    }
    if label.contains('%') {
        return format!("A percentage — e.g. {eg} means {eg}%");
    }
    format!("A number — e.g. {eg}")
}

/// The field labels a user sees (and a batch CSV header names), in
/// order: date groups collapse to their one merged label, inner date
/// fields disappear, list calculators have their single list label.
pub fn display_fields(c: &crate::Calc) -> Vec<String> {
    if c.list {
        return vec![c.inputs.first().copied().unwrap_or("Numbers").to_string()];
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < c.inputs.len() {
        if let Some(g) = crate::date_group_at(c, i) {
            out.push(g.label.to_string());
            i += 3;
        } else if c.date_groups.iter().any(|g| i > g.start && i < g.start + 3) {
            i += 1;
        } else {
            out.push(c.inputs[i].to_string());
            i += 1;
        }
    }
    out
}
