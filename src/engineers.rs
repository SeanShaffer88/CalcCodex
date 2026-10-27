//! Engineers & professional polish (v0.8.0, items #41–#45): the
//! data and engines behind the electronics pack, the mechanical
//! pack, the tolerance stack-up, the safety factor and the
//! built-in materials table.
//!
//! The materials table holds TYPICAL published textbook values —
//! real alloys and batches vary, so every surface that shows them
//! (the MATERIALS view, `CalcDex --materials`, the calculator
//! descriptions) says "typical values" in plain words.

use crate::{n, nd};

// ---------- materials table (item #44) ----------

/// One row of the materials table. `resistivity` is None for
/// electrical insulators (glass, wood, plastics): the table says
/// so honestly instead of inventing a number.
pub struct Material {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    /// Density in kg/m^3 (typical).
    pub density: f64,
    /// Electrical resistivity in ohm metres (typical), metals only.
    pub resistivity: Option<f64>,
    /// Young's modulus in GPa (typical).
    pub youngs: f64,
}

/// The built-in materials (item #44). Order matters: the beam
/// calculator's material code is the 1-based position in this
/// table, and `CalcDex --materials` prints the same codes.
pub const MATERIALS: &[Material] = &[
    Material {
        name: "Aluminium 6061-T6",
        aliases: &["aluminium 6061", "aluminum 6061", "6061"],
        density: 2700.0,
        resistivity: Some(4.0e-8),
        youngs: 68.9,
    },
    Material {
        name: "Aluminium (pure)",
        aliases: &["aluminium pure", "aluminum", "aluminium"],
        density: 2700.0,
        resistivity: Some(2.65e-8),
        youngs: 69.0,
    },
    Material {
        name: "Copper (annealed)",
        aliases: &["copper"],
        density: 8960.0,
        resistivity: Some(1.68e-8),
        youngs: 117.0,
    },
    Material {
        name: "Brass (cartridge, 70/30)",
        aliases: &["brass"],
        density: 8530.0,
        resistivity: Some(6.2e-8),
        youngs: 110.0,
    },
    Material {
        name: "Mild steel (low-carbon)",
        aliases: &["mild steel", "steel", "structural steel"],
        density: 7850.0,
        resistivity: Some(1.6e-7),
        youngs: 200.0,
    },
    Material {
        name: "Stainless steel 304",
        aliases: &["stainless 304", "stainless steel", "stainless", "304"],
        density: 7900.0,
        resistivity: Some(7.2e-7),
        youngs: 193.0,
    },
    Material {
        name: "Titanium Ti-6Al-4V",
        aliases: &["titanium", "ti-6al-4v"],
        density: 4430.0,
        resistivity: Some(1.78e-6),
        youngs: 114.0,
    },
    Material {
        name: "Cast iron (gray)",
        aliases: &["cast iron", "gray iron"],
        density: 7200.0,
        resistivity: Some(8.0e-7),
        youngs: 110.0,
    },
    Material {
        name: "Glass (soda-lime)",
        aliases: &["glass"],
        density: 2500.0,
        resistivity: None,
        youngs: 70.0,
    },
    Material {
        name: "Oak (along the grain)",
        aliases: &["oak", "wood"],
        density: 720.0,
        resistivity: None,
        youngs: 11.0,
    },
    Material {
        name: "ABS plastic",
        aliases: &["abs"],
        density: 1050.0,
        resistivity: None,
        youngs: 2.3,
    },
    Material {
        name: "Nylon 6/6",
        aliases: &["nylon", "nylon 66"],
        density: 1140.0,
        resistivity: None,
        youngs: 2.9,
    },
];

/// Look a material up by name or alias (case-insensitive, extra
/// spaces ignored) or by its 1-based table code as text ("7").
pub fn find_material(query: &str) -> Option<&'static Material> {
    let q = query
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if q.is_empty() {
        return None;
    }
    if let Ok(code) = q.parse::<usize>() {
        if (1..=MATERIALS.len()).contains(&code) {
            return Some(&MATERIALS[code - 1]);
        }
        return None;
    }
    MATERIALS
        .iter()
        .find(|mat| mat.name.to_lowercase() == q || mat.aliases.iter().any(|a| *a == q))
}

/// The material for a numeric code (the beam calculator's material
/// field): 1..=MATERIALS.len(). Anything else is a teaching error
/// naming the valid range.
pub fn material_by_code(code: f64) -> Result<&'static Material, String> {
    if code.fract() != 0.0 || code < 1.0 || code > MATERIALS.len() as f64 {
        return Err(format!(
            "Material code must be a whole number from 1 to {} — CalcDex --materials lists the table with its codes (1 = {}). Got {}.",
            MATERIALS.len(),
            MATERIALS[0].name,
            n(code)
        ));
    }
    Ok(&MATERIALS[code as usize - 1])
}

