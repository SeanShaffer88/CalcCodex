//! Sports, Food, Ecology and Other calculators (Omni categories:
//! Sports, Food, Ecology, Other). Estimates are labelled.
use crate::{a, m, n, nd, nz, Calc};
use std::f64::consts::PI;

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

pub fn register(v: &mut Vec<Calc>) {
    // ---- Sports ----
    reg(
        v,
        Calc::new(
            "race-pace",
            "Race Pace & Finish Time",
            "Sports",
            "Finish time = pace (min per km) x distance (km). Also speed.",
            &["Pace (minutes per km)", "Race distance (km)"],
            |x| {
                let t = a(x, 0) * a(x, 1);
                Ok(format!(
                    "Finish time: {} minutes ({} h {} min). Speed: {} km/h.",
                    nd(t, 2),
                    (t / 60.0) as i64,
                    (t % 60.0) as i64,
                    if a(x, 0) > 0.0 {
                        nd(60.0 / a(x, 0), 2)
                    } else {
                        nd(0.0, 2)
                    }
                ))
            },
        ),
    );
    reg(v, Calc::new("race-predictor-riegel", "Race Time Predictor (Riegel)", "Sports",
        "Riegel formula: T2 = T1 x (D2/D1)^1.06. Predicts a time at a new distance from a known race result. ESTIMATE.", &["Known time (minutes)", "Known distance (km)", "New distance (km)"],
        |x| { nz(a(x,1),"Known distance")?; let t=a(x,0)*(a(x,2)/a(x,1)).powf(1.06); Ok(format!("Predicted time for {} km: {} minutes ({} h {} min). ESTIMATE.", n(a(x,2)), nd(t,2), (t/60.0) as i64, (t%60.0) as i64)) }));
    reg(v, Calc::new("swim-pace", "Swimming Pace (Per 100 m)", "Sports",
        "Pace per 100 m = total time / (distance / 100). Enter time in minutes, distance in metres.", &["Total time (minutes)", "Distance (metres)"],
        |x| { nz(a(x,1),"Distance")?; Ok(format!("Pace: {} min per 100 m. Speed: {} km/h.", nd(a(x,0)/(a(x,1)/100.0),2), nd(a(x,1)/1000.0/(a(x,0)/60.0),2))) }));
    reg(
        v,
        Calc::new(
            "cycling-speed",
            "Cycling Average Speed & Time",
            "Sports",
            "Speed = distance / time. Enter distance km, time hours (decimal).",
            &["Distance (km)", "Time (hours)"],
            |x| {
                nz(a(x, 1), "Time")?;
                Ok(format!(
                    "Average speed: {} km/h ({} mph).",
                    nd(a(x, 0) / a(x, 1), 2),
                    nd(a(x, 0) / a(x, 1) * 0.621371, 2)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "vo2max-estimate",
            "VO2 Max Estimate (Cooper 12-Minute Test)",
            "Sports",
            "Cooper formula: VO2max ≈ (distance in metres - 504.9) / 44.73. ESTIMATE.",
            &["Distance run in 12 minutes (metres)"],
            |x| {
                Ok(format!(
                    "Estimated VO2 max: {} ml/kg/min. ESTIMATE.",
                    nd((a(x, 0) - 504.9) / 44.73, 1)
                ))
            },
        ),
    );
    reg(v, Calc::new("one-rep-max", "One-Rep Max Estimate (Epley)", "Sports",
        "Epley: 1RM = weight x (1 + reps/30). ESTIMATE — do not attempt a true max unspotted based on this.", &["Weight lifted", "Reps completed"],
        |x| Ok(format!("Estimated one-rep max: {}. ESTIMATE.", nd(a(x,0)*(1.0+a(x,1)/30.0),1)))));
    reg(v, Calc::new("marathon-splits", "Even Split Times (Any Race)", "Sports",
        "Per-km split = goal time / distance, even pacing. Marathon = 42.195 km, half = 21.0975, 10K = 10, 5K = 5.", &["Goal time (minutes)", "Distance (km)"],
        |x| { nz(a(x,1),"Distance")?; Ok(format!("Even pace: {} min/km. Halfway time: {} minutes.", nd(a(x,0)/a(x,1),3), nd(a(x,0)/2.0,2))) }));
    // ---- Food ----
    reg(
        v,
        Calc::new(
            "recipe-scale",
            "Recipe Scaling",
            "Food",
            "Scaled amount = original amount x (servings wanted / servings in recipe).",
            &[
                "Original amount of one ingredient",
                "Recipe servings",
                "Servings you want",
            ],
            |x| {
                nz(a(x, 1), "Recipe servings")?;
                Ok(format!(
                    "Scale factor: {}x. Scaled amount: {}.",
                    nd(a(x, 2) / a(x, 1), 3),
                    nd(a(x, 0) * a(x, 2) / a(x, 1), 3)
                ))
            },
        ),
    );
    reg(v, Calc::new("bakers-percent", "Baker's Percentage", "Food",
        "In baking, every ingredient is a % of the flour weight: % = ingredient / flour x 100. Also shows the ingredient weight for a target %.", &["Flour weight (g)", "Ingredient weight (g)", "Target % for this ingredient"],
        |x| { nz(a(x,0),"Flour weight")?; Ok(format!("That ingredient is {}% of the flour. For {}% you would use {} g.", nd(a(x,1)/a(x,0)*100.0,2), n(a(x,2)), nd(a(x,0)*a(x,2)/100.0,1))) }));
    reg(v, Calc::new("oven-temp", "Oven Temperature (C / F / Gas Mark)", "Food",
        "F = C x 9/5 + 32. Gas Mark ≈ (F - 250) / 25 for marks 1-9 (standard chart). Enter Celsius.", &["Temperature (Celsius)"],
        |x| { let f=a(x,0)*9.0/5.0+32.0; let gm=(f-250.0)/25.0; Ok(format!("{} C = {} F. Gas Mark ≈ {} (chart value, rounded to nearest half in most charts). Fan oven: usually {} C (about 20 C lower).", n(a(x,0)), nd(f,0), nd(gm,1), n(a(x,0)-20.0))) }));
    reg(v, Calc::new("alcohol-units", "Alcohol Units in a Drink", "Food",
        "UK units = volume (ml) x ABV% / 1000. US standard drinks: 14 g pure alcohol each; alcohol grams = ml x ABV% x 0.789 / 100.", &["Drink volume (ml)", "ABV %"],
        |x| Ok(format!("UK units: {}. Pure alcohol: {} g = {} US standard drinks.", nd(a(x,0)*a(x,1)/1000.0,2), nd(a(x,0)*a(x,1)*0.789/100.0,1), nd(a(x,0)*a(x,1)*0.789/100.0/14.0,2)))));
    reg(v, Calc::new("coffee-water-ratio", "Coffee to Water Ratio", "Food",
        "Water = coffee x ratio (e.g. 1:16 means ratio 16). Or coffee = water / ratio — both directions shown from the coffee amount; also grams of coffee for a litre of water.", &["Coffee (g)", "Ratio (water per 1 coffee, e.g. 16)"],
        |x| { nz(a(x,1),"Ratio")?; Ok(format!("For {} g coffee at 1:{}: use {} g (ml) of water. For 1000 ml of water at this ratio: {} g coffee.", n(a(x,0)), n(a(x,1)), nd(a(x,0)*a(x,1),0), nd(1000.0/a(x,1),1))) }));
    reg(v, Calc::new("pizza-dough", "Pizza Dough Calculator", "Food",
        "Dough ball = flour + water (hydration%) + salt (2%) + yeast (0.5% instant, standard starting points). Flour = total dough / (1 + hydration + 0.025). Enter one ball's total weight and hydration.", &["Dough ball weight (g)", "Hydration %", "Number of balls"],
        |x| { let flour=(a(x,0)/(1.0+a(x,1)/100.0+0.025))*a(x,2); let water=flour*a(x,1)/100.0; Ok(format!("For {} ball(s): flour {} g, water {} g, salt {} g (2%), instant yeast {} g (0.5%).", n(a(x,2)), nd(flour,0), nd(water,0), nd(flour*0.02,1), nd(flour*0.005,1))) }));
    reg(v, Calc::new("meat-cooking-time", "Roast Cooking Time Estimate", "Food",
        "Common guide: minutes = weight (lb) x minutes-per-lb for your style (enter it: e.g. beef rare 15, medium 20, well 25 per lb at high heat charts vary). ESTIMATE — use a thermometer: beef/lamb 63 C+, poultry 74 C.", &["Weight (lb)", "Minutes per lb (from your recipe)"],
        |x| { let t=a(x,0)*a(x,1); Ok(format!("Estimated time: {} minutes ({} h {} min), plus resting. ESTIMATE — check the internal temperature.", nd(t,0), (t/60.0) as i64, (t%60.0) as i64)) }));
    // ---- Ecology ----
    reg(v, Calc::new("carbon-driving", "Carbon Footprint of Driving (Estimate)", "Ecology",
        "Petrol/gasoline ≈ 2.31 kg CO2 per litre burned (2.68 for diesel — enter your factor). CO2 = litres used x factor; litres = distance / economy (km per litre).", &["Distance (km)", "Economy (km per litre)", "CO2 per litre (kg, 2.31 petrol / 2.68 diesel)"],
        |x| { nz(a(x,1),"Economy")?; let litres=a(x,0)/a(x,1); Ok(format!("Fuel used: {} L. CO2: about {} kg. ESTIMATE.", nd(litres,2), nd(litres*a(x,2),2))) }));
    reg(v, Calc::new("solar-output", "Solar Panel Output Estimate", "Ecology",
        "Daily energy ≈ panel watts x peak sun hours x system factor 0.75 (losses). Enter total array watts and your area's peak sun hours (often 3-6). ESTIMATE.", &["Total panel watts", "Peak sun hours per day", "Electricity rate per kWh"],
        |x| { let kwh=a(x,0)*a(x,1)*0.75/1000.0; Ok(format!("About {} kWh/day, {} kWh/year. Value at your rate: {} per day, {} per year. ESTIMATE.", nd(kwh,2), nd(kwh*365.0,0), m(kwh*a(x,2)), m(kwh*365.0*a(x,2)))) }));
    reg(v, Calc::new("rainwater", "Rainwater Collection", "Ecology",
        "Collected litres ≈ roof area (m^2) x rainfall (mm) x 0.8 runoff factor (1 mm on 1 m^2 = 1 litre before losses).", &["Roof area (m^2)", "Rainfall (mm)"],
        |x| Ok(format!("About {} litres collected. ESTIMATE.", nd(a(x,0)*a(x,1)*0.8,1)))));
    reg(v, Calc::new("pool-volume", "Pool Volume", "Ecology",
        "Rectangular pool: volume = L x W x average depth. Enter metres for litres (x 1000), or feet for US gallons (cu ft x 7.48052). Both shown from a metre entry AND a feet entry is not mixed — this tool takes METRES for L/W/depth and ALSO shows gallons from those metres (1 m^3 = 264.172 gal).", &["Length (m)", "Width (m)", "Average depth (m)"],
        |x| { let m3=a(x,0)*a(x,1)*a(x,2); Ok(format!("Volume: {} m^3 = {} litres = {} US gallons.", nd(m3,3), nd(m3*1000.0,0), nd(m3*264.172,0))) }));
    reg(v, Calc::new("pool-chlorine", "Pool Chlorine Dose Estimate", "Ecology",
        "Common guide: raising chlorine by 1 ppm in 10,000 gallons needs about 2 oz of 65% calcium hypochlorite. Dose scales with pool gallons and wanted ppm rise. ESTIMATE — follow your product label and test the water.", &["Pool volume (US gallons)", "Wanted chlorine rise (ppm)"],
        |x| Ok(format!("About {} oz of 65% calcium hypochlorite. ESTIMATE — product labels win.", nd(a(x,0)/10000.0*a(x,1)*2.0,2)))));
    reg(v, Calc::new("tree-oxygen-co2", "Tree CO2 Absorption Estimate", "Ecology",
        "A mature tree absorbs roughly 21 kg of CO2 per year (widely cited average). Total for your trees and years. ESTIMATE.", &["Number of trees", "Years"],
        |x| Ok(format!("About {} kg of CO2 absorbed ({} tonnes). ESTIMATE.", nd(a(x,0)*a(x,1)*21.0,0), nd(a(x,0)*a(x,1)*21.0/1000.0,3)))));
    reg(
        v,
        Calc::new(
            "lighting-cost-year",
            "Lighting Cost Per Year",
            "Ecology",
            "Yearly kWh = bulb watts x hours/day x 365 / 1000 x number of bulbs.",
            &[
                "Watts per bulb",
                "Hours per day",
                "Number of bulbs",
                "Rate per kWh",
            ],
            |x| {
                let kwh = a(x, 0) * a(x, 1) * 365.0 * a(x, 2) / 1000.0;
                Ok(format!(
                    "{} kWh per year. Cost: {} per year.",
                    nd(kwh, 1),
                    m(kwh * a(x, 3))
                ))
            },
        ),
    );
    // ---- Other / business ----
    reg(v, Calc::new("working-hours-pay", "Working Hours Between Clock Times (Pay)", "Other",
        "Paid hours = end - start minus unpaid break minutes. Uses 24h clock times; crosses midnight if end < start.", &["Start hour", "Start minute", "End hour", "End minute", "Unpaid break (minutes)"],
        |x| { let s=a(x,0)*60.0+a(x,1); let mut e=a(x,2)*60.0+a(x,3); if e<s { e+=1440.0; } let paid=(e-s-a(x,4)).max(0.0); Ok(format!("Paid time: {} hours {} minutes ({} decimal hours).", (paid/60.0) as i64, (paid%60.0) as i64, nd(paid/60.0,4))) }));
    reg(v, Calc::new("gross-profit-business", "Business Gross Profit & Margin", "Other",
        "Gross profit = revenue - cost of goods sold. Gross margin = gross profit / revenue x 100.", &["Revenue", "Cost of goods sold"],
        |x| { nz(a(x,0),"Revenue")?; Ok(format!("Gross profit: {}. Gross margin: {}%.", m(a(x,0)-a(x,1)), nd((a(x,0)-a(x,1))/a(x,0)*100.0,4))) }));
    reg(v, Calc::new("price-per-unit-other", "Wholesale Case Price Per Item", "Other",
        "Per item = case price / items per case. Cases needed for a wanted count = ceil(count / per case).", &["Case price", "Items per case", "Items you need"],
        |x| { nz(a(x,1),"Items per case")?; Ok(format!("Per item: {}. Cases needed for {} items: {}. Total spend: {}.", m(a(x,0)/a(x,1)), n(a(x,2)), (a(x,2)/a(x,1)).ceil(), m((a(x,2)/a(x,1)).ceil()*a(x,0)))) }));
    reg(
        v,
        Calc::new(
            "shrinkage",
            "Inventory Shrinkage %",
            "Other",
            "Shrinkage % = (recorded stock - actual stock) / recorded stock x 100, valued at cost.",
            &["Recorded stock value", "Actual stock value"],
            |x| {
                nz(a(x, 0), "Recorded stock value")?;
                Ok(format!(
                    "Shrinkage: {} ({}%).",
                    m(a(x, 0) - a(x, 1)),
                    nd((a(x, 0) - a(x, 1)) / a(x, 0) * 100.0, 4)
                ))
            },
        ),
    );
    reg(v, Calc::new("circumference", "Circumference From Diameter", "Other",
        "C = pi x d. (Also under Math as the full circle calc — this is Omni's separate 'Circumference' listing.)", &["Diameter"],
        |x| Ok(format!("Circumference = {}. Radius = {}. Area = {}.", nd(PI*a(x,0),6), nd(a(x,0)/2.0,6), nd(PI*(a(x,0)/2.0).powi(2),6)))));
}
