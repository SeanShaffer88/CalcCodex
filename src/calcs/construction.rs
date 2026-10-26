//! Construction calculators (Omni: Construction; heavily Nutilz:
//! concrete, paint, tile, mulch, gravel, flooring, fence, stair...).
//! US customary units (feet/inches/yards) unless the name says metric.
//! Waste factors are inputs, not hidden magic.
use crate::{a, n, nd, nz, Calc};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

pub fn register(v: &mut Vec<Calc>) {
    reg(v, Calc::new("square-footage-rect", "Square Footage (Rectangle)", "Construction",
        "Area = length x width. Works in feet (result sq ft) or metres (sq m) — keep units consistent.", &["Length", "Width"],
        |x| Ok(format!("Area = {} square units.", nd(a(x,0)*a(x,1),4)))));
    reg(
        v,
        Calc::new(
            "square-footage-circle",
            "Square Footage (Circle)",
            "Construction",
            "Area = pi x (diameter/2)^2.",
            &["Diameter"],
            |x| {
                Ok(format!(
                    "Area = {} square units.",
                    nd(std::f64::consts::PI * (a(x, 0) / 2.0).powi(2), 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "square-footage-triangle",
            "Square Footage (Triangle)",
            "Construction",
            "Area = base x height / 2.",
            &["Base", "Height"],
            |x| {
                Ok(format!(
                    "Area = {} square units.",
                    nd(a(x, 0) * a(x, 1) / 2.0, 4)
                ))
            },
        ),
    );
    reg(v, Calc::new("concrete-slab", "Concrete Calculator (Slab, Bags)", "Construction",
        "Volume = length ft x width ft x thickness in / 12. Cubic yards = cu ft / 27. Bag yields (standard): 80 lb ≈ 0.60 cu ft, 60 lb ≈ 0.45, 40 lb ≈ 0.30. Bags rounded UP, plus your waste %.", &["Length (ft)", "Width (ft)", "Thickness (inches)", "Waste %"],
        |x| { let cf=a(x,0)*a(x,1)*(a(x,2)/12.0); let cf_w=cf*(1.0+a(x,3)/100.0); Ok(format!("Concrete volume: {} cu ft = {} cubic yards ({} cu ft incl. waste).\nBags needed (rounded up, with waste): 80 lb: {} bags. 60 lb: {} bags. 40 lb: {} bags.", nd(cf,3), nd(cf/27.0,3), nd(cf_w,3), (cf_w/0.60).ceil(), (cf_w/0.45).ceil(), (cf_w/0.30).ceil())) }));
    reg(v, Calc::new("concrete-column", "Concrete Calculator (Round Column/Footing)", "Construction",
        "Volume = pi (d/2)^2 x depth, all in feet (depth in inches? divide by 12 yourself — depth here is FEET). Bags as in the slab calculator: 80 lb ≈ 0.60 cu ft etc.", &["Diameter (ft)", "Depth (ft)", "Number of columns", "Waste %"],
        |x| { let cf=std::f64::consts::PI*(a(x,0)/2.0).powi(2)*a(x,1)*a(x,2); let w=cf*(1.0+a(x,3)/100.0); Ok(format!("Volume: {} cu ft = {} cubic yards (incl waste {} cu ft). 80 lb bags: {}. 60 lb: {}. 40 lb: {}.", nd(cf,3), nd(cf/27.0,3), nd(w,3), (w/0.60).ceil(), (w/0.45).ceil(), (w/0.30).ceil())) }));
    reg(v, Calc::new("concrete-stairs", "Concrete Calculator (Stairs, Approx)", "Construction",
        "Estimate: treats the staircase as a triangle prism: volume ≈ width x total rise x total run / 2. Good planning estimate, not an engineering figure.", &["Stair width (ft)", "Total rise (ft)", "Total run (ft)", "Waste %"],
        |x| { let cf=a(x,0)*a(x,1)*a(x,2)/2.0; let w=cf*(1.0+a(x,3)/100.0); Ok(format!("Estimated volume: {} cu ft = {} cubic yards (incl waste {} cu ft). 80 lb bags: {}. ESTIMATE.", nd(cf,3), nd(cf/27.0,3), nd(w,3), (w/0.60).ceil())) }));
    reg(v, Calc::new("paint", "Paint Calculator", "Construction",
        "Paintable area = 4 walls (2 x (L+W) x height) minus doors (about 21 sq ft each) and windows (about 15 sq ft each). One gallon covers about 350 sq ft per coat. Enter your coverage if your paint differs.", &["Room length (ft)", "Room width (ft)", "Wall height (ft)", "Doors", "Windows", "Coats", "Coverage per gallon (sq ft)"],
        |x| { nz(a(x,6),"Coverage per gallon")?; let walls=2.0*(a(x,0)+a(x,1))*a(x,2); let area=(walls-a(x,3)*21.0-a(x,4)*15.0).max(0.0); let gal=area*a(x,5)/a(x,6); Ok(format!("Wall area: {} sq ft. Paintable after doors/windows: {} sq ft.\nPaint needed: {} gallons for {} coat(s) (about {} quarts).", nd(walls,1), nd(area,1), nd(gal,2), n(a(x,5)), nd(gal*4.0,1))) }));
    reg(v, Calc::new("tile", "Tile Calculator", "Construction",
        "Tiles = ceil(area incl waste / tile size). Tile area from inches: (tile L in x tile W in) / 144 sq ft. Boxes = ceil(tiles / tiles per box).", &["Area length (ft)", "Area width (ft)", "Tile length (in)", "Tile width (in)", "Tiles per box", "Waste %"],
        |x| { nz(a(x,4),"Tiles per box")?; let area=a(x,0)*a(x,1)*(1.0+a(x,5)/100.0); let tile=a(x,2)*a(x,3)/144.0; nz(tile,"Tile size")?; let tiles=(area/tile).ceil(); Ok(format!("Area (incl waste): {} sq ft. Tiles needed: {}. Boxes: {}. Grout-gap note: real layouts need extra for cuts — waste % covers it roughly.", nd(area,2), tiles, (tiles/a(x,4)).ceil())) }));
    reg(v, Calc::new("mulch", "Mulch / Topsoil / Compost Calculator", "Construction",
        "Volume cu ft = area sq ft x depth in / 12. Cubic yards = cu ft / 27. One standard bag = 2 cu ft; one yard = 27 cu ft ≈ 13.5 bags. Cost if you give a price per yard.", &["Area (sq ft)", "Depth (inches)", "Price per cubic yard (0 if unknown)"],
        |x| { let cf=a(x,0)*a(x,1)/12.0; let yd=cf/27.0; Ok(format!("Volume: {} cu ft = {} cubic yards. Bags (2 cu ft): {}. Cost at your yard price: ${}.", nd(cf,2), nd(yd,3), (cf/2.0).ceil(), nd(yd*a(x,2),2))) }));
    reg(v, Calc::new("gravel", "Gravel / Sand / Crushed Stone Calculator", "Construction",
        "Volume cu ft = L x W x depth. Tons ≈ cubic yards x density factor (gravel ≈ 1.4 tons/yard; sand ≈ 1.3; enter yours).", &["Length (ft)", "Width (ft)", "Depth (inches)", "Density (tons per cubic yard)"],
        |x| { let cf=a(x,0)*a(x,1)*a(x,2)/12.0; let yd=cf/27.0; Ok(format!("Volume: {} cu ft = {} cubic yards. Weight: about {} tons.", nd(cf,2), nd(yd,3), nd(yd*a(x,3),2))) }));
    reg(
        v,
        Calc::new(
            "flooring",
            "Flooring Calculator (Boxes)",
            "Construction",
            "Area incl waste / coverage per box, rounded up to whole boxes.",
            &[
                "Room length (ft)",
                "Room width (ft)",
                "Coverage per box (sq ft)",
                "Waste %",
                "Price per box (0 if unknown)",
            ],
            |x| {
                nz(a(x, 2), "Coverage per box")?;
                let area = a(x, 0) * a(x, 1) * (1.0 + a(x, 3) / 100.0);
                let boxes = (area / a(x, 2)).ceil();
                Ok(format!(
                    "Floor area (incl waste): {} sq ft. Boxes needed: {}. Cost: ${}.",
                    nd(area, 2),
                    boxes,
                    nd(boxes * a(x, 4), 2)
                ))
            },
        ),
    );
    reg(v, Calc::new("fence", "Fence Calculator (Posts, Rails, Pickets)", "Construction",
        "Posts = ceil(perimeter / spacing) + 1 per run — enter total linear feet for ONE straight run. Rails = runs of rail per section x sections. Pickets = total ft x 12 / (picket width + gap, inches).", &["Run length (ft)", "Post spacing (ft)", "Rails per section", "Picket width (in)", "Gap between pickets (in)"],
        |x| { nz(a(x,1),"Post spacing")?; let sections=(a(x,0)/a(x,1)).ceil(); let pw=a(x,3)+a(x,4); nz(pw,"Picket width + gap")?; Ok(format!("Sections: {}. Posts: {} (sections + 1). Rails: {}. Pickets: about {}. Concrete: plan about 1 bag (60 lb) per post = {} bags (rule of thumb).", sections, sections+1.0, sections*a(x,2), (a(x,0)*12.0/pw).ceil(), sections+1.0)) }));
    reg(v, Calc::new("stairs", "Stair Calculator (Rise, Run, Stringer)", "Construction",
        "Steps = total rise / target riser height (rounded). Actual riser = total rise / steps. Total run = steps x tread run (last tread is the landing, so runs = steps - 1 in many builds — this tool reports steps x run and flags it). Stringer length ≈ sqrt(rise^2 + run^2). Common guide: riser ≈ 7-7.75 in, tread ≈ 10-11 in; 2 x riser + tread ≈ 25 in. Check YOUR local code — this is not a code check.", &["Total rise (in)", "Target riser height (in)", "Tread run per step (in)"],
        |x| { nz(a(x,1),"Target riser height")?; let steps=(a(x,0)/a(x,1)).round().max(1.0); let riser=a(x,0)/steps; let run_tot=steps*a(x,2); let stringer=(a(x,0).powi(2)+run_tot.powi(2)).sqrt(); Ok(format!("Steps: {}. Actual riser height: {} in. Total run: {} in ({} ft). Stringer length (approx): {} in. Comfort check 2R+T = {} in (aim ≈ 25).", steps, nd(riser,3), nd(run_tot,1), nd(run_tot/12.0,2), nd(stringer,1), nd(2.0*riser+a(x,2),2))) }));
    reg(v, Calc::new("roofing", "Roofing Calculator (Squares & Bundles)", "Construction",
        "Roof area (flat footprint x slope factor) — slope factor = sqrt(1 + (rise/12)^2). 1 square = 100 sq ft. Standard shingles: 3 bundles per square.", &["Footprint length (ft)", "Footprint width (ft)", "Roof rise (in per 12 in)", "Waste %"],
        |x| { let factor=(1.0+(a(x,2)/12.0).powi(2)).sqrt(); let area=a(x,0)*a(x,1)*factor*(1.0+a(x,3)/100.0); let squares=area/100.0; Ok(format!("Roof area (incl slope & waste): {} sq ft = {} squares. Shingle bundles (3/square): {}. Slope factor: {}.", nd(area,1), nd(squares,2), (squares*3.0).ceil(), nd(factor,3))) }));
    reg(v, Calc::new("drywall", "Drywall Sheets Calculator", "Construction",
        "Sheets = ceil(area / sheet size). Standard sheet 4x8 ft = 32 sq ft (enter 48 for 4x12). Area = walls + ceiling if you add them up first — or enter wall perimeter x height as the 'area' input directly below.", &["Total area to cover (sq ft)", "Sheet coverage (sq ft, 32 for 4x8)"],
        |x| { nz(a(x,1),"Sheet coverage")?; Ok(format!("Sheets needed: {}.", (a(x,0)/a(x,1)).ceil())) }));
    reg(v, Calc::new("board-feet", "Board Feet", "Construction",
        "Board feet = thickness in x width in x length ft / 12. Also totals a count of boards and a price per board foot.", &["Thickness (in)", "Width (in)", "Length (ft)", "Number of boards", "Price per board foot (0 if unknown)"],
        |x| { let bf=a(x,0)*a(x,1)*a(x,2)/12.0; Ok(format!("Per board: {} board feet. Total for {} boards: {} bf. Cost: ${}.", nd(bf,3), n(a(x,3)), nd(bf*a(x,3),2), nd(bf*a(x,3)*a(x,4),2))) }));
    reg(v, Calc::new("deck-boards", "Deck Boards Calculator", "Construction",
        "Boards across = deck width / (board width + gap). Boards = ceil(that) and length = deck length. Linear feet = boards x length.", &["Deck length (ft)", "Deck width (ft)", "Board width (in)", "Gap (in)"],
        |x| { let bw=a(x,2)+a(x,3); nz(bw,"Board width + gap")?; let boards=(a(x,1)*12.0/bw).ceil(); Ok(format!("Boards needed: {} (each {} ft long). Total linear feet: {}. Add ~10% for cuts: plan {} boards.", boards, n(a(x,0)), nd(boards*a(x,0),1), (boards*1.1).ceil())) }));
    reg(v, Calc::new("brick", "Brick Calculator (Wall Estimate)", "Construction",
        "Standard modular brick with mortar covers about 6.55 bricks per sq ft (brick 7.625 x 2.25 in face + 3/8 in joints ≈ 7 bricks — this tool uses 6.55, a common published figure). Mortar: about 1 bag (60 lb) per 35 bricks (common estimate). ESTIMATE.", &["Wall length (ft)", "Wall height (ft)", "Waste %"],
        |x| { let area=a(x,0)*a(x,1); let bricks=(area*6.55*(1.0+a(x,2)/100.0)).ceil(); Ok(format!("Wall area: {} sq ft. Bricks: about {}. Mortar bags (60 lb): about {}. ESTIMATE.", nd(area,1), bricks, (bricks/35.0).ceil())) }));
    reg(v, Calc::new("wallpaper", "Wallpaper Rolls Calculator", "Construction",
        "One standard roll covers about 56 sq ft but only about 30 sq ft is usable after matching/cuts (common planning figure — enter your roll's usable coverage). Rolls = ceil(paintable wall area / usable coverage).", &["Total wall area (sq ft)", "Usable coverage per roll (sq ft, ~30 standard)"],
        |x| { nz(a(x,1),"Usable coverage per roll")?; Ok(format!("Rolls needed: {}.", (a(x,0)/a(x,1)).ceil())) }));
    reg(v, Calc::new("insulation-bags", "Blown Insulation Bags Estimate", "Construction",
        "Bags = area x depth-inches / coverage: enter your product's coverage in cu ft per bag (printed on the bag; typical cellulose bag ≈ 25-30 cu ft loose). Volume cu ft = area sq ft x depth in / 12.", &["Attic/floor area (sq ft)", "Depth wanted (inches)", "Coverage per bag (cu ft)"],
        |x| { nz(a(x,2),"Coverage per bag")?; let cf=a(x,0)*a(x,1)/12.0; Ok(format!("Volume: {} cu ft. Bags: {}.", nd(cf,1), (cf/a(x,2)).ceil())) }));
    reg(v, Calc::new("asphalt-driveway", "Asphalt / Paving Tons Estimate", "Construction",
        "Weight uses compacted asphalt density of about 145 lb per cubic foot. Volume = length x width x thickness. ESTIMATE.", &["Length (ft)", "Width (ft)", "Thickness (inches)"],
        |x| { let cf=a(x,0)*a(x,1)*a(x,2)/12.0; let tons=cf*145.0/2000.0; Ok(format!("Volume: {} cu ft. Weight at compacted asphalt density ≈145 lb/cu ft: {} lb = {} tons. ESTIMATE — order via your paver.", nd(cf,1), nd(cf*145.0,0), nd(tons,2))) }));
    reg(
        v,
        Calc::new(
            "room-area-walls",
            "Room Wall Area (For Paint/Drywall)",
            "Construction",
            "Wall area = 2 x (L + W) x H. Ceiling = L x W. Total with ceiling shown too.",
            &["Length (ft)", "Width (ft)", "Height (ft)"],
            |x| {
                Ok(format!(
                    "Walls: {} sq ft. Ceiling: {} sq ft. Walls + ceiling: {} sq ft.",
                    nd(2.0 * (a(x, 0) + a(x, 1)) * a(x, 2), 2),
                    nd(a(x, 0) * a(x, 1), 2),
                    nd(2.0 * (a(x, 0) + a(x, 1)) * a(x, 2) + a(x, 0) * a(x, 1), 2)
                ))
            },
        ),
    );
    reg(v, Calc::new("cubic-yards", "Cubic Yards From Feet & Inches", "Construction",
        "Cubic yards = (L ft x W ft x depth in / 12) / 27. Also cubic metres (x 0.7646 per cu yd).", &["Length (ft)", "Width (ft)", "Depth (inches)"],
        |x| { let cf=a(x,0)*a(x,1)*a(x,2)/12.0; Ok(format!("{} cu ft = {} cubic yards = {} cubic metres.", nd(cf,3), nd(cf/27.0,4), nd(cf*0.0283168,4))) }));
    reg(v, Calc::new("angle-iron-weight", "Steel Weight Estimate (Plate)", "Construction",
        "Steel weighs about 490 lb per cu ft. Weight = L x W x thickness (all converted to feet) x 490.", &["Length (in)", "Width (in)", "Thickness (in)"],
        |x| Ok(format!("Weight: about {} lb ({} kg).", nd(a(x,0)*a(x,1)*a(x,2)/1728.0*490.0,2), nd(a(x,0)*a(x,1)*a(x,2)/1728.0*490.0*0.453592,2)))));
    reg(v, Calc::new("concrete-blocks", "Concrete Block (CMU) Wall Estimate", "Construction",
        "Standard 8x16 in block face = 0.889 sq ft; blocks per sq ft ≈ 1.125. Blocks = ceil(area x 1.125 x (1 + waste)). ESTIMATE.", &["Wall length (ft)", "Wall height (ft)", "Waste %"],
        |x| { let area=a(x,0)*a(x,1); Ok(format!("Wall area: {} sq ft. Blocks (8x16): about {}. ESTIMATE.", nd(area,1), (area*1.125*(1.0+a(x,2)/100.0)).ceil())) }));
    reg(v, Calc::new("post-holes-concrete", "Fence Post Hole Concrete Bags", "Construction",
        "Hole volume = pi (d/2)^2 depth minus the post's own volume (post treated as round with its width as diameter, a small correction). Bags: 80 lb ≈ 0.60 cu ft, 60 lb ≈ 0.45.", &["Hole diameter (in)", "Hole depth (in)", "Post width (in)", "Number of holes"],
        |x| { let hole=std::f64::consts::PI*(a(x,0)/2.0).powi(2)*a(x,1); let post=std::f64::consts::PI*(a(x,2)/2.0).powi(2)*a(x,1); let cf=(hole-post)/1728.0*a(x,3); Ok(format!("Concrete volume total: {} cu ft. 80 lb bags: {}. 60 lb bags: {}.", nd(cf.max(0.0),3), (cf.max(0.0)/0.60).ceil(), (cf.max(0.0)/0.45).ceil())) }));
}
