//! v0.1.2 audit tests — Conversion and Statistics.
//! Conversion factors are the published exact/standard definitions
//! (1 acre = 43,560 sq ft, 1 atm = 101,325 Pa, 1 hp = 745.699872 W,
//! 1 US gallon = 3.785411784 L, ...). Statistics expectations were
//! computed independently in Python (sample variance uses n-1).
use calcdex::run_calc;

fn out(id: &str, args: &[f64]) -> String {
    run_calc(id, args).unwrap_or_else(|e| panic!("{id} {args:?} errored: {e}"))
}
fn has(id: &str, args: &[f64], needle: &str) {
    let o = out(id, args);
    assert!(o.contains(needle), "{id} {args:?} missing {needle:?}:\n{o}");
}
fn err(id: &str, args: &[f64], needle: &str) {
    let e = run_calc(id, args).unwrap_err();
    assert!(
        e.contains(needle),
        "{id} {args:?} wrong error {e:?}, want {needle:?}"
    );
}
fn has_list(id: &str, list: &[f64], needle: &str) {
    let o = out(id, list);
    assert!(o.contains(needle), "{id} {list:?} missing {needle:?}:\n{o}");
}

#[test]
fn conversions() {
    // 1 acre -> square feet (to-unit 5): exactly 43,560.
    has("convert-area", &[1.0, 7.0, 5.0], "43560");
    // 100 km/h -> m/s: 27.77777778.
    has("convert-speed", &[100.0, 1.0, 0.0], "27.77777778");
    // 2 hours -> minutes: 120.
    has("convert-time", &[2.0, 3.0, 2.0], "120");
    // 1 GiB -> MiB: 1024.
    has("convert-data", &[1.0, 7.0, 6.0], "1024");
    // 1 atm -> psi: 101325 / 6894.757 = 14.6959494.
    has("convert-pressure", &[1.0, 3.0, 4.0], "14.6959494");
    // 1 kWh -> BTU: 3.6e6 / 1055.05585 = 3412.1416416.
    has("convert-energy", &[1.0, 5.0, 6.0], "3412.1416416");
    // 1 mechanical hp -> watts: 745.699872 exactly.
    has("convert-power-units", &[1.0, 2.0, 0.0], "745.699872");
    // 10 lbf -> newtons: 44.48221615.
    has("convert-force-units", &[10.0, 3.0, 0.0], "44.48221615");
    // 10 ft-lb -> N m: 13.55817948.
    has("convert-torque-units", &[10.0, 1.0, 0.0], "13.55817948");
    // 1 g/cm3 -> kg/m3: 1000.
    has("convert-density-units", &[1.0, 1.0, 0.0], "1000");
    // 60 L/min -> L/s: 1.
    has("convert-flow", &[60.0, 1.0, 0.0], "1");
    // 1 foot-candle -> lux: 10.7639.
    has("convert-illuminance", &[1.0, 1.0, 0.0], "10.7639");
    // 1 US gallon -> litres: 3.78541178.
    has("convert-volume", &[1.0, 10.0, 0.0], "3.78541178");
    // BUG PIN (v0.1.2 fix): fractional unit numbers (1.9) used to be
    // silently truncated to unit 1. Now a teaching error.
    err("convert-length", &[1.0, 1.9, 0.0], "whole numbers");
    err("convert-temperature", &[32.0, 1.5, 0.0], "whole numbers");
}

