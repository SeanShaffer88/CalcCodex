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
    ("serial-dilution", &[100.0, 1.0, 1.0, 9.0, 10.0, 4.0]),
    ("resistors-series-parallel", &[100.0, 200.0, 300.0]),
    ("voltage-divider", &[12.0, 1000.0, 2000.0]),
    ("tolerance-stack-up", &[10.0, 0.1, 20.0, 0.2, 5.0, 0.05]),
    ("safety-factor", &[1.0, 250.0, 100.0, 0.0, 0.0]),
    ("bolt-torque", &[0.2, 10.0, 10000.0]),
    ("beam-bending", &[2.0, 1000.0, 200.0, 0.0, 1.0, 0.05, 0.1]),
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
    (
        "replicates-summary",
        &[10.0, 12.0, 23.0, 23.0, 16.0, 23.0, 21.0, 16.0],
    ),
    (
        "outlier-check",
        &[10.0, 12.0, 11.0, 13.0, 12.0, 11.0, 10.0, 12.0, 13.0, 100.0],
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
    ("compound-density", &[1000.0, 0.0, 1.0]),
    ("compound-fuel-consumption", &[8.0, 0.0, 1.0]),
    ("compound-data-rate", &[100.0, 2.0, 4.0]),
    ("compound-concentration", &[5.0, 0.0, 1.0]),
    ("compound-energy-density", &[3600000.0, 0.0, 3.0]),
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
    (
        "expression-calculator",
        &["Type an expression — e.g. 2*(3+4)^2. Operators + - * / ^ and brackets; functions sin cos tan asin acos atan sqrt ln log10 exp abs; constants pi, e, tau, the CODATA library (c, h, k_b, n_a...) and your own defined constants; measurements like 10 \u{b1} 0.2 carry their uncertainty through."],
    ),
    (
        "serial-dilution",
        &[
            "The concentration of your stock — e.g. 100 (its unit carries through every step)",
            "1 = give transfer + diluent volumes; 2 = give the factor per step",
            "Mode 1: the volume you move into each new tube — e.g. 1",
            "Mode 1: the diluent already in each new tube — e.g. 9 (with a transfer of 1 that makes each step 1:10)",
            "Mode 2: the factor each step divides by — e.g. 10 for a 1:10 series",
            "How many steps to print — a whole number from 1 to 100, e.g. 4",
        ],
    ),
    (
        "replicates-summary",
        &["The repeated measurements, separated by commas, spaces or new lines — a pasted column works as-is. At least 2."],
    ),
    (
        "outlier-check",
        &["The values to check, separated by commas, spaces or new lines — a pasted column works as-is. At least 4."],
    ),
    (
        "resistor-colour-code",
        &["Type the band colours in order (red red brown gold), or a resistance to get its bands (4700, 4.7k, 1M — add a tolerance colour if you like: 4700 gold)"],
    ),
    (
        "resistors-series-parallel",
        &["The resistances in ohms, separated by commas, spaces or new lines — a pasted column works as-is."],
    ),
    (
        "voltage-divider",
        &[
            "The supply voltage across R1 + R2 together — e.g. 12",
            "The input-side resistor in ohms — e.g. 1000",
            "The output-side resistor in ohms — the output voltage is measured across this one — e.g. 2000",
        ],
    ),
    (
        "tolerance-stack-up",
        &["Dimensions in pairs: nominal then its +/- tolerance, e.g. 10 0.1 20 0.2 5 0.05. Same units throughout."],
    ),
    (
        "safety-factor",
        &[
            "1 = you enter the applied stress; 2 = you enter a load and an area and the stress is load / area",
            "The material limit you check against — its yield strength or its ultimate strength, in the same units as the stress (e.g. 250 MPa)",
            "Mode 1: the applied stress, same units as the strength (mode 2 ignores this — still type a number, e.g. 0)",
            "Mode 2: the load on the part (mode 1 ignores this — still type a number, e.g. 0)",
            "Mode 2: the cross-section area carrying the load, in units matching the load and strength (mode 1 ignores this — still type a number, e.g. 0)",
        ],
    ),
    (
        "bolt-torque",
        &[
            "The nut factor for the friction condition — about 0.20 for dry plain steel, about 0.15 lubricated",
            "The bolt's nominal diameter in millimetres — e.g. 10 for an M10 bolt",
            "The clamp force the joint needs, in newtons — e.g. 10000",
        ],
    ),
    (
        "beam-bending",
        &[
            "The span between the two supports, in metres — e.g. 2",
            "The single load at the middle of the span, in newtons — e.g. 1000",
            "Young's modulus in GPa (mild steel about 200) — or 0 to take it from the material code instead",
            "Used when E is 0: the material's code from the materials table (CalcDex --materials) — 5 is mild steel",
            "1 = rectangular section, 2 = circular section",
            "Rectangle: the width in metres. Circle: the diameter in metres — e.g. 0.05",
            "Rectangle only: the vertical depth in metres — e.g. 0.1 (type 0 for a circle)",
        ],
    ),
];

/// Typed-quantity metadata (v0.4.0 items #6/#7): per calculator,
/// one entry per input — (unit family for parsing, expected unit
/// code, expected dimension for the dimensional check). "" means
/// "not known", and unknown dimensions never warn. Coverage: the
/// Physics module, the BMI/BMR health set, and (attached in
/// conversion.rs itself) every parametric converter's value field.
/// Typed quantities are converted into the expected unit named
/// here (the SI unit where the field works in SI).
/// One UNIT_INFO row: (calculator id, per-field unit families,
/// per-field expected units, per-field dimensions).
pub type UnitInfoRow = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
);

