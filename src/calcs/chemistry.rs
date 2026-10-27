//! Chemistry calculators (Omni: Chemistry).
use crate::{a, chk, n, nd, nz, sci, Calc};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}
const R_GAS: f64 = 0.082057; // L·atm/(mol·K)

pub fn register(v: &mut Vec<Calc>) {
    reg(
        v,
        Calc::new(
            "moles-from-mass",
            "Moles From Mass",
            "Chemistry",
            "moles = mass (g) / molar mass (g/mol).",
            &["Mass (g)", "Molar mass (g/mol)"],
            |x| {
                nz(a(x, 1), "Molar mass")?;
                Ok(format!("Amount = {} mol.", nd(a(x, 0) / a(x, 1), 6)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "mass-from-moles",
            "Mass From Moles",
            "Chemistry",
            "mass (g) = moles x molar mass (g/mol).",
            &["Moles (mol)", "Molar mass (g/mol)"],
            |x| Ok(format!("Mass = {} g.", nd(a(x, 0) * a(x, 1), 6))),
        ),
    );
    reg(
        v,
        Calc::new(
            "molecules",
            "Molecules / Atoms From Moles (Avogadro)",
            "Chemistry",
            "particles = moles x 6.02214076e23.",
            &["Moles (mol)"],
            |x| Ok(format!("Particles = {}.", sci(a(x, 0) * 6.02214076e23))),
        ),
    );
    reg(
        v,
        Calc::new(
            "molarity",
            "Molarity",
            "Chemistry",
            "Molarity (mol/L) = moles / volume in litres.",
            &["Moles (mol)", "Volume (L)"],
            |x| {
                nz(a(x, 1), "Volume")?;
                Ok(format!(
                    "Molarity = {} mol/L (M).",
                    nd(a(x, 0) / a(x, 1), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "molarity-from-mass",
            "Molarity From Mass",
            "Chemistry",
            "Molarity = (mass g / molar mass) / volume L.",
            &["Mass (g)", "Molar mass (g/mol)", "Volume (L)"],
            |x| {
                nz(a(x, 1), "Molar mass")?;
                nz(a(x, 2), "Volume")?;
                Ok(format!(
                    "Moles = {}. Molarity = {} M.",
                    nd(a(x, 0) / a(x, 1), 6),
                    nd(a(x, 0) / a(x, 1) / a(x, 2), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "dilution",
            "Dilution (Solve Final Concentration M2)",
            "Chemistry",
            "M1 V1 = M2 V2, so M2 = M1 V1 / V2. Volumes in the same units.",
            &[
                "Stock concentration M1",
                "Stock volume V1",
                "Final volume V2",
            ],
            |x| {
                nz(a(x, 2), "Final volume V2")?;
                Ok(format!(
                    "Final concentration M2 = {}.",
                    nd(a(x, 0) * a(x, 1) / a(x, 2), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "dilution-volume",
            "Dilution (Solve Stock Volume V1 Needed)",
            "Chemistry",
            "M1 V1 = M2 V2, so V1 = M2 V2 / M1.",
            &[
                "Stock concentration M1",
                "Target concentration M2",
                "Target final volume V2",
            ],
            |x| {
                nz(a(x, 0), "Stock concentration M1")?;
                Ok(format!(
                    "Stock volume needed V1 = {}. Then add solvent up to {}.",
                    nd(a(x, 1) * a(x, 2) / a(x, 0), 6),
                    n(a(x, 2))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ph",
            "pH From [H+]",
            "Chemistry",
            "pH = -log10[H+], with [H+] in mol/L. Also gives pOH = 14 - pH.",
            &["[H+] concentration (mol/L)"],
            |x| {
                chk(a(x, 0) > 0.0, "Concentration must be over 0.")?;
                let ph = -a(x, 0).log10();
                Ok(format!("pH = {}. pOH = {}.", nd(ph, 4), nd(14.0 - ph, 4)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ph-inverse",
            "[H+] From pH",
            "Chemistry",
            "[H+] = 10^(-pH) mol/L.",
            &["pH"],
            |x| {
                Ok(format!(
                    "[H+] = {} mol/L. pOH = {}.",
                    sci(10f64.powf(-a(x, 0))),
                    nd(14.0 - a(x, 0), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "poh",
            "pOH From [OH-]",
            "Chemistry",
            "pOH = -log10[OH-]. pH = 14 - pOH.",
            &["[OH-] concentration (mol/L)"],
            |x| {
                chk(a(x, 0) > 0.0, "Concentration must be over 0.")?;
                let poh = -a(x, 0).log10();
                Ok(format!("pOH = {}. pH = {}.", nd(poh, 4), nd(14.0 - poh, 4)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ideal-gas-pressure",
            "Ideal Gas Law (Solve Pressure)",
            "Chemistry",
            "PV = nRT, so P = nRT / V. R = 0.082057 L atm/(mol K) (= 8.314 J/(mol K) in SI units). T must be kelvin (C + 273.15).",
            &["Moles n", "Temperature T (K)", "Volume V (L)"],
            |x| {
                nz(a(x, 2), "Volume")?;
                chk(a(x, 1) > 0.0, "Temperature in kelvin must be over 0.")?;
                Ok(format!(
                    "Pressure P = {} atm ({} kPa).",
                    nd(a(x, 0) * R_GAS * a(x, 1) / a(x, 2), 6),
                    nd(a(x, 0) * R_GAS * a(x, 1) / a(x, 2) * 101.325, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ideal-gas-volume",
            "Ideal Gas Law (Solve Volume)",
            "Chemistry",
            "V = nRT / P. R = 0.082057 L atm/(mol K) (= 8.314 J/(mol K) in SI units).",
            &["Moles n", "Temperature T (K)", "Pressure P (atm)"],
            |x| {
                nz(a(x, 2), "Pressure")?;
                Ok(format!(
                    "Volume V = {} L.",
                    nd(a(x, 0) * R_GAS * a(x, 1) / a(x, 2), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ideal-gas-moles",
            "Ideal Gas Law (Solve Moles)",
            "Chemistry",
            "n = PV / RT. R = 0.082057 L atm/(mol K) (= 8.314 J/(mol K) in SI units).",
            &["Pressure P (atm)", "Volume V (L)", "Temperature T (K)"],
            |x| {
                chk(a(x, 2) > 0.0, "Temperature in kelvin must be over 0.")?;
                Ok(format!(
                    "Moles n = {}.",
                    nd(a(x, 0) * a(x, 1) / (R_GAS * a(x, 2)), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ideal-gas-temp",
            "Ideal Gas Law (Solve Temperature)",
            "Chemistry",
            "T = PV / nR, in kelvin (R = 0.082057 L atm/(mol K) = 8.314 J/(mol K)). Also converts to Celsius.",
            &["Pressure P (atm)", "Volume V (L)", "Moles n"],
            |x| {
                nz(a(x, 2), "Moles")?;
                let t = a(x, 0) * a(x, 1) / (a(x, 2) * R_GAS);
                Ok(format!(
                    "Temperature T = {} K ({} C).",
                    nd(t, 4),
                    nd(t - 273.15, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "boyle",
            "Boyle's Law (P1V1 = P2V2, Solve P2)",
            "Chemistry",
            "At constant temperature: P2 = P1 V1 / V2.",
            &["P1", "V1", "V2"],
            |x| {
                nz(a(x, 2), "V2")?;
                Ok(format!("P2 = {}.", nd(a(x, 0) * a(x, 1) / a(x, 2), 6)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "charles",
            "Charles's Law (V1/T1 = V2/T2, Solve V2)",
            "Chemistry",
            "At constant pressure: V2 = V1 x T2 / T1. Temperatures in kelvin.",
            &["V1", "T1 (K)", "T2 (K)"],
            |x| {
                nz(a(x, 1), "T1")?;
                Ok(format!("V2 = {}.", nd(a(x, 0) * a(x, 2) / a(x, 1), 6)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "gay-lussac",
            "Gay-Lussac's Law (P1/T1 = P2/T2, Solve P2)",
            "Chemistry",
            "At constant volume: P2 = P1 x T2 / T1. Temperatures in kelvin.",
            &["P1", "T1 (K)", "T2 (K)"],
            |x| {
                nz(a(x, 1), "T1")?;
                Ok(format!("P2 = {}.", nd(a(x, 0) * a(x, 2) / a(x, 1), 6)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "combined-gas",
            "Combined Gas Law (Solve P2)",
            "Chemistry",
            "P1 V1 / T1 = P2 V2 / T2, so P2 = P1 V1 T2 / (T1 V2). Temperatures in kelvin.",
            &["P1", "V1", "T1 (K)", "V2", "T2 (K)"],
            |x| {
                nz(a(x, 2) * a(x, 3), "T1 x V2")?;
                Ok(format!(
                    "P2 = {}.",
                    nd(a(x, 0) * a(x, 1) * a(x, 4) / (a(x, 2) * a(x, 3)), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "percent-composition",
            "Percent Composition by Mass (2 Parts)",
            "Chemistry",
            "Part % = part mass / total mass x 100, for a two-part mixture or compound split.",
            &["Mass of part 1", "Mass of part 2"],
            |x| {
                let t = a(x, 0) + a(x, 1);
                nz(t, "Total mass")?;
                Ok(format!(
                    "Part 1: {}%. Part 2: {}%. Total mass: {}.",
                    nd(a(x, 0) / t * 100.0, 4),
                    nd(a(x, 1) / t * 100.0, 4),
                    n(t)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "solution-percent",
            "Solution Concentration (Mass % & ppm)",
            "Chemistry",
            "Mass % = solute / solution x 100. ppm = mass % x 10,000.",
            &["Solute mass", "Total solution mass"],
            |x| {
                nz(a(x, 1), "Total solution mass")?;
                let pc = a(x, 0) / a(x, 1) * 100.0;
                Ok(format!(
                    "Concentration: {}% by mass = {} ppm.",
                    nd(pc, 4),
                    nd(pc * 10000.0, 2)
                ))
            },
        ),
    );
    reg(v, Calc::new("empirical-ratio", "Mole Ratio (Divide by Smallest)", "Chemistry",
        "Divides two mole amounts by the smaller to give the simplest whole-number-ish ratio used in empirical formulas.", &["Moles of element 1", "Moles of element 2"],
        |x| { chk(a(x,0)>0.0 && a(x,1)>0.0,"Both mole amounts must be over 0.")?; let mn=a(x,0).min(a(x,1)); Ok(format!("Ratio: {} : {} (to 2 decimals — round to whole numbers for the formula).", nd(a(x,0)/mn,2), nd(a(x,1)/mn,2))) }));
    reg(
        v,
        Calc::new(
            "reaction-yield",
            "Percent Yield",
            "Chemistry",
            "Percent yield = actual yield / theoretical yield x 100.",
            &["Actual yield", "Theoretical yield"],
            |x| {
                nz(a(x, 1), "Theoretical yield")?;
                Ok(format!(
                    "Percent yield: {}%.",
                    nd(a(x, 0) / a(x, 1) * 100.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "half-life",
            "Half-Life Remaining",
            "Chemistry",
            "Amount left = start x (1/2)^(time / half-life).",
            &[
                "Starting amount",
                "Time elapsed",
                "Half-life (same time unit)",
            ],
            |x| {
                nz(a(x, 2), "Half-life")?;
                Ok(format!(
                    "Remaining: {}. Half-lives passed: {}. Fraction left: {}%.",
                    nd(a(x, 0) * 0.5f64.powf(a(x, 1) / a(x, 2)), 6),
                    nd(a(x, 1) / a(x, 2), 4),
                    nd(0.5f64.powf(a(x, 1) / a(x, 2)) * 100.0, 4)
                ))
            },
        ),
    );
    // v0.7.0 (#35): serial dilution planner. Two modes, picked
    // by the mode field like the other code fields in this
    // registry: 1 = each step moves a transfer volume into a
    // diluent volume (factor = (transfer + diluent) / transfer);
    // 2 = a plain dilution factor per step (10 = 1:10).
    reg(
        v,
        Calc::new(
            "serial-dilution",
            "Serial Dilution Planner",
            "Chemistry",
            "Plans a serial dilution: every step dilutes by the same factor, so step k has concentration = start / factor^k. Mode 1: the factor comes from the volumes — (transfer + diluent) / transfer. Mode 2: enter the factor directly (10 for a 1:10 series). Concentrations are printed in the same unit as the starting concentration; volumes share one unit.",
            &[
                "Starting concentration",
                "Mode (1 = volumes, 2 = factor)",
                "Transfer volume (mode 1)",
                "Diluent volume (mode 1)",
                "Dilution factor per step (mode 2)",
                "Number of steps",
            ],
            |x| {
                let start = a(x, 0);
                chk(
                    start > 0.0,
                    "The starting concentration must be over 0.",
                )?;
                let mode = a(x, 1);
                let factor = if mode == 1.0 {
                    let (tr, dil) = (a(x, 2), a(x, 3));
                    chk(
                        tr > 0.0,
                        "Mode 1 needs a transfer volume over 0 — it is the volume moved into each new tube.",
                    )?;
                    chk(
                        dil > 0.0,
                        "Mode 1 needs a diluent volume over 0 — with no diluent there is no dilution. For a plain factor per step, use mode 2.",
                    )?;
                    (tr + dil) / tr
                } else if mode == 2.0 {
                    let f = a(x, 4);
                    chk(
                        f > 1.0,
                        "Mode 2 needs a dilution factor over 1 — 10 means each step is 1 part sample in 10 parts total. A factor of 1 would not dilute at all.",
                    )?;
                    f
                } else {
                    return Err(
                        "Mode must be 1 (transfer + diluent volumes) or 2 (dilution factor per step)."
                            .into(),
                    );
                };
                let steps = a(x, 5);
                chk(
                    steps >= 1.0 && steps == steps.trunc() && steps <= 100.0,
                    "The number of steps must be a whole number from 1 to 100.",
                )?;
                let how = if mode == 1.0 {
                    format!(
                        "transfer {} + diluent {} = {} in total",
                        n(a(x, 2)),
                        n(a(x, 3)),
                        n(a(x, 2) + a(x, 3))
                    )
                } else {
                    "the factor you entered".to_string()
                };
                let mut out = format!(
                    "Serial dilution: each step divides the concentration by {} ({}).\nStart: {}.",
                    nd(factor, 6),
                    how,
                    n(start)
                );
                let mut c = start;
                for i in 1..=(steps as usize) {
                    c /= factor;
                    out.push_str(&format!("\nStep {i}: {}.", n(c)));
                }
                out.push_str(
                    "\nConcentrations are in the same unit as the starting concentration; volumes in one shared unit.",
                );
                Ok(out)
            },
        ),
    );
}
