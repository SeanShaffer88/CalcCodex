//! Known-answer tests: at least 30 calculators across every category,
//! checked against hand-computed / published values.
use calcdex::{all_calcs, run_calc};

fn run(id: &str, args: &[f64]) -> String {
    run_calc(id, args).unwrap_or_else(|e| panic!("{id} failed: {e}"))
}
fn has(id: &str, args: &[f64], want: &str) {
    let out = run(id, args);
    assert!(
        out.contains(want),
        "{id} {args:?} -> '{out}' should contain '{want}'"
    );
}

#[test]
fn registry_is_big_and_unique() {
    let calcs = all_calcs();
    assert!(
        calcs.len() >= 180,
        "registry has {} calculators, floor is 180",
        calcs.len()
    );
    let mut ids: Vec<&str> = calcs.iter().map(|c| c.id).collect();
    let total = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(total, ids.len(), "duplicate calculator ids exist");
    // Every Omni top-level category must be present by name somewhere.
    for cat in [
        "Math",
        "Finance",
        "Health",
        "Physics",
        "Chemistry",
        "Construction",
        "Conversion",
        "Statistics",
        "Sports",
        "Food",
        "Ecology",
        "Everyday",
    ] {
        assert!(
            calcs
                .iter()
                .any(|c| c.category == cat || c.category.starts_with(cat)),
            "category {cat} is empty"
        );
    }
}

#[test]
fn math_known_answers() {
    has("percent-of", &[25.0, 200.0], "= 50");
    has("percent-change", &[100.0, 125.0], "Change: 25%");
    has("quadratic", &[1.0, -3.0, 2.0], "x1 = 2");
    has("quadratic", &[1.0, 0.0, 1.0], "Complex roots");
    has("pythagorean", &[3.0, 4.0], "= 5");
    has("circle", &[1.0], "Area = 3.141593");
    has("factorial", &[5.0], "5! = 120");
    has("log", &[1000.0, 10.0], "= 3");
    has("gcd", &[48.0, 36.0], "= 12");
    has("lcm", &[4.0, 6.0], "= 12");
    has("fraction-add", &[1.0, 2.0, 1.0, 3.0], "5/6");
    has("slope", &[0.0, 0.0, 2.0, 4.0], "Slope = 2");
    has("nth-root", &[27.0, 3.0], "= 3");
}

#[test]
fn finance_known_answers() {
    // 1000 at 5% for 10 years, yearly compounding = 1628.89
    has("compound-interest", &[1000.0, 5.0, 10.0], "1628.89");
    has("simple-interest", &[1000.0, 5.0, 10.0], "Interest: 500");
    // Loan: 10000 at 0% for 1 year -> 833.33/month
    has("loan-payment", &[10000.0, 0.0, 1.0], "833.33");
    // Mortgage P&I on 200k at 0% 30y = 555.56, tax/ins 0
    has(
        "mortgage-payment",
        &[200000.0, 0.0, 0.0, 30.0, 0.0, 0.0, 0.0],
        "555.56",
    );
    has("discount", &[100.0, 20.0], "Sale price: 80");
    has("tip-split", &[100.0, 20.0, 4.0], "Each person pays: 30");
    has("salary-hourly", &[52000.0, 2080.0], "Hourly wage: 25");
    has("rule-of-72", &[8.0], "9 years");
    has("cagr", &[100.0, 121.0, 2.0], "10%");
}

#[test]
fn health_known_answers() {
    // BMI: 70 kg, 175 cm -> 22.86
    has("bmi", &[70.0, 175.0], "22.86");
    has("bmi", &[70.0, 175.0], "healthy range");
    // BMR male Mifflin: 10*70+6.25*175-5*30+5 = 1645 (rounded 0dp -> 1649? compute: 700+1093.75-150+5=1648.75 -> 1649)
    has("bmr-male", &[70.0, 175.0, 30.0], "1649");
    has("water-intake", &[70.0], "2.45");
    // Due date: LMP 2025-01-01 + 280 days = 2025-10-08
    has("due-date", &[2025.0, 1.0, 1.0], "2025-10-08");
}

#[test]
fn physics_known_answers() {
    has("speed-from-distance-time", &[100.0, 20.0], "= 5");
    has("force", &[10.0, 2.0], "20 N");
    has("kinetic-energy", &[2.0, 3.0], "9 J");
    has("ohms-law", &[12.0, 4.0], "Current I = 3 A");
    has("weight-from-mass", &[1.0], "9.806");
    has("pendulum", &[9.80665], "Period = 6.283");
}

#[test]
fn chemistry_known_answers() {
    has("moles-from-mass", &[18.0, 18.0], "1 mol");
    has("ph", &[0.01], "pH = 2");
    // Ideal gas: 1 mol, 273.15 K, 22.414 L -> ~1 atm
    let out = run("ideal-gas-pressure", &[1.0, 273.15, 22.414]);
    assert!(
        out.contains("0.999") || out.contains("1 atm") || out.contains("= 1"),
        "ideal gas: {out}"
    );
    has("dilution", &[10.0, 1.0, 10.0], "M2 = 1");
    has("half-life", &[100.0, 10.0, 10.0], "Remaining: 50");
}

