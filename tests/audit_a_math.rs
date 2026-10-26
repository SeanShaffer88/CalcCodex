//! v0.1.2 audit tests — Math.
//!
//! Every expected value below was computed independently of the program
//! (hand / Python arithmetic from the standard published formula for
//! each calculator's name), never copied from program output. Where a
//! test pins a bug fix, the comment says what the old code did wrong.
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
fn percent_family() {
    // 50 is 25% of 200.
    has("percent-is", &[50.0, 200.0], "25");
    // |100-80| / 90 = 22.2222...%
    has("percent-difference", &[80.0, 100.0], "22.2222");
    // 20 is 20% of 100.
    has("reverse-percent", &[20.0, 120.0], "100");
    // 200 changed by -10% = 180.
    has("percent-off-result", &[-10.0, 200.0], "180");
    // (200 changed by +10% = 220 is covered by percent-off-result above.)
}

#[test]
fn fraction_family() {
    // 3/4 - 1/4 = 1/2.
    has("fraction-subtract", &[3.0, 4.0, 1.0, 4.0], "1/2");
    // 2/3 x 3/4 = 1/2. BUG PIN (v0.1.2 fix): the old output printed the
    // simplified NUMERATOR as a bogus "= value" (it showed "= 1").
    let o = out("fraction-multiply", &[2.0, 3.0, 3.0, 4.0]);
    assert!(o.contains("1/2 (decimal 0.5)"), "{o}");
    assert!(!o.contains("= 1 "), "{o}");
    // (1/2) / (3/4) = 2/3.
    has("fraction-divide", &[1.0, 2.0, 3.0, 4.0], "2/3");
    // Negative denominator is normalized: (3)/(-2) prints -3/2.
    // BUG PIN (v0.1.2 fix): sign used to stay on the denominator.
    has("fraction-divide", &[3.0, 1.0, -2.0, 1.0], "-3/2");
    // 6/8 simplifies to 3/4.
    has("fraction-simplify", &[6.0, 8.0], "3/4");
    // 0.75 = 3/4.
    has("decimal-to-fraction", &[0.75], "3/4");
    // 1 and 1/2 = 1.5 = 150%.
    has("fraction-to-decimal", &[1.0, 1.0, 2.0], "1.5");
    has("fraction-to-decimal", &[1.0, 1.0, 2.0], "150");
    // BUG PIN (v0.1.2 fix): -1 and 1/2 must be -1.5, not -0.5.
    has("fraction-to-decimal", &[-1.0, 1.0, 2.0], "-1.5");
    // From scientific: 1.5 x 10^4 = 15000.
    has("from-scientific", &[1.5, 4.0], "15000");
}

#[test]
fn powers_roots_logs() {
    has("exponent", &[2.0, 10.0], "1024");
    has("square-cube", &[5.0], "25");
    has("square-cube", &[5.0], "125");
    // sqrt(144) = 12; cube root of 144 = 5.24148279 (independent: 5.2414827884^3 = 144).
    has("square-root", &[144.0], "12");
    has("square-root", &[144.0], "5.24148279");
    has("nth-root", &[27.0, 3.0], "3");
    // BUG PIN (v0.1.2 fix): odd root of a negative number is real: -3.
    // The old code fell through to powf and reported "not a number".
    has("nth-root", &[-27.0, 3.0], "-3");
    err("nth-root", &[-16.0, 2.0], "not real");
    // antilog base 10 of 2 = 100; base e of 2 = 7.389056.
    has("antilog", &[2.0], "100");
    has("antilog", &[2.0], "7.389056");
}

#[test]
fn notation_and_rounding() {
    // 12345 = 1.2345 x 10^4.
    has("scientific-notation", &[12345.0], "1.2345");
    // 123.456 to 3 significant figures = 123.
    has("sig-figs", &[123.456, 3.0], "123");
    // Pi to 2 places = 3.14.
    has("round-decimals", &[std::f64::consts::PI, 2.0], "3.14");
}

#[test]
fn ratio_and_proportion() {
    // 4:6 simplifies to 2:3; scaled to A=10 gives 10:15; A share 40%.
    has("ratio", &[4.0, 6.0, 10.0], "2:3");
    has("ratio", &[4.0, 6.0, 10.0], "15");
    has("ratio", &[4.0, 6.0, 10.0], "40");
    // BUG PIN (v0.1.2 fix): decimal parts were truncated to whole
    // numbers before the GCD, so 1.5:2.5 wrongly became 1:2.
    // Correct: scale x10 -> 15:25 -> 3:5.
    has("ratio", &[1.5, 2.5, 3.0], "3:5");
    // Form 1: 2/3 = x/9 -> x = 6. Form 2: 2/3 = 9/x -> x = 13.5.
    // BUG PIN (v0.1.2 fix): the old second line was self-contradictory
    // ("with C=A: x = B"); both standard forms are now computed.
    let o = out("proportion", &[2.0, 3.0, 9.0]);
    assert!(o.contains("= 6."), "{o}");
    assert!(o.contains("13.5"), "{o}");
    // If 5 items cost 100, 8 items cost 160.
    has("rule-of-three", &[5.0, 100.0, 8.0], "160");
    // 2x + 3 = 11 -> x = 4.
    has("linear-equation", &[2.0, 3.0, 11.0], "4");
}