pub static UNIT_INFO: &[UnitInfoRow] = &[
    (
        "speed-from-distance-time",
        &["length", "time"],
        &["m", "s"],
        &["length", "time"],
    ),
    (
        "distance-from-speed-time",
        &["speed", "time"],
        &["m/s", "s"],
        &["speed", "time"],
    ),
    (
        "time-from-speed-distance",
        &["length", "speed"],
        &["m", "m/s"],
        &["length", "speed"],
    ),
    (
        "average-velocity",
        &["length", "length", "time", "time"],
        &["m", "m", "s", "s"],
        &["length", "length", "time", "time"],
    ),
    (
        "acceleration",
        &["speed", "speed", "time"],
        &["m/s", "m/s", "s"],
        &["speed", "speed", "time"],
    ),
    (
        "final-velocity",
        &["speed", "acceleration", "time"],
        &["m/s", "m/s2", "s"],
        &["speed", "acceleration", "time"],
    ),
    (
        "force",
        &["mass", "acceleration"],
        &["kg", "m/s2"],
        &["mass", "acceleration"],
    ),
    ("weight-from-mass", &["mass"], &["kg"], &["mass"]),
    ("mass-from-weight", &["force"], &["N"], &["force"]),
    (
        "kinetic-energy",
        &["mass", "speed"],
        &["kg", "m/s"],
        &["mass", "speed"],
    ),
    (
        "potential-energy",
        &["mass", "length"],
        &["kg", "m"],
        &["mass", "length"],
    ),
    (
        "mechanical-energy",
        &["mass", "speed", "length"],
        &["kg", "m/s", "m"],
        &["mass", "speed", "length"],
    ),
    (
        "power-physics",
        &["energy", "time"],
        &["J", "s"],
        &["energy", "time"],
    ),
    (
        "work",
        &["force", "length"],
        &["N", "m"],
        &["force", "length"],
    ),
    (
        "density-physics",
        &["mass", "volume"],
        &["kg", "L"],
        &["mass", "volume"],
    ),
    (
        "pressure",
        &["force", "area"],
        &["N", "m2"],
        &["force", "area"],
    ),
    (
        "ohms-law",
        &["voltage", "resistance"],
        &["V", "ohm"],
        &["voltage", "resistance"],
    ),
    (
        "ohms-solve-i",
        &["voltage", "current"],
        &["V", "A"],
        &["voltage", "current"],
    ),
    (
        "ohms-solve-v",
        &["current", "resistance"],
        &["A", "ohm"],
        &["current", "resistance"],
    ),
    (
        "electrical-power",
        &["voltage", "current", "time"],
        &["V", "A", "h"],
        &["voltage", "current", "time"],
    ),
    (
        "resistors-parallel",
        &["resistance", "resistance"],
        &["ohm", "ohm"],
        &["resistance", "resistance"],
    ),
    (
        "projectile",
        &["speed", "angle"],
        &["m/s", "deg"],
        &["speed", "angle"],
    ),
    ("pendulum", &["length"], &["m"], &["length"]),
    (
        "wave",
        &["frequency", "length"],
        &["Hz", "m"],
        &["frequency", "length"],
    ),
    (
        "torque",
        &["force", "length"],
        &["N", "m"],
        &["force", "length"],
    ),
    (
        "momentum",
        &["mass", "speed"],
        &["kg", "m/s"],
        &["mass", "speed"],
    ),
    ("friction", &["", "mass"], &["", "kg"], &["", "mass"]),
    ("hooke", &["", "length"], &["", "m"], &["", "length"]),
    (
        "gravity-force",
        &["mass", "mass", "length"],
        &["kg", "kg", "m"],
        &["mass", "mass", "length"],
    ),
    (
        "centripetal",
        &["mass", "speed", "length"],
        &["kg", "m/s", "m"],
        &["mass", "speed", "length"],
    ),
    ("free-fall", &["time"], &["s"], &["time"]),
    (
        "bmi",
        &["mass", "length"],
        &["kg", "cm"],
        &["mass", "length"],
    ),
    (
        "bmi-imperial",
        &["mass", "length"],
        &["lb", "in"],
        &["mass", "length"],
    ),
    (
        "bmr-male",
        &["mass", "length", ""],
        &["kg", "cm", ""],
        &["mass", "length", ""],
    ),
    (
        "bmr-female",
        &["mass", "length", ""],
        &["kg", "cm", ""],
        &["mass", "length", ""],
    ),
    (
        "bmr-harris-male",
        &["mass", "length", ""],
        &["kg", "cm", ""],
        &["mass", "length", ""],
    ),
    (
        "bmr-harris-female",
        &["mass", "length", ""],
        &["kg", "cm", ""],
        &["mass", "length", ""],
    ),
    ("water-intake", &["mass"], &["kg"], &["mass"]),
    ("protein-needs", &["mass", ""], &["kg", ""], &["mass", ""]),
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

// ---------- v0.5.0: formulas + symbol glossaries (items #19/#12) ----------
//
// Every calculator carries (a) its FORMULA in plain text symbols
// and (b) a SYMBOL GLOSSARY: (symbol, plain-words meaning) pairs
// written IN INPUT ORDER, so the deeper-steps substitution can
// pair glossary[i] with input i. Symbols are the letters used in
// the formula; where a calculator's inputs are plain quantities
// (a price, a count), the glossary names the quantity itself.

/// (calculator id, formula in plain text symbols).
pub static FORMULAS: &[(&str, &str)] = &[
    ("expression-calculator", "the expression itself, worked with + - * / ^, brackets, functions and constants"),
    ("percent-of", "result = Y x X / 100"),
    ("percent-is", "percent = X / Y x 100"),
    ("percent-change", "change % = (new - old) / |old| x 100"),
    ("percent-difference", "difference % = |A - B| / ((A + B) / 2) x 100"),
    ("reverse-percent", "original = Y / (1 + X/100)"),
    ("percent-off-result", "result = Y x (1 + X/100)"),
    ("fraction-simplify", "p/q in lowest terms = (p / GCD) / (q / GCD)"),
    ("fraction-add", "a/b + c/d = (a x d + c x b) / (b x d), then simplified"),
    ("fraction-subtract", "a/b - c/d = (a x d - c x b) / (b x d), then simplified"),
    ("fraction-multiply", "a/b x c/d = (a x c) / (b x d), then simplified"),
    ("fraction-divide", "a/b / (c/d) = (a x d) / (b x c), then simplified"),
    ("fraction-to-decimal", "decimal = whole + numerator / denominator; percent = decimal x 100"),
    ("decimal-to-fraction", "decimal written over a power of ten, then divided top and bottom by the GCD"),
    ("exponent", "result = base ^ exponent"),
    ("square-cube", "square = x^2; cube = x^3"),
    ("square-root", "root = sqrt(x); cube root = x^(1/3)"),
    ("nth-root", "root = x^(1/n)"),
    ("log", "log_base(x) = ln(x) / ln(base)"),
    ("ln-log10", "ln(x) and log10(x)"),
    ("antilog", "10^x and e^x"),
    ("factorial", "x! = 1 x 2 x ... x x"),
    ("scientific-notation", "number = a x 10^b"),
    ("from-scientific", "plain number = a x 10^b"),
    ("sig-figs", "the number rounded to the chosen count of significant figures"),
    ("round-decimals", "the number rounded to the chosen decimal places (half up)"),
    ("gcd", "GCD = the largest whole number dividing both A and B (Euclid's method)"),
    ("lcm", "LCM = |A x B| / GCD(A, B)"),
    ("prime-check", "trial division by every whole number up to sqrt(n)"),
    ("ratio", "A:B divided top and bottom by the GCD; scaled so A becomes the target"),
    ("proportion", "form 1: x = A x C / B; form 2: x = B x C / A"),
    ("rule-of-three", "x = B x C / A"),
    ("quadratic", "x = (-b +- sqrt(b^2 - 4ac)) / (2a)"),
    ("linear-equation", "x = (c - b) / a"),
    ("slope", "m = (y2 - y1) / (x2 - x1)"),
    ("distance-points", "d = sqrt((x2 - x1)^2 + (y2 - y1)^2)"),
    ("pythagorean", "c = sqrt(a^2 + b^2)"),
    ("pythagorean-leg", "leg = sqrt(c^2 - other^2)"),
    ("arithmetic-sequence", "term_n = a1 + (n - 1) x d; sum = n/2 x (2a1 + (n - 1)d)"),
    ("geometric-sequence", "term_n = a1 x r^(n-1); sum = a1 x (r^n - 1) / (r - 1)"),
    ("circle", "A = pi x r^2; C = 2 x pi x r; d = 2r"),
    ("circle-from-area", "r = sqrt(A / pi)"),
    ("rectangle", "A = l x w; P = 2(l + w); diagonal = sqrt(l^2 + w^2)"),
    ("square-shape", "A = s^2; P = 4s; diagonal = s x sqrt(2)"),
    ("triangle-area", "A = base x height / 2"),
    ("triangle-heron", "s = (a + b + c) / 2; A = sqrt(s(s - a)(s - b)(s - c))"),
    ("trapezoid", "A = (a + b) / 2 x height; P = a + b + leg1 + leg2"),
    ("parallelogram", "A = base x height; P = 2(base + side)"),
    ("rhombus", "A = (d1 x d2) / 2; side = sqrt((d1/2)^2 + (d2/2)^2); P = 4 x side"),
    ("ellipse", "A = pi x a x b; P by Ramanujan's approximation"),
    ("regular-polygon", "A = n x s^2 / (4 x tan(pi / n)); P = n x s"),
    ("sector", "arc = 2 x pi x r x (angle / 360); sector A = pi x r^2 x (angle / 360)"),
    ("cube-box", "V = l x w x h; S = 2(lw + lh + wh)"),
    ("cylinder", "V = pi x r^2 x h; S = 2 x pi x r x (r + h)"),
    ("sphere", "V = (4/3) x pi x r^3; S = 4 x pi x r^2"),
    ("cone", "V = (1/3) x pi x r^2 x h; slant = sqrt(r^2 + h^2); S = pi x r x (r + slant)"),
    ("pyramid", "V = base^2 x h / 3; S = base^2 + 2 x base x slant"),
    ("frustum", "V = (1/3) x pi x h x (R^2 + R x r + r^2)"),
    ("trig-deg", "sin(angle), cos(angle), tan(angle) with the angle in degrees"),
    ("trig-inverse", "asin(ratio), acos(ratio), atan(ratio), answered in degrees"),
    ("angle-convert", "the value rewritten in degrees, radians, gradians and turns"),
    ("average", "mean = (A + B) / 2"),
    ("compound-interest", "FV = P x (1 + r)^t, with r = annual rate / 100"),
    ("compound-interest-monthly-deposit", "FV = P x (1 + r/12)^(12t) + D x (((1 + r/12)^(12t) - 1) / (r/12))"),
    ("simple-interest", "interest = P x r x t, with r = annual rate / 100"),
    ("loan-payment", "payment = P x r / (1 - (1 + r)^-n), with r = annual rate / 100 / 12 and n = years x 12"),
    ("mortgage-payment", "loan payment on (price - down payment), plus tax/12 + insurance/12 + PMI"),
    ("auto-loan", "loan = price - down payment - trade-in + tax % x (price - trade-in); then the loan payment formula"),
    ("loan-affordability", "P = payment x (1 - (1 + r)^-n) / r"),
    ("present-value", "PV = FV / (1 + r)^t"),
    ("future-value", "FV = PV x (1 + r)^t"),
    ("annuity-pv", "PV = payment x (1 - (1 + r)^-t) / r"),
    ("annuity-fv", "FV = payment x ((1 + r)^t - 1) / r"),
    ("cagr", "CAGR = (end / start)^(1 / years) - 1"),
    ("roi", "ROI % = (gain - cost) / cost x 100"),
    ("profit-margin", "profit = revenue - cost; margin % = profit / revenue x 100; markup % = profit / cost x 100"),
    ("price-from-margin", "price = cost / (1 - margin/100)"),
    ("price-from-markup", "price = cost x (1 + markup/100)"),
    ("discount", "sale price = price x (1 - discount/100); savings = price - sale price"),
    ("double-discount", "result = price x (1 - d1/100) x (1 - d2/100)"),
    ("sales-tax-add", "total = price + price x tax/100"),
    ("sales-tax-extract", "price before tax = total / (1 + tax/100)"),
    ("tip", "tip = bill x tip % / 100; total = bill + tip"),
    ("tip-split", "each pays = bill x (1 + tip % / 100) / people"),
    ("salary-hourly", "hourly = yearly salary / hours per year"),
    ("hourly-salary", "weekly = rate x regular hours + rate x 1.5 x overtime hours; yearly = weekly x 52"),
    ("savings-goal", "monthly deposit = goal x r / ((1 + r)^n - 1), with r = annual rate / 100 / 12 and n = months"),
    ("break-even", "break-even units = fixed costs / (price per unit - variable cost per unit)"),
    ("effective-rate", "EAR = (1 + nominal/n)^n - 1"),
    ("rule-of-72", "years to double is about 72 / rate %; exact = ln(2) / ln(1 + r)"),
    ("credit-card-payoff", "months = -ln(1 - balance x r / payment) / ln(1 + r), with r = APR / 100 / 12"),
    ("inflation", "future price = amount x (1 + inflation/100)^years"),
    ("net-margin", "net income = revenue - costs; net margin % = net income / revenue x 100"),
    ("payback-period", "years = initial investment / yearly cash inflow"),
    ("depreciation-straight", "yearly depreciation = (cost - salvage value) / useful life"),
    ("rent-vs-buy-simple", "rent total = monthly rent x 12 x years; buy cost = mortgage interest + tax and insurance over the years"),
    ("bmi", "BMI = weight (kg) / height (m)^2"),
    ("bmi-imperial", "BMI = 703 x weight (lb) / height (in)^2"),
    ("bmr-male", "BMR = 10 x kg + 6.25 x cm - 5 x age + 5"),
    ("bmr-female", "BMR = 10 x kg + 6.25 x cm - 5 x age - 161"),
    ("bmr-harris-male", "BMR = 66.5 + 13.75 x kg + 5.003 x cm - 6.75 x age"),
    ("bmr-harris-female", "BMR = 655.1 + 9.563 x kg + 1.850 x cm - 4.676 x age"),
    ("tdee", "TDEE = BMR x activity factor"),
    ("calorie-target", "daily adjustment = (kg to change x 7700) / days; target = TDEE - adjustment"),
    ("body-fat-navy-male", "BF% = 495 / (1.0324 - 0.19077 x log10(waist - neck) + 0.15456 x log10(height)) - 450"),
    ("body-fat-navy-female", "BF% = 495 / (1.29579 - 0.35004 x log10(waist + hip - neck) + 0.22100 x log10(height)) - 450"),
    ("ideal-weight-devine", "men: 50 kg + 2.3 kg per inch over 5 ft; women: 45.5 kg + 2.3 kg per inch over 5 ft (minus below 5 ft)"),
    ("water-intake", "water per day is about 35 ml x body weight (kg)"),
    ("target-heart-rate", "max HR = 220 - age; zones are % of max HR"),
    ("karvonen", "target HR = resting HR + intensity % x (max HR - resting HR), max HR = 220 - age"),
    ("pace", "pace = time / distance; speed = distance / time"),
    ("calories-burned-walk", "calories = 3.5 (MET) x kg x hours"),
    ("calories-burned-run", "calories = 1.05 x kg x km"),
    ("steps-distance", "distance = steps x stride length; calories is about 0.04 per step at 70 kg, scaled by weight"),
    ("due-date", "due date = first day of last period + 280 days"),
    ("ovulation", "ovulation = (LMP + cycle length) - 14 days; fertile window = ovulation - 5 days to ovulation + 1 day"),
    ("bac-widmark", "BAC % = (alcohol grams / (weight grams x r)) x 100 - 0.015 x hours, r = 0.68 men / 0.55 women"),
    ("sleep-cycles", "full sleep cycles of 90 minutes: 4, 5 or 6 cycles = 6 h, 7.5 h, 9 h, plus about 14 minutes to fall asleep"),
    ("macro-split", "protein grams = calories x protein % / 100 / 4; carbs the same / 4; fat grams = calories x fat % / 100 / 9"),
    ("protein-needs", "protein per day = weight (kg) x factor (g per kg)"),
    ("speed-from-distance-time", "speed = distance / time"),
    ("distance-from-speed-time", "distance = speed x time"),
    ("time-from-speed-distance", "time = distance / speed"),
    ("average-velocity", "average velocity = (x2 - x1) / (t2 - t1)"),
    ("acceleration", "a = (v - u) / t"),
    ("final-velocity", "v = u + a x t; distance s = u x t + 1/2 x a x t^2"),
    ("force", "F = m x a"),
    ("weight-from-mass", "weight = mass x g, g = 9.80665 m/s^2"),
    ("mass-from-weight", "mass = weight / g, g = 9.80665 m/s^2"),
    ("kinetic-energy", "KE = 1/2 x m x v^2"),
    ("potential-energy", "PE = m x g x h, g = 9.80665 m/s^2"),
    ("mechanical-energy", "E = 1/2 x m x v^2 + m x g x h"),
    ("power-physics", "P = work / time"),
    ("work", "work = force x distance"),
    ("density-physics", "density = mass / volume"),
    ("pressure", "pressure = force / area"),
    ("ohms-law", "I = V / R; P = V x I"),
    ("ohms-solve-i", "R = V / I; P = V x I"),
    ("ohms-solve-v", "V = I x R; P = V x I"),
    ("electrical-power", "P = V x I; energy (kWh) = P (W) x hours / 1000"),
    ("resistors-parallel", "1/R = 1/R1 + 1/R2; series total = R1 + R2"),
    ("projectile", "range = v^2 x sin(2 x angle) / g; max height = (v x sin(angle))^2 / (2g); flight time = 2 x v x sin(angle) / g"),
    ("pendulum", "T = 2 x pi x sqrt(L / g); frequency = 1 / T"),
    ("wave", "v = f x wavelength; period T = 1 / f"),
    ("torque", "torque = force x lever arm"),
    ("momentum", "p = m x v"),
    ("friction", "friction = coefficient x normal force; on flat ground the normal force = m x g"),
    ("hooke", "force = k x stretch; stored energy = 1/2 x k x stretch^2"),
    ("gravity-force", "F = G x m1 x m2 / r^2, G = 6.674e-11"),
    ("centripetal", "a = v^2 / r; F = m x v^2 / r"),
    ("free-fall", "v = g x t; distance = 1/2 x g x t^2"),
    ("moles-from-mass", "moles = mass (g) / molar mass (g/mol)"),
    ("mass-from-moles", "mass (g) = moles x molar mass (g/mol)"),
    ("molecules", "particles = moles x 6.02214076e23"),
    ("molarity", "molarity = moles / volume (L)"),
    ("molarity-from-mass", "molarity = (mass / molar mass) / volume (L)"),
    ("dilution", "M1 x V1 = M2 x V2, so M2 = M1 x V1 / V2"),
    ("serial-dilution", "step k concentration = start / factor^k; mode 1 factor = (transfer + diluent) / transfer"),
    ("dilution-volume", "M1 x V1 = M2 x V2, so V1 = M2 x V2 / M1"),
    ("ph", "pH = -log10[H+]; pOH = 14 - pH"),
    ("ph-inverse", "[H+] = 10^(-pH)"),
    ("poh", "pOH = -log10[OH-]; pH = 14 - pOH"),
    ("ideal-gas-pressure", "P = n x R x T / V (from PV = nRT)"),
    ("ideal-gas-volume", "V = n x R x T / P (from PV = nRT)"),
    ("ideal-gas-moles", "n = P x V / (R x T) (from PV = nRT)"),
    ("ideal-gas-temp", "T = P x V / (n x R) (from PV = nRT)"),
    ("boyle", "P2 = P1 x V1 / V2 (from P1 x V1 = P2 x V2)"),
    ("charles", "V2 = V1 x T2 / T1 (from V1 / T1 = V2 / T2)"),
    ("gay-lussac", "P2 = P1 x T2 / T1 (from P1 / T1 = P2 / T2)"),
    ("combined-gas", "P2 = P1 x V1 x T2 / (T1 x V2)"),
    ("percent-composition", "part % = part mass / total mass x 100"),
    ("solution-percent", "mass % = solute / solution x 100; ppm = mass % x 10,000"),
    ("empirical-ratio", "each mole amount / the smallest mole amount"),
    ("reaction-yield", "percent yield = actual yield / theoretical yield x 100"),
    ("half-life", "amount left = start x (1/2)^(time / half-life)"),
    ("square-footage-rect", "A = length x width"),
    ("square-footage-circle", "A = pi x (diameter / 2)^2"),
    ("square-footage-triangle", "A = base x height / 2"),
    ("concrete-slab", "volume = length x width x thickness / 12 (cu ft); cubic yards = cu ft / 27; bags = volume incl waste / bag yield"),
    ("concrete-column", "volume = pi x (d/2)^2 x depth x number of columns; bags as in the slab calculator"),
    ("concrete-stairs", "volume is about width x total rise x total run / 2 (a triangle prism)"),
    ("paint", "paintable area = 2 x (L + W) x height - 21 x doors - 15 x windows; gallons = area x coats / coverage"),
    ("tile", "tiles = whole tiles covering (area incl waste) / (tile length x tile width / 144); boxes = tiles / tiles per box, rounded up"),
    ("mulch", "volume (cu ft) = area x depth / 12; cubic yards = cu ft / 27; bags = cu ft / 2"),
    ("gravel", "volume (cu ft) = L x W x depth / 12; cubic yards = cu ft / 27; tons = cubic yards x density"),
    ("flooring", "boxes = area x (1 + waste/100) / coverage per box, rounded up"),
    ("fence", "posts = whole sections along the run + 1; pickets = run length x 12 / (picket width + gap)"),
    ("stairs", "steps = total rise / target riser, rounded; actual riser = total rise / steps; total run = steps x tread run; stringer is about sqrt(rise^2 + run^2)"),
    ("roofing", "roof area = footprint x sqrt(1 + (rise/12)^2) x (1 + waste/100); squares = area / 100; bundles = 3 x squares"),
    ("drywall", "sheets = area / sheet coverage, rounded up"),
    ("board-feet", "board feet = thickness (in) x width (in) x length (ft) / 12"),
    ("deck-boards", "boards = deck width / (board width + gap), rounded up; linear feet = boards x deck length"),
    ("brick", "bricks = wall area x 6.55 x (1 + waste/100); mortar bags is about bricks / 35"),
    ("wallpaper", "rolls = wall area / usable coverage per roll, rounded up"),
    ("insulation-bags", "volume (cu ft) = area x depth / 12; bags = volume / coverage per bag, rounded up"),
    ("asphalt-driveway", "volume (cu ft) = length x width x thickness / 12; tons = cu ft x 145 lb / 2000"),
    ("room-area-walls", "wall area = 2 x (L + W) x H; ceiling = L x W"),
    ("cubic-yards", "cubic yards = (L x W x depth / 12) / 27"),
    ("angle-iron-weight", "weight = (L x W x thickness in cubic feet) x 490 lb"),
    ("concrete-blocks", "blocks = wall area x 1.125 x (1 + waste/100), rounded up"),
    ("post-holes-concrete", "hole volume = pi x (d/2)^2 x depth - post volume; bags = volume / bag yield"),
    ("convert-length", "result = value x factor(from) / factor(to)"),
    ("convert-mass", "result = value x factor(from) / factor(to)"),
    ("convert-area", "result = value x factor(from) / factor(to)"),
    ("convert-volume", "result = value x factor(from) / factor(to)"),
    ("convert-speed", "result = value x factor(from) / factor(to)"),
    ("convert-time", "result = value x factor(from) / factor(to)"),
    ("convert-data", "result = value x factor(from) / factor(to)"),
    ("convert-pressure", "result = value x factor(from) / factor(to)"),
    ("convert-energy", "result = value x factor(from) / factor(to)"),
    ("convert-power-units", "result = value x factor(from) / factor(to)"),
    ("convert-force-units", "result = value x factor(from) / factor(to)"),
    ("convert-torque-units", "result = value x factor(from) / factor(to)"),
    ("convert-density-units", "result = value x factor(from) / factor(to)"),
    ("convert-frequency", "result = value x factor(from) / factor(to)"),
    ("convert-flow", "result = value x factor(from) / factor(to)"),
    ("convert-illuminance", "result = value x factor(from) / factor(to)"),
    ("convert-temperature", "F = C x 9/5 + 32; K = C + 273.15"),
    ("convert-fuel-economy", "L/100km = 235.215 / mpg (US); UK mpg = 282.481 / (L/100km)"),
    ("compound-density", "result = value x factor(from) / factor(to)"),
    ("compound-fuel-consumption", "L/100km = 235.214583 / mpg (US) = 282.480936 / mpg (UK); km/L = 100 / (L/100km)"),
    ("compound-data-rate", "result = value x factor(from) / factor(to), with 1 byte = 8 bits"),
    ("compound-concentration", "result = value x factor(from) / factor(to)"),
    ("compound-energy-density", "result = value x factor(from) / factor(to)"),
    ("grams-cups-flour", "cups = grams / 125; grams = cups x 125"),
    ("grams-cups-sugar", "cups = grams / 200; grams = cups x 200"),
    ("grams-cups-brown-sugar", "cups = grams / 220; grams = cups x 220"),
    ("grams-cups-butter", "cups = grams / 227; grams = cups x 227"),
    ("grams-cups-cocoa", "cups = grams / 100; grams = cups x 100"),
    ("grams-cups-rice", "cups = grams / 185; grams = cups x 185"),
    ("grams-cups-oats", "cups = grams / 90; grams = cups x 90"),
    ("grams-cups-honey", "cups = grams / 340; grams = cups x 340"),
    ("grams-cups-milk", "cups = grams / 245; grams = cups x 245"),
    ("grams-cups-water", "cups = grams / 236.6; grams = cups x 236.6"),
    ("grams-cups-oil", "cups = grams / 218; grams = cups x 218"),
    ("grams-cups-powdered-sugar", "cups = grams / 120; grams = cups x 120"),
    ("shoe-size-men", "a published size chart (nearest listed size)"),
    ("mean", "mean = sum of values / count of values"),
    ("median", "the middle value when sorted (or the average of the two middle values)"),
    ("mode-range", "mode = most frequent value; range = max - min"),
    ("sum-product", "sum = all values added; product = all values multiplied"),
    ("std-population", "SD = sqrt(average of (value - mean)^2)"),
    ("std-sample", "SD = sqrt(sum of (value - mean)^2 / (n - 1))"),
    ("geometric-mean", "geometric mean = (product of values)^(1/n)"),
    ("harmonic-mean", "harmonic mean = n / (sum of 1/value)"),
    ("z-score", "z = (value - mean) / standard deviation"),
    ("percentile-from-z", "the standard normal curve areas P(Z < z), P(Z > z) and the two-sided p-value"),
    ("p-value-z", "two-sided p = 2 x (1 - Phi(|z|))"),
    ("probability", "probability = favourable outcomes / total outcomes"),
    ("combinations", "nCr = n! / (r! x (n - r)!)"),
    ("permutations", "nPr = n! / (n - r)!"),
    ("confidence-interval-mean", "CI = mean +- z* x (SD / sqrt(n))"),
    ("margin-of-error", "MoE = z* x sqrt(p x (1 - p) / n)"),
    ("sample-size-proportion", "n = z*^2 x p x (1 - p) / MoE^2, rounded up"),
    ("expected-value", "EV = sum of (value x probability) over the pairs"),
    ("dice-probability", "chance of a total = pairs (of 36) making it / 36; expected total = 7"),
    ("percentile-rank", "percentile rank = (values below + 0.5 x values equal) / total x 100"),
    ("weighted-average", "result = sum(value x weight) / sum(weights)"),
    ("linear-regression", "m = (n x sum(xy) - sum(x) x sum(y)) / (n x sum(x^2) - (sum(x))^2); b = (sum(y) - m x sum(x)) / n; calibration x = (y - b) / m"),
    ("replicates-summary", "mean = sum / n; SD = sqrt(sum of (value - mean)^2 / (n - 1)); SEM = SD / sqrt(n); 95% CI = mean +- t(n - 1) x SEM"),
    ("outlier-check", "IQR = Q3 - Q1; outliers lie below Q1 - 1.5 x IQR or above Q3 + 1.5 x IQR"),
    ("resistor-colour-code", "value = (the digit bands read as one number) x 10^(multiplier band); the last band is the tolerance"),
    ("resistors-series-parallel", "series R = R1 + R2 + ...; parallel 1/R = 1/R1 + 1/R2 + ..."),
    ("voltage-divider", "Vout = Vin x R2 / (R1 + R2); I = Vin / (R1 + R2); P = I^2 x R in each resistor"),
    ("tolerance-stack-up", "total = sum of nominals; worst case = total +- sum of tolerances; RSS = total +- sqrt(sum of tolerance^2)"),
    ("safety-factor", "FoS = strength / applied stress; MoS = FoS - 1; applied stress = load / area in mode 2"),
    ("bolt-torque", "T = K x D x F, with D in metres"),
    ("beam-bending", "M = P x L / 4; I = b x h^3 / 12 (rectangle) or pi x d^4 / 64 (circle); max stress = M x c / I; deflection = P x L^3 / (48 x E x I)"),
    ("days-between-dates", "the real calendar day count from the start date to the end date"),
    ("date-plus-days", "the start date moved by N days on the real calendar"),
    ("age", "whole years from the birth date to the end date, plus the total days lived"),
    ("weekday", "day count from 1970-01-01 (a Thursday), taken mod 7"),
    ("time-duration", "minutes from the start clock time to the end clock time (wrapping past midnight)"),
    ("countdown-days", "the real calendar day count from the from date to the target date"),
    ("business-days", "the count of Monday-to-Friday days from the start date to the end date"),
    ("dog-age", "year 1 = 15 human years; year 2 = 24 total; then +5 per year (+6 for large dogs)"),
    ("cat-age", "year 1 = 15 human years; year 2 = 24 total; then +4 per year"),
    ("fuel-cost-trip", "fuel used = distance / economy; cost = fuel used x price per unit"),
    ("mpg-trip", "economy = distance driven / fuel used"),
    ("electricity-cost", "kWh = watts x hours / 1000; cost = kWh x rate"),
    ("download-time", "time = file size (MB) x 8 / speed (Mbps)"),
    ("aspect-ratio", "ratio = width : height divided by their GCD; new height = new width x height / width"),
    ("screen-ppi", "PPI = sqrt(w^2 + h^2) / diagonal"),
    ("gpa", "GPA = sum(points x credits) / sum(credits)"),
    ("test-grade", "grade % = correct / total x 100"),
    ("attendance", "attendance % = attended / total x 100"),
    ("age-in-days", "the real calendar day count from the birth date to the end date, also in weeks and hours"),
    ("split-bill", "total = amount x (1 + tax/100) + amount x tip/100; each pays = total / people"),
    ("unit-price", "unit price = price / quantity, for each product"),
    ("overtime-pay", "pay = rate x regular hours + rate x 1.5 x overtime hours"),
    ("race-pace", "finish time = pace x distance"),
    ("race-predictor-riegel", "T2 = T1 x (D2 / D1)^1.06"),
    ("swim-pace", "pace per 100 m = total time / (distance / 100)"),
    ("cycling-speed", "speed = distance / time"),
    ("vo2max-estimate", "VO2max is about (distance (m) - 504.9) / 44.73"),
    ("one-rep-max", "1RM = weight x (1 + reps / 30)"),
    ("marathon-splits", "per-km split = goal time / distance"),
    ("recipe-scale", "scaled amount = original amount x (servings wanted / recipe servings)"),
    ("bakers-percent", "% = ingredient / flour x 100"),
    ("oven-temp", "F = C x 9/5 + 32; Gas Mark is about (F - 250) / 25"),
    ("alcohol-units", "UK units = volume (ml) x ABV % / 1000; US standard drinks use 14 g of pure alcohol"),
    ("coffee-water-ratio", "water = coffee x ratio; coffee = water / ratio"),
    ("pizza-dough", "flour = total dough / (1 + hydration/100 + 0.025); water = flour x hydration/100; salt = flour x 2%; yeast = flour x 0.5%"),
    ("meat-cooking-time", "minutes = weight (lb) x minutes per lb"),
    ("carbon-driving", "litres used = distance / economy; CO2 = litres x factor"),
    ("solar-output", "daily energy is about panel watts x peak sun hours x 0.75 / 1000 (kWh)"),
    ("rainwater", "litres collected is about roof area x rainfall x 0.8"),
    ("pool-volume", "volume = L x W x average depth"),
    ("pool-chlorine", "dose scales with pool gallons and the wanted ppm rise (2 oz of 65% calcium hypochlorite per 1 ppm per 10,000 gallons)"),
    ("tree-oxygen-co2", "CO2 absorbed is about 21 kg x trees x years"),
    ("lighting-cost-year", "yearly kWh = watts x hours per day x 365 / 1000 x number of bulbs; cost = kWh x rate"),
    ("working-hours-pay", "paid hours = (end - start) - unpaid break (crossing midnight if end < start)"),
    ("gross-profit-business", "gross profit = revenue - cost of goods sold; gross margin = gross profit / revenue x 100"),
    ("price-per-unit-other", "per item = case price / items per case; cases needed = wanted count / items per case, rounded up"),
    ("shrinkage", "shrinkage % = (recorded stock - actual stock) / recorded stock x 100"),
    ("circumference", "C = pi x d"),
];

/// The formula for a calculator (item #19); "" only if a future
/// calculator forgets its row — the registry-wide test forbids it.
pub fn formula_for(c: &crate::Calc) -> &'static str {
    for (id, f) in FORMULAS {
        if *id == c.id {
            return f;
        }
    }
    ""
}

