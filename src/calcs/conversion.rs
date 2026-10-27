//! Conversion calculators (Omni: Conversion). Parametric engine: ONE
//! calculator per dimension. Inputs are [value, from-unit number,
//! to-unit number]; the numbered unit table is in each description —
//! exactly the collapsing documented in docs/COVERAGE.md (Omni lists a
//! separate page for every pair; here the pair is an input).
use crate::{a, chk, nd, Calc};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

/// Build a linear converter (factor = how many base units in 1 unit).
fn linear(
    v: &mut Vec<Calc>,
    id: &'static str,
    name: &'static str,
    hint: &'static str,
    family: &'static str,
    units: &'static [(&'static str, f64)],
) {
    let table = units
        .iter()
        .enumerate()
        .map(|(i, (u, _))| format!("{i}={u}"))
        .collect::<Vec<_>>()
        .join(", ");
    let desc: &'static str = Box::leak(
        format!(
            "Convert {hint}. Inputs: value, FROM unit number, TO unit number. Units: {table}. Formula: result = value x factor(from) / factor(to)."
        )
        .into_boxed_str(),
    );
    let unit_names: &'static [&'static str] = Box::leak(
        units
            .iter()
            .map(|(u, _)| *u)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    reg(
        v,
        Calc::new(
            id,
            name,
            "Conversion",
            desc,
            &[
                "Value",
                "From unit (number from list)",
                "To unit (number from list)",
            ],
            move |x| {
                chk(
                    a(x, 1) == (a(x, 1) as i64) as f64 && a(x, 2) == (a(x, 2) as i64) as f64,
                    "Unit numbers must be whole numbers from the list in the description (e.g. 0, 1, 2 — not 1.5).",
                )?;
                let fi = a(x, 1) as i64;
                let ti = a(x, 2) as i64;
                chk(
                    fi >= 0
                        && (fi as usize) < units.len()
                        && ti >= 0
                        && (ti as usize) < units.len(),
                    "Unit number is out of range — use a number from the list in the description.",
                )?;
                let (fu, ff) = units[fi as usize];
                let (tu, tf) = units[ti as usize];
                let r = a(x, 0) * ff / tf;
                Ok(format!(
                    "{} {} = {} {}.\nSteps: {} x {} / {} = {}.",
                    nd(a(x, 0), 6),
                    fu,
                    nd(r, 8),
                    tu,
                    nd(a(x, 0), 6),
                    nd(ff, 8),
                    nd(tf, 8),
                    nd(r, 8)
                ))
            },
        )
        .with_unit_info(
            Box::leak(vec![family, "", ""].into_boxed_slice()),
            &["", "", ""],
            Box::leak(vec![family, "", ""].into_boxed_slice()),
        )
        .with_converter_units(unit_names),
    );
}