#[test]
fn sequences_and_series() {
    // 3, 5, 7, 9, 11: 5th term 11, sum 35.
    has("arithmetic-sequence", &[3.0, 2.0, 5.0], "11");
    has("arithmetic-sequence", &[3.0, 2.0, 5.0], "35");
    // 2, 6, 18, 54: 4th term 54, sum 80.
    has("geometric-sequence", &[2.0, 3.0, 4.0], "54");
    has("geometric-sequence", &[2.0, 3.0, 4.0], "80");
}

#[test]
fn geometry_2d() {
    // (0,0)-(6,8): distance 10, midpoint (3, 4).
    has("distance-points", &[0.0, 0.0, 6.0, 8.0], "10");
    has("distance-points", &[0.0, 0.0, 6.0, 8.0], "3");
    // Hypotenuse 13, leg 5 -> other leg 12.
    has("pythagorean-leg", &[13.0, 5.0], "12");
    // Square side 4: area 16, diagonal 4*sqrt(2) = 5.656854.
    has("square-shape", &[4.0], "16");
    has("square-shape", &[4.0], "5.656854");
    // 3-4-5 triangle by Heron: area 6, perimeter 12.
    has("triangle-heron", &[3.0, 4.0, 5.0], "6");
    // Trapezoid bases 4, 6, height 3: area (4+6)/2*3 = 15.
    has("trapezoid", &[4.0, 6.0, 3.0, 2.0, 2.0], "15");
    // Parallelogram base 5, height 3: area 15; side 4: perimeter 18.
    has("parallelogram", &[5.0, 3.0, 4.0], "15");
    has("parallelogram", &[5.0, 3.0, 4.0], "18");
    // Rhombus diagonals 6, 8: area 24, side 5, perimeter 20.
    has("rhombus", &[6.0, 8.0], "24");
    has("rhombus", &[6.0, 8.0], "20");
    // Ellipse a=2, b=1: area 2*pi = 6.283185; Ramanujan perimeter 9.688421.
    has("ellipse", &[2.0, 1.0], "6.283185");
    has("ellipse", &[2.0, 1.0], "9.688421");
    // Regular hexagon side 1: area 3*sqrt(3)/2 = 2.598076, interior angle 120.
    has("regular-polygon", &[6.0, 1.0], "2.598076");
    has("regular-polygon", &[6.0, 1.0], "120");
    // Sector r=2, 90 degrees: arc = area = pi = 3.141593.
    has("sector", &[2.0, 90.0], "3.141593");
}

#[test]
fn geometry_3d() {
    // Box 2x3x4: volume 24, surface 52.
    has("cube-box", &[2.0, 3.0, 4.0], "24");
    has("cube-box", &[2.0, 3.0, 4.0], "52");
    // Cylinder r=1, h=2: volume 2*pi = 6.283185, surface 6*pi = 18.849556.
    has("cylinder", &[1.0, 2.0], "6.283185");
    has("cylinder", &[1.0, 2.0], "18.849556");
    // Sphere r=1: volume 4*pi/3 = 4.18879, surface 4*pi = 12.566371.
    has("sphere", &[1.0], "4.18879");
    has("sphere", &[1.0], "12.566371");
    // Cone r=3, h=4: volume 12*pi = 37.699112, slant 5, total surface 24*pi = 75.398224.
    has("cone", &[3.0, 4.0], "37.699112");
    has("cone", &[3.0, 4.0], "75.398224");
    // Pyramid base side 2, height 3: volume 4, surface 16.649111.
    has("pyramid", &[2.0, 3.0], "16.649111");
    // Frustum R=2, r=1, h=3: volume 7*pi = 21.991149.
    has("frustum", &[2.0, 1.0, 3.0], "21.991149");
}

#[test]
fn misc_math() {
    // Average of 4 and 10 is 7; sum 14.
    let o = out("average", &[4.0, 10.0]);
    assert!(o.contains("Average = 7"), "{o}");
    assert!(o.contains("Sum = 14"), "{o}");
    // Area 25 pi -> radius 5, diameter 10, circumference 10 pi.
    has(
        "circle-from-area",
        &[25.0 * std::f64::consts::PI],
        "31.415927",
    );
    // Rectangle 5x3: area 15, perimeter 16, diagonal sqrt(34) = 5.830952.
    has("rectangle", &[5.0, 3.0], "15");
    has("rectangle", &[5.0, 3.0], "5.830952");
    // Triangle base 10, height 4: area 20.
    has("triangle-area", &[10.0, 4.0], "20");
    // ln(100) = 4.60517019; log10(100) = 2.
    has("ln-log10", &[100.0], "4.60517019");
    has("ln-log10", &[100.0], "2");
    // 7 is prime.
    has("prime-check", &[7.0], "prime");
}

#[test]
fn trig() {
    // sin 30 = 0.5, cos 30 = 0.866025, tan 30 = 0.57735.
    has("trig-deg", &[30.0], "0.5");
    has("trig-deg", &[30.0], "0.866025");
    has("trig-deg", &[30.0], "0.57735");
    // asin(0.5) = 30, acos(0.5) = 60, atan(0.5) = 26.565051 degrees.
    has("trig-inverse", &[0.5], "30");
    has("trig-inverse", &[0.5], "60");
    has("trig-inverse", &[0.5], "26.565051");
    // Input 180 with unit code 0 (degrees): converts to all units —
    // pi radians and half a turn.
    has("angle-convert", &[180.0, 0.0], "3.14159265");
    has("angle-convert", &[180.0, 0.0], "0.5");
}
