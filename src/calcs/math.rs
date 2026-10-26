//! Math calculators (Omni: Math; Nutilz calculator subset).
use crate::{a, chk, factorial, finite, gcd_i, n, nd, nz, Calc};
use std::f64::consts::PI;

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

pub fn register(v: &mut Vec<Calc>) {
    // ---- Percentages ----
    reg(
        v,
        Calc::new(
            "percent-of",
            "Percentage Of a Number",
            "Math",
            "What is X% of Y? Formula: result = Y * X / 100.",
            &["Percent X", "Number Y"],
            |x| {
                Ok(format!(
                    "{}% of {} = {}\nSteps: {} x {} / 100 = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    n(a(x, 1) * a(x, 0) / 100.0),
                    n(a(x, 1)),
                    n(a(x, 0)),
                    n(a(x, 1) * a(x, 0) / 100.0)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "percent-is",
            "X Is What Percent of Y",
            "Math",
            "X is what percent of Y? Formula: percent = X / Y * 100.",
            &["Part X", "Whole Y"],
            |x| {
                nz(a(x, 1), "The whole (Y)")?;
                Ok(format!(
                    "{} is {}% of {}.\nSteps: {} / {} x 100 = {}%.",
                    n(a(x, 0)),
                    nd(a(x, 0) / a(x, 1) * 100.0, 4),
                    n(a(x, 1)),
                    n(a(x, 0)),
                    n(a(x, 1)),
                    nd(a(x, 0) / a(x, 1) * 100.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "percent-change",
            "Percentage Change",
            "Math",
            "Percentage change from old to new. Formula: (new - old) / |old| * 100.",
            &["Old value", "New value"],
            |x| {
                nz(a(x, 0), "The old value")?;
                let c = (a(x, 1) - a(x, 0)) / a(x, 0).abs() * 100.0;
                Ok(format!(
                    "Change: {}% ({}).\nSteps: ({} - {}) / {} x 100 = {}%. Absolute change: {}.",
                    nd(c, 4),
                    if c >= 0.0 { "increase" } else { "decrease" },
                    n(a(x, 1)),
                    n(a(x, 0)),
                    n(a(x, 0).abs()),
                    nd(c, 4),
                    n(a(x, 1) - a(x, 0))
                ))
            },
        ),
    );
    reg(v, Calc::new("percent-difference", "Percentage Difference", "Math",
        "Percentage difference between two values (neither is 'the original'). Formula: |a-b| / ((a+b)/2) * 100.", &["Value A", "Value B"],
        |x| { let avg=(a(x,0)+a(x,1))/2.0; nz(avg,"The average of the two values")?; Ok(format!("Percentage difference: {}%.\nSteps: |{} - {}| = {}; average = {}; {} / {} x 100 = {}%.", nd((a(x,0)-a(x,1)).abs()/avg*100.0,4), n(a(x,0)), n(a(x,1)), n((a(x,0)-a(x,1)).abs()), n(avg), n((a(x,0)-a(x,1)).abs()), n(avg), nd((a(x,0)-a(x,1)).abs()/avg*100.0,4))) }));
    reg(v, Calc::new("reverse-percent", "Reverse Percentage (Original Before %)", "Math",
        "A price of Y is after an X% increase (use negative X for a decrease). Find the original. Formula: original = Y / (1 + X/100).", &["Percent change X", "Final value Y"],
        |x| { let d=1.0+a(x,0)/100.0; nz(d,"1 + X/100")?; Ok(format!("Original value: {}.\nSteps: {} / (1 + {}/100) = {} / {} = {}.", nd(a(x,1)/d,4), n(a(x,1)), n(a(x,0)), n(a(x,1)), nd(d,4), nd(a(x,1)/d,4))) }));
    reg(v, Calc::new("percent-off-result", "Value After Percent Increase/Decrease", "Math",
        "Apply an X% change to Y. Formula: result = Y * (1 + X/100). Use negative X to decrease.", &["Percent change X", "Starting value Y"],
        |x| Ok(format!("{} changed by {}% = {}.\nSteps: {} x {} = {}.", n(a(x,1)), n(a(x,0)), nd(a(x,1)*(1.0+a(x,0)/100.0),4), n(a(x,1)), nd(1.0+a(x,0)/100.0,4), nd(a(x,1)*(1.0+a(x,0)/100.0),4)))));

    // ---- Fractions (numerator, denominator pairs as fixed 4-input where needed) ----
    reg(v, Calc::new("fraction-simplify", "Simplify a Fraction", "Math",
        "Reduce a fraction to lowest terms by dividing top and bottom by their greatest common divisor (GCD).", &["Numerator", "Denominator"],
        |x| { let (p,q)=(a(x,0) as i64, a(x,1) as i64); chk(a(x,0)==p as f64 && a(x,1)==q as f64, "Use whole numbers for the numerator and denominator.")?; chk(q!=0,"The denominator cannot be zero.")?; let g=gcd_i(p,q).max(1); let (np,nq)=norm_frac(p/g, q/g); Ok(format!("{}/{} simplifies to {}/{} (GCD = {}). Whole part: {}.", p,q, np, nq, g, np/nq)) }));
    reg(
        v,
        Calc::new(
            "fraction-add",
            "Add Two Fractions",
            "Math",
            "a/b + c/d = (a*d + c*b) / (b*d), then simplified.",
            &[
                "Numerator 1",
                "Denominator 1",
                "Numerator 2",
                "Denominator 2",
            ],
            |x| frac_op(x, 1.0, "+"),
        ),
    );
    reg(
        v,
        Calc::new(
            "fraction-subtract",
            "Subtract Two Fractions",
            "Math",
            "a/b - c/d = (a*d - c*b) / (b*d), then simplified.",
            &[
                "Numerator 1",
                "Denominator 1",
                "Numerator 2",
                "Denominator 2",
            ],
            |x| frac_op(x, -1.0, "-"),
        ),
    );
    reg(
        v,
        Calc::new(
            "fraction-multiply",
            "Multiply Two Fractions",
            "Math",
            "a/b x c/d = (a*c) / (b*d), then simplified.",
            &[
                "Numerator 1",
                "Denominator 1",
                "Numerator 2",
                "Denominator 2",
            ],
            |x| {
                chk(
                    a(x, 1) != 0.0 && a(x, 3) != 0.0,
                    "Denominators cannot be zero.",
                )?;
                let p = (a(x, 0) * a(x, 2)) as i64;
                let q = (a(x, 1) * a(x, 3)) as i64;
                let g = gcd_i(p, q).max(1);
                let (rp, rq) = norm_frac(p / g, q / g);
                Ok(format!(
                    "Result: {}/{} (decimal {}).",
                    rp,
                    rq,
                    nd(rp as f64 / rq as f64, 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "fraction-divide",
            "Divide Two Fractions",
            "Math",
            "a/b / (c/d) = (a*d) / (b*c), then simplified.",
            &[
                "Numerator 1",
                "Denominator 1",
                "Numerator 2",
                "Denominator 2",
            ],
            |x| {
                chk(
                    a(x, 1) != 0.0 && a(x, 2) != 0.0,
                    "Denominators and the divisor's numerator cannot be zero.",
                )?;
                let p = (a(x, 0) * a(x, 3)) as i64;
                let q = (a(x, 1) * a(x, 2)) as i64;
                let g = gcd_i(p, q).max(1);
                let (rp, rq) = norm_frac(p / g, q / g);
                Ok(format!(
                    "Result: {}/{} (decimal {}).",
                    rp,
                    rq,
                    nd(rp as f64 / rq as f64, 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "fraction-to-decimal",
            "Fraction / Mixed Number to Decimal & Percent",
            "Math",
            "Decimal = whole + numerator / denominator (a negative whole part makes the fraction part negative too, so -1 and 1/2 means -1.5). Percent = decimal x 100.",
            &["Whole part (0 if none)", "Numerator", "Denominator"],
            |x| {
                nz(a(x, 2), "The denominator")?;
                let frac = a(x, 1) / a(x, 2);
                let d = if a(x, 0) < 0.0 {
                    a(x, 0) - frac.abs()
                } else {
                    a(x, 0) + frac
                };
                Ok(format!(
                    "Decimal: {}. Percent: {}%.",
                    nd(d, 8),
                    nd(d * 100.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "decimal-to-fraction",
            "Decimal to Fraction",
            "Math",
            "Writes a decimal as a fraction over a power of ten, then simplifies with the GCD. Up to 9 decimal places are used; a longer decimal is rounded at the 9th place first.",
            &["Decimal"],
            |x| {
                let mut den = 1i64;
                let mut val = a(x, 0);
                let mut guard = 0;
                while (val - val.trunc()).abs() > 1e-9 && guard < 9 {
                    val *= 10.0;
                    den *= 10;
                    guard += 1;
                }
                let p = val.round() as i64;
                let g = gcd_i(p, den).max(1);
                Ok(format!(
                    "{} = {}/{} (before simplifying: {}/{}).",
                    n(a(x, 0)),
                    p / g,
                    den / g,
                    p,
                    den
                ))
            },
        ),
    );

    // ---- Powers, roots, logs ----
    reg(
        v,
        Calc::new(
            "exponent",
            "Exponent (Power)",
            "Math",
            "base ^ exponent.",
            &["Base", "Exponent"],
            |x| {
                let r = a(x, 0).powf(a(x, 1));
                finite(r)?;
                Ok(format!("{} ^ {} = {}", n(a(x, 0)), n(a(x, 1)), n(r)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "square-cube",
            "Square & Cube of a Number",
            "Math",
            "x^2 and x^3.",
            &["Number x"],
            |x| {
                Ok(format!(
                    "{} squared = {}. {} cubed = {}.",
                    n(a(x, 0)),
                    n(a(x, 0) * a(x, 0)),
                    n(a(x, 0)),
                    n(a(x, 0).powi(3))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "square-root",
            "Square Root",
            "Math",
            "sqrt(x). Also shows the cube root.",
            &["Number x"],
            |x| {
                chk(
                    a(x, 0) >= 0.0,
                    "No real answer: the square root of a negative number is not a real number (it is imaginary). Enter 0 or a positive number.",
                )?;
                Ok(format!(
                    "sqrt({}) = {}. Cube root: {}.",
                    n(a(x, 0)),
                    nd(a(x, 0).sqrt(), 8),
                    nd(a(x, 0).cbrt(), 8)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "nth-root",
            "Nth Root",
            "Math",
            "The n-th root of x: x^(1/n).",
            &["Number x", "Root n"],
            |x| {
                nz(a(x, 1), "The root n")?;
                if a(x, 0) < 0.0 {
                    let ni = a(x, 1) as i64;
                    chk(
                        a(x, 1) == ni as f64 && ni % 2 != 0,
                        "A negative number has a real root only for odd whole-number roots (3, 5, 7, ...). Even or non-whole roots of a negative number are not real.",
                    )?;
                    let r = -((-a(x, 0)).powf(1.0 / a(x, 1)));
                    return Ok(format!(
                        "The {}-th root of {} = {}.",
                        n(a(x, 1)),
                        n(a(x, 0)),
                        nd(r, 8)
                    ));
                }
                let r = a(x, 0).powf(1.0 / a(x, 1));
                finite(r)?;
                Ok(format!(
                    "The {}-th root of {} = {}.",
                    n(a(x, 1)),
                    n(a(x, 0)),
                    nd(r, 8)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "log",
            "Logarithm (Any Base)",
            "Math",
            "log_base(x) = ln(x) / ln(base).",
            &["Number x", "Base"],
            |x| {
                chk(a(x, 0) > 0.0, "No real answer: log(x) is only defined for x > 0 - the log of zero or a negative number has no real value.")?;
                chk(
                    a(x, 1) > 0.0 && a(x, 1) != 1.0,
                    "The base of a logarithm must be positive and not equal to 1.",
                )?;
                Ok(format!(
                    "log base {} of {} = {}",
                    n(a(x, 1)),
                    n(a(x, 0)),
                    nd(a(x, 0).ln() / a(x, 1).ln(), 8)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ln-log10",
            "Natural Log & Log10",
            "Math",
            "ln(x) and log10(x).",
            &["Number x"],
            |x| {
                chk(
                    a(x, 0) > 0.0,
                    "No real answer: ln(x) and log10(x) are only defined for x > 0.",
                )?;
                Ok(format!(
                    "ln({}) = {}. log10({}) = {}.",
                    n(a(x, 0)),
                    nd(a(x, 0).ln(), 8),
                    n(a(x, 0)),
                    nd(a(x, 0).log10(), 8)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "antilog",
            "Antilog (10^x and e^x)",
            "Math",
            "10^x and e^x.",
            &["Number x"],
            |x| {
                Ok(format!(
                    "10^{} = {}. e^{} = {}.",
                    n(a(x, 0)),
                    n(10f64.powf(a(x, 0))),
                    n(a(x, 0)),
                    n(a(x, 0).exp())
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "factorial",
            "Factorial",
            "Math",
            "x! = 1 x 2 x ... x x, for whole numbers 0..170.",
            &["Whole number x"],
            |x| {
                let f = factorial(a(x, 0))?;
                Ok(format!("{}! = {}", n(a(x, 0)), n(f)))
            },
        ),
    );
    reg(v, Calc::new("scientific-notation", "Scientific Notation Converter", "Math",
        "Writes a number as a x 10^b, and shows its plain form from a x 10^b inputs (use the second calculator).", &["Number"],
        |x| { chk(a(x,0)!=0.0,"Zero has no scientific-notation exponent.")?; let e=a(x,0).abs().log10().floor(); let mant=a(x,0)/10f64.powf(e); Ok(format!("{} = {} x 10^{} (engineering: {}).", n(a(x,0)), nd(mant,6), e as i64, n(a(x,0)))) }));
    reg(
        v,
        Calc::new(
            "from-scientific",
            "From Scientific Notation (a x 10^b)",
            "Math",
            "Plain number = a x 10^b.",
            &["Coefficient a", "Exponent b"],
            |x| {
                Ok(format!(
                    "{} x 10^{} = {}",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    n(a(x, 0) * 10f64.powf(a(x, 1)))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "sig-figs",
            "Significant Figures Rounding",
            "Math",
            "Rounds a number to the chosen number of significant figures.",
            &["Number", "Significant figures"],
            |x| {
                chk(a(x, 1) >= 1.0, "Significant figures must be at least 1.")?;
                chk(a(x, 0) != 0.0, "Zero stays zero at any sig figs.")?;
                let sf = a(x, 1) as i32;
                let e = a(x, 0).abs().log10().floor() as i32;
                let scale = 10f64.powi(sf - 1 - e);
                let r = (a(x, 0) * scale).round() / scale;
                Ok(format!("{} rounded to {} significant figure(s) = {}. Sig figs in the original: about {}.", n(a(x,0)), sf, n(r), count_sf(a(x,0))))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "round-decimals",
            "Round to Decimal Places",
            "Math",
            "Rounds a number to the chosen decimal places (half up).",
            &["Number", "Decimal places"],
            |x| {
                let p = 10f64.powi(a(x, 1) as i32);
                Ok(format!(
                    "{} rounded to {} place(s) = {}",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    nd((a(x, 0) * p).round() / p, a(x, 1) as usize)
                ))
            },
        ),
    );

    // ---- Number theory ----
    reg(
        v,
        Calc::new(
            "gcd",
            "Greatest Common Divisor (GCD)",
            "Math",
            "Largest whole number dividing both, by Euclid's method.",
            &["Whole number A", "Whole number B"],
            |x| {
                Ok(format!(
                    "GCD({}, {}) = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    gcd_i(a(x, 0) as i64, a(x, 1) as i64)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "lcm",
            "Least Common Multiple (LCM)",
            "Math",
            "LCM = |A x B| / GCD(A,B).",
            &["Whole number A", "Whole number B"],
            |x| {
                let g = gcd_i(a(x, 0) as i64, a(x, 1) as i64);
                chk(
                    g != 0,
                    "LCM with zero is zero/undefined here — use two non-zero numbers.",
                )?;
                Ok(format!(
                    "LCM({}, {}) = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    (a(x, 0) as i64).abs() / g * (a(x, 1) as i64).abs()
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "prime-check",
            "Prime Check & Factors",
            "Math",
            "Tests primality by trial division and lists all factors.",
            &["Whole number"],
            |x| {
                let val = a(x, 0) as i64;
                chk(
                    a(x, 0) == val as f64 && val >= 2,
                    "Enter a whole number of 2 or more.",
                )?;
                let mut fac = Vec::new();
                let mut d = 1;
                while d * d <= val {
                    if val % d == 0 {
                        fac.push(d);
                        if d != val / d {
                            fac.push(val / d);
                        }
                    }
                    d += 1;
                }
                fac.sort();
                let prime = fac.len() == 2;
                Ok(format!(
                    "{} is {}prime. Factors: {}.",
                    val,
                    if prime { "" } else { "not " },
                    fac.iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ratio",
            "Ratio Simplify & Scale",
            "Math",
            "Simplifies A:B by the GCD (decimal parts like 1.5 are scaled up to whole numbers first, so 1.5:2.5 correctly becomes 3:5) and scales A:B so the first part becomes the target.",
            &["Ratio part A", "Ratio part B", "Scale A to (target)"],
            |x| {
                // Scale both parts by a power of ten until they are whole
                // numbers (up to 6 decimal places), then use the GCD.
                let mut scale = 1.0f64;
                for _ in 0..6 {
                    if (a(x, 0) * scale).fract() == 0.0 && (a(x, 1) * scale).fract() == 0.0 {
                        break;
                    }
                    scale *= 10.0;
                }
                let (ia, ib) = (
                    (a(x, 0) * scale).round() as i64,
                    (a(x, 1) * scale).round() as i64,
                );
                let g = gcd_i(ia, ib).max(1);
                nz(a(x, 0), "Ratio part A (for scaling)")?;
                nz(a(x, 0) + a(x, 1), "A + B (for the share)")?;
                let k = a(x, 2) / a(x, 0);
                Ok(format!(
                    "Simplified ratio: {}:{}. Scaled to A={}: {}:{}. A share of total: {}%.",
                    ia / g,
                    ib / g,
                    n(a(x, 2)),
                    n(a(x, 2)),
                    nd(a(x, 1) * k, 4),
                    nd(a(x, 0) / (a(x, 0) + a(x, 1)) * 100.0, 2)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "proportion",
            "Proportion / Cross-Multiply (Solve for x)",
            "Math",
            "Solves a proportion in either standard form. Form 1: A/B = x/C, so x = A x C / B. Form 2: A/B = C/x, so x = B x C / A. The third input is C in both forms; both answers are shown so you can use the one that matches your equation.",
            &["A", "B", "C (third known value)"],
            |x| {
                nz(a(x, 0), "A (for form 2)")?;
                nz(a(x, 1), "B (for form 1)")?;
                Ok(format!("Form 1 (A/B = x/C): x = {} x {} / {} = {}.\nForm 2 (A/B = C/x): x = {} x {} / {} = {}.", n(a(x,0)), n(a(x,2)), n(a(x,1)), nd(a(x,0)*a(x,2)/a(x,1),6), n(a(x,1)), n(a(x,2)), n(a(x,0)), nd(a(x,1)*a(x,2)/a(x,0),6)))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "rule-of-three",
            "Rule of Three (Direct Proportion)",
            "Math",
            "If A corresponds to B, then C corresponds to x = B*C/A.",
            &["A", "B", "C"],
            |x| {
                nz(a(x, 0), "A")?;
                Ok(format!(
                    "x = {} x {} / {} = {}.",
                    n(a(x, 1)),
                    n(a(x, 2)),
                    n(a(x, 0)),
                    nd(a(x, 1) * a(x, 2) / a(x, 0), 6)
                ))
            },
        ),
    );

    // ---- Algebra ----
    reg(v, Calc::new("quadratic", "Quadratic Solver (ax^2+bx+c=0)", "Math",
        "Quadratic formula: x = (-b +- sqrt(b^2-4ac)) / 2a. Handles negative discriminant with complex roots.", &["a", "b", "c"],
        |x| { nz(a(x,0),"a")?; let d=a(x,1)*a(x,1)-4.0*a(x,0)*a(x,2); if d>=0.0 { let r1=(-a(x,1)+d.sqrt())/(2.0*a(x,0)); let r2=(-a(x,1)-d.sqrt())/(2.0*a(x,0)); Ok(format!("Discriminant = {}. Roots: x1 = {}, x2 = {}. Vertex x = {}.", nd(d,6), nd(r1,8), nd(r2,8), nd(-a(x,1)/(2.0*a(x,0)),6))) } else { let re=-a(x,1)/(2.0*a(x,0)); let im=(-d).sqrt()/(2.0*a(x,0)).abs(); Ok(format!("Discriminant = {} (negative). Complex roots: x = {} + {}i and {} - {}i.", nd(d,6), nd(re,8), nd(im,8), nd(re,8), nd(im,8))) } }));
    reg(
        v,
        Calc::new(
            "linear-equation",
            "Linear Equation Solver (ax+b=c)",
            "Math",
            "x = (c - b) / a.",
            &["a", "b", "c"],
            |x| {
                nz(a(x, 0), "a")?;
                Ok(format!(
                    "x = ({} - {}) / {} = {}.",
                    n(a(x, 2)),
                    n(a(x, 1)),
                    n(a(x, 0)),
                    nd((a(x, 2) - a(x, 1)) / a(x, 0), 8)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "slope",
            "Slope of a Line (Two Points)",
            "Math",
            "Slope m = (y2-y1)/(x2-x1). Also gives the y-intercept and line equation.",
            &["x1", "y1", "x2", "y2"],
            |x| {
                nz(a(x, 2) - a(x, 0), "x2 - x1")?;
                let mm = (a(x, 3) - a(x, 1)) / (a(x, 2) - a(x, 0));
                let b = a(x, 1) - mm * a(x, 0);
                Ok(format!(
                    "Slope = {}. Line: y = {}x + {}. Angle: {} degrees.",
                    nd(mm, 6),
                    nd(mm, 6),
                    nd(b, 6),
                    nd(mm.atan() * 180.0 / PI, 3)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "distance-points",
            "Distance Between Two Points",
            "Math",
            "d = sqrt((x2-x1)^2 + (y2-y1)^2). Also gives the midpoint.",
            &["x1", "y1", "x2", "y2"],
            |x| {
                let d = ((a(x, 2) - a(x, 0)).powi(2) + (a(x, 3) - a(x, 1)).powi(2)).sqrt();
                Ok(format!(
                    "Distance = {}. Midpoint = ({}, {}).",
                    nd(d, 6),
                    nd((a(x, 0) + a(x, 2)) / 2.0, 4),
                    nd((a(x, 1) + a(x, 3)) / 2.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "pythagorean",
            "Pythagorean Theorem (Solve Hypotenuse)",
            "Math",
            "c = sqrt(a^2 + b^2) for a right triangle's legs a and b.",
            &["Leg a", "Leg b"],
            |x| {
                Ok(format!(
                    "Hypotenuse c = sqrt({}^2 + {}^2) = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    nd((a(x, 0).powi(2) + a(x, 1).powi(2)).sqrt(), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "pythagorean-leg",
            "Pythagorean Theorem (Solve a Leg)",
            "Math",
            "leg = sqrt(c^2 - other^2).",
            &["Hypotenuse c", "Other leg"],
            |x| {
                chk(
                    a(x, 0) > a(x, 1),
                    "The hypotenuse must be longer than the other leg.",
                )?;
                Ok(format!(
                    "Missing leg = sqrt({}^2 - {}^2) = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    nd((a(x, 0).powi(2) - a(x, 1).powi(2)).sqrt(), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "arithmetic-sequence",
            "Arithmetic Sequence (nth Term & Sum)",
            "Math",
            "term_n = a1 + (n-1)d. Sum = n/2 * (2a1 + (n-1)d).",
            &["First term a1", "Common difference d", "Term number n"],
            |x| {
                let t = a(x, 0) + (a(x, 2) - 1.0) * a(x, 1);
                let s = a(x, 2) / 2.0 * (2.0 * a(x, 0) + (a(x, 2) - 1.0) * a(x, 1));
                Ok(format!(
                    "Term {} = {}. Sum of first {} terms = {}.",
                    n(a(x, 2)),
                    n(t),
                    n(a(x, 2)),
                    n(s)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "geometric-sequence",
            "Geometric Sequence (nth Term & Sum)",
            "Math",
            "term_n = a1 * r^(n-1). Sum = a1 * (r^n - 1)/(r - 1).",
            &["First term a1", "Common ratio r", "Term number n"],
            |x| {
                let t = a(x, 0) * a(x, 1).powf(a(x, 2) - 1.0);
                let s = if a(x, 1) == 1.0 {
                    a(x, 0) * a(x, 2)
                } else {
                    a(x, 0) * (a(x, 1).powf(a(x, 2)) - 1.0) / (a(x, 1) - 1.0)
                };
                Ok(format!(
                    "Term {} = {}. Sum of first {} terms = {}.",
                    n(a(x, 2)),
                    nd(t, 6),
                    n(a(x, 2)),
                    nd(s, 6)
                ))
            },
        ),
    );

    // ---- Geometry: areas ----
    reg(
        v,
        Calc::new(
            "circle",
            "Circle (Area, Circumference, Diameter)",
            "Math",
            "Area = pi r^2. Circumference = 2 pi r. Diameter = 2r.",
            &["Radius r"],
            |x| {
                chk(a(x, 0) >= 0.0, "Radius cannot be negative.")?;
                Ok(format!(
                    "Area = {}. Circumference = {}. Diameter = {}.",
                    nd(PI * a(x, 0).powi(2), 6),
                    nd(2.0 * PI * a(x, 0), 6),
                    n(2.0 * a(x, 0))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "circle-from-area",
            "Circle Radius From Area",
            "Math",
            "r = sqrt(A / pi).",
            &["Area A"],
            |x| {
                chk(a(x, 0) >= 0.0, "Area cannot be negative.")?;
                Ok(format!(
                    "Radius = {}. Diameter = {}. Circumference = {}.",
                    nd((a(x, 0) / PI).sqrt(), 6),
                    nd(2.0 * (a(x, 0) / PI).sqrt(), 6),
                    nd(2.0 * PI * (a(x, 0) / PI).sqrt(), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "rectangle",
            "Rectangle (Area & Perimeter)",
            "Math",
            "Area = l x w. Perimeter = 2(l + w). Diagonal = sqrt(l^2 + w^2).",
            &["Length", "Width"],
            |x| {
                Ok(format!(
                    "Area = {}. Perimeter = {}. Diagonal = {}.",
                    n(a(x, 0) * a(x, 1)),
                    n(2.0 * (a(x, 0) + a(x, 1))),
                    nd((a(x, 0).powi(2) + a(x, 1).powi(2)).sqrt(), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "square-shape",
            "Square (Area, Perimeter, Diagonal)",
            "Math",
            "Area = s^2. Perimeter = 4s. Diagonal = s*sqrt(2).",
            &["Side s"],
            |x| {
                Ok(format!(
                    "Area = {}. Perimeter = {}. Diagonal = {}.",
                    n(a(x, 0).powi(2)),
                    n(4.0 * a(x, 0)),
                    nd(a(x, 0) * std::f64::consts::SQRT_2, 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "triangle-area",
            "Triangle Area (Base & Height)",
            "Math",
            "Area = base x height / 2.",
            &["Base", "Height"],
            |x| {
                Ok(format!(
                    "Area = {} x {} / 2 = {}.",
                    n(a(x, 0)),
                    n(a(x, 1)),
                    n(a(x, 0) * a(x, 1) / 2.0)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "triangle-heron",
            "Triangle Area & Perimeter (Heron, 3 Sides)",
            "Math",
            "s = (a+b+c)/2. Area = sqrt(s(s-a)(s-b)(s-c)). Sides must form a real triangle.",
            &["Side a", "Side b", "Side c"],
            |x| {
                chk(a(x,0)+a(x,1)>a(x,2) && a(x,0)+a(x,2)>a(x,1) && a(x,1)+a(x,2)>a(x,0),"Those sides cannot form a triangle (two short sides must add to more than the long side).")?;
                let s = (a(x, 0) + a(x, 1) + a(x, 2)) / 2.0;
                Ok(format!(
                    "Perimeter = {}. Area = {}.",
                    n(2.0 * s),
                    nd(
                        (s * (s - a(x, 0)) * (s - a(x, 1)) * (s - a(x, 2))).sqrt(),
                        6
                    )
                ))
            },
        ),
    );
    reg(v, Calc::new("trapezoid", "Trapezoid (Area & Perimeter)", "Math",
        "Area = (a + b)/2 x height, where a and b are the two parallel sides. Perimeter needs both legs too: enter leg1 and leg2.", &["Parallel side a", "Parallel side b", "Height", "Leg 1", "Leg 2"],
        |x| Ok(format!("Area = {}. Perimeter = {}.", n((a(x,0)+a(x,1))/2.0*a(x,2)), n(a(x,0)+a(x,1)+a(x,3)+a(x,4))))));
    reg(
        v,
        Calc::new(
            "parallelogram",
            "Parallelogram (Area & Perimeter)",
            "Math",
            "Area = base x height. Perimeter = 2(base + side).",
            &["Base", "Height", "Side"],
            |x| {
                Ok(format!(
                    "Area = {}. Perimeter = {}.",
                    n(a(x, 0) * a(x, 1)),
                    n(2.0 * (a(x, 0) + a(x, 2)))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "rhombus",
            "Rhombus (Area From Diagonals)",
            "Math",
            "Area = (d1 x d2) / 2. Side = sqrt((d1/2)^2 + (d2/2)^2). Perimeter = 4 x side.",
            &["Diagonal 1", "Diagonal 2"],
            |x| {
                let side = ((a(x, 0) / 2.0).powi(2) + (a(x, 1) / 2.0).powi(2)).sqrt();
                Ok(format!(
                    "Area = {}. Side = {}. Perimeter = {}.",
                    n(a(x, 0) * a(x, 1) / 2.0),
                    nd(side, 6),
                    nd(4.0 * side, 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "ellipse",
            "Ellipse (Area & Perimeter Estimate)",
            "Math",
            "Area = pi a b. Perimeter uses Ramanujan's approximation.",
            &["Semi-axis a", "Semi-axis b"],
            |x| {
                let per = PI
                    * (3.0 * (a(x, 0) + a(x, 1))
                        - ((3.0 * a(x, 0) + a(x, 1)) * (a(x, 0) + 3.0 * a(x, 1))).sqrt());
                Ok(format!(
                    "Area = {}. Perimeter (approx) = {}.",
                    nd(PI * a(x, 0) * a(x, 1), 6),
                    nd(per, 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "regular-polygon",
            "Regular Polygon (Area & Perimeter)",
            "Math",
            "Area = n s^2 / (4 tan(pi/n)). Perimeter = n s.",
            &["Number of sides n", "Side length s"],
            |x| {
                chk(a(x, 0) >= 3.0, "A polygon needs at least 3 sides.")?;
                Ok(format!(
                    "Area = {}. Perimeter = {}. Interior angle = {} degrees.",
                    nd(a(x, 0) * a(x, 1).powi(2) / (4.0 * (PI / a(x, 0)).tan()), 6),
                    n(a(x, 0) * a(x, 1)),
                    nd((a(x, 0) - 2.0) * 180.0 / a(x, 0), 3)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "sector",
            "Circle Sector & Arc Length",
            "Math",
            "Arc = 2 pi r x (angle/360). Sector area = pi r^2 x (angle/360).",
            &["Radius r", "Angle (degrees)"],
            |x| {
                Ok(format!(
                    "Arc length = {}. Sector area = {}.",
                    nd(2.0 * PI * a(x, 0) * a(x, 1) / 360.0, 6),
                    nd(PI * a(x, 0).powi(2) * a(x, 1) / 360.0, 6)
                ))
            },
        ),
    );

    // ---- Geometry: volumes ----
    reg(v, Calc::new("cube-box", "Box / Cube (Volume & Surface Area)", "Math",
        "Volume = l x w x h. Surface = 2(lw + lh + wh). For a cube, enter the same side three times.", &["Length", "Width", "Height"],
        |x| Ok(format!("Volume = {}. Surface area = {}.", n(a(x,0)*a(x,1)*a(x,2)), n(2.0*(a(x,0)*a(x,1)+a(x,0)*a(x,2)+a(x,1)*a(x,2)))))));
    reg(
        v,
        Calc::new(
            "cylinder",
            "Cylinder (Volume & Surface Area)",
            "Math",
            "Volume = pi r^2 h. Surface = 2 pi r (r + h).",
            &["Radius r", "Height h"],
            |x| {
                Ok(format!(
                    "Volume = {}. Surface area = {}.",
                    nd(PI * a(x, 0).powi(2) * a(x, 1), 6),
                    nd(2.0 * PI * a(x, 0) * (a(x, 0) + a(x, 1)), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "sphere",
            "Sphere (Volume & Surface Area)",
            "Math",
            "Volume = 4/3 pi r^3. Surface = 4 pi r^2.",
            &["Radius r"],
            |x| {
                Ok(format!(
                    "Volume = {}. Surface area = {}.",
                    nd(4.0 / 3.0 * PI * a(x, 0).powi(3), 6),
                    nd(4.0 * PI * a(x, 0).powi(2), 6)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "cone",
            "Cone (Volume & Surface Area)",
            "Math",
            "Volume = 1/3 pi r^2 h. Slant = sqrt(r^2+h^2). Surface = pi r (r + slant).",
            &["Radius r", "Height h"],
            |x| {
                let sl = (a(x, 0).powi(2) + a(x, 1).powi(2)).sqrt();
                Ok(format!(
                    "Volume = {}. Slant height = {}. Surface area = {}.",
                    nd(PI * a(x, 0).powi(2) * a(x, 1) / 3.0, 6),
                    nd(sl, 6),
                    nd(PI * a(x, 0) * (a(x, 0) + sl), 6)
                ))
            },
        ),
    );
    reg(v, Calc::new("pyramid", "Square Pyramid (Volume & Surface)", "Math",
        "Volume = base^2 x h / 3. Surface = base^2 + 2 x base x slant, slant = sqrt((base/2)^2 + h^2).", &["Base side", "Height h"],
        |x| { let sl=((a(x,0)/2.0).powi(2)+a(x,1).powi(2)).sqrt(); Ok(format!("Volume = {}. Surface area = {}.", nd(a(x,0).powi(2)*a(x,1)/3.0,6), nd(a(x,0).powi(2)+2.0*a(x,0)*sl,6))) }));
    reg(
        v,
        Calc::new(
            "frustum",
            "Cone Frustum (Volume)",
            "Math",
            "Volume = (1/3) pi h (R^2 + Rr + r^2).",
            &["Bottom radius R", "Top radius r", "Height h"],
            |x| {
                Ok(format!(
                    "Volume = {}.",
                    nd(
                        PI * a(x, 2) * (a(x, 0).powi(2) + a(x, 0) * a(x, 1) + a(x, 1).powi(2))
                            / 3.0,
                        6
                    )
                ))
            },
        ),
    );

    // ---- Trig & angles ----
    reg(
        v,
        Calc::new(
            "trig-deg",
            "Trig Functions (Degrees)",
            "Math",
            "sin, cos, tan of an angle in degrees.",
            &["Angle (degrees)"],
            |x| {
                let r = a(x, 0) * PI / 180.0;
                Ok(format!(
                    "sin = {}. cos = {}. tan = {}.",
                    nd(r.sin(), 8),
                    nd(r.cos(), 8),
                    nd(r.tan(), 8)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "trig-inverse",
            "Inverse Trig (Result in Degrees)",
            "Math",
            "asin, acos, atan of a ratio. asin/acos need a value between -1 and 1.",
            &["Ratio"],
            |x| {
                chk(
                    a(x, 0).abs() <= 1.0,
                    "asin/acos need a value between -1 and 1.",
                )?;
                Ok(format!(
                    "asin = {} deg. acos = {} deg. atan = {} deg.",
                    nd(a(x, 0).asin() * 180.0 / PI, 6),
                    nd(a(x, 0).acos() * 180.0 / PI, 6),
                    nd(a(x, 0).atan() * 180.0 / PI, 6)
                ))
            },
        ),
    );
    reg(v, Calc::new("angle-convert", "Angle Converter (Deg / Rad / Grad / Turn)", "Math",
        "Inputs: value and a unit code: 0=degrees, 1=radians, 2=gradians, 3=turns. Converts to all.", &["Value", "Unit code (0 deg,1 rad,2 grad,3 turn)"],
        |x| { let deg=match a(x,1) as i64 { 0=>a(x,0), 1=>a(x,0)*180.0/PI, 2=>a(x,0)*0.9, 3=>a(x,0)*360.0, _=>return Err("Unit code must be 0, 1, 2 or 3.".into()) }; Ok(format!("Degrees: {}. Radians: {}. Gradians: {}. Turns: {}.", nd(deg,8), nd(deg*PI/180.0,8), nd(deg/0.9,6), nd(deg/360.0,8))) }));
    reg(
        v,
        Calc::new(
            "average",
            "Average of Two Numbers",
            "Math",
            "Mean of two numbers, plus sum and mid-range helpers.",
            &["Number A", "Number B"],
            |x| {
                Ok(format!(
                    "Average = {}. Sum = {}. Difference = {}.",
                    nd((a(x, 0) + a(x, 1)) / 2.0, 6),
                    n(a(x, 0) + a(x, 1)),
                    n((a(x, 0) - a(x, 1)).abs())
                ))
            },
        ),
    );
}

/// Normalize a fraction so the denominator is positive (sign moves to
/// the numerator): 3/-2 becomes -3/2.
fn norm_frac(p: i64, q: i64) -> (i64, i64) {
    if q < 0 {
        (-p, -q)
    } else {
        (p, q)
    }
}

fn frac_op(x: &[f64], sign: f64, sym: &str) -> Result<String, String> {
    chk(
        a(x, 1) != 0.0 && a(x, 3) != 0.0,
        "Denominators cannot be zero.",
    )?;
    let p = (a(x, 0) * a(x, 3) + sign * a(x, 2) * a(x, 1)) as i64;
    let q = (a(x, 1) * a(x, 3)) as i64;
    let g = gcd_i(p, q).max(1);
    let (rp, rq) = norm_frac(p / g, q / g);
    Ok(format!(
        "Result: {}/{} {} {}/{} = {}/{} (decimal {}).",
        a(x, 0) as i64,
        a(x, 1) as i64,
        sym,
        a(x, 2) as i64,
        a(x, 3) as i64,
        rp,
        rq,
        nd(rp as f64 / rq as f64, 6)
    ))
}

fn count_sf(x: f64) -> usize {
    // Rough count: digits ignoring leading zeros (a guide only, labelled "about").
    let s = format!("{}", x.abs());
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        1
    } else {
        trimmed.len()
    }
}
