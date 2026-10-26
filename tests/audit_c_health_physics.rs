//! v0.1.2 audit tests — Health and Physics.
//! Expected values computed independently (Python/hand) from the
//! standard published formulas (Mifflin-St Jeor, Harris-Benedict,
//! Devine, US Navy body fat, Karvonen, Widmark; g = 9.80665 m/s^2).
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
fn health_body() {
    // BMI imperial: 703 x 154 / 68^2 = 23.41.
    has("bmi-imperial", &[154.0, 68.0], "23.41");
    // Mifflin male 70/175/30: 700 + 1093.75 - 150 + 5 = 1648.75 -> 1649.
    has("bmr-male", &[70.0, 175.0, 30.0], "1649");
    // Mifflin female 60/165/30: 600 + 1031.25 - 150 - 161 = 1320.
    has("bmr-female", &[60.0, 165.0, 30.0], "1320");
    // Harris-Benedict male 70/175/30: 66.5+962.5+875.525-202.5 = 1702.
    has("bmr-harris-male", &[70.0, 175.0, 30.0], "1702");
    // Harris-Benedict female 60/165/30: 655.1+573.78+305.25-140.28 = 1394.
    has("bmr-harris-female", &[60.0, 165.0, 30.0], "1394");
    // TDEE = BMR x activity factor: 1600 x 1.55 = 2480.
    has("tdee", &[1600.0, 1.55], "2480");
    // Lose 0.5 kg over 35 days: adjustment = -0.5 x 7700 / 35 = -110/day,
    // so eat 2500 - 110 = 2390 cal/day.
    has("calorie-target", &[2500.0, -0.5, 35.0], "2390");
    // US Navy male: waist 86, neck 38, height 178 -> 17.2%.
    has("body-fat-navy-male", &[86.0, 38.0, 178.0], "17.2");
    // US Navy female: waist 70, hip 95, neck 34, height 165 -> 23.81%.
    has("body-fat-navy-female", &[70.0, 95.0, 34.0, 165.0], "23.81");
    // Devine male at 5'7" (170.18 cm): 50 + 2.3 x 7 = 66.1 kg.
    has("ideal-weight-devine", &[1.0, 170.18], "66.1");
    // BUG PIN (v0.1.2 fix): Devine is SIGNED per inch from 5 ft. At
    // 4'10" (147.32 cm) a man is 50 - 2.3 x 2 = 45.4 kg. The old code
    // clamped inches-over-5ft at 0 and silently answered 50 kg.
    has("ideal-weight-devine", &[1.0, 147.32], "45.4");
}

#[test]
fn health_activity() {
    // Age 40: estimated max HR 180; fat-burn zone 60-70% = 108-126.
    has("target-heart-rate", &[40.0], "108");
    has("target-heart-rate", &[40.0], "126");
    // Karvonen: max 180, resting 60, 70% -> 60 + 0.7 x 120 = 144 bpm.
    has("karvonen", &[40.0, 60.0, 70.0], "144");
    // 10 km in 50 min: 5 min/km; 8.05 min/mile; 12 km/h.
    has("pace", &[10.0, 50.0], "8.05");
    has("pace", &[10.0, 50.0], "12");
    // Walking 3.5 METs, 70 kg, 60 min: 3.5 x 70 x 1 = 245 kcal.
    has("calories-burned-walk", &[70.0, 60.0], "245");
    // Running guide 1.05 kcal/kg/km: 70 x 10 x 1.05 = 735 kcal.
    has("calories-burned-run", &[70.0, 10.0], "735");
    // 10000 steps x 0.762 m stride = 7.62 km; ~40 kcal/1000 steps = 400.
    has("steps-distance", &[10000.0, 70.0, 0.762], "7.62");
    has("steps-distance", &[10000.0, 70.0, 0.762], "400");
    // 2000 kcal at 30/40/30: protein 150 g, carbs 200 g, fat 67 g.
    has("macro-split", &[2000.0, 30.0, 40.0, 30.0], "150");
    has("macro-split", &[2000.0, 30.0, 40.0, 30.0], "67");
    // Protein at 1.6 g per kg: 70 kg -> 112 g.
    has("protein-needs", &[70.0, 1.6], "112");
}

#[test]
fn health_estimates() {
    // LMP 2026-01-01, 28-day cycle: ovulation = LMP + 14 = 2026-01-15;
    // fertile window 2026-01-10 to 2026-01-16.
    has("ovulation", &[2026.0, 1.0, 1.0, 28.0], "2026-01-15");
    has("ovulation", &[2026.0, 1.0, 1.0, 28.0], "2026-01-10");
    // Widmark male: 40 g / (80000 g x 0.68) x 100 = 0.074%.
    has("bac-widmark", &[40.0, 80.0, 1.0, 0.0], "0.074");
    // 8 hours: five full 90-minute cycles fit (7.5 h).
    has("sleep-cycles", &[8.0], "5");
}