#[test]
fn construction_known_answers() {
    // Slab 10x10 ft, 4 in: 33.333 cu ft; 80lb bags ceil(33.333/0.6)=56
    has("concrete-slab", &[10.0, 10.0, 4.0, 0.0], "80 lb: 56");
    // Paint: 10x10 room, 8 ft walls, no doors/windows, 1 coat, 350 coverage: 320/350=0.91 gal
    has("paint", &[10.0, 10.0, 8.0, 0.0, 0.0, 1.0, 350.0], "0.91");
    // Stairs: rise 70, target 7, run 11 -> 10 steps, riser 7
    has("stairs", &[70.0, 7.0, 11.0], "Steps: 10");
    has("board-feet", &[1.0, 12.0, 1.0, 1.0, 0.0], "Per board: 1");
    has("square-footage-rect", &[10.0, 12.0], "120");
}

#[test]
fn conversion_known_answers() {
    // 1 mile -> km: value 1, from 7 (miles), to 3 (km)
    has("convert-length", &[1.0, 7.0, 3.0], "1.609344");
    // 100 C -> F = 212
    has("convert-temperature", &[100.0, 0.0, 1.0], "212 Fahrenheit");
    // 1 kg -> lb: mass units 2=kg, 5=lb -> 2.204623
    has("convert-mass", &[1.0, 2.0, 5.0], "2.204622");
    // Flour: 250 g -> 2 cups
    has("grams-cups-flour", &[250.0, 0.0], "2 cups");
    // 30 US mpg -> 7.84 L/100km
    has("convert-fuel-economy", &[30.0, 0.0], "7.84");
}

#[test]
fn stats_known_answers() {
    has("mean", &[1.0, 2.0, 3.0, 4.0], "Mean: 2.5");
    has("median", &[1.0, 2.0, 3.0], "Median: 2");
    has(
        "std-population",
        &[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0],
        "standard deviation: 2",
    );
    has("combinations", &[5.0, 2.0], "= 10");
    has("permutations", &[5.0, 2.0], "= 20");
    has("probability", &[1.0, 6.0], "16.6667%");
    has("z-score", &[85.0, 70.0, 15.0], "z-score: 1");
    has(
        "expected-value",
        &[10.0, 0.5, 20.0, 0.5],
        "Expected value: 15",
    );
}

#[test]
fn dates_known_answers() {
    // Days between 2025-01-01 and 2025-01-10 = 9
    has(
        "days-between-dates",
        &[2025.0, 1.0, 1.0, 2025.0, 1.0, 10.0],
        "Days between: 9",
    );
    // 2025-01-01 + 30 days = 2025-01-31
    has("date-plus-days", &[2025.0, 1.0, 1.0, 30.0], "2025-01-31");
    // 2025-10-25 was a Saturday
    has("weekday", &[2025.0, 10.0, 25.0], "Saturday");
    // Age: born 2000-06-15, on 2025-06-14 -> 24 years; on 2025-06-15 -> 25
    has(
        "age",
        &[2000.0, 6.0, 15.0, 2025.0, 6.0, 14.0],
        "Age: 24 years",
    );
    has(
        "age",
        &[2000.0, 6.0, 15.0, 2025.0, 6.0, 15.0],
        "Age: 25 years",
    );
    // Feb 29, 2024 exists; Feb 29, 2025 does not
    assert!(run_calc("weekday", &[2025.0, 2.0, 29.0]).is_err());
    assert!(run_calc("weekday", &[2024.0, 2.0, 29.0]).is_ok());
}

#[test]
fn sports_food_eco_known_answers() {
    // Epley: 100 kg x 10 reps -> 133.3
    has("one-rep-max", &[100.0, 10.0], "133.3");
    // Oven: 200C -> 392F
    has("oven-temp", &[200.0], "392 F");
    // Pool: 10x5x1.5 = 75 m3 -> 75000 litres
    has("pool-volume", &[10.0, 5.0, 1.5], "75000 litres");
    // Dough: 300g ball 60% hydration -> flour = 300/1.625 * 1 = 184.6 -> 185 per formatting nd(...,0)
    has("pizza-dough", &[300.0, 60.0, 1.0], "flour 185 g");
}

#[test]
fn errors_are_clear_and_never_panic() {
    assert!(run_calc("square-root", &[-4.0]).is_err());
    assert!(run_calc("log", &[-1.0, 10.0]).is_err());
    assert!(run_calc("loan-payment", &[1000.0, 5.0]).is_err()); // too few numbers
    assert!(run_calc("no-such-calc", &[1.0]).is_err());
    assert!(run_calc("bmi", &[0.0, 0.0]).is_err());
    assert!(run_calc("convert-length", &[1.0, 99.0, 0.0]).is_err());
    assert!(run_calc("mean", &[]).is_err());
}