/// (calculator id, [(symbol, plain-words meaning), ...]) — pairs
/// in INPUT ORDER (see the note above FORMULAS).
pub static GLOSSARY: &[(&str, &[(&str, &str)])] = &[
    (
        "expression-calculator",
        &[
            ("x", "the variable (used by the plot)"),
            ("pi", "pi, about 3.14159"),
            ("e", "Euler's number, about 2.71828"),
        ],
    ),
    (
        "percent-of",
        &[
            ("X", "the percent"),
            ("Y", "the number the percent is taken of"),
        ],
    ),
    ("percent-is", &[("X", "the part"), ("Y", "the whole")]),
    (
        "percent-change",
        &[("old", "the starting value"), ("new", "the ending value")],
    ),
    (
        "percent-difference",
        &[("A", "the first value"), ("B", "the second value")],
    ),
    (
        "reverse-percent",
        &[
            ("X", "the percent change that was applied"),
            ("Y", "the final value after the change"),
        ],
    ),
    (
        "percent-off-result",
        &[
            ("X", "the percent change to apply"),
            ("Y", "the starting value"),
        ],
    ),
    (
        "fraction-simplify",
        &[
            ("p", "the numerator (top number)"),
            ("q", "the denominator (bottom number)"),
            ("GCD", "greatest common divisor of p and q"),
        ],
    ),
    (
        "fraction-add",
        &[
            ("a", "numerator of the first fraction"),
            ("b", "denominator of the first fraction"),
            ("c", "numerator of the second fraction"),
            ("d", "denominator of the second fraction"),
        ],
    ),
    (
        "fraction-subtract",
        &[
            ("a", "numerator of the first fraction"),
            ("b", "denominator of the first fraction"),
            ("c", "numerator of the second fraction"),
            ("d", "denominator of the second fraction"),
        ],
    ),
    (
        "fraction-multiply",
        &[
            ("a", "numerator of the first fraction"),
            ("b", "denominator of the first fraction"),
            ("c", "numerator of the second fraction"),
            ("d", "denominator of the second fraction"),
        ],
    ),
    (
        "fraction-divide",
        &[
            ("a", "numerator of the first fraction"),
            ("b", "denominator of the first fraction"),
            ("c", "numerator of the second fraction"),
            ("d", "denominator of the second fraction"),
        ],
    ),
    (
        "fraction-to-decimal",
        &[
            ("whole", "the whole-number part (0 if none)"),
            ("numerator", "the top number of the fraction part"),
            ("denominator", "the bottom number of the fraction part"),
        ],
    ),
    (
        "decimal-to-fraction",
        &[("decimal", "the decimal number to write as a fraction")],
    ),
    (
        "exponent",
        &[
            ("base", "the number being multiplied by itself"),
            ("exponent", "how many times the base is multiplied"),
        ],
    ),
    ("square-cube", &[("x", "the number")]),
    ("square-root", &[("x", "the number")]),
    (
        "nth-root",
        &[
            ("x", "the number"),
            ("n", "which root (2 = square root, 3 = cube root)"),
        ],
    ),
    (
        "log",
        &[("x", "the number"), ("base", "the logarithm base")],
    ),
    ("ln-log10", &[("x", "the number")]),
    ("antilog", &[("x", "the exponent")]),
    ("factorial", &[("x", "the whole number")]),
    (
        "scientific-notation",
        &[
            ("a", "the coefficient, between 1 and 10"),
            ("b", "the power of ten"),
        ],
    ),
    (
        "from-scientific",
        &[
            ("a", "the coefficient, between 1 and 10"),
            ("b", "the power of ten"),
        ],
    ),
    (
        "sig-figs",
        &[
            ("number", "the number to round"),
            ("figures", "how many significant figures to keep"),
        ],
    ),
    (
        "round-decimals",
        &[
            ("number", "the number to round"),
            ("places", "how many decimal places to keep"),
        ],
    ),
    (
        "gcd",
        &[
            ("A", "the first whole number"),
            ("B", "the second whole number"),
        ],
    ),
    (
        "lcm",
        &[
            ("A", "the first whole number"),
            ("B", "the second whole number"),
        ],
    ),
    ("prime-check", &[("n", "the whole number to test")]),
    (
        "ratio",
        &[
            ("A", "the first part of the ratio"),
            ("B", "the second part of the ratio"),
            ("target", "the value A should scale to"),
        ],
    ),
    (
        "proportion",
        &[
            ("A", "the first known value"),
            ("B", "the second known value"),
            ("C", "the third known value"),
            ("x", "the unknown being solved for"),
        ],
    ),
    (
        "rule-of-three",
        &[
            ("A", "the first quantity"),
            ("B", "the quantity A corresponds to"),
            ("C", "the new quantity"),
            ("x", "what C corresponds to"),
        ],
    ),
    (
        "quadratic",
        &[
            ("a", "the x^2 coefficient"),
            ("b", "the x coefficient"),
            ("c", "the constant term"),
            ("x", "the unknown"),
        ],
    ),
    (
        "linear-equation",
        &[
            ("a", "the x coefficient"),
            ("b", "the added constant"),
            ("c", "the right-hand side"),
            ("x", "the unknown"),
        ],
    ),
    (
        "slope",
        &[
            ("x1", "x of the first point"),
            ("y1", "y of the first point"),
            ("x2", "x of the second point"),
            ("y2", "y of the second point"),
            ("m", "the slope"),
        ],
    ),
    (
        "distance-points",
        &[
            ("x1", "x of the first point"),
            ("y1", "y of the first point"),
            ("x2", "x of the second point"),
            ("y2", "y of the second point"),
            ("d", "the distance"),
        ],
    ),
    (
        "pythagorean",
        &[
            ("a", "one leg of the right triangle"),
            ("b", "the other leg"),
            ("c", "the hypotenuse (longest side)"),
        ],
    ),
    (
        "pythagorean-leg",
        &[
            ("c", "the hypotenuse (longest side)"),
            ("other leg", "the leg you know"),
        ],
    ),
    (
        "arithmetic-sequence",
        &[
            ("a1", "the first term"),
            ("d", "the common difference (what each term adds)"),
            ("n", "the term number"),
        ],
    ),
    (
        "geometric-sequence",
        &[
            ("a1", "the first term"),
            ("r", "the common ratio (what each term multiplies by)"),
            ("n", "the term number"),
        ],
    ),
    (
        "circle",
        &[
            ("r", "the radius — centre to edge"),
            ("A", "the area"),
            ("C", "the circumference — distance around"),
            ("d", "the diameter — right across, twice the radius"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "circle-from-area",
        &[
            ("A", "the area"),
            ("r", "the radius — centre to edge"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "rectangle",
        &[
            ("l", "the length"),
            ("w", "the width"),
            ("A", "the area"),
            ("P", "the perimeter"),
        ],
    ),
    (
        "square-shape",
        &[
            ("s", "the side length"),
            ("A", "the area"),
            ("P", "the perimeter"),
        ],
    ),
    (
        "triangle-area",
        &[
            ("base", "the base of the triangle"),
            ("height", "the vertical height"),
            ("A", "the area"),
        ],
    ),
    (
        "triangle-heron",
        &[
            ("a", "the first side"),
            ("b", "the second side"),
            ("c", "the third side"),
            ("s", "half the perimeter"),
            ("A", "the area"),
        ],
    ),
    (
        "trapezoid",
        &[
            ("a", "one parallel side"),
            ("b", "the other parallel side"),
            ("height", "the distance between the parallel sides"),
            ("leg1", "the first slanted side"),
            ("leg2", "the second slanted side"),
        ],
    ),
    (
        "parallelogram",
        &[
            ("base", "the base"),
            ("height", "the vertical height"),
            ("side", "the slanted side"),
        ],
    ),
    (
        "rhombus",
        &[("d1", "the first diagonal"), ("d2", "the second diagonal")],
    ),
    (
        "ellipse",
        &[
            ("a", "the longer semi-axis (half the widest width)"),
            ("b", "the shorter semi-axis"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "regular-polygon",
        &[
            ("n", "the number of sides"),
            ("s", "the side length"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "sector",
        &[
            ("r", "the radius"),
            ("angle", "the sector angle in degrees"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "cube-box",
        &[
            ("l", "the length"),
            ("w", "the width"),
            ("h", "the height"),
            ("V", "the volume"),
            ("S", "the surface area"),
        ],
    ),
    (
        "cylinder",
        &[
            ("r", "the radius of the circular end"),
            ("h", "the height"),
            ("V", "the volume"),
            ("S", "the surface area"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "sphere",
        &[
            ("r", "the radius"),
            ("V", "the volume"),
            ("S", "the surface area"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "cone",
        &[
            ("r", "the radius of the base"),
            ("h", "the vertical height"),
            ("slant", "the slant height, up the sloping side"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "pyramid",
        &[
            ("base", "the side of the square base"),
            ("h", "the vertical height"),
            ("slant", "the slant height of a triangular face"),
        ],
    ),
    (
        "frustum",
        &[
            ("R", "the bottom radius"),
            ("r", "the top radius"),
            ("h", "the height"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    ("trig-deg", &[("angle", "the angle in degrees")]),
    (
        "trig-inverse",
        &[(
            "ratio",
            "the trig ratio (sin/cos results lie between -1 and 1)",
        )],
    ),
    (
        "angle-convert",
        &[
            ("value", "the angle"),
            (
                "unit code",
                "which unit the value is in: 0 deg, 1 rad, 2 grad, 3 turn",
            ),
        ],
    ),
    (
        "average",
        &[("A", "the first number"), ("B", "the second number")],
    ),
    (
        "compound-interest",
        &[
            ("P", "the principal — the starting amount"),
            (
                "r",
                "the annual interest rate as a decimal in the formula (the field takes %)",
            ),
            ("t", "the time in years"),
        ],
    ),
    (
        "compound-interest-monthly-deposit",
        &[
            ("P", "the principal — the starting amount"),
            ("r", "the annual interest rate (the field takes %)"),
            ("t", "the time in years"),
            ("D", "the monthly deposit"),
        ],
    ),
    (
        "simple-interest",
        &[
            ("P", "the principal — the starting amount"),
            ("r", "the annual interest rate (the field takes %)"),
            ("t", "the time in years"),
        ],
    ),
    (
        "loan-payment",
        &[
            ("P", "the loan amount (principal)"),
            ("rate", "the annual interest rate %"),
            ("years", "the loan term in years"),
        ],
    ),
    (
        "mortgage-payment",
        &[
            ("price", "the home price"),
            ("down payment", "the amount paid up front"),
            ("rate", "the annual interest rate %"),
            ("years", "the loan term in years"),
            ("tax", "the yearly property tax"),
            ("insurance", "the yearly home insurance"),
            ("PMI", "the monthly PMI, if any"),
        ],
    ),
    (
        "auto-loan",
        &[
            ("price", "the car price"),
            ("down payment", "the amount paid up front"),
            ("trade-in", "the trade-in value"),
            ("tax %", "the sales tax rate"),
            ("rate", "the annual interest rate %"),
            ("years", "the loan term in years"),
        ],
    ),
    (
        "loan-affordability",
        &[
            ("payment", "the monthly payment you can afford"),
            ("rate", "the annual interest rate %"),
            ("years", "the loan term in years"),
            ("P", "the loan amount you can borrow"),
        ],
    ),
    (
        "present-value",
        &[
            ("FV", "the future value"),
            ("rate", "the annual interest rate %"),
            ("t", "the time in years"),
            ("PV", "the present value"),
        ],
    ),
    (
        "future-value",
        &[
            ("PV", "the present value"),
            ("rate", "the annual interest rate %"),
            ("t", "the time in years"),
            ("FV", "the future value"),
        ],
    ),
    (
        "annuity-pv",
        &[
            ("payment", "the payment made each year"),
            ("rate", "the annual interest rate %"),
            ("t", "the number of years"),
            ("PV", "the present value of all the payments"),
        ],
    ),
    (
        "annuity-fv",
        &[
            ("payment", "the payment made each year"),
            ("rate", "the annual interest rate %"),
            ("t", "the number of years"),
            ("FV", "the future value of all the payments"),
        ],
    ),
    (
        "cagr",
        &[
            ("start", "the starting value"),
            ("end", "the ending value"),
            ("years", "the number of years between them"),
        ],
    ),
    (
        "roi",
        &[("cost", "the amount invested"), ("gain", "the final value")],
    ),
    (
        "profit-margin",
        &[
            ("revenue", "the total selling price (money in)"),
            ("cost", "what the goods cost"),
        ],
    ),
    (
        "price-from-margin",
        &[
            ("cost", "what the item costs"),
            (
                "margin %",
                "the margin you want, as a percent of the selling price",
            ),
        ],
    ),
    (
        "price-from-markup",
        &[
            ("cost", "what the item costs"),
            ("markup %", "the markup you want, as a percent of the cost"),
        ],
    ),
    (
        "discount",
        &[
            ("price", "the original price"),
            ("discount %", "the percent off"),
        ],
    ),
    (
        "double-discount",
        &[
            ("price", "the original price"),
            ("d1", "the first discount %"),
            ("d2", "the second discount %"),
        ],
    ),
    (
        "sales-tax-add",
        &[("price", "the price before tax"), ("tax %", "the tax rate")],
    ),
    (
        "sales-tax-extract",
        &[
            ("total", "the total including tax"),
            ("tax %", "the tax rate"),
        ],
    ),
    (
        "tip",
        &[("bill", "the bill amount"), ("tip %", "the tip percent")],
    ),
    (
        "tip-split",
        &[
            ("bill", "the bill amount"),
            ("tip %", "the tip percent"),
            ("people", "how many people split the bill"),
        ],
    ),
    (
        "salary-hourly",
        &[
            ("salary", "the yearly salary"),
            (
                "hours",
                "the hours worked per year (2080 = 40 h x 52 weeks)",
            ),
        ],
    ),
    (
        "hourly-salary",
        &[
            ("rate", "the hourly pay rate"),
            ("regular hours", "regular hours per week"),
            ("overtime hours", "overtime hours per week, paid at 1.5x"),
        ],
    ),
    (
        "savings-goal",
        &[
            ("goal", "the amount you want saved"),
            ("rate", "the annual interest rate %"),
            ("months", "the months you have to save"),
        ],
    ),
    (
        "break-even",
        &[
            ("fixed costs", "costs that do not change with sales"),
            ("price per unit", "the selling price of one unit"),
            ("variable cost per unit", "the cost of making one unit"),
        ],
    ),
    (
        "effective-rate",
        &[
            ("nominal", "the nominal annual rate %"),
            ("n", "how many times a year interest compounds"),
        ],
    ),
    ("rule-of-72", &[("rate %", "the annual interest rate")]),
    (
        "credit-card-payoff",
        &[
            ("balance", "the amount owed"),
            ("APR", "the annual percentage rate %"),
            ("payment", "the fixed monthly payment"),
        ],
    ),
    (
        "inflation",
        &[
            ("amount", "the amount today"),
            ("inflation %", "the yearly inflation rate"),
            ("years", "the number of years"),
        ],
    ),
    (
        "net-margin",
        &[
            ("revenue", "the total money in"),
            ("costs", "the total costs"),
        ],
    ),
    (
        "payback-period",
        &[
            ("investment", "the initial investment"),
            ("inflow", "the cash coming in each year"),
        ],
    ),
    (
        "depreciation-straight",
        &[
            ("cost", "what the asset cost"),
            ("salvage value", "what it will still be worth at the end"),
            ("useful life", "the useful life in years"),
            ("Y", "the years from now the book value is shown for"),
        ],
    ),
    (
        "rent-vs-buy-simple",
        &[
            ("rent", "the monthly rent"),
            ("price", "the home price"),
            ("rate", "the mortgage rate %"),
            ("years", "the years compared"),
            ("tax+insurance", "the yearly tax and insurance"),
        ],
    ),
    (
        "bmi",
        &[
            ("weight", "body weight in kilograms"),
            (
                "height",
                "height in centimetres (converted to metres in the formula)",
            ),
            ("BMI", "body mass index"),
        ],
    ),
    (
        "bmi-imperial",
        &[
            ("weight", "body weight in pounds"),
            ("height", "height in inches"),
            ("703", "the imperial-units constant in the BMI formula"),
        ],
    ),
    (
        "bmr-male",
        &[
            ("kg", "body weight in kilograms"),
            ("cm", "height in centimetres"),
            ("age", "age in years"),
            (
                "BMR",
                "basal metabolic rate — calories burned at complete rest",
            ),
        ],
    ),
    (
        "bmr-female",
        &[
            ("kg", "body weight in kilograms"),
            ("cm", "height in centimetres"),
            ("age", "age in years"),
            (
                "BMR",
                "basal metabolic rate — calories burned at complete rest",
            ),
        ],
    ),
    (
        "bmr-harris-male",
        &[
            ("kg", "body weight in kilograms"),
            ("cm", "height in centimetres"),
            ("age", "age in years"),
            (
                "BMR",
                "basal metabolic rate — calories burned at complete rest",
            ),
        ],
    ),
    (
        "bmr-harris-female",
        &[
            ("kg", "body weight in kilograms"),
            ("cm", "height in centimetres"),
            ("age", "age in years"),
            (
                "BMR",
                "basal metabolic rate — calories burned at complete rest",
            ),
        ],
    ),
    (
        "tdee",
        &[
            ("BMR", "basal metabolic rate in calories per day"),
            (
                "activity factor",
                "how active you are: 1.2 sedentary to 1.9 athlete",
            ),
            (
                "TDEE",
                "total daily energy expenditure — calories burned per day",
            ),
        ],
    ),
    (
        "calorie-target",
        &[
            ("TDEE", "total daily energy expenditure (cal/day)"),
            ("kg change", "the weight change wanted (+ gain, - lose)"),
            ("days", "the days to do it in"),
            ("7700", "the calories in about 1 kg of body fat"),
        ],
    ),
    (
        "body-fat-navy-male",
        &[
            ("waist", "waist in cm, at the navel"),
            ("neck", "neck in cm"),
            ("height", "height in cm"),
        ],
    ),
    (
        "body-fat-navy-female",
        &[
            ("waist", "waist in cm"),
            ("hip", "hip in cm"),
            ("neck", "neck in cm"),
            ("height", "height in cm"),
        ],
    ),
    (
        "ideal-weight-devine",
        &[
            ("sex code", "1 = male, 2 = female"),
            ("height", "height in centimetres"),
        ],
    ),
    ("water-intake", &[("weight", "body weight in kilograms")]),
    (
        "target-heart-rate",
        &[
            ("age", "age in years"),
            ("max HR", "maximum heart rate, about 220 - age"),
        ],
    ),
    (
        "karvonen",
        &[
            ("age", "age in years"),
            ("resting HR", "resting heart rate in beats per minute"),
            (
                "intensity %",
                "how hard, as a percent of the heart-rate reserve",
            ),
        ],
    ),
    (
        "pace",
        &[
            ("distance", "distance in kilometres"),
            ("time", "time in minutes"),
        ],
    ),
    (
        "calories-burned-walk",
        &[
            ("weight", "body weight in kilograms"),
            ("minutes", "minutes walked"),
            ("MET", "metabolic equivalent — 3.5 for brisk walking"),
        ],
    ),
    (
        "calories-burned-run",
        &[
            ("weight", "body weight in kilograms"),
            ("distance", "distance run in kilometres"),
        ],
    ),
    (
        "steps-distance",
        &[
            ("steps", "the step count"),
            ("weight", "body weight in kilograms"),
            ("stride", "stride length in metres (0.762 is average)"),
        ],
    ),
    (
        "due-date",
        &[
            ("LMP", "the first day of the last period"),
            (
                "280 days",
                "the standard pregnancy length in Naegele's rule",
            ),
        ],
    ),
    (
        "ovulation",
        &[
            ("LMP", "the first day of the last period"),
            ("cycle length", "the usual cycle length in days"),
        ],
    ),
    (
        "bac-widmark",
        &[
            ("alcohol", "pure alcohol in grams"),
            ("weight", "body weight in kilograms"),
            ("sex code", "1 = male, 2 = female"),
            ("hours", "hours since drinking started"),
            ("r", "the Widmark factor: 0.68 men, 0.55 women"),
        ],
    ),
    (
        "sleep-cycles",
        &[
            ("hours", "hours until you must wake up"),
            ("90 minutes", "the length of one full sleep cycle"),
        ],
    ),
    (
        "macro-split",
        &[
            ("calories", "daily calories"),
            ("protein %", "percent of calories from protein"),
            ("carbs %", "percent of calories from carbohydrates"),
            ("fat %", "percent of calories from fat"),
        ],
    ),
    (
        "protein-needs",
        &[
            ("weight", "body weight in kilograms"),
            ("factor", "protein grams per kilogram of body weight"),
        ],
    ),
    (
        "speed-from-distance-time",
        &[
            ("distance", "the distance travelled"),
            ("time", "the time taken"),
        ],
    ),
    (
        "distance-from-speed-time",
        &[("speed", "the speed"), ("time", "the time travelled")],
    ),
    (
        "time-from-speed-distance",
        &[
            ("distance", "the distance travelled"),
            ("speed", "the speed"),
        ],
    ),
    (
        "average-velocity",
        &[
            ("x1", "the starting position"),
            ("x2", "the ending position"),
            ("t1", "the starting time"),
            ("t2", "the ending time"),
        ],
    ),
    (
        "acceleration",
        &[
            ("u", "the starting speed"),
            ("v", "the final speed"),
            ("t", "the time taken"),
            ("a", "the acceleration"),
        ],
    ),
    (
        "final-velocity",
        &[
            ("u", "the starting speed"),
            ("a", "the acceleration"),
            ("t", "the time"),
            ("v", "the final speed"),
        ],
    ),
    (
        "force",
        &[
            ("m", "the mass in kilograms"),
            ("a", "the acceleration in m/s^2"),
            ("F", "the force in newtons"),
        ],
    ),
    (
        "weight-from-mass",
        &[
            ("mass", "the mass in kilograms"),
            ("g", "gravity, 9.80665 m/s^2 on Earth"),
        ],
    ),
    (
        "mass-from-weight",
        &[
            ("weight", "the weight (force) in newtons"),
            ("g", "gravity, 9.80665 m/s^2 on Earth"),
        ],
    ),
    (
        "kinetic-energy",
        &[
            ("m", "the mass in kilograms"),
            ("v", "the speed in m/s"),
            ("KE", "kinetic energy in joules"),
        ],
    ),
    (
        "potential-energy",
        &[
            ("m", "the mass in kilograms"),
            ("h", "the height in metres"),
            ("g", "gravity, 9.80665 m/s^2"),
        ],
    ),
    (
        "mechanical-energy",
        &[
            ("m", "the mass in kilograms"),
            ("v", "the speed in m/s"),
            ("h", "the height in metres"),
        ],
    ),
    (
        "power-physics",
        &[
            ("work", "the work done in joules"),
            ("time", "the time taken in seconds"),
            ("P", "the power in watts"),
        ],
    ),
    (
        "work",
        &[
            ("force", "the force in newtons"),
            ("distance", "the distance moved in metres"),
        ],
    ),
    (
        "density-physics",
        &[("mass", "the mass"), ("volume", "the volume")],
    ),
    (
        "pressure",
        &[
            ("force", "the force in newtons"),
            ("area", "the area in square metres"),
        ],
    ),
    (
        "ohms-law",
        &[
            ("V", "the voltage in volts"),
            ("R", "the resistance in ohms"),
            ("I", "the current in amps"),
            ("P", "the power in watts"),
        ],
    ),
    (
        "ohms-solve-i",
        &[
            ("V", "the voltage in volts"),
            ("I", "the current in amps"),
            ("R", "the resistance in ohms"),
        ],
    ),
    (
        "ohms-solve-v",
        &[
            ("I", "the current in amps"),
            ("R", "the resistance in ohms"),
            ("V", "the voltage in volts"),
        ],
    ),
    (
        "electrical-power",
        &[
            ("V", "the voltage in volts"),
            ("I", "the current in amps"),
            ("hours", "the hours used"),
        ],
    ),
    (
        "resistors-parallel",
        &[
            ("R1", "the first resistor in ohms"),
            ("R2", "the second resistor in ohms"),
            ("R", "the combined resistance"),
        ],
    ),
    (
        "projectile",
        &[
            ("v", "the launch speed in m/s"),
            ("angle", "the launch angle in degrees"),
            ("g", "gravity, 9.80665 m/s^2"),
        ],
    ),
    (
        "pendulum",
        &[
            ("L", "the pendulum length in metres"),
            ("T", "the period — seconds for one full swing"),
            ("g", "gravity, 9.80665 m/s^2"),
        ],
    ),
    (
        "wave",
        &[
            ("f", "the frequency in hertz"),
            ("wavelength", "the wavelength in metres"),
            ("v", "the wave speed"),
            ("T", "the period, seconds per wave"),
        ],
    ),
    (
        "torque",
        &[
            ("force", "the force in newtons"),
            ("lever arm", "the distance from the pivot in metres"),
        ],
    ),
    (
        "momentum",
        &[
            ("m", "the mass in kilograms"),
            ("v", "the velocity in m/s"),
            ("p", "the momentum in kg m/s"),
        ],
    ),
    (
        "friction",
        &[
            ("coefficient", "the coefficient of friction (no unit)"),
            ("mass", "the mass in kilograms"),
        ],
    ),
    (
        "hooke",
        &[
            ("k", "the spring constant in N/m — how stiff the spring is"),
            ("stretch", "how far the spring is stretched, in metres"),
        ],
    ),
    (
        "gravity-force",
        &[
            ("m1", "the first mass in kilograms"),
            ("m2", "the second mass in kilograms"),
            ("r", "the distance between their centres in metres"),
            ("G", "the gravitational constant, 6.674e-11"),
        ],
    ),
    (
        "centripetal",
        &[
            ("m", "the mass in kilograms"),
            ("v", "the speed in m/s"),
            ("r", "the radius of the circle in metres"),
            ("a", "the centripetal acceleration"),
            ("F", "the centripetal force"),
        ],
    ),
    (
        "free-fall",
        &[
            ("t", "the time falling in seconds"),
            ("g", "gravity, 9.80665 m/s^2"),
        ],
    ),
    (
        "moles-from-mass",
        &[
            ("mass", "the mass in grams"),
            ("molar mass", "the molar mass in g/mol"),
        ],
    ),
    (
        "mass-from-moles",
        &[
            ("moles", "the amount in moles"),
            ("molar mass", "the molar mass in g/mol"),
        ],
    ),
    (
        "molecules",
        &[
            ("moles", "the amount in moles"),
            ("6.02214076e23", "Avogadro's number — particles per mole"),
        ],
    ),
    (
        "molarity",
        &[
            ("moles", "the amount of solute in moles"),
            ("volume", "the solution volume in litres"),
        ],
    ),
    (
        "molarity-from-mass",
        &[
            ("mass", "the solute mass in grams"),
            ("molar mass", "the molar mass in g/mol"),
            ("volume", "the solution volume in litres"),
        ],
    ),
    (
        "dilution",
        &[
            ("M1", "the stock concentration"),
            ("V1", "the stock volume"),
            ("V2", "the final volume"),
            ("M2", "the final concentration"),
        ],
    ),
    (
        "serial-dilution",
        &[
            (
                "start",
                "the starting concentration, in any concentration unit",
            ),
            (
                "mode",
                "1 = work the factor out from the volumes, 2 = enter the factor",
            ),
            ("transfer", "the volume moved into each new tube (mode 1)"),
            ("diluent", "the diluent waiting in each new tube (mode 1)"),
            (
                "factor",
                "the dilution factor per step in mode 2 (10 means 1:10)",
            ),
            ("steps", "how many dilution steps to plan"),
        ],
    ),
    (
        "dilution-volume",
        &[
            ("M1", "the stock concentration"),
            ("M2", "the target concentration"),
            ("V2", "the target final volume"),
            ("V1", "the stock volume needed"),
        ],
    ),
    (
        "ph",
        &[
            ("[H+]", "the hydrogen-ion concentration in mol/L"),
            ("pH", "the acidity measure, 0-14"),
        ],
    ),
    (
        "ph-inverse",
        &[
            ("pH", "the acidity measure, 0-14"),
            ("[H+]", "the hydrogen-ion concentration in mol/L"),
        ],
    ),
    (
        "poh",
        &[
            ("[OH-]", "the hydroxide-ion concentration in mol/L"),
            ("pOH", "the partner measure to pH: pOH = 14 - pH"),
        ],
    ),
    (
        "ideal-gas-pressure",
        &[
            ("n", "the amount of gas in moles"),
            ("T", "the temperature in kelvin"),
            ("V", "the volume in litres"),
            ("R", "the gas constant, 0.082057 L atm/(mol K)"),
            ("P", "the pressure"),
        ],
    ),
    (
        "ideal-gas-volume",
        &[
            ("n", "the amount of gas in moles"),
            ("T", "the temperature in kelvin"),
            ("P", "the pressure in atm"),
            ("R", "the gas constant, 0.082057 L atm/(mol K)"),
            ("V", "the volume"),
        ],
    ),
    (
        "ideal-gas-moles",
        &[
            ("P", "the pressure in atm"),
            ("V", "the volume in litres"),
            ("T", "the temperature in kelvin"),
            ("R", "the gas constant, 0.082057 L atm/(mol K)"),
            ("n", "the amount of gas in moles"),
        ],
    ),
    (
        "ideal-gas-temp",
        &[
            ("P", "the pressure in atm"),
            ("V", "the volume in litres"),
            ("n", "the amount of gas in moles"),
            ("R", "the gas constant, 0.082057 L atm/(mol K)"),
            ("T", "the temperature in kelvin"),
        ],
    ),
    (
        "boyle",
        &[
            ("P1", "the starting pressure"),
            ("V1", "the starting volume"),
            ("V2", "the new volume"),
            ("P2", "the new pressure"),
        ],
    ),
    (
        "charles",
        &[
            ("V1", "the starting volume"),
            ("T1", "the starting temperature in kelvin"),
            ("T2", "the new temperature in kelvin"),
            ("V2", "the new volume"),
        ],
    ),
    (
        "gay-lussac",
        &[
            ("P1", "the starting pressure"),
            ("T1", "the starting temperature in kelvin"),
            ("T2", "the new temperature in kelvin"),
            ("P2", "the new pressure"),
        ],
    ),
    (
        "combined-gas",
        &[
            ("P1", "the starting pressure"),
            ("V1", "the starting volume"),
            ("T1", "the starting temperature in kelvin"),
            ("V2", "the new volume"),
            ("T2", "the new temperature in kelvin"),
            ("P2", "the new pressure"),
        ],
    ),
    (
        "percent-composition",
        &[
            ("part 1", "the mass of the first part"),
            ("part 2", "the mass of the second part"),
        ],
    ),
    (
        "solution-percent",
        &[
            ("solute", "the mass of the dissolved substance"),
            ("solution", "the total solution mass"),
        ],
    ),
    (
        "empirical-ratio",
        &[
            ("element 1", "the moles of the first element"),
            ("element 2", "the moles of the second element"),
        ],
    ),
    (
        "reaction-yield",
        &[
            ("actual yield", "what the reaction really made"),
            ("theoretical yield", "the most it could have made"),
        ],
    ),
    (
        "half-life",
        &[
            ("start", "the starting amount"),
            ("time", "the time elapsed"),
            ("half-life", "the half-life, in the same time unit"),
        ],
    ),
    (
        "square-footage-rect",
        &[
            ("length", "the length"),
            ("width", "the width"),
            ("A", "the area"),
        ],
    ),
    (
        "square-footage-circle",
        &[
            ("diameter", "the diameter — right across"),
            ("A", "the area"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
    (
        "square-footage-triangle",
        &[
            ("base", "the base"),
            ("height", "the vertical height"),
            ("A", "the area"),
        ],
    ),
    (
        "concrete-slab",
        &[
            ("length", "the slab length in feet"),
            ("width", "the slab width in feet"),
            ("thickness", "the slab thickness in inches"),
            ("waste %", "extra to allow for waste and spillage"),
        ],
    ),
    (
        "concrete-column",
        &[
            ("diameter", "the column diameter in feet"),
            ("depth", "the depth in feet"),
            ("columns", "how many columns"),
            ("waste %", "extra to allow for waste"),
        ],
    ),
    (
        "concrete-stairs",
        &[
            ("width", "the stair width in feet"),
            ("rise", "the total rise in feet"),
            ("run", "the total run in feet"),
            ("waste %", "extra to allow for waste"),
        ],
    ),
    (
        "paint",
        &[
            ("length", "the room length in feet"),
            ("width", "the room width in feet"),
            ("height", "the wall height in feet"),
            ("doors", "the number of doors (about 21 sq ft each)"),
            ("windows", "the number of windows (about 15 sq ft each)"),
            ("coats", "the number of coats"),
            ("coverage", "the paint's coverage in sq ft per gallon"),
        ],
    ),
    (
        "tile",
        &[
            ("length", "the area length in feet"),
            ("width", "the area width in feet"),
            ("tile length", "the tile length in inches"),
            ("tile width", "the tile width in inches"),
            ("per box", "tiles in one box"),
            ("waste %", "extra to allow for cuts and breakage"),
        ],
    ),
    (
        "mulch",
        &[
            ("area", "the area in square feet"),
            ("depth", "the depth in inches"),
            ("price", "the price per cubic yard (0 if unknown)"),
        ],
    ),
    (
        "gravel",
        &[
            ("length", "the length in feet"),
            ("width", "the width in feet"),
            ("depth", "the depth in inches"),
            ("density", "tons per cubic yard (gravel about 1.4)"),
        ],
    ),
    (
        "flooring",
        &[
            ("length", "the room length in feet"),
            ("width", "the room width in feet"),
            ("coverage", "the square feet one box covers"),
            ("waste %", "extra to allow for cuts"),
            ("price", "the price per box (0 if unknown)"),
        ],
    ),
    (
        "fence",
        &[
            ("run length", "the fence length in feet"),
            ("spacing", "the post spacing in feet"),
            ("rails", "rails per section"),
            ("picket width", "the picket width in inches"),
            ("gap", "the gap between pickets in inches"),
        ],
    ),
    (
        "stairs",
        &[
            ("total rise", "the full height to climb, in inches"),
            ("riser", "the target height of each step, in inches"),
            ("tread run", "the depth of each tread, in inches"),
        ],
    ),
    (
        "roofing",
        &[
            ("length", "the footprint length in feet"),
            ("width", "the footprint width in feet"),
            ("rise", "the roof rise in inches per 12 inches"),
            ("waste %", "extra to allow for waste"),
            ("square", "100 sq ft of roofing"),
        ],
    ),
    (
        "drywall",
        &[
            ("area", "the total area to cover in sq ft"),
            ("coverage", "one sheet's coverage in sq ft (32 for 4x8 ft)"),
        ],
    ),
    (
        "board-feet",
        &[
            ("thickness", "the board thickness in inches"),
            ("width", "the board width in inches"),
            ("length", "the board length in feet"),
            ("boards", "the number of boards"),
            ("price", "the price per board foot (0 if unknown)"),
        ],
    ),
    (
        "deck-boards",
        &[
            ("deck length", "the deck length in feet"),
            ("deck width", "the deck width in feet"),
            ("board width", "the board width in inches"),
            ("gap", "the gap between boards in inches"),
        ],
    ),
    (
        "brick",
        &[
            ("length", "the wall length in feet"),
            ("height", "the wall height in feet"),
            ("waste %", "extra to allow for breakage"),
        ],
    ),
    (
        "wallpaper",
        &[
            ("wall area", "the total wall area in sq ft"),
            ("coverage", "the usable coverage of one roll in sq ft"),
        ],
    ),
    (
        "insulation-bags",
        &[
            ("area", "the area in square feet"),
            ("depth", "the depth wanted in inches"),
            ("coverage", "one bag's coverage in cubic feet"),
        ],
    ),
    (
        "asphalt-driveway",
        &[
            ("length", "the length in feet"),
            ("width", "the width in feet"),
            ("thickness", "the thickness in inches"),
            ("145 lb/cu ft", "the density of compacted asphalt used"),
        ],
    ),
    (
        "room-area-walls",
        &[
            ("length", "the room length in feet"),
            ("width", "the room width in feet"),
            ("height", "the wall height in feet"),
        ],
    ),
    (
        "cubic-yards",
        &[
            ("length", "the length in feet"),
            ("width", "the width in feet"),
            ("depth", "the depth in inches"),
            ("27", "the cubic feet in one cubic yard"),
        ],
    ),
    (
        "angle-iron-weight",
        &[
            ("length", "the length in inches"),
            ("width", "the width in inches"),
            ("thickness", "the thickness in inches"),
            ("490 lb/cu ft", "the density of steel used"),
        ],
    ),
    (
        "concrete-blocks",
        &[
            ("length", "the wall length in feet"),
            ("height", "the wall height in feet"),
            ("waste %", "extra to allow for breakage"),
        ],
    ),
    (
        "post-holes-concrete",
        &[
            ("diameter", "the hole diameter in inches"),
            ("depth", "the hole depth in inches"),
            ("post width", "the post width in inches"),
            ("holes", "the number of holes"),
        ],
    ),
    (
        "convert-length",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-mass",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-area",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-volume",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-speed",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-time",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-data",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-pressure",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-energy",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-power-units",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-force-units",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-torque-units",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-density-units",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-frequency",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-flow",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-illuminance",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-temperature",
        &[
            ("value", "the temperature to convert"),
            (
                "from",
                "the unit the value is in: 0 Celsius, 1 Fahrenheit, 2 Kelvin",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "convert-fuel-economy",
        &[
            (
                "US mpg",
                "US miles per gallon (fill this OR the next field)",
            ),
            (
                "L/100km",
                "litres per 100 km (fill this OR the previous field)",
            ),
        ],
    ),
    (
        "compound-density",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "compound-fuel-consumption",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "compound-data-rate",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "compound-concentration",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "compound-energy-density",
        &[
            ("value", "the amount to convert"),
            (
                "from",
                "the unit the value is in (number from the list in the description)",
            ),
            ("to", "the unit to convert to"),
        ],
    ),
    (
        "grams-cups-flour",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "125 g",
                "the grams in one cup of all-purpose flour (table value)",
            ),
        ],
    ),
    (
        "grams-cups-sugar",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "200 g",
                "the grams in one cup of granulated sugar (table value)",
            ),
        ],
    ),
    (
        "grams-cups-brown-sugar",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "220 g",
                "the grams in one packed cup of brown sugar (table value)",
            ),
        ],
    ),
    (
        "grams-cups-butter",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            ("227 g", "the grams in one cup of butter (table value)"),
        ],
    ),
    (
        "grams-cups-cocoa",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "100 g",
                "the grams in one cup of cocoa powder (table value)",
            ),
        ],
    ),
    (
        "grams-cups-rice",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "185 g",
                "the grams in one cup of uncooked rice (table value)",
            ),
        ],
    ),
    (
        "grams-cups-oats",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            ("90 g", "the grams in one cup of rolled oats (table value)"),
        ],
    ),
    (
        "grams-cups-honey",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            ("340 g", "the grams in one cup of honey (table value)"),
        ],
    ),
    (
        "grams-cups-milk",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            ("245 g", "the grams in one cup of milk (table value)"),
        ],
    ),
    (
        "grams-cups-water",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            ("236.6 g", "the grams in one cup of water (table value)"),
        ],
    ),
    (
        "grams-cups-oil",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "218 g",
                "the grams in one cup of vegetable oil (table value)",
            ),
        ],
    ),
    (
        "grams-cups-powdered-sugar",
        &[
            ("grams", "the amount in grams (or 0)"),
            ("cups", "the amount in US cups (or 0)"),
            (
                "120 g",
                "the grams in one cup of powdered sugar (table value)",
            ),
        ],
    ),
    (
        "shoe-size-men",
        &[(
            "US size",
            "the US men's shoe size, whole or half sizes 6 to 14",
        )],
    ),
    ("mean", &[("values", "the list of numbers")]),
    ("median", &[("values", "the list of numbers")]),
    ("mode-range", &[("values", "the list of numbers")]),
    ("sum-product", &[("values", "the list of numbers")]),
    (
        "std-population",
        &[
            ("values", "the list of numbers — the whole group"),
            ("mean", "the average of the values"),
            ("n", "how many values there are"),
        ],
    ),
    (
        "std-sample",
        &[
            ("values", "the list of numbers — a sample"),
            ("mean", "the average of the values"),
            ("n", "how many values there are"),
        ],
    ),
    (
        "geometric-mean",
        &[
            ("values", "the list of numbers, all over 0"),
            ("n", "how many values there are"),
        ],
    ),
    (
        "harmonic-mean",
        &[
            ("values", "the list of numbers, none zero"),
            ("n", "how many values there are"),
        ],
    ),
    (
        "z-score",
        &[
            ("value", "the value being scored"),
            ("mean", "the group's mean"),
            ("SD", "the group's standard deviation"),
            ("z", "how many standard deviations from the mean"),
        ],
    ),
    (
        "percentile-from-z",
        &[
            ("z", "the z value"),
            (
                "Phi",
                "the standard normal curve area to the left of a z value",
            ),
        ],
    ),
    (
        "p-value-z",
        &[
            ("z", "the z statistic"),
            (
                "Phi",
                "the standard normal curve area to the left of a z value",
            ),
        ],
    ),
    (
        "probability",
        &[
            ("favourable", "outcomes that count as a success"),
            ("total", "all possible outcomes"),
        ],
    ),
    (
        "combinations",
        &[
            ("n", "the total items"),
            ("r", "the items chosen"),
            ("n!", "n factorial — n x (n-1) x ... x 1"),
        ],
    ),
    (
        "permutations",
        &[
            ("n", "the total items"),
            ("r", "the positions filled"),
            ("n!", "n factorial — n x (n-1) x ... x 1"),
        ],
    ),
    (
        "confidence-interval-mean",
        &[
            ("mean", "the sample mean"),
            ("SD", "the population standard deviation"),
            ("n", "the sample size"),
            (
                "z*",
                "the critical value for your confidence level (1.96 for 95%)",
            ),
        ],
    ),
    (
        "margin-of-error",
        &[
            ("p", "the sample proportion, 0 to 1"),
            ("n", "the sample size"),
            (
                "z*",
                "the critical value for your confidence level (1.96 for 95%)",
            ),
        ],
    ),
    (
        "sample-size-proportion",
        &[
            ("MoE", "the margin of error you want, 0 to 1"),
            ("p", "the proportion you expect, 0 to 1 (0.5 if unsure)"),
            (
                "z*",
                "the critical value for your confidence level (1.96 for 95%)",
            ),
            ("n", "the sample size needed"),
        ],
    ),
    (
        "expected-value",
        &[(
            "pairs",
            "value, probability pairs — each value with its probability as a decimal",
        )],
    ),
    (
        "dice-probability",
        &[
            ("total", "the exact total wanted, 2 to 12"),
            ("36", "the equally likely pairs two dice can make"),
        ],
    ),
    (
        "percentile-rank",
        &[
            ("score", "the score entered first"),
            ("data", "the data values after the score"),
        ],
    ),
    (
        "weighted-average",
        &[("pairs", "value, weight pairs — each value with its weight")],
    ),
    (
        "linear-regression",
        &[
            ("pairs", "x, y pairs — each measured point"),
            ("m", "the slope of the fitted line"),
            ("b", "the intercept — where the line crosses the y axis"),
            ("n", "the number of points"),
        ],
    ),
    (
        "replicates-summary",
        &[
            ("values", "the repeated measurements of the same quantity"),
            ("mean", "the average of the measurements"),
            (
                "SD",
                "the sample standard deviation — the spread of single measurements",
            ),
            (
                "SEM",
                "the standard error of the mean — the uncertainty of the mean itself",
            ),
            (
                "t",
                "the Student t critical value for n - 1 degrees of freedom",
            ),
        ],
    ),
    (
        "outlier-check",
        &[
            ("values", "the data values to check"),
            ("Q1", "the first quartile — the median of the lower half"),
            ("Q3", "the third quartile — the median of the upper half"),
            ("IQR", "the interquartile range, Q3 - Q1"),
        ],
    ),
    (
        "resistor-colour-code",
        &[
            (
                "bands",
                "the colour bands in order, or a resistance to encode",
            ),
            ("digits", "the significant digits from the first bands"),
            (
                "multiplier",
                "the power of ten the digits are multiplied by",
            ),
            (
                "tolerance",
                "how far the real value may sit from the marked one, in percent",
            ),
        ],
    ),
    (
        "resistors-series-parallel",
        &[("R list", "the resistances in ohms, in any order")],
    ),
    (
        "voltage-divider",
        &[
            ("Vin", "the input (supply) voltage"),
            ("R1", "the input-side resistor in ohms"),
            ("R2", "the output-side resistor in ohms — Vout is across it"),
        ],
    ),
    (
        "tolerance-stack-up",
        &[
            ("nominal", "one dimension's target size"),
            ("tolerance", "that dimension's allowed +/- variation"),
        ],
    ),
    (
        "safety-factor",
        &[
            ("mode", "1 = stress given, 2 = load and area given"),
            (
                "strength",
                "the material limit checked against — yield or ultimate",
            ),
            ("stress", "the applied stress (mode 1)"),
            ("load", "the force on the part (mode 2)"),
            ("area", "the cross-section carrying the load (mode 2)"),
        ],
    ),
    (
        "bolt-torque",
        &[
            ("K", "the nut factor — friction condition of the joint"),
            ("D", "the bolt's nominal diameter"),
            ("F", "the target clamp force"),
        ],
    ),
    (
        "beam-bending",
        &[
            ("L", "the span between the supports"),
            ("P", "the point load at midspan"),
            ("E", "Young's modulus of the beam material"),
            ("material", "the materials-table code used when E is 0"),
            ("section", "1 = rectangle, 2 = circle"),
            ("b / d", "the section width, or the circle's diameter"),
            ("h", "the rectangle's vertical depth"),
        ],
    ),
    (
        "days-between-dates",
        &[
            ("start date", "the first date (MM/DD/YYYY)"),
            ("end date", "the second date (MM/DD/YYYY)"),
        ],
    ),
    (
        "date-plus-days",
        &[
            ("start date", "the date to start from (MM/DD/YYYY)"),
            ("days", "the days to add (negative goes backwards)"),
        ],
    ),
    (
        "age",
        &[
            ("birth date", "the date of birth (MM/DD/YYYY)"),
            ("end date", "the date to measure the age at (MM/DD/YYYY)"),
        ],
    ),
    ("weekday", &[("date", "the date (MM/DD/YYYY)")]),
    (
        "time-duration",
        &[
            ("start hour", "the start hour, 0-23"),
            ("start minute", "the start minute"),
            ("end hour", "the end hour, 0-23"),
            ("end minute", "the end minute"),
        ],
    ),
    (
        "countdown-days",
        &[
            ("from date", "the date to count from (MM/DD/YYYY)"),
            ("target date", "the date counted to (MM/DD/YYYY)"),
        ],
    ),
    (
        "business-days",
        &[
            ("start date", "the first date (MM/DD/YYYY)"),
            ("end date", "the second date (MM/DD/YYYY)"),
        ],
    ),
    (
        "dog-age",
        &[
            ("dog years", "the dog's age in years"),
            ("size code", "1 = small, 2 = medium, 3 = large"),
        ],
    ),
    ("cat-age", &[("cat years", "the cat's age in years")]),
    (
        "fuel-cost-trip",
        &[
            ("distance", "the trip distance"),
            ("economy", "the fuel economy, distance per unit of fuel"),
            ("price", "the price per unit of fuel"),
        ],
    ),
    (
        "mpg-trip",
        &[
            ("distance", "the distance driven"),
            ("fuel", "the fuel used"),
        ],
    ),
    (
        "electricity-cost",
        &[
            ("watts", "the appliance's watts"),
            ("hours", "the hours used"),
            ("rate", "the price per kWh"),
        ],
    ),
    (
        "download-time",
        &[
            ("file size", "the file size in MB"),
            ("speed", "the download speed in Mbps"),
        ],
    ),
    (
        "aspect-ratio",
        &[
            ("width", "the original width"),
            ("height", "the original height"),
            ("new width", "the new width to match"),
            ("GCD", "greatest common divisor of width and height"),
        ],
    ),
    (
        "screen-ppi",
        &[
            ("w", "the horizontal pixels"),
            ("h", "the vertical pixels"),
            ("diagonal", "the screen diagonal in inches"),
            ("PPI", "pixels per inch"),
        ],
    ),
    (
        "gpa",
        &[(
            "pairs",
            "grade points, credits pairs — each course's points with its credits",
        )],
    ),
    (
        "test-grade",
        &[
            ("correct", "the correct answers"),
            ("total", "the total questions"),
            ("target %", "the percent you are aiming for"),
        ],
    ),
    (
        "attendance",
        &[
            ("attended", "the sessions attended"),
            ("total", "the total sessions"),
            ("target %", "the attendance percent to stay at or above"),
        ],
    ),
    (
        "age-in-days",
        &[
            ("birth date", "the date of birth (MM/DD/YYYY)"),
            ("end date", "the date to measure at (MM/DD/YYYY)"),
        ],
    ),
    (
        "split-bill",
        &[
            ("amount", "the bill amount"),
            ("tax %", "the tax percent"),
            ("tip %", "the tip percent"),
            ("people", "how many people split the bill"),
        ],
    ),
    (
        "unit-price",
        &[
            ("price 1", "the first product's price"),
            ("quantity 1", "the first product's quantity"),
            ("price 2", "the second product's price"),
            ("quantity 2", "the second product's quantity"),
        ],
    ),
    (
        "overtime-pay",
        &[
            ("rate", "the hourly pay rate"),
            ("regular hours", "regular hours worked"),
            ("overtime hours", "overtime hours, paid at 1.5x"),
        ],
    ),
    (
        "race-pace",
        &[
            ("pace", "minutes per kilometre"),
            ("distance", "the race distance in kilometres"),
        ],
    ),
    (
        "race-predictor-riegel",
        &[
            ("T1", "the known race time in minutes"),
            ("D1", "the known distance in kilometres"),
            ("D2", "the new distance in kilometres"),
            ("T2", "the predicted time"),
        ],
    ),
    (
        "swim-pace",
        &[
            ("time", "the total time in minutes"),
            ("distance", "the distance in metres"),
        ],
    ),
    (
        "cycling-speed",
        &[
            ("distance", "the distance in kilometres"),
            ("time", "the time in hours"),
        ],
    ),
    (
        "vo2max-estimate",
        &[
            ("distance", "the metres run in 12 minutes"),
            ("VO2max", "the maximum oxygen uptake estimate"),
        ],
    ),
    (
        "one-rep-max",
        &[
            ("weight", "the weight lifted"),
            ("reps", "the reps completed"),
            ("1RM", "the estimated one-rep maximum"),
        ],
    ),
    (
        "marathon-splits",
        &[
            ("goal time", "the goal time in minutes"),
            ("distance", "the race distance in kilometres"),
        ],
    ),
    (
        "recipe-scale",
        &[
            ("amount", "the original amount of one ingredient"),
            ("recipe servings", "the servings the recipe makes"),
            ("servings wanted", "the servings you want"),
        ],
    ),
    (
        "bakers-percent",
        &[
            ("flour", "the flour weight in grams"),
            ("ingredient", "the ingredient weight in grams"),
            ("target %", "the percent to find a weight for"),
        ],
    ),
    (
        "oven-temp",
        &[
            ("C", "the temperature in Celsius"),
            ("F", "the temperature in Fahrenheit"),
        ],
    ),
    (
        "alcohol-units",
        &[
            ("volume", "the drink volume in millilitres"),
            ("ABV %", "the alcohol by volume percent"),
        ],
    ),
    (
        "coffee-water-ratio",
        &[
            ("coffee", "the coffee in grams"),
            ("ratio", "the water per 1 coffee (16 means 1:16)"),
        ],
    ),
    (
        "pizza-dough",
        &[
            ("dough ball", "one ball's total weight in grams"),
            ("hydration %", "the water as a percent of the flour"),
            ("balls", "the number of dough balls"),
        ],
    ),
    (
        "meat-cooking-time",
        &[
            ("weight", "the weight in pounds"),
            ("minutes per lb", "the minutes per pound from your recipe"),
        ],
    ),
    (
        "carbon-driving",
        &[
            ("distance", "the distance in kilometres"),
            ("economy", "kilometres per litre"),
            ("factor", "kg of CO2 per litre (2.31 petrol, 2.68 diesel)"),
        ],
    ),
    (
        "solar-output",
        &[
            ("watts", "the total panel watts"),
            ("sun hours", "the peak sun hours per day"),
            ("rate", "the electricity rate per kWh"),
            ("0.75", "the system-loss factor used"),
        ],
    ),
    (
        "rainwater",
        &[
            ("roof area", "the roof area in square metres"),
            ("rainfall", "the rainfall in millimetres"),
            ("0.8", "the runoff factor used"),
        ],
    ),
    (
        "pool-volume",
        &[
            ("L", "the pool length in metres"),
            ("W", "the pool width in metres"),
            ("depth", "the average depth in metres"),
        ],
    ),
    (
        "pool-chlorine",
        &[
            ("gallons", "the pool volume in US gallons"),
            ("ppm rise", "the chlorine rise wanted, in ppm"),
        ],
    ),
    (
        "tree-oxygen-co2",
        &[
            ("trees", "the number of trees"),
            ("years", "the number of years"),
            (
                "21 kg",
                "the CO2 one mature tree absorbs in a year, the average used",
            ),
        ],
    ),
    (
        "lighting-cost-year",
        &[
            ("watts", "the watts per bulb"),
            ("hours/day", "the hours per day the bulbs are on"),
            ("bulbs", "the number of bulbs"),
            ("rate", "the price per kWh"),
        ],
    ),
    (
        "working-hours-pay",
        &[
            ("start hour", "the start hour, 0-23"),
            ("start minute", "the start minute"),
            ("end hour", "the end hour, 0-23"),
            ("end minute", "the end minute"),
            ("break", "the unpaid break in minutes"),
        ],
    ),
    (
        "gross-profit-business",
        &[
            ("revenue", "the total money in"),
            ("COGS", "the cost of goods sold"),
        ],
    ),
    (
        "price-per-unit-other",
        &[
            ("case price", "the price of one case"),
            ("per case", "the items in one case"),
            ("needed", "the items you need"),
        ],
    ),
    (
        "shrinkage",
        &[
            ("recorded", "the recorded stock value"),
            ("actual", "the actual stock value"),
        ],
    ),
    (
        "circumference",
        &[
            ("d", "the diameter — right across the circle"),
            ("C", "the circumference — distance around"),
            ("pi", "pi, about 3.14159"),
        ],
    ),
];

/// The symbol glossary for a calculator (item #19); empty only if
/// a future calculator forgets its row — the test forbids it.
pub fn glossary_for(c: &crate::Calc) -> &'static [(&'static str, &'static str)] {
    for (id, g) in GLOSSARY {
        if *id == c.id {
            return g;
        }
    }
    &[]
}

// ---------- v0.6.0: course packs (item #24) ----------
//
// Each calculator is tagged with the course levels where it
// genuinely belongs (GCSE, A-Level, AP, Undergrad) — a
// calculator may carry several tags or none. Tagging is
// conservative: decided per calculator from real curriculum
// fit (school/university syllabuses), and when unsure the
// calculator is left untagged (see DECISIONS.md). The counts
// printed in the window and the docs are computed from this
// data, never typed by hand.

/// The four course packs, in display order.
pub const COURSE_PACK_NAMES: &[&str] = &["GCSE", "A-Level", "AP", "Undergrad"];

/// (pack name, member calculator ids in registry-friendly order).
pub static COURSE_PACKS: &[(&str, &[&str])] = &[
    (
        "GCSE",
        &[
            "expression-calculator",
            "percent-of",
            "percent-is",
            "percent-change",
            "percent-difference",
            "reverse-percent",
            "percent-off-result",
            "fraction-simplify",
            "fraction-add",
            "fraction-subtract",
            "fraction-multiply",
            "fraction-divide",
            "fraction-to-decimal",
            "decimal-to-fraction",
            "exponent",
            "square-cube",
            "square-root",
            "nth-root",
            "scientific-notation",
            "from-scientific",
            "sig-figs",
            "round-decimals",
            "gcd",
            "lcm",
            "prime-check",
            "ratio",
            "proportion",
            "rule-of-three",
            "quadratic",
            "linear-equation",
            "slope",
            "distance-points",
            "pythagorean",
            "pythagorean-leg",
            "arithmetic-sequence",
            "geometric-sequence",
            "circle",
            "circle-from-area",
            "rectangle",
            "square-shape",
            "triangle-area",
            "triangle-heron",
            "trapezoid",
            "parallelogram",
            "rhombus",
            "regular-polygon",
            "sector",
            "cube-box",
            "cylinder",
            "sphere",
            "cone",
            "pyramid",
            "trig-deg",
            "trig-inverse",
            "angle-convert",
            "average",
            "compound-interest",
            "simple-interest",
            "discount",
            "double-discount",
            "sales-tax-add",
            "sales-tax-extract",
            "speed-from-distance-time",
            "distance-from-speed-time",
            "time-from-speed-distance",
            "average-velocity",
            "acceleration",
            "final-velocity",
            "force",
            "weight-from-mass",
            "mass-from-weight",
            "kinetic-energy",
            "potential-energy",
            "power-physics",
            "work",
            "density-physics",
            "pressure",
            "ohms-law",
            "ohms-solve-i",
            "ohms-solve-v",
            "electrical-power",
            "resistors-parallel",
            "wave",
            "momentum",
            "friction",
            "hooke",
            "free-fall",
            "torque",
            "moles-from-mass",
            "mass-from-moles",
            "percent-composition",
            "empirical-ratio",
            "reaction-yield",
            "solution-percent",
            "half-life",
            "convert-length",
            "convert-mass",
            "convert-area",
            "convert-volume",
            "convert-speed",
            "convert-time",
            "convert-temperature",
            "convert-pressure",
            "convert-energy",
            "convert-power-units",
            "mean",
            "median",
            "mode-range",
            "probability",
            "bmi",
            "target-heart-rate",
            "test-grade",
            "carbon-driving",
            "solar-output",
            "tree-oxygen-co2",
        ],
    ),
    (
        "A-Level",
        &[
            "expression-calculator",
            "percent-of",
            "percent-is",
            "percent-change",
            "percent-difference",
            "reverse-percent",
            "percent-off-result",
            "fraction-simplify",
            "fraction-add",
            "fraction-subtract",
            "fraction-multiply",
            "fraction-divide",
            "fraction-to-decimal",
            "decimal-to-fraction",
            "exponent",
            "square-cube",
            "square-root",
            "nth-root",
            "log",
            "ln-log10",
            "antilog",
            "factorial",
            "scientific-notation",
            "from-scientific",
            "sig-figs",
            "round-decimals",
            "gcd",
            "lcm",
            "prime-check",
            "ratio",
            "proportion",
            "rule-of-three",
            "quadratic",
            "linear-equation",
            "slope",
            "distance-points",
            "pythagorean",
            "pythagorean-leg",
            "arithmetic-sequence",
            "geometric-sequence",
            "circle",
            "circle-from-area",
            "rectangle",
            "square-shape",
            "triangle-area",
            "triangle-heron",
            "trapezoid",
            "parallelogram",
            "rhombus",
            "ellipse",
            "regular-polygon",
            "sector",
            "cube-box",
            "cylinder",
            "sphere",
            "cone",
            "pyramid",
            "frustum",
            "trig-deg",
            "trig-inverse",
            "angle-convert",
            "average",
            "mean",
            "median",
            "mode-range",
            "sum-product",
            "std-population",
            "std-sample",
            "geometric-mean",
            "harmonic-mean",
            "z-score",
            "percentile-from-z",
            "p-value-z",
            "probability",
            "combinations",
            "permutations",
            "confidence-interval-mean",
            "margin-of-error",
            "sample-size-proportion",
            "expected-value",
            "dice-probability",
            "percentile-rank",
            "weighted-average",
            "linear-regression",
            "speed-from-distance-time",
            "distance-from-speed-time",
            "time-from-speed-distance",
            "average-velocity",
            "acceleration",
            "final-velocity",
            "force",
            "weight-from-mass",
            "mass-from-weight",
            "kinetic-energy",
            "potential-energy",
            "mechanical-energy",
            "power-physics",
            "work",
            "density-physics",
            "pressure",
            "ohms-law",
            "ohms-solve-i",
            "ohms-solve-v",
            "electrical-power",
            "resistors-parallel",
            "projectile",
            "pendulum",
            "wave",
            "torque",
            "momentum",
            "friction",
            "hooke",
            "gravity-force",
            "centripetal",
            "free-fall",
            "moles-from-mass",
            "mass-from-moles",
            "molecules",
            "molarity",
            "molarity-from-mass",
            "dilution",
            "dilution-volume",
            "ph",
            "ph-inverse",
            "poh",
            "ideal-gas-pressure",
            "ideal-gas-volume",
            "ideal-gas-moles",
            "ideal-gas-temp",
            "boyle",
            "charles",
            "gay-lussac",
            "combined-gas",
            "percent-composition",
            "solution-percent",
            "empirical-ratio",
            "reaction-yield",
            "half-life",
            "compound-interest",
            "compound-interest-monthly-deposit",
            "simple-interest",
            "present-value",
            "future-value",
            "annuity-pv",
            "annuity-fv",
            "cagr",
            "roi",
            "effective-rate",
            "break-even",
            "payback-period",
            "net-margin",
            "profit-margin",
            "depreciation-straight",
            "convert-length",
            "convert-mass",
            "convert-volume",
            "convert-temperature",
            "convert-pressure",
            "convert-energy",
            "compound-density",
            "compound-concentration",
        ],
    ),
    (
        "AP",
        &[
            "expression-calculator",
            "exponent",
            "square-cube",
            "square-root",
            "nth-root",
            "log",
            "ln-log10",
            "antilog",
            "factorial",
            "scientific-notation",
            "sig-figs",
            "percent-change",
            "quadratic",
            "linear-equation",
            "slope",
            "distance-points",
            "pythagorean",
            "arithmetic-sequence",
            "geometric-sequence",
            "circle",
            "cylinder",
            "sphere",
            "cone",
            "sector",
            "ellipse",
            "frustum",
            "trig-deg",
            "trig-inverse",
            "angle-convert",
            "ratio",
            "proportion",
            "mean",
            "median",
            "mode-range",
            "sum-product",
            "std-population",
            "std-sample",
            "geometric-mean",
            "harmonic-mean",
            "z-score",
            "percentile-from-z",
            "p-value-z",
            "probability",
            "combinations",
            "permutations",
            "confidence-interval-mean",
            "margin-of-error",
            "sample-size-proportion",
            "expected-value",
            "dice-probability",
            "percentile-rank",
            "weighted-average",
            "linear-regression",
            "speed-from-distance-time",
            "distance-from-speed-time",
            "time-from-speed-distance",
            "average-velocity",
            "acceleration",
            "final-velocity",
            "force",
            "weight-from-mass",
            "mass-from-weight",
            "kinetic-energy",
            "potential-energy",
            "mechanical-energy",
            "power-physics",
            "work",
            "density-physics",
            "pressure",
            "ohms-law",
            "ohms-solve-i",
            "ohms-solve-v",
            "electrical-power",
            "resistors-parallel",
            "projectile",
            "pendulum",
            "wave",
            "torque",
            "momentum",
            "friction",
            "hooke",
            "gravity-force",
            "centripetal",
            "free-fall",
            "moles-from-mass",
            "mass-from-moles",
            "molecules",
            "molarity",
            "molarity-from-mass",
            "dilution",
            "dilution-volume",
            "ph",
            "ph-inverse",
            "poh",
            "ideal-gas-pressure",
            "ideal-gas-volume",
            "ideal-gas-moles",
            "ideal-gas-temp",
            "boyle",
            "charles",
            "gay-lussac",
            "combined-gas",
            "percent-composition",
            "solution-percent",
            "empirical-ratio",
            "reaction-yield",
            "half-life",
            "compound-interest",
            "simple-interest",
            "loan-payment",
            "mortgage-payment",
            "auto-loan",
            "present-value",
            "future-value",
            "roi",
            "inflation",
            "rule-of-72",
            "savings-goal",
            "credit-card-payoff",
            "gpa",
            "test-grade",
        ],
    ),
    (
        "Undergrad",
        &[
            "expression-calculator",
            "percent-of",
            "percent-is",
            "percent-change",
            "percent-difference",
            "reverse-percent",
            "percent-off-result",
            "fraction-simplify",
            "fraction-add",
            "fraction-subtract",
            "fraction-multiply",
            "fraction-divide",
            "fraction-to-decimal",
            "decimal-to-fraction",
            "exponent",
            "square-cube",
            "square-root",
            "nth-root",
            "log",
            "ln-log10",
            "antilog",
            "factorial",
            "scientific-notation",
            "from-scientific",
            "sig-figs",
            "round-decimals",
            "gcd",
            "lcm",
            "prime-check",
            "ratio",
            "proportion",
            "rule-of-three",
            "quadratic",
            "linear-equation",
            "slope",
            "distance-points",
            "pythagorean",
            "pythagorean-leg",
            "arithmetic-sequence",
            "geometric-sequence",
            "circle",
            "circle-from-area",
            "rectangle",
            "square-shape",
            "triangle-area",
            "triangle-heron",
            "trapezoid",
            "parallelogram",
            "rhombus",
            "ellipse",
            "regular-polygon",
            "sector",
            "cube-box",
            "cylinder",
            "sphere",
            "cone",
            "pyramid",
            "frustum",
            "trig-deg",
            "trig-inverse",
            "angle-convert",
            "average",
            "mean",
            "median",
            "mode-range",
            "sum-product",
            "std-population",
            "std-sample",
            "geometric-mean",
            "harmonic-mean",
            "z-score",
            "percentile-from-z",
            "p-value-z",
            "probability",
            "combinations",
            "permutations",
            "confidence-interval-mean",
            "margin-of-error",
            "sample-size-proportion",
            "expected-value",
            "dice-probability",
            "percentile-rank",
            "weighted-average",
            "linear-regression",
            "speed-from-distance-time",
            "distance-from-speed-time",
            "time-from-speed-distance",
            "average-velocity",
            "acceleration",
            "final-velocity",
            "force",
            "weight-from-mass",
            "mass-from-weight",
            "kinetic-energy",
            "potential-energy",
            "mechanical-energy",
            "power-physics",
            "work",
            "density-physics",
            "pressure",
            "ohms-law",
            "ohms-solve-i",
            "ohms-solve-v",
            "electrical-power",
            "resistors-parallel",
            "projectile",
            "pendulum",
            "wave",
            "torque",
            "momentum",
            "friction",
            "hooke",
            "gravity-force",
            "centripetal",
            "free-fall",
            "moles-from-mass",
            "mass-from-moles",
            "molecules",
            "molarity",
            "molarity-from-mass",
            "dilution",
            "dilution-volume",
            "ph",
            "ph-inverse",
            "poh",
            "ideal-gas-pressure",
            "ideal-gas-volume",
            "ideal-gas-moles",
            "ideal-gas-temp",
            "boyle",
            "charles",
            "gay-lussac",
            "combined-gas",
            "percent-composition",
            "solution-percent",
            "empirical-ratio",
            "reaction-yield",
            "half-life",
            "compound-interest",
            "compound-interest-monthly-deposit",
            "simple-interest",
            "loan-payment",
            "mortgage-payment",
            "auto-loan",
            "loan-affordability",
            "present-value",
            "future-value",
            "annuity-pv",
            "annuity-fv",
            "cagr",
            "roi",
            "profit-margin",
            "price-from-margin",
            "price-from-markup",
            "break-even",
            "effective-rate",
            "credit-card-payoff",
            "inflation",
            "net-margin",
            "payback-period",
            "depreciation-straight",
            "rent-vs-buy-simple",
            "convert-length",
            "convert-mass",
            "convert-area",
            "convert-volume",
            "convert-speed",
            "convert-time",
            "convert-data",
            "convert-pressure",
            "convert-energy",
            "convert-power-units",
            "convert-force-units",
            "convert-torque-units",
            "convert-density-units",
            "convert-frequency",
            "convert-flow",
            "convert-illuminance",
            "convert-temperature",
            "convert-fuel-economy",
            "compound-density",
            "compound-data-rate",
            "compound-concentration",
            "compound-energy-density",
            "compound-fuel-consumption",
            "bmi",
            "bmi-imperial",
            "bmr-male",
            "bmr-female",
            "bmr-harris-male",
            "bmr-harris-female",
            "tdee",
            "calorie-target",
            "body-fat-navy-male",
            "body-fat-navy-female",
            "ideal-weight-devine",
            "water-intake",
            "macro-split",
            "protein-needs",
            "bac-widmark",
            "carbon-driving",
            "solar-output",
            "rainwater",
            "pool-volume",
            "lighting-cost-year",
            "square-footage-rect",
            "concrete-slab",
            "stairs",
            "board-feet",
            "cubic-yards",
        ],
    ),
];

/// The packs a calculator belongs to (empty when untagged).
pub fn packs_for(c: &crate::Calc) -> Vec<&'static str> {
    let mut out = Vec::new();
    for (pack, ids) in COURSE_PACKS {
        if ids.contains(&c.id) {
            out.push(*pack);
        }
    }
    out
}

/// Member indices of one pack, in registry order.
pub fn pack_members(calcs: &[crate::Calc], pack: &str) -> Vec<usize> {
    let Some((_, ids)) = COURSE_PACKS.iter().find(|(name, _)| *name == pack) else {
        return Vec::new();
    };
    calcs
        .iter()
        .enumerate()
        .filter(|(_, c)| ids.contains(&c.id))
        .map(|(i, _)| i)
        .collect()
}
