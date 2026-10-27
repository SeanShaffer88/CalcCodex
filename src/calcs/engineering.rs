//! Engineering calculators (v0.8.0, items #41–#45): the
//! electronics pack and the mechanical pack.
//!
//! Categories (decision in DECISIONS.md): the electronics pack
//! sits in Physics, where Ohm's law and the other electricity
//! calculators already live; the mechanical pack (tolerance
//! stack-up, safety factor, bolt torque, beam bending) sits in
//! Construction, the registry's home for structural and
//! load-bearing calculations.

use crate::engineers::{material_by_code, ohms_text, resistor_colour_run};
use crate::{a, chk, nd, Calc};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

pub fn register(v: &mut Vec<Calc>) {
    // ---- #41a electronics: resistor colour code (text input) ----
    reg(
        v,
        Calc::new(
            "resistor-colour-code",
            "Resistor Colour Code (Bands <-> Value)",
            "Physics",
            "Read a resistor's colour bands, or get the bands for a value. Type the band colours in order (4 or 5 words, e.g. red red brown gold) to decode: the first bands are the significant digits, the next is the multiplier, the last is the tolerance (gold 5%, silver 10%, brown 1%, red 2%, none 20%). Or type a resistance — 4700, 4.7k, 1M — with an optional tolerance colour (e.g. 4700 gold) to encode it: 4 bands when the value is exactly two significant digits times a power of ten, 5 bands for three digits, and an explanation when no exact bands exist.",
            &["Band colours, or a resistance (text)"],
            |_| {
                Err(
                    "This calculator takes text (band colours or a resistance), not numbers."
                        .into(),
                )
            },
        )
        .text_mode("red red brown gold", |text, _degrees| {
            resistor_colour_run(text)
        }),
    );
    // ---- #41b electronics: resistors in series & parallel ----
    reg(
        v,
        Calc::new(
            "resistors-series-parallel",
            "Resistors in Series & Parallel",
            "Physics",
            "One list of resistances (ohms), both answers: in series the resistances simply add (R = R1 + R2 + ...); in parallel the conductances add (1/R = 1/R1 + 1/R2 + ...), so the parallel result is always smaller than the smallest resistor. A 0 ohm resistor in parallel is a short circuit — the parallel equivalent is 0.",
            &["Resistances in ohms (list)"],
            |x| {
                for r in x {
                    chk(
                        *r >= 0.0,
                        "A resistance cannot be negative — check the list.",
                    )?;
                }
                let series: f64 = x.iter().sum();
                let shorted = x.contains(&0.0);
                let parallel = if shorted {
                    0.0
                } else {
                    1.0 / x.iter().map(|r| 1.0 / r).sum::<f64>()
                };
                let list = x
                    .iter()
                    .map(|r| nd(*r, 6))
                    .collect::<Vec<_>>()
                    .join(", ");
                let mut out = format!(
                    "Resistors: {list} \u{3a9}. Series total = {} ({} added). Parallel equivalent = {} (1 / ({} ) ).",
                    ohms_text(series),
                    x.iter().map(|r| nd(*r, 6)).collect::<Vec<_>>().join(" + "),
                    ohms_text(parallel),
                    x.iter()
                        .map(|r| format!("1/{}", nd(*r, 6)))
                        .collect::<Vec<_>>()
                        .join(" + ")
                );
                if shorted {
                    out.push_str(
                        " One resistor is 0 \u{3a9}: in parallel that is a short circuit, so the parallel equivalent is 0 \u{3a9}.",
                    );
                }
                Ok(out)
            },
        )
        .list_mode(),
    );
    // ---- #41c electronics: voltage divider ----
    reg(
        v,
        Calc::new(
            "voltage-divider",
            "Voltage Divider",
            "Physics",
            "Two resistors in series across Vin: the output is taken across R2. Vout = Vin x R2 / (R1 + R2). The same current flows through both: I = Vin / (R1 + R2); each resistor's power is P = I^2 x R.",
            &[
                "Input voltage Vin (volts)",
                "Resistor R1 (ohms, input side)",
                "Resistor R2 (ohms, output side)",
            ],
            |x| {
                let (vin, r1, r2) = (a(x, 0), a(x, 1), a(x, 2));
                chk(r1 > 0.0, "R1 must be more than 0 ohms — a zero resistor makes the divider a short circuit.")?;
                chk(r2 > 0.0, "R2 must be more than 0 ohms — with R2 = 0 the output is shorted to 0 V; enter the real load resistance.")?;
                let i = vin / (r1 + r2);
                let vout = vin * r2 / (r1 + r2);
                Ok(format!(
                    "Vout = {} V (across R2). Current through both resistors = {} A. Power: R1 dissipates {} W, R2 dissipates {} W. Steps: Vout = {} x {} / ({} + {}) = {} V; I = {} / {} = {} A; P = I^2 x R.",
                    nd(vout, 6),
                    nd(i, 6),
                    nd(i * i * r1, 6),
                    nd(i * i * r2, 6),
                    nd(vin, 6),
                    nd(r2, 6),
                    nd(r1, 6),
                    nd(r2, 6),
                    nd(vout, 6),
                    nd(vin, 6),
                    nd(r1 + r2, 6),
                    nd(i, 6)
                ))
            },
        ),
    );
    // ---- #42 tolerance stack-up (pairs: nominal, tolerance) ----
    reg(
        v,
        Calc::new(
            "tolerance-stack-up",
            "Tolerance Stack-Up",
            "Construction",
            "A chain of dimensions that add up (a gap, a fit, an assembly length). Enter the dimensions in pairs: nominal, tolerance — e.g. 10 0.1 20 0.2 5 0.05. The total is the sum of the nominals. The worst case is every tolerance at its extreme together: total +- the sum of the tolerances. The RSS (root-sum-square) figure treats the tolerances as independent: sqrt of the sum of the squared tolerances — the usual statistical estimate, always smaller than the worst case.",
            &["Dimensions as pairs (nominal, tolerance)"],
            |x| {
                let mut total = 0.0;
                let mut worst = 0.0;
                let mut squares = 0.0;
                let mut lines: Vec<String> = Vec::new();
                for (i, pair) in x.chunks_exact(2).enumerate() {
                    let (nom, tol) = (pair[0], pair[1]);
                    chk(
                        tol >= 0.0,
                        "A tolerance is a +/- amount, so it cannot be negative — check the pair after the nominal it belongs to.",
                    )?;
                    total += nom;
                    worst += tol;
                    squares += tol * tol;
                    lines.push(format!(
                        "Dimension {}: {} \u{b1} {}",
                        i + 1,
                        nd(nom, 6),
                        nd(tol, 6)
                    ));
                }
                let rss = squares.sqrt();
                Ok(format!(
                    "{}. Total = {} \u{b1} {} worst case (from {} to {}). RSS estimate = {} \u{b1} {} (from {} to {}). Steps: worst case adds every tolerance: {}; RSS squares and square-roots them: sqrt({}) = {}.",
                    lines.join(". "),
                    nd(total, 6),
                    nd(worst, 6),
                    nd(total - worst, 6),
                    nd(total + worst, 6),
                    nd(total, 6),
                    nd(rss, 6),
                    nd(total - rss, 6),
                    nd(total + rss, 6),
                    nd(worst, 6),
                    nd(squares, 6),
                    nd(rss, 6)
                ))
            },
        )
        .pair_mode(),
    );
    // ---- #43 safety factor / margin of safety ----
    reg(
        v,
        Calc::new(
            "safety-factor",
            "Safety Factor & Margin of Safety",
            "Construction",
            "Factor of safety FoS = strength / applied stress, margin of safety MoS = FoS - 1. Strength is the limit you are checking against — a material's yield strength or its ultimate strength; say which in your own records, and keep the units the same on both sides (e.g. both in MPa). Mode 1: enter the applied stress directly. Mode 2: enter a load and the area carrying it, and the stress is load / area. An FoS below 1 means the applied stress exceeds the strength entered: on these numbers the part is predicted to fail.",
            &[
                "Mode (1 = stress given, 2 = load and area)",
                "Strength — yield or ultimate (same units as the stress)",
                "Applied stress (mode 1)",
                "Load (mode 2)",
                "Cross-section area (mode 2)",
            ],
            |x| {
                let (mode, strength, stress_in, load, area) =
                    (a(x, 0), a(x, 1), a(x, 2), a(x, 3), a(x, 4));
                chk(
                    mode == 1.0 || mode == 2.0,
                    "Mode must be 1 (you give the applied stress) or 2 (you give a load and an area).",
                )?;
                chk(
                    strength > 0.0,
                    "Strength must be more than zero — it is the material limit (yield or ultimate) you are checking against.",
                )?;
                let stress = if mode == 1.0 {
                    chk(
                        stress_in > 0.0,
                        "Applied stress must be more than zero in mode 1 — if the stress is zero there is nothing to check.",
                    )?;
                    stress_in
                } else {
                    chk(load > 0.0, "Load must be more than zero in mode 2.")?;
                    chk(
                        area > 0.0,
                        "Area must be more than zero in mode 2 — stress is load / area, and dividing by a zero area is impossible.",
                    )?;
                    load / area
                };
                let fos = strength / stress;
                let mos = fos - 1.0;
                let headroom = if mos >= 0.0 {
                    format!(
                        "the applied stress could grow by {}% before it reaches the strength entered",
                        nd(mos * 100.0, 4)
                    )
                } else {
                    format!(
                        "the applied stress is already {} times the strength entered",
                        nd(stress / strength, 4)
                    )
                };
                let reading = if fos < 1.0 {
                    "Below 1: the applied stress is greater than the strength entered — on these numbers the part is predicted to fail. Do not read anything reassuring into a margin printed below zero."
                } else if fos < 1.5 {
                    "Just above 1: the part holds on these numbers, but the margin is slim — many design codes ask for 1.5 or more even for well-understood static loads."
                } else {
                    "1.5 or above: a working margin on these numbers. The factor a real design must reach depends on its code, its loads and the cost of failure — this calculator reports the numbers, it does not approve a design."
                };
                Ok(format!(
                    "Applied stress = {}. Factor of safety FoS = strength / stress = {} / {} = {}. Margin of safety MoS = FoS - 1 = {} — {headroom}. Reading: {reading}",
                    nd(stress, 6),
                    nd(strength, 6),
                    nd(stress, 6),
                    nd(fos, 6),
                    nd(mos, 6)
                ))
            },
        ),
    );
    // ---- #45a mechanical: bolt torque ----
    reg(
        v,
        Calc::new(
            "bolt-torque",
            "Bolt Torque (Clamp Force)",
            "Construction",
            "The standard short-form torque relation T = K x D x F: the torque T that produces a target clamp force F on a bolt of nominal diameter D, with K the nut factor for the friction condition. K is about 0.20 for dry plain steel and about 0.15 lubricated — friction dominates this estimate, so real values scatter by roughly a quarter either way; critical joints need the manufacturer's table.",
            &[
                "Nut factor K (about 0.20 dry steel, 0.15 lubricated)",
                "Bolt diameter D (mm)",
                "Target clamp force F (newtons)",
            ],
            |x| {
                let (k, d_mm, f) = (a(x, 0), a(x, 1), a(x, 2));
                chk(k > 0.0, "The nut factor K must be more than zero (about 0.20 dry, 0.15 lubricated).")?;
                chk(d_mm > 0.0, "The bolt diameter must be more than zero millimetres.")?;
                chk(f > 0.0, "The target clamp force must be more than zero newtons.")?;
                let t = k * (d_mm / 1000.0) * f;
                Ok(format!(
                    "Torque T = {} N\u{b7}m ({} ft\u{b7}lbf). Steps: T = K x D x F = {} x {} m x {} N = {} N\u{b7}m. (D converted: {} mm = {} m; 1 N\u{b7}m = 0.737562 ft\u{b7}lbf.)",
                    nd(t, 6),
                    nd(t * 0.737_562_149, 6),
                    nd(k, 4),
                    nd(d_mm / 1000.0, 6),
                    nd(f, 4),
                    nd(t, 6),
                    nd(d_mm, 4),
                    nd(d_mm / 1000.0, 6)
                ))
            },
        ),
    );
    // ---- #45b mechanical: beam stress & deflection ----
    reg(
        v,
        Calc::new(
            "beam-bending",
            "Beam — Stress & Deflection (Central Load)",
            "Construction",
            "A simply supported beam with one point load at midspan. Max moment M = P x L / 4; midspan deflection = P x L^3 / (48 x E x I). Section: rectangle I = b x h^3 / 12 (bending about the strong axis, h is the vertical depth), circle I = pi x d^4 / 64; the furthest fibre is c = h/2 or d/2 and the max bending stress is M x c / I. E: enter a value in GPa, or enter 0 and a material code (CalcDex --materials lists the table) to take the table's typical Young's modulus.",
            &[
                "Span L (metres)",
                "Central load P (newtons)",
                "Young's modulus E (GPa; 0 = take from material)",
                "Material code (used when E is 0)",
                "Section (1 = rectangle, 2 = circle)",
                "Width b / diameter d (metres)",
                "Height h (metres; rectangle only)",
            ],
            |x| {
                let (l, p, e_in, code, section, b, h) =
                    (a(x, 0), a(x, 1), a(x, 2), a(x, 3), a(x, 4), a(x, 5), a(x, 6));
                chk(l > 0.0, "The span must be more than zero metres.")?;
                chk(p > 0.0, "The load must be more than zero newtons.")?;
                let (e_gpa, e_source) = if e_in > 0.0 {
                    (e_in, format!("E = {} GPa (the value you entered)", nd(e_in, 4)))
                } else if e_in == 0.0 {
                    let mat = material_by_code(code)?;
                    (
                        mat.youngs,
                        format!(
                            "E = {} GPa ({} — a typical value from the built-in materials table)",
                            nd(mat.youngs, 4),
                            mat.name
                        ),
                    )
                } else {
                    return Err("E cannot be negative — enter a Young's modulus in GPa, or 0 with a material code.".into());
                };
                chk(
                    section == 1.0 || section == 2.0,
                    "Section must be 1 (rectangle) or 2 (circle).",
                )?;
                chk(b > 0.0, "The section width/diameter must be more than zero metres.")?;
                let (inertia, c, section_text) = if section == 1.0 {
                    chk(h > 0.0, "A rectangle needs a height h of more than zero metres.")?;
                    (
                        b * h.powi(3) / 12.0,
                        h / 2.0,
                        format!("rectangle {} m wide x {} m deep", nd(b, 4), nd(h, 4)),
                    )
                } else {
                    (
                        std::f64::consts::PI * b.powi(4) / 64.0,
                        b / 2.0,
                        format!("circle of diameter {} m", nd(b, 4)),
                    )
                };
                let e_pa = e_gpa * 1.0e9;
                let moment = p * l / 4.0;
                let stress_pa = moment * c / inertia;
                let deflection = p * l.powi(3) / (48.0 * e_pa * inertia);
                // Section inertias are tiny numbers: scientific
                // notation reads better than a long run of zeros.
                let i_text = if inertia < 1.0e-3 {
                    crate::sci(inertia)
                } else {
                    crate::n_full(inertia)
                };
                Ok(format!(
                    "Beam {} m, central load {} N, {section_text}. {e_source}. Moment of inertia I = {} m^4. Max moment M = P x L / 4 = {} N\u{b7}m. Max bending stress = M x c / I = {} MPa. Midspan deflection = P L^3 / (48 E I) = {} m = {} mm. (Linear elastic, small-deflection formulas; the load sits at midspan and the beam's own weight is not counted.)",
                    nd(l, 4),
                    nd(p, 4),
                    i_text,
                    nd(moment, 6),
                    nd(stress_pa / 1.0e6, 6),
                    crate::n_full(deflection),
                    nd(deflection * 1000.0, 6)
                ))
            },
        ),
    );
    // The materials table itself is data, not a calculator: it is
    // served by `CalcDex --materials`, the MATERIALS view and the
    // beam calculator's material code field (item #44).
}