/// Resistivity as shown everywhere: the value in ohm metres in
/// trimmed scientific notation (resistivities span 1e-8 to 1e-6,
/// where decimal form is a run of zeros), or an honest "not
/// applicable" for insulators.
pub fn resistivity_text(mat: &Material) -> String {
    match mat.resistivity {
        Some(r) => {
            let s = format!("{r:.6e}");
            let (mant, exp) = s.split_once('e').unwrap_or((&s, ""));
            let mant = mant.trim_end_matches('0').trim_end_matches('.');
            format!("{mant}e{exp} \u{3a9}\u{b7}m")
        }
        None => "not applicable (electrical insulator)".to_string(),
    }
}

// ---------- resistor colour code (item #41a) ----------

/// Colour names in digit order (index = digit value), plus the
/// two metallic colours used by the multiplier and tolerance
/// bands.
pub const BAND_DIGITS: &[&str] = &[
    "black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "gray", "white",
];

fn colour_index(word: &str) -> Option<usize> {
    let w = match word {
        "grey" => "gray",
        other => other,
    };
    BAND_DIGITS.iter().position(|c| *c == w)
}

/// Tolerance (percent) for a tolerance-band colour.
fn tolerance_of(word: &str) -> Option<f64> {
    match word {
        "brown" => Some(1.0),
        "red" => Some(2.0),
        "green" => Some(0.5),
        "blue" => Some(0.25),
        "violet" => Some(0.1),
        "gray" | "grey" => Some(0.05),
        "gold" => Some(5.0),
        "silver" => Some(10.0),
        "none" => Some(20.0),
        _ => None,
    }
}

const COLOUR_WORDS: &str =
    "black, brown, red, orange, yellow, green, blue, violet, gray, white, gold, silver, none";

/// A resistance in the friendly form used by the colour code and
/// electronics results: ohms below 1000, kΩ below a megaohm, MΩ
/// above — with the exact ohm value always in the steps text.
pub fn ohms_text(v: f64) -> String {
    if v != 0.0 && v.abs() >= 1.0e6 {
        format!("{} M\u{3a9}", nd(v / 1.0e6, 6))
    } else if v != 0.0 && v.abs() >= 1000.0 {
        format!("{} k\u{3a9}", nd(v / 1000.0, 6))
    } else {
        format!("{} \u{3a9}", nd(v, 6))
    }
}

/// Decode band colours (4 or 5 words) to (ohms, tolerance %).
pub fn decode_bands(words: &[&str]) -> Result<(f64, f64), String> {
    if words.len() != 4 && words.len() != 5 {
        return Err(format!(
            "A resistor has 4 or 5 colour bands — you gave {} word(s). Type the colours in order, e.g. red red brown gold, or type a resistance like 4700 (or 4.7k) to get its bands.",
            words.len()
        ));
    }
    for w in words {
        if colour_index(w).is_none() && *w != "gold" && *w != "silver" && *w != "none" {
            return Err(format!(
                "'{w}' is not a resistor colour — the colours are {COLOUR_WORDS}."
            ));
        }
    }
    let ndigits = words.len() - 2;
    let mut mantissa = 0f64;
    for (i, w) in words[..ndigits].iter().enumerate() {
        let d = colour_index(w).ok_or_else(|| {
            format!(
                "'{w}' cannot be a value band (band {}) — value bands are the digit colours black to white, never gold or silver.",
                i + 1
            )
        })?;
        mantissa = mantissa * 10.0 + d as f64;
    }
    let mult_word = words[ndigits];
    let exp: i32 = if mult_word == "gold" {
        -1
    } else if mult_word == "silver" {
        -2
    } else {
        colour_index(mult_word).ok_or_else(|| {
            format!("'{mult_word}' cannot be a multiplier band — it is a digit colour, gold (x0.1) or silver (x0.01).")
        })? as i32
    };
    let tol_word = words[ndigits + 1];
    let tol = tolerance_of(tol_word).ok_or_else(|| {
        format!(
            "'{tol_word}' cannot be a tolerance band — tolerance colours are brown (1%), red (2%), green (0.5%), blue (0.25%), violet (0.1%), gray (0.05%), gold (5%), silver (10%) or none (20%)."
        )
    })?;
    Ok((mantissa * 10f64.powi(exp), tol))
}

