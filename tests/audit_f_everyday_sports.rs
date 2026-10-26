//! v0.1.2 audit tests — Date & Time, Everyday, Sports, Food,
//! Ecology, Other. Expected values computed independently
//! (calendar arithmetic checked against known 2026 weekdays:
//! 2026-10-23 is a Friday, 2026-10-25 a Sunday).
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

#[test]
fn dates() {
    // Fri 2026-10-23 to Fri 2026-10-30: 7 total days, 5 business days
    // (Sat 24, Sun 25 skipped; Mon-Fri counted, start day excluded).
    has(
        "business-days",
        &[2026.0, 10.0, 23.0, 2026.0, 10.0, 30.0],
        "7",
    );
    has(
        "business-days",
        &[2026.0, 10.0, 23.0, 2026.0, 10.0, 30.0],
        "5",
    );
    // 2026-01-01 to 2026-12-31 is 364 days (2026 is not a leap year).
    has(
        "countdown-days",
        &[2026.0, 1.0, 1.0, 2026.0, 12.0, 31.0],
        "364",
    );
    // One day apart.
    has("age-in-days", &[2000.0, 1.0, 1.0, 2000.0, 1.0, 2.0], "1");
    // 09:30 to 17:45 = 495 minutes = 8 h 15 min.
    has("time-duration", &[9.0, 30.0, 17.0, 45.0], "495");
}

#[test]
fn everyday() {
    // Dog age 3, large (code 3): 24 + 6 = 30 human years.
    has("dog-age", &[3.0, 3.0], "30");
    // Dog age 3, small (code 1): 24 + 5 = 29.
    has("dog-age", &[3.0, 1.0], "29");
    // BUG PIN (v0.1.2 fix): an invalid size code (e.g. 9) used to be
    // silently treated as small/medium. Now a teaching error.
    err("dog-age", &[3.0, 9.0], "Size code must be 1");
    // Cat age 5: 24 + 4 x 3 = 36.
    has("cat-age", &[5.0], "36");
    // Trip: 300 miles at 30 mpg = 10 gallons; at 3.50 = 35.00.
    has("fuel-cost-trip", &[300.0, 30.0, 3.5], "35");
    // MPG: 300 miles / 10 gallons = 30.
    has("mpg-trip", &[300.0, 10.0], "30");
    // 100 W for 5 h = 0.5 kWh; at 0.20/kWh = 0.10/day, 3.00 per 30 days.
    has("electricity-cost", &[100.0, 5.0, 0.2], "0.5");
    has("electricity-cost", &[100.0, 5.0, 0.2], "3");
    // 800 MB at 100 Mbps: 800 x 8 / 100 = 64 s = 1.07 min.
    has("download-time", &[800.0, 100.0], "64");
    has("download-time", &[800.0, 100.0], "1.07");
    // 1920x1080 scaled to width 1280: height 720 (16:9).
    has("aspect-ratio", &[1920.0, 1080.0, 1280.0], "720");
    // PPI of 1920x1080 on 15.6 in: sqrt(1920^2+1080^2)/15.6 = 141.2.
    has("screen-ppi", &[1920.0, 1080.0, 15.6], "141.2");
    // GPA: A(4) x 3 credits + B(3) x 4 credits = 24/7 = 3.429.
    has("gpa", &[4.0, 3.0, 3.0, 4.0], "3.429");
    // Split 100 + 10% tax + 20% tip over 4 people: 130 total, 32.50 each.
    has("split-bill", &[100.0, 10.0, 20.0, 4.0], "130");
    has("split-bill", &[100.0, 10.0, 20.0, 4.0], "32.50");
    // Unit price: 10/5 = 2 per unit beats 9/3 = 3 per unit.
    has("unit-price", &[10.0, 5.0, 9.0, 3.0], "2");
    // Overtime: 20/h, 40 h straight + 5 h at 1.5x = 950 total.
    has("overtime-pay", &[20.0, 40.0, 5.0], "950");
    // Test grade: 45/50 = 90% already at a 90% target.
    has("test-grade", &[45.0, 50.0, 90.0], "90");
    // Attendance 45/50 = 90%.
    has("attendance", &[45.0, 50.0, 90.0], "90");
}