#[test]
fn physics_motion() {
    has("speed-from-distance-time", &[60.0, 5.0], "12");
    has("distance-from-speed-time", &[12.0, 5.0], "60");
    has("time-from-speed-distance", &[100.0, 20.0], "5");
    // Inputs are x1, x2, t1, t2: (50-0)/(10-0) = 5.
    has("average-velocity", &[0.0, 50.0, 0.0, 10.0], "5");
    // BUG PIN (v0.1.2 fix): the zero-check tested t2 - x2 instead of
    // t2 - t1. Equal times (t1 = t2 = 5) slipped through and printed
    // an infinite velocity; now a teaching error.
    err("average-velocity", &[0.0, 50.0, 5.0, 5.0], "must differ");
    // ...and the mirror image: x2 == t2 made the old check fire a
    // false error on a perfectly good problem. (5-0)/(5-0) = 1.
    has("average-velocity", &[0.0, 5.0, 0.0, 5.0], "1");
    // a = (20-0)/4 = 5; v = 5 + 2x3 = 11, s = 5x3 + 0.5x2x9 = 24.
    has("acceleration", &[0.0, 20.0, 4.0], "5");
    has("final-velocity", &[5.0, 2.0, 3.0], "11");
    has("final-velocity", &[5.0, 2.0, 3.0], "24");
    // Free fall 2 s: v = 2g = 19.6133 m/s, distance = 19.6133 m.
    has("free-fall", &[2.0], "19.6133");
}

#[test]
fn physics_energy_force() {
    // m = W/g = 98.0665 / 9.80665 = 10 kg.
    has("mass-from-weight", &[98.0665], "10");
    // PE = 2 x 9.80665 x 10 = 196.133 J.
    has("potential-energy", &[2.0, 10.0], "196.133");
    // KE = 9 J, PE = 196.133 J, total 205.133 J.
    has("mechanical-energy", &[2.0, 3.0, 10.0], "205.133");
    // P = 1000/20 = 50 W = 0.0671 hp.
    has("power-physics", &[1000.0, 20.0], "50");
    has("power-physics", &[1000.0, 20.0], "0.0671");
    has("work", &[50.0, 3.0], "150");
    has("density-physics", &[100.0, 20.0], "5");
    // Pressure = 100/2 = 50 Pa = 0.05 kPa.
    has("pressure", &[100.0, 2.0], "50");
    has("pressure", &[100.0, 2.0], "0.05");
    // Momentum = 6 kg m/s; KE = 9 J.
    has("momentum", &[2.0, 3.0], "6");
    has("momentum", &[2.0, 3.0], "9");
    // Torque = 50 x 0.4 = 20 N m.
    has("torque", &[50.0, 0.4], "20");
    // Normal force = 98.0665 N; friction = 0.3 x that = 29.42 N.
    has("friction", &[0.3, 10.0], "29.42");
    // Hooke: F = 200 x 0.1 = 20 N; PE = 1 J.
    has("hooke", &[200.0, 0.1], "20");
    // Gravity between two 1 kg masses 1 m apart: G = 6.674e-11 N.
    has("gravity-force", &[1.0, 1.0, 1.0], "6.674");
    // Centripetal: a = 3^2/1.5 = 6 m/s^2; F = 2 x 6 = 12 N.
    has("centripetal", &[2.0, 3.0, 1.5], "6");
    has("centripetal", &[2.0, 3.0, 1.5], "12");
    // Projectile 20 m/s at 45 deg: range = 400/9.80665 = 40.7886 m;
    // max height = 200/19.6133 = 10.1972 m.
    has("projectile", &[20.0, 45.0], "40.7886");
    has("projectile", &[20.0, 45.0], "10.1972");
    // Pendulum L=1 m: T = 2 pi sqrt(1/9.80665) = 2.006409 s.
    has("pendulum", &[1.0], "2.006409");
    // Wave: v = 10 Hz x 2 m = 20 m/s; period 0.1 s.
    has("wave", &[10.0, 2.0], "20");
    has("wave", &[10.0, 2.0], "0.1");
}

#[test]
fn physics_electric() {
    // From V=12, I=3: R = 4 ohms, P = 36 W.
    has("ohms-solve-i", &[12.0, 3.0], "4");
    has("ohms-solve-i", &[12.0, 3.0], "36");
    // From I=3, R=4: V = 12 V, P = 36 W.
    has("ohms-solve-v", &[3.0, 4.0], "12");
    has("ohms-solve-v", &[3.0, 4.0], "36");
    // 230 V x 2 A = 460 W; over 3 h = 1.38 kWh.
    has("electrical-power", &[230.0, 2.0, 3.0], "460");
    has("electrical-power", &[230.0, 2.0, 3.0], "1.38");
    // 6 || 3 = 2 ohms; series = 9 ohms.
    has("resistors-parallel", &[6.0, 3.0], "2");
    has("resistors-parallel", &[6.0, 3.0], "9");
}
