//! v0.1.2 audit tests — Chemistry and Construction.
//! Expected values computed independently (Python/hand) from the
//! standard published formulas (R = 0.082057 L atm/(mol K),
//! Avogadro 6.02214076e23, standard material densities/coverage).
use calcdex::run_calc;

fn out(id: &str, args: &[f64]) -> String {
    run_calc(id, args).unwrap_or_else(|e| panic!("{id} {args:?} errored: {e}"))
}
fn has(id: &str, args: &[f64], needle: &str) {
    let o = out(id, args);
    assert!(o.contains(needle), "{id} {args:?} missing {needle:?}:\n{o}");
}

#[test]
fn chemistry_amounts() {
    // 2 mol x 18 g/mol = 36 g.
    has("mass-from-moles", &[2.0, 18.0], "36");
    // 36 g / 18 g/mol = 2 mol.
    has("moles-from-mass", &[36.0, 18.0], "2");
    // 1 mol = Avogadro's number of particles (Rust sci format).
    has("molecules", &[1.0], "6.022141e23");
    // 0.5 mol in 0.25 L = 2 mol/L.
    has("molarity", &[0.5, 0.25], "2");
    // 58.5 g NaCl (58.5 g/mol) in 1 L = 1 mol/L.
    has("molarity-from-mass", &[58.5, 58.5, 1.0], "1");
    // Dilution: V1 = M2 V2 / M1 = 1 x 100 / 10 = 10.
    has("dilution-volume", &[10.0, 1.0, 100.0], "10");
    // Dilution forward: 10 mL of 10 M made to 100 mL -> 1 M.
    has("dilution", &[10.0, 10.0, 100.0], "1");
}

#[test]
fn chemistry_acid_gas() {
    // pH 7 -> [H+] = 1e-7 mol/L.
    has("ph-inverse", &[7.0], "1.000000e-7");
    // [OH-] = 0.001 -> pOH 3, pH 11.
    has("poh", &[0.001], "3");
    has("poh", &[0.001], "11");
    // Ideal gas: V = nRT/P = 0.082057 x 273.15 = 22.41387 L at 1 atm.
    has("ideal-gas-volume", &[1.0, 273.15, 1.0], "22.41387");
    // n = PV/RT = 22.414 / 22.41387 = 1.000006 mol.
    has("ideal-gas-moles", &[1.0, 22.414, 273.15], "1.000006");
    // T = PV/(nR) = 22.414 / 0.082057 = 273.1516 K.
    has("ideal-gas-temp", &[1.0, 22.414, 1.0], "273.1516");
    // Boyle: P2 = 2 x 3 / 6 = 1.
    has("boyle", &[2.0, 3.0, 6.0], "1");
    // Charles: V2 = 3 x 600/300 = 6.
    has("charles", &[3.0, 300.0, 600.0], "6");
    // Gay-Lussac: P2 = 2 x 450/300 = 3.
    has("gay-lussac", &[2.0, 300.0, 450.0], "3");
    // Combined: P2 = 1 x 2 x 600 / (1 x 300) = 4.
    has("combined-gas", &[1.0, 2.0, 300.0, 1.0, 600.0], "4");
}

#[test]
fn chemistry_mixtures() {
    // 30 g + 70 g -> 30% and 70%.
    has("percent-composition", &[30.0, 70.0], "30");
    has("percent-composition", &[30.0, 70.0], "70");
    // 10 g in 200 g solution = 5% = 50,000 ppm.
    has("solution-percent", &[10.0, 200.0], "5");
    has("solution-percent", &[10.0, 200.0], "50000");
    // 0.3 mol : 0.1 mol -> ratio 3 : 1.
    has("empirical-ratio", &[0.3, 0.1], "3");
    // Yield: 45 actual / 50 theoretical = 90%.
    has("reaction-yield", &[45.0, 50.0], "90");
}

#[test]
fn construction_areas_volumes() {
    // Circle diameter 10: pi x 25 = 78.5398.
    has("square-footage-circle", &[10.0], "78.5398");
    // Triangle base 10, height 4: 20.
    has("square-footage-triangle", &[10.0, 4.0], "20");
    // Column d=2 ft, depth 1 ft: pi cu ft = 3.142; 0.116 cu yd; 6 bags (80 lb).
    has("concrete-column", &[2.0, 1.0, 1.0, 0.0], "3.142");
    has("concrete-column", &[2.0, 1.0, 1.0, 0.0], "6");
    // Stairs as triangle prism: 3 x 6 x 9 / 2 = 81 cu ft = 3 cu yd.
    has("concrete-stairs", &[3.0, 6.0, 9.0, 0.0], "81");
    // Cubic yards: 9x9 ft at 12 in = 81 cu ft = 3 cu yd = 2.2937 m3.
    has("cubic-yards", &[9.0, 9.0, 12.0], "2.2937");
    // Room 10x12, walls 8 ft: walls 352, ceiling 120, total 472 sq ft.
    has("room-area-walls", &[10.0, 12.0, 8.0], "352");
    has("room-area-walls", &[10.0, 12.0, 8.0], "472");
    // Angle iron 12x12x0.5 in: 72 cu in x 490 lb/cu ft / 1728 = 20.42 lb.
    has("angle-iron-weight", &[12.0, 12.0, 0.5], "20.42");
}