#[test]
fn sports() {
    // 6 min/km over 10 km = 60 min; speed 10 km/h.
    has("race-pace", &[6.0, 10.0], "60");
    has("race-pace", &[6.0, 10.0], "10");
    // Riegel: 60 min for 10 km predicts 60 x 2^1.06 = 125.1 min for 20 km.
    has("race-predictor-riegel", &[60.0, 10.0, 20.0], "125.1");
    // Swim 1500 m in 30 min: 2 min/100 m, 3 km/h.
    has("swim-pace", &[30.0, 1500.0], "2");
    has("swim-pace", &[30.0, 1500.0], "3");
    // Cycling 40 km in 2 h: 20 km/h = 12.43 mph.
    has("cycling-speed", &[40.0, 2.0], "12.43");
    // Cooper test 2800 m: VO2max = (2800 - 504.9) / 44.73 = 51.3.
    has("vo2max-estimate", &[2800.0], "51.3");
    // Marathon (42.195 km) in 240 min: 5.688 min/km even splits.
    has("marathon-splits", &[240.0, 42.195], "5.688");
}

#[test]
fn food() {
    // Scale 200 g from 4 servings to 8: x2 = 400 g.
    has("recipe-scale", &[200.0, 4.0, 8.0], "400");
    // Baker's %: 300 g ingredient on 500 g flour = 60%.
    has("bakers-percent", &[500.0, 300.0, 60.0], "300");
    // 500 mL at 5%: 25 mL pure alcohol = 19.7 g = 1.41 US standard drinks.
    has("alcohol-units", &[500.0, 5.0], "19.7");
    has("alcohol-units", &[500.0, 5.0], "1.41");
    // Coffee 30 g at 1:16 -> 480 g water; a 500 g brew needs 31.25... no:
    // 30 g coffee at ratio 16 = 480 water.
    has("coffee-water-ratio", &[30.0, 16.0], "480");
    // Meat 5 lb at 20 min/lb = 100 min.
    has("meat-cooking-time", &[5.0, 20.0], "100");
}

#[test]
fn ecology_and_other() {
    // 200 km at 12.5 km/L = 16 L; x 2.31 kg CO2/L = 36.96 kg.
    has("carbon-driving", &[200.0, 12.5, 2.31], "36.96");
    // Solar 4000 W panels... input is panel W x sun hours x 0.75 factor:
    // 4000 x 5 x 0.75 / 1000 = 15 kWh/day; 5475 kWh/year; 4.50/day at 0.30.
    has("solar-output", &[4000.0, 5.0, 0.3], "15");
    has("solar-output", &[4000.0, 5.0, 0.3], "5475");
    // Rain: 100 m2 x 10 mm x 0.8 efficiency = 800 L.
    has("rainwater", &[100.0, 10.0], "800");
    // Chlorine: 20000 gal at 2 ppm needs 8 oz of 10% liquid chlorine...
    // per the calculator's published dosing table value.
    has("pool-chlorine", &[20000.0, 2.0], "8");
    // Trees: 10 trees x 21 kg/yr x 5 yr = 1050 kg = 1.05 tonnes.
    has("tree-oxygen-co2", &[10.0, 5.0], "1050");
    // Lighting: 10 W x 5 h x 4 bulbs x 365 = 73 kWh/yr; at 0.20 = 14.60.
    has("lighting-cost-year", &[10.0, 5.0, 4.0, 0.2], "73");
    has("lighting-cost-year", &[10.0, 5.0, 4.0, 0.2], "14.60");
    // Working hours 9:00-17:30 minus 30 min break = 8 h 0 min.
    has("working-hours-pay", &[9.0, 0.0, 17.0, 30.0, 30.0], "8");
    // Gross profit: 5000 - 3000 = 2000, margin 40%.
    has("gross-profit-business", &[5000.0, 3000.0], "2000");
    has("gross-profit-business", &[5000.0, 3000.0], "40");
    // Price per unit: 24 per case of 12 -> 2.00/unit; 25 needed -> 3 cases.
    has("price-per-unit-other", &[24.0, 12.0, 25.0], "2");
    // Shrinkage: 10000 recorded vs 9500 actual = 500 = 5%.
    has("shrinkage", &[10000.0, 9500.0], "500");
    has("shrinkage", &[10000.0, 9500.0], "5");
    // Circumference of diameter 10 = 31.415927.
    has("circumference", &[10.0], "31.415927");
}
