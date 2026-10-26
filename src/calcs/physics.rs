//! Physics calculators (Omni: Physics).
use crate::{a, chk, n, nd, nz, sci, Calc};
use std::f64::consts::PI;

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}
const G: f64 = 9.80665;

pub fn register(v: &mut Vec<Calc>) {
    reg(
        v,
        Calc::new(
            "speed-from-distance-time",
            "Speed From Distance & Time",
            "Physics",
            "speed = distance / time.",
            &["Distance", "Time"],
            |x| {
                nz(a(x, 1), "Time")?;
                Ok(format!("Speed = {} / {} = {}. Keep the units consistent (e.g. metres & seconds gives m/s).", n(a(x,0)), n(a(x,1)), nd(a(x,0)/a(x,1),6)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "distance-from-speed-time",
            "Distance From Speed & Time",
            "Physics",
            "distance = speed x time.",
            &["Speed", "Time"],
            |x| {
                Ok(format!(
                    "Distance = {} x {} = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    n(a(x, 0) * a(x, 1))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "time-from-speed-distance",
            "Time From Speed & Distance",
            "Physics",
            "time = distance / speed.",
            &["Distance", "Speed"],
            |x| {
                nz(a(x, 1), "Speed")?;
                Ok(format!(
                    "Time = {} / {} = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    nd(a(x, 0) / a(x, 1), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "average-velocity",
            "Average Velocity",
            "Physics",
            "Average velocity = change in position / change in time = (x2 - x1) / (t2 - t1).",
            &["Position x1", "Position x2", "Time t1", "Time t2"],
            |x| {
                nz(a(x, 3) - a(x, 1), "t2 - t1")?;
                Ok(format!(
                    "Average velocity = {}.",
                    nd((a(x, 1) - a(x, 0)) / (a(x, 3) - a(x, 2)), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "acceleration",
            "Acceleration",
            "Physics",
            "a = (v - u) / t, from starting speed u to final speed v in time t.",
            &["Starting speed u", "Final speed v", "Time t (s)"],
            |x| {
                nz(a(x, 2), "Time")?;
                Ok(format!(
                    "Acceleration = {} m/s^2.",
                    nd((a(x, 1) - a(x, 0)) / a(x, 2), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "final-velocity",
            "Final Velocity (v = u + a t)",
            "Physics",
            "v = u + a t. Also shows distance travelled s = u t + 1/2 a t^2.",
            &[
                "Starting speed u (m/s)",
                "Acceleration a (m/s^2)",
                "Time t (s)",
            ],
            |x| {
                Ok(format!(
                    "Final velocity v = {} m/s. Distance s = {} m.",
                    nd(a(x, 0) + a(x, 1) * a(x, 2), 6),
                    nd(a(x, 0) * a(x, 2) + 0.5 * a(x, 1) * a(x, 2).powi(2), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "force",
            "Force (Newton's Second Law)",
            "Physics",
            "F = m x a. Result in newtons when mass is kg and acceleration is m/s^2.",
            &["Mass (kg)", "Acceleration (m/s^2)"],
            |x| Ok(format!("Force = {} N.", n(a(x, 0) * a(x, 1)))),
        ),
    );
    reg(
        v,
        Calc::new(
            "weight-from-mass",
            "Weight From Mass",
            "Physics",
            "Weight = mass x g (g = 9.80665 m/s^2 on Earth).",
            &["Mass (kg)"],
            |x| {
                Ok(format!(
                    "Weight on Earth = {} N (about {} kgf).",
                    nd(a(x, 0) * G, 4),
                    nd(a(x, 0), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "mass-from-weight",
            "Mass From Weight",
            "Physics",
            "Mass = weight / g (g = 9.80665 m/s^2).",
            &["Weight (N)"],
            |x| Ok(format!("Mass = {} kg.", nd(a(x, 0) / G, 6))),
        ),
    );
    reg(
        v,
        Calc::new(
            "kinetic-energy",
            "Kinetic Energy",
            "Physics",
            "KE = 1/2 m v^2, in joules.",
            &["Mass (kg)", "Speed v (m/s)"],
            |x| {
                Ok(format!(
                    "Kinetic energy = {} J.",
                    nd(0.5 * a(x, 0) * a(x, 1).powi(2), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "potential-energy",
            "Potential Energy (Gravity)",
            "Physics",
            "PE = m g h, g = 9.80665, in joules.",
            &["Mass (kg)", "Height (m)"],
            |x| {
                Ok(format!(
                    "Potential energy = {} J.",
                    nd(a(x, 0) * G * a(x, 1), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "mechanical-energy",
            "Total Mechanical Energy (KE + PE)",
            "Physics",
            "E = 1/2 m v^2 + m g h.",
            &["Mass (kg)", "Speed (m/s)", "Height (m)"],
            |x| {
                Ok(format!(
                    "KE = {} J. PE = {} J. Total = {} J.",
                    nd(0.5 * a(x, 0) * a(x, 1).powi(2), 4),
                    nd(a(x, 0) * G * a(x, 2), 4),
                    nd(0.5 * a(x, 0) * a(x, 1).powi(2) + a(x, 0) * G * a(x, 2), 4)
                ))
            },
        ),
    );
    reg(v, Calc::new("power-physics", "Power (Work Over Time / Force x Velocity)", "Physics",
        "Power = work / time. If you also give force x velocity it is the same number — this calc uses work & time, and shows watts and horsepower (1 hp = 745.7 W).", &["Work (J)", "Time (s)"],
        |x| { nz(a(x,1),"Time")?; let p=a(x,0)/a(x,1); Ok(format!("Power = {} W ({} hp, {} kW).", nd(p,4), nd(p/745.7,4), nd(p/1000.0,4))) }));
    reg(
        v,
        Calc::new(
            "work",
            "Work Done (Force x Distance)",
            "Physics",
            "Work = force x distance (force along the direction of travel), in joules.",
            &["Force (N)", "Distance (m)"],
            |x| Ok(format!("Work = {} J.", n(a(x, 0) * a(x, 1)))),
        ),
    );
    reg(
        v,
        Calc::new(
            "density-physics",
            "Density",
            "Physics",
            "density = mass / volume.",
            &["Mass", "Volume"],
            |x| {
                nz(a(x, 1), "Volume")?;
                Ok(format!(
                    "Density = {} (units: mass-unit / volume-unit, e.g. kg/m^3).",
                    nd(a(x, 0) / a(x, 1), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "pressure",
            "Pressure",
            "Physics",
            "Pressure = force / area, in pascals when force is N and area is m^2.",
            &["Force (N)", "Area (m^2)"],
            |x| {
                nz(a(x, 1), "Area")?;
                Ok(format!(
                    "Pressure = {} Pa ({} kPa).",
                    nd(a(x, 0) / a(x, 1), 4),
                    nd(a(x, 0) / a(x, 1) / 1000.0, 6)
                ))
            },
        ),
    );
    reg(v, Calc::new("ohms-law", "Ohm's Law (All Four Values)", "Physics",
        "Enter voltage V and resistance R: I = V/R, P = V x I. Or reason backwards — this calc fixes V and R as the inputs.", &["Voltage V (volts)", "Resistance R (ohms)"],
        |x| { nz(a(x,1),"Resistance")?; let i=a(x,0)/a(x,1); Ok(format!("Current I = {} A. Power P = {} W. (Check: V = I x R = {} x {} = {} V.)", nd(i,6), nd(a(x,0)*i,6), nd(i,6), n(a(x,1)), nd(i*a(x,1),4))) }));
    reg(
        v,
        Calc::new(
            "ohms-solve-i",
            "Ohm's Law — Resistance From V & I",
            "Physics",
            "R = V / I. Also P = V x I.",
            &["Voltage V", "Current I (amps)"],
            |x| {
                nz(a(x, 1), "Current")?;
                Ok(format!(
                    "Resistance R = {} ohms. Power P = {} W.",
                    nd(a(x, 0) / a(x, 1), 6),
                    nd(a(x, 0) * a(x, 1), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ohms-solve-v",
            "Ohm's Law — Voltage From I & R",
            "Physics",
            "V = I x R. Also P = V x I.",
            &["Current I (amps)", "Resistance R (ohms)"],
            |x| {
                Ok(format!(
                    "Voltage V = {} V. Power P = {} W.",
                    n(a(x, 0) * a(x, 1)),
                    nd(a(x, 0) * a(x, 1) * a(x, 0), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "electrical-power",
            "Electrical Power (V x I)",
            "Physics",
            "P = V x I (DC, or AC resistive load). Energy over hours: kWh = P(W) x hours / 1000.",
            &["Voltage V", "Current I (amps)", "Hours used"],
            |x| {
                let p = a(x, 0) * a(x, 1);
                Ok(format!(
                    "Power = {} W ({} kW). Energy in {} h = {} kWh.",
                    nd(p, 4),
                    nd(p / 1000.0, 4),
                    n(a(x, 2)),
                    nd(p * a(x, 2) / 1000.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "resistors-parallel",
            "Resistors in Parallel",
            "Physics",
            "1/R = 1/R1 + 1/R2. Also shows the series total R1 + R2.",
            &["Resistor R1 (ohms)", "Resistor R2 (ohms)"],
            |x| {
                chk(
                    a(x, 0) > 0.0 && a(x, 1) > 0.0,
                    "Resistances must be over 0.",
                )?;
                Ok(format!(
                    "Parallel total = {} ohms. Series total = {} ohms.",
                    nd(1.0 / (1.0 / a(x, 0) + 1.0 / a(x, 1)), 6),
                    n(a(x, 0) + a(x, 1))
                ))
            },
        ),
    );
    reg(v, Calc::new("projectile", "Projectile Motion (Range & Max Height)", "Physics",
        "Flat ground, no air resistance: range = v^2 sin(2 angle) / g; max height = (v sin angle)^2 / 2g; flight time = 2 v sin(angle) / g.", &["Launch speed v (m/s)", "Angle (degrees)"],
        |x| { let r=a(x,1)*PI/180.0; Ok(format!("Range = {} m. Max height = {} m. Flight time = {} s.", nd(a(x,0).powi(2)*(2.0*r).sin()/G,4), nd((a(x,0)*r.sin()).powi(2)/(2.0*G),4), nd(2.0*a(x,0)*r.sin()/G,4))) }));
    reg(
        v,
        Calc::new(
            "pendulum",
            "Pendulum Period",
            "Physics",
            "Period T = 2 pi sqrt(L / g) for small swings. Frequency = 1/T.",
            &["Length L (m)"],
            |x| {
                chk(a(x, 0) > 0.0, "Length must be over 0.")?;
                let t = 2.0 * PI * (a(x, 0) / G).sqrt();
                Ok(format!(
                    "Period = {} s. Frequency = {} Hz.",
                    nd(t, 6),
                    nd(1.0 / t, 6)
                ))
            },
        ),
    );
    reg(v, Calc::new("wave", "Wave: Frequency, Wavelength, Speed", "Physics",
        "v = f x wavelength. Enter frequency and wavelength to get speed; also shows the period T = 1/f.", &["Frequency f (Hz)", "Wavelength (m)"],
        |x| { nz(a(x,0),"Frequency (for the period)")?; Ok(format!("Wave speed = {} m/s. Period T = {} s.", n(a(x,0)*a(x,1)), nd(1.0/a(x,0),6))) }));
    reg(
        v,
        Calc::new(
            "torque",
            "Torque",
            "Physics",
            "Torque = force x lever arm (force at right angles), in newton-metres.",
            &["Force (N)", "Lever arm (m)"],
            |x| Ok(format!("Torque = {} Nm.", n(a(x, 0) * a(x, 1)))),
        ),
    );
    reg(
        v,
        Calc::new(
            "momentum",
            "Momentum",
            "Physics",
            "Momentum p = m x v, in kg m/s. Also KE from the same inputs.",
            &["Mass (kg)", "Velocity (m/s)"],
            |x| {
                Ok(format!(
                    "Momentum = {} kg m/s. Kinetic energy = {} J.",
                    n(a(x, 0) * a(x, 1)),
                    nd(0.5 * a(x, 0) * a(x, 1).powi(2), 4)
                ))
            },
        ),
    );
    reg(v, Calc::new("friction", "Friction Force", "Physics",
        "Friction = coefficient of friction x normal force. On flat ground the normal force = m g.", &["Coefficient of friction", "Mass (kg)"],
        |x| Ok(format!("Normal force = {} N. Friction force = {} N.", nd(a(x,1)*G,4), nd(a(x,0)*a(x,1)*G,4)))));
    reg(
        v,
        Calc::new(
            "hooke",
            "Hooke's Law (Spring)",
            "Physics",
            "Force = k x stretch. Stored energy = 1/2 k x stretch^2.",
            &["Spring constant k (N/m)", "Stretch (m)"],
            |x| {
                Ok(format!(
                    "Force = {} N. Stored energy = {} J.",
                    n(a(x, 0) * a(x, 1)),
                    nd(0.5 * a(x, 0) * a(x, 1).powi(2), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "gravity-force",
            "Gravitational Force Between Two Masses",
            "Physics",
            "F = G m1 m2 / r^2, G = 6.674e-11. Result in newtons.",
            &["Mass 1 (kg)", "Mass 2 (kg)", "Distance between centres (m)"],
            |x| {
                nz(a(x, 2), "Distance")?;
                Ok(format!(
                    "Gravitational force = {} N.",
                    sci(6.674e-11 * a(x, 0) * a(x, 1) / a(x, 2).powi(2))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "centripetal",
            "Centripetal Force & Acceleration",
            "Physics",
            "a = v^2 / r. F = m v^2 / r.",
            &["Mass (kg)", "Speed v (m/s)", "Radius r (m)"],
            |x| {
                nz(a(x, 2), "Radius")?;
                Ok(format!(
                    "Centripetal acceleration = {} m/s^2. Force = {} N.",
                    nd(a(x, 1).powi(2) / a(x, 2), 6),
                    nd(a(x, 0) * a(x, 1).powi(2) / a(x, 2), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "free-fall",
            "Free Fall (Speed & Distance)",
            "Physics",
            "After t seconds from rest: v = g t, distance = 1/2 g t^2 (no air resistance).",
            &["Time falling (s)"],
            |x| {
                Ok(format!(
                    "Speed = {} m/s ({} km/h). Distance fallen = {} m.",
                    nd(G * a(x, 0), 4),
                    nd(G * a(x, 0) * 3.6, 2),
                    nd(0.5 * G * a(x, 0).powi(2), 4)
                ))
            },
        ),
    );
}