pub fn register(v: &mut Vec<Calc>) {
    linear(
        v,
        "convert-length",
        "Length Converter",
        "length",
        "length",
        &[
            ("millimetres", 0.001),
            ("centimetres", 0.01),
            ("metres", 1.0),
            ("kilometres", 1000.0),
            ("inches", 0.0254),
            ("feet", 0.3048),
            ("yards", 0.9144),
            ("miles", 1609.344),
            ("nautical miles", 1852.0),
            ("astronomical units", 1.495978707e11),
            ("light years", 9.4607304725808e15),
            ("parsecs", 3.085677581491367e16),
        ],
    );
    linear(
        v,
        "convert-mass",
        "Mass / Weight Converter",
        "mass",
        "mass",
        &[
            ("milligrams", 0.001),
            ("grams", 1.0),
            ("kilograms", 1000.0),
            ("metric tonnes", 1.0e6),
            ("ounces", 28.349523125),
            ("pounds", 453.59237),
            ("stones", 6350.29318),
            ("US tons", 907184.74),
            ("imperial tons", 1016046.9088),
        ],
    );
    linear(
        v,
        "convert-area",
        "Area Converter",
        "area",
        "area",
        &[
            ("square metres", 1.0),
            ("square centimetres", 0.0001),
            ("hectares", 10000.0),
            ("square kilometres", 1.0e6),
            ("square inches", 0.00064516),
            ("square feet", 0.09290304),
            ("square yards", 0.83612736),
            ("acres", 4046.8564224),
            ("square miles", 2589988.110336),
        ],
    );
    linear(
        v,
        "convert-volume",
        "Volume Converter",
        "volume",
        "volume",
        &[
            ("millilitres", 0.001),
            ("litres", 1.0),
            ("cubic metres", 1000.0),
            ("cubic centimetres", 0.001),
            ("teaspoons (US)", 0.00492892159375),
            ("tablespoons (US)", 0.01478676478125),
            ("fluid ounces (US)", 0.0295735295625),
            ("cups (US)", 0.2365882365),
            ("pints (US)", 0.473176473),
            ("quarts (US)", 0.946352946),
            ("gallons (US)", 3.785411784),
            ("gallons (UK)", 4.54609),
            ("cubic inches", 0.016387064),
            ("cubic feet", 28.316846592),
        ],
    );
    linear(
        v,
        "convert-speed",
        "Speed Converter",
        "speed",
        "speed",
        &[
            ("m/s", 1.0),
            ("km/h", 1.0 / 3.6),
            ("mph", 0.44704),
            ("knots", 0.514444),
            ("feet/s", 0.3048),
            ("Mach (at sea level, approx)", 340.29),
        ],
    );
    linear(
        v,
        "convert-time",
        "Time Converter",
        "time",
        "time",
        &[
            ("milliseconds", 0.001),
            ("seconds", 1.0),
            ("minutes", 60.0),
            ("hours", 3600.0),
            ("days", 86400.0),
            ("weeks", 604800.0),
            ("years (365 d)", 31536000.0),
        ],
    );
    linear(
        v,
        "convert-data",
        "Data Storage Converter",
        "data size (decimal SI: 1 KB = 1000 bytes; binary values are separate units)",
        "",
        &[
            ("bytes", 1.0),
            ("KB (1000 B)", 1e3),
            ("MB", 1e6),
            ("GB", 1e9),
            ("TB", 1e12),
            ("KiB (1024 B)", 1024.0),
            ("MiB", 1048576.0),
            ("GiB", 1073741824.0),
            ("TiB", 1099511627776.0),
            ("bits", 0.125),
        ],
    );
    linear(
        v,
        "convert-pressure",
        "Pressure Converter",
        "pressure",
        "pressure",
        &[
            ("pascals", 1.0),
            ("kilopascals", 1000.0),
            ("bars", 100000.0),
            ("atmospheres", 101325.0),
            ("psi", 6894.757),
            ("mmHg (torr)", 133.322),
            ("inches of water", 249.082),
        ],
    );
    linear(
        v,
        "convert-energy",
        "Energy Converter",
        "energy",
        "energy",
        &[
            ("joules", 1.0),
            ("kilojoules", 1000.0),
            ("calories (thermochemical)", 4.184),
            ("kilocalories (food Cal)", 4184.0),
            ("watt-hours", 3600.0),
            ("kilowatt-hours", 3.6e6),
            ("BTU", 1055.05585),
            ("foot-pounds", 1.355818),
        ],
    );
    linear(
        v,
        "convert-power-units",
        "Power Converter",
        "power",
        "power",
        &[
            ("watts", 1.0),
            ("kilowatts", 1000.0),
            ("horsepower (mech)", 745.699872),
            ("horsepower (metric)", 735.49875),
            ("BTU/hour", 0.29307107),
        ],
    );
    linear(
        v,
        "convert-force-units",
        "Force Converter",
        "force",
        "force",
        &[
            ("newtons", 1.0),
            ("kilonewtons", 1000.0),
            ("dynes", 1e-5),
            ("pounds-force", 4.4482216152605),
            ("kilograms-force", 9.80665),
        ],
    );
    linear(
        v,
        "convert-torque-units",
        "Torque Converter",
        "torque",
        "",
        &[
            ("newton-metres", 1.0),
            ("foot-pounds", 1.3558179483314),
            ("inch-pounds", 0.11298482902762),
            ("kilogram-metres", 9.80665),
        ],
    );
    linear(
        v,
        "convert-density-units",
        "Density Converter",
        "density",
        "density",
        &[
            ("kg/m^3", 1.0),
            ("g/cm^3", 1000.0),
            ("g/mL", 1000.0),
            ("lb/ft^3", 16.018463),
            ("lb/gal (US)", 119.8264),
        ],
    );
    linear(
        v,
        "convert-frequency",
        "Frequency Converter",
        "frequency",
        "frequency",
        &[
            ("hertz", 1.0),
            ("kilohertz", 1e3),
            ("megahertz", 1e6),
            ("gigahertz", 1e9),
            ("rpm", 1.0 / 60.0),
        ],
    );
    linear(
        v,
        "convert-flow",
        "Flow Rate Converter",
        "volume flow",
        "",
        &[
            ("litres/second", 1.0),
            ("litres/minute", 1.0 / 60.0),
            ("cubic metres/hour", 1000.0 / 3600.0),
            ("gallons (US)/minute", 3.785411784 / 60.0),
            ("cubic feet/second", 28.316846592),
        ],
    );
    linear(
        v,
        "convert-illuminance",
        "Illuminance Converter",
        "light",
        "",
        &[("lux", 1.0), ("foot-candles", 10.7639), ("phot", 10000.0)],
    );
    // Temperature — special (offset) formulas, unit numbers: 0=C, 1=F, 2=K.
    reg(v, Calc::new("convert-temperature", "Temperature Converter", "Conversion",
        "Inputs: value, FROM unit number, TO unit number. Units: 0=Celsius, 1=Fahrenheit, 2=Kelvin. Formulas: F = C x 9/5 + 32; K = C + 273.15.", &["Value", "From unit (0 C, 1 F, 2 K)", "To unit (0 C, 1 F, 2 K)"],
        |x| {
            chk(a(x,1)==(a(x,1) as i64) as f64 && a(x,2)==(a(x,2) as i64) as f64, "Unit numbers must be whole numbers: 0, 1 or 2.")?;
            let f = a(x,1) as i64; let t = a(x,2) as i64;
            let c = match f { 0 => a(x,0), 1 => (a(x,0)-32.0)*5.0/9.0, 2 => a(x,0)-273.15, _ => return Err("From-unit must be 0, 1 or 2.".into()) };
            let (r, name) = match t { 0 => (c, "Celsius"), 1 => (c*9.0/5.0+32.0, "Fahrenheit"), 2 => (c+273.15, "Kelvin"), _ => return Err("To-unit must be 0, 1 or 2.".into()) };
            Ok(format!("Result: {} {}. (Via Celsius: {} C.)", nd(r,4), name, nd(c,4)))
        })
        .with_unit_info(&["temperature", "", ""], &["Celsius", "", ""], &["temperature", "", ""])
        .with_converter_units(&["Celsius", "Fahrenheit", "Kelvin"]));
    // Fuel economy — inverse relationship, dedicated calc.
    reg(v, Calc::new("convert-fuel-economy", "Fuel Economy Converter (mpg <-> L/100km)", "Conversion",
        "US mpg and L/100km are inverses: L/100km = 235.215 / mpg(US). UK mpg = 282.481 / (L/100km). Enter a US-mpg value OR an L/100km value — the other input 0 is ignored (give exactly one).", &["US mpg (or 0)", "L/100km (or 0)"],
        |x| {
            if a(x,0) > 0.0 && a(x,1) == 0.0 { return Ok(format!("{} US mpg = {} L/100km = {} UK mpg.", nd(a(x,0),3), nd(235.215/a(x,0),3), nd(a(x,0)*1.20095,2))); }
            if a(x,1) > 0.0 && a(x,0) == 0.0 { return Ok(format!("{} L/100km = {} US mpg = {} UK mpg.", nd(a(x,1),3), nd(235.215/a(x,1),3), nd(282.481/a(x,1),2))); }
            Err("Give exactly one of the two values (the other must be 0).".into())
        }));

    // ---- v0.4.0 (#8): compound-unit converters ----
    linear(
        v,
        "compound-density",
        "Density Converter (Compound Units)",
        "density (mass per volume)",
        "density",
        &[
            ("kg/m^3", 1.0),
            ("g/cm^3", 1000.0),
            ("g/mL", 1000.0),
            ("kg/L", 1000.0),
            ("g/L", 1.0),
            ("lb/ft^3", 16.018463),
            ("lb/gal (US)", 119.8264),
        ],
    );
    linear(
        v,
        "compound-data-rate",
        "Data Rate Converter (Compound Units)",
        "data rate (bits and bytes per second; 1 byte = 8 bits)",
        "",
        &[
            ("bit/s", 1.0e-6),
            ("Kbit/s", 0.001),
            ("Mbit/s (Mbps)", 1.0),
            ("Gbit/s", 1000.0),
            ("KB/s", 0.008),
            ("MB/s", 8.0),
            ("GB/s", 8000.0),
            ("KiB/s", 0.008192),
            ("MiB/s", 8.388608),
            ("GiB/s", 8589.934592),
        ],
    );
    linear(
        v,
        "compound-concentration",
        "Concentration Converter (Compound Units)",
        "concentration (mass per volume; % w/v = grams per 100 mL, ppm here = mg/L)",
        "",
        &[
            ("mg/mL", 1.0),
            ("g/L", 1.0),
            ("kg/m^3", 1.0),
            ("% w/v", 10.0),
            ("ppm (mg/L)", 0.001),
            ("\u{b5}g/mL", 0.001),
            ("g/mL", 1000.0),
        ],
    );
    linear(
        v,
        "compound-energy-density",
        "Energy Density Converter (Compound Units)",
        "energy per volume (dimensionally the same as pressure: 1 J/m^3 = 1 Pa)",
        "",
        &[
            ("J/m^3", 1.0),
            ("kJ/m^3", 1000.0),
            ("MJ/m^3", 1.0e6),
            ("kWh/m^3", 3.6e6),
            ("J/L", 1000.0),
            ("kJ/L", 1.0e6),
            ("BTU/ft^3", 37258.9),
        ],
    );
    // Fuel consumption — inverse relationships, dedicated calc (#8).
    reg(v, Calc::new("compound-fuel-consumption", "Fuel Consumption Converter (Compound Units)", "Conversion",
        "Compound fuel-consumption units are inverses: L/100km = 235.214583 / mpg (US) = 282.480936 / mpg (UK); km/L = 100 / (L/100km). Inputs: value, FROM unit number, TO unit number. Units: 0=L/100km, 1=mpg (US), 2=mpg (UK), 3=km/L.", &["Value", "From unit (number from list)", "To unit (number from list)"],
        |x| {
            chk(a(x,0) > 0.0, "Fuel consumption must be over 0 (these units are inverses — 0 and negatives have no conversion).")?;
            chk(a(x,1)==(a(x,1) as i64) as f64 && a(x,2)==(a(x,2) as i64) as f64, "Unit numbers must be whole numbers: 0, 1, 2 or 3.")?;
            let f = a(x,1) as i64; let t = a(x,2) as i64;
            let names = ["L/100km", "mpg (US)", "mpg (UK)", "km/L"];
            chk((0..=3).contains(&f) && (0..=3).contains(&t), "Unit number is out of range — use 0, 1, 2 or 3 from the list in the description.")?;
            let l100 = match f { 0 => a(x,0), 1 => 235.214583/a(x,0), 2 => 282.480936/a(x,0), _ => 100.0/a(x,0) };
            let r = match t { 0 => l100, 1 => 235.214583/l100, 2 => 282.480936/l100, _ => 100.0/l100 };
            Ok(format!("{} {} = {} {}.\nSteps: via L/100km = {}.", nd(a(x,0),6), names[f as usize], nd(r,6), names[t as usize], nd(l100,6)))
        }));
    // Cooking: grams <-> cups by ingredient density (g per US cup, standard published table values).
    let ingredients: &[(&str, &str, f64)] = &[
        (
            "grams-cups-flour",
            "Grams <-> Cups: Flour (All-Purpose)",
            125.0,
        ),
        (
            "grams-cups-sugar",
            "Grams <-> Cups: Sugar (Granulated)",
            200.0,
        ),
        (
            "grams-cups-brown-sugar",
            "Grams <-> Cups: Brown Sugar (Packed)",
            220.0,
        ),
        ("grams-cups-butter", "Grams <-> Cups: Butter", 227.0),
        ("grams-cups-cocoa", "Grams <-> Cups: Cocoa Powder", 100.0),
        ("grams-cups-rice", "Grams <-> Cups: Rice (Uncooked)", 185.0),
        ("grams-cups-oats", "Grams <-> Cups: Rolled Oats", 90.0),
        ("grams-cups-honey", "Grams <-> Cups: Honey", 340.0),
        ("grams-cups-milk", "Grams <-> Cups: Milk", 245.0),
        ("grams-cups-water", "Grams <-> Cups: Water", 236.6),
        ("grams-cups-oil", "Grams <-> Cups: Vegetable Oil", 218.0),
        (
            "grams-cups-powdered-sugar",
            "Grams <-> Cups: Powdered Sugar",
            120.0,
        ),
    ];
    for (id, name, g_per_cup) in ingredients {
        let g = *g_per_cup;
        let desc: &'static str = Box::leak(format!(
            "Uses the standard table value of {g} g per US cup for this ingredient. Give an amount and read both ways: grams -> cups (g / {g}) and cups -> grams (cups x {g}) — enter grams in input 1, or cups in input 2 (the other 0). Densities vary by brand/how packed; labelled estimate."
        ).into_boxed_str());
        reg(
            v,
            Calc::new(
                id,
                name,
                "Conversion",
                desc,
                &["Grams (or 0)", "Cups (or 0)"],
                move |x| {
                    if a(x, 0) > 0.0 && a(x, 1) == 0.0 {
                        return Ok(format!(
                            "{} g = {} cups.",
                            nd(a(x, 0), 2),
                            nd(a(x, 0) / g, 4)
                        ));
                    }
                    if a(x, 1) > 0.0 && a(x, 0) == 0.0 {
                        return Ok(format!(
                            "{} cups = {} g.",
                            nd(a(x, 1), 3),
                            nd(a(x, 1) * g, 2)
                        ));
                    }
                    Err("Give exactly one value (grams OR cups); the other must be 0.".into())
                },
            ),
        );
    }
    // Shoe size — lookup table US men's <-> EU / UK / cm (standard published chart, main sizes).
    reg(v, Calc::new("shoe-size-men", "Shoe Size Converter (Men, US -> EU/UK/cm)", "Conversion",
        "Standard published chart for whole and half sizes from US 6 to 14. Enter US men's size (e.g. 9.5). Chart: US6=EU38.5/UK5.5/24.5cm, 7=40/6/25, 7.5=40.5/6.5/25.4, 8=41/7/25.7, 8.5=42/7.5/26, 9=42.5/8/26.7, 9.5=43/8.5/27.1, 10=44/9/27.4, 10.5=44.5/9.5/27.8, 11=45/10/28.3, 12=46/11/29.2, 13=47.5/12/30, 14=48.5/13/30.8. Nearest listed size is used.", &["US men's size"],
        |x| {
            let chart: [(f64,&str,&str,&str);15] = [(6.0,"38.5","5.5","24.5"),(6.5,"39","6","24.8"),(7.0,"40","6","25.0"),(7.5,"40.5","6.5","25.4"),(8.0,"41","7","25.7"),(8.5,"42","7.5","26.0"),(9.0,"42.5","8","26.7"),(9.5,"43","8.5","27.1"),(10.0,"44","9","27.4"),(10.5,"44.5","9.5","27.8"),(11.0,"45","10","28.3"),(11.5,"45.5","10.5","28.6"),(12.0,"46","11","29.2"),(13.0,"47.5","12","30.0"),(14.0,"48.5","13","30.8")];
            let mut best = &chart[0]; let mut bd = f64::MAX;
            for row in &chart { let d = (row.0 - a(x,0)).abs(); if d < bd { bd = d; best = row; } }
            Ok(format!("US men's {} ≈ EU {}, UK {}, foot {} cm (nearest chart size US {}). Charts differ slightly by brand.", nd(a(x,0),1), best.1, best.2, best.3, best.0))
        }));
}