#[test]
fn temperature_and_kitchen() {
    // 32 F = 0 C; the -40 crossover: -40 F = -40 C; 0 C = 273.15 K.
    has("convert-temperature", &[32.0, 1.0, 0.0], "0");
    has("convert-temperature", &[-40.0, 1.0, 0.0], "-40");
    has("convert-temperature", &[0.0, 0.0, 2.0], "273.15");
    // Kitchen (published per-cup gram weights; inputs are
    // [grams, cups] with exactly one given): 2 cups sugar = 400 g.
    has("grams-cups-sugar", &[0.0, 2.0], "400");
    has("grams-cups-butter", &[0.0, 2.0], "454");
    has("grams-cups-cocoa", &[0.0, 2.0], "200");
    has("grams-cups-water", &[0.0, 2.0], "473.2");
    // US men's 9.5 -> EU 43, UK 8.5, 27.1 cm.
    has("shoe-size-men", &[9.5], "43");
    has("shoe-size-men", &[9.5], "27.1");
    // Remaining per-cup gram weights (published tables), 2 cups each.
    has("grams-cups-brown-sugar", &[0.0, 2.0], "440");
    has("grams-cups-honey", &[0.0, 2.0], "680");
    has("grams-cups-milk", &[0.0, 2.0], "490");
    has("grams-cups-oats", &[0.0, 2.0], "180");
    has("grams-cups-oil", &[0.0, 2.0], "436");
    has("grams-cups-powdered-sugar", &[0.0, 2.0], "240");
    has("grams-cups-rice", &[0.0, 2.0], "370");
    // 60 rpm -> Hz (from-unit 4): exactly 1.
    has("convert-frequency", &[60.0, 4.0, 0.0], "1");
}

#[test]
fn stats_central() {
    // Median of 1,3,5,7 = 4.
    has_list("median", &[1.0, 3.0, 5.0, 7.0], "4");
    // Mode of 1,2,2,3 is 2; range 2.
    has_list("mode-range", &[1.0, 2.0, 2.0, 3.0], "2");
    // Sum 10, product 24.
    has_list("sum-product", &[1.0, 2.0, 3.0, 4.0], "10");
    has_list("sum-product", &[1.0, 2.0, 3.0, 4.0], "24");
    // Classic set 2,4,4,4,5,5,7,9: sample variance 32/7 = 4.571429,
    // sample SD = 2.13809.
    has_list(
        "std-sample",
        &[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0],
        "4.571429",
    );
    has_list(
        "std-sample",
        &[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0],
        "2.13809",
    );
    // Geometric mean of 1,3,9 = 3.
    has_list("geometric-mean", &[1.0, 3.0, 9.0], "3");
    // Harmonic mean of 2,4 = 2.666667.
    has_list("harmonic-mean", &[2.0, 4.0], "2.666667");
    // Weighted average: 80x0.3 + 90x0.7 = 87.
    has("weighted-average", &[80.0, 0.3, 90.0, 0.7], "87");
    // Percentile rank of 50 in 10..100 step 10: (4 + 0.5) / 10 = 45th.
    let mut v = vec![50.0];
    v.extend((1..=10).map(|i| i as f64 * 10.0));
    has_list("percentile-rank", &v, "45");
}

#[test]
fn stats_inference() {
    // Phi(1.96) = 97.5002% (normal table).
    has("percentile-from-z", &[1.96], "97.5002");
    // Two-sided p for z = 2 is 0.0455 (normal table).
    has("p-value-z", &[2.0], "0.0455");
    // CI: 100 +/- 1.96 x 15/5 = 100 +/- 5.88 -> (94.12, 105.88).
    has(
        "confidence-interval-mean",
        &[100.0, 15.0, 25.0, 1.96],
        "94.12",
    );
    has(
        "confidence-interval-mean",
        &[100.0, 15.0, 25.0, 1.96],
        "105.88",
    );
    // MOE for p=0.5, n=1000, z=1.96: 1.96 x sqrt(0.25/1000) = 0.03099.
    has("margin-of-error", &[0.5, 1000.0, 1.96], "0.03099");
    // Sample size for MOE 0.03 at p=0.5, z=1.96: ceil(1067.11) = 1068.
    has("sample-size-proportion", &[0.03, 0.5, 1.96], "1068");
    // BUG PIN (v0.1.2 fix): p = 0 or 1 used to answer "0 people
    // needed". p must be strictly between 0 and 1.
    err(
        "sample-size-proportion",
        &[0.03, 0.0, 1.96],
        "between 0 and 1",
    );
    err(
        "sample-size-proportion",
        &[0.03, 1.0, 1.96],
        "between 0 and 1",
    );
    // Two dice: total 7 has 6 of 36 outcomes = 16.667%.
    has("dice-probability", &[7.0], "16.667");
}