/// Encode a resistance (ohms) as band colours: 4 bands when the
/// value is exactly two significant digits times a power of ten,
/// else 5 bands (three significant digits), else a teaching error.
pub fn encode_bands(ohms: f64, tol_word: &str) -> Result<(Vec<String>, f64), String> {
    if !ohms.is_finite() || ohms <= 0.0 {
        return Err("A resistance must be a positive number of ohms.".into());
    }
    let tol = tolerance_of(tol_word).ok_or_else(|| {
        format!("'{tol_word}' is not a tolerance colour — tolerance colours are brown, red, green, blue, violet, gray, gold, silver or none.")
    })?;
    // Normalise to mantissa in [1, 1000) and an exponent.
    let mut exp = ohms.log10().floor() as i32;
    let mut mant = ohms / 10f64.powi(exp);
    if mant >= 999.999 {
        mant /= 10.0;
        exp += 1;
    }
    // 4-band: mantissa x 10 is a whole number of two digits.
    let two = mant * 10.0;
    if (two.round() - two).abs() < 1e-9 && (10.0..=99.0).contains(&two.round()) {
        let d = two.round() as i32;
        let mult_exp = exp - 1;
        return Ok((
            vec![
                BAND_DIGITS[(d / 10) as usize].to_string(),
                BAND_DIGITS[(d % 10) as usize].to_string(),
                multiplier_word(mult_exp)?,
                tol_word.to_string(),
            ],
            tol,
        ));
    }
    // 5-band: mantissa x 100 is a whole number of three digits.
    let three = mant * 100.0;
    if (three.round() - three).abs() < 1e-9 && (100.0..=999.0).contains(&three.round()) {
        let d = three.round() as i32;
        let mult_exp = exp - 2;
        return Ok((
            vec![
                BAND_DIGITS[(d / 100) as usize].to_string(),
                BAND_DIGITS[(d / 10 % 10) as usize].to_string(),
                BAND_DIGITS[(d % 10) as usize].to_string(),
                multiplier_word(mult_exp)?,
                tol_word.to_string(),
            ],
            tol,
        ));
    }
    Err(format!(
        "{} cannot be shown exactly with 4 or 5 bands — band values are two (or three) whole significant digits times a power of ten. The nearest 4-band value is {}.",
        ohms_text(ohms),
        ohms_text((two.round() / 10.0) * 10f64.powi(exp))
    ))
}

fn multiplier_word(exp: i32) -> Result<String, String> {
    match exp {
        -2 => Ok("silver".to_string()),
        -1 => Ok("gold".to_string()),
        0..=9 => Ok(BAND_DIGITS[exp as usize].to_string()),
        _ => Err(
            "That resistance is outside the range colour bands can show (0.1 \u{3a9} to 999 M\u{3a9} or so)."
                .into(),
        ),
    }
}

/// Parse a typed resistance for encoding: a plain number of ohms,
/// or with a k/K (x1000) or M (x1,000,000) suffix, e.g. "4.7k".
pub fn parse_resistance(text: &str) -> Option<f64> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    let (num, mult) = match t.chars().last().unwrap() {
        'k' | 'K' => (&t[..t.len() - 1], 1000.0),
        'M' => (&t[..t.len() - 1], 1.0e6),
        _ => (t, 1.0),
    };
    num.trim().parse::<f64>().ok().map(|v| v * mult)
}

/// The resistor colour code engine (item #41a): one text in, one
/// result out — band colours decode, a resistance encodes.
pub fn resistor_colour_run(text: &str) -> Result<String, String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Err(
            "Type the band colours (e.g. red red brown gold) or a resistance (e.g. 4700 or 4.7k)."
                .into(),
        );
    }
    let first = words[0].to_lowercase();
    // Encode path: the first word is a resistance (maybe with a
    // tolerance colour as the second word).
    if let Some(v) = parse_resistance(&first) {
        if words.len() > 2 {
            return Err("To get the bands for a resistance, type just the value and optionally one tolerance colour, e.g. 4700 gold.".into());
        }
        let tol_word = if words.len() == 2 {
            words[1].to_lowercase()
        } else {
            "gold".to_string()
        };
        let (bands, tol) = encode_bands(v, &tol_word)?;
        let ndig = bands.len() - 2;
        let digits: Vec<&str> = bands[..ndig].iter().map(|s| s.as_str()).collect();
        return Ok(format!(
            "{} = {} bands ({}-band, tolerance {} = \u{b1}{}%): {}. Steps: the significant digits {} come from the first {} band(s); the {} band is the multiplier; the last band is the tolerance.",
            ohms_text(v),
            bands.join(" "),
            bands.len(),
            tol_word,
            nd(tol, 2),
            bands.join(", "),
            digits.join(", "),
            ndig,
            bands[ndig]
        ));
    }
    // Decode path: every word must be a colour.
    let lower: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
    let refs: Vec<&str> = lower.iter().map(|s| s.as_str()).collect();
    let (ohms, tol) = decode_bands(&refs)?;
    let ndig = refs.len() - 2;
    let mut mantissa = 0f64;
    for w in &refs[..ndig] {
        mantissa = mantissa * 10.0 + colour_index(w).unwrap_or(0) as f64;
    }
    Ok(format!(
        "{} = {} \u{b1} {}%. Bands: {} significant digits {}, multiplier band {}, tolerance band {} (\u{b1}{}%). Range with tolerance: {} to {}.",
        refs.join(" "),
        ohms_text(ohms),
        nd(tol, 2),
        ndig,
        refs[..ndig].join(", "),
        refs[ndig],
        refs[ndig + 1],
        nd(tol, 2),
        ohms_text(ohms * (1.0 - tol / 100.0)),
        ohms_text(ohms * (1.0 + tol / 100.0))
    ))
}