#[test]
fn construction_materials() {
    // Tile: 100 sq ft, 12x12 in tiles (1 sq ft), 10/box -> 100 tiles, 10 boxes.
    has("tile", &[10.0, 10.0, 12.0, 12.0, 10.0, 0.0], "100");
    // Mulch: 100 sq ft at 3 in = 25 cu ft = 0.926 cu yd; 13 bags of 2 cu ft;
    // at 50/cu yd cost 46.30.
    has("mulch", &[100.0, 3.0, 50.0], "0.926");
    has("mulch", &[100.0, 3.0, 50.0], "46.3");
    // Gravel 10x10 at 3 in = 0.926 cu yd; at 1.4 tons/cu yd = 1.3 tons.
    has("gravel", &[10.0, 10.0, 3.0, 1.4], "1.3");
    // Flooring 10x12 = 120 sq ft + 10% waste = 132; 20/box -> 7 boxes; 210 cost.
    has("flooring", &[10.0, 12.0, 20.0, 10.0, 30.0], "132");
    has("flooring", &[10.0, 12.0, 20.0, 10.0, 30.0], "210");
    // Fence 100 ft, 8 ft spacing: 13 sections, 14 posts, 26 rails;
    // pickets 5.5 in + 0.5 gap: 1200/6 = 200.
    has("fence", &[100.0, 8.0, 2.0, 5.5, 0.5], "14");
    has("fence", &[100.0, 8.0, 2.0, 5.5, 0.5], "200");
    // Roofing 20x10, rise 6: factor 1.118, area 223.6, 2.24 squares, 7 bundles.
    has("roofing", &[20.0, 10.0, 6.0, 0.0], "223.6");
    has("roofing", &[20.0, 10.0, 6.0, 0.0], "7");
    // Drywall: 320 sq ft / 32 per sheet = 10 sheets.
    has("drywall", &[320.0, 32.0], "10");
    // Deck 12 long x 10 wide, boards 5.5 in + 0.5 gap: 120/6 = 20 boards.
    has("deck-boards", &[12.0, 10.0, 5.5, 0.5], "20");
    // Brick wall 10x8 = 80 sq ft x 6.55 = 524 bricks.
    has("brick", &[10.0, 8.0, 0.0], "524");
    // Wallpaper: 300 sq ft / 30 per roll = 10 rolls.
    has("wallpaper", &[300.0, 30.0], "10");
    // Insulation: 1000 sq ft at 10 in = 833.3 cu ft; 25/bag -> 34 bags.
    has("insulation-bags", &[1000.0, 10.0, 25.0], "34");
    // Asphalt 20x10 at 4 in = 66.7 cu ft x 145 lb = 9667 lb = 4.83 tons.
    has("asphalt-driveway", &[20.0, 10.0, 4.0], "4.83");
    // Blocks: 80 sq ft wall x 1.125 blocks/sq ft = 90 blocks.
    has("concrete-blocks", &[10.0, 8.0, 0.0], "90");
    // Post holes: 10 holes of 12 in dia x 24 in deep minus 4 in posts:
    // net 13.963 cu ft; 24 bags of 80 lb.
    has("post-holes-concrete", &[12.0, 24.0, 4.0, 10.0], "13.963");
    has("post-holes-concrete", &[12.0, 24.0, 4.0, 10.0], "24");
    // Board feet: 2x4 in x 8 ft, 1 board = 2x4x8/12 = 5.333 bf.
    has("board-feet", &[2.0, 4.0, 8.0, 1.0, 0.0], "5.333");
    // Stairs: total rise 84 in, target riser 7.5 -> steps round to
    // nearest = 11, actual riser = 84/11 = 7.636 in, run = 11x10 = 110,
    // stringer = sqrt(110^2 + 84^2) = 138.4 in.
    has("stairs", &[84.0, 7.5, 10.0], "11");
    has("stairs", &[84.0, 7.5, 10.0], "7.636");
    has("stairs", &[84.0, 7.5, 10.0], "138.4");
}
