//! Batch-1 feature engines (v0.3.0, 50-enhancement program items
//! #1, #16, #31, #34, #47). Everything here is a pure library
//! operation: the GUI and the CLI both call these functions, so no
//! feature is GUI-only or CLI-only in its logic.

use crate::{expand_args, run_calc, Calc};

// ---------- #1 significant figures ----------

/// Format `value` to `n` significant figures, counted correctly:
/// leading zeros are not significant, trailing zeros after a decimal
/// point are, and a result whose plain form would need ambiguous
/// trailing zeros in the integer part (or is very small) is shown in
/// scientific form (`1.63e3`) so the count is unambiguous — the
/// display choice is documented in DECISIONS.md. Display only: the
/// stored/full-precision result is never altered.
pub fn format_sig_figs(value: f64, n: u8) -> String {
    let n = n.max(1) as i32;
    if !value.is_finite() {
        return crate::n(value);
    }
    if value == 0.0 {
        return "0".to_string();
    }
    let sign = if value < 0.0 { "-" } else { "" };
    let a = value.abs();
    let mut e = a.log10().floor() as i32;
    // Round to n significant digits via the scaled integer.
    let scale_exp = e - n + 1;
    let scaled = a / 10f64.powi(scale_exp);
    let mut r = scaled.round();
    if r >= 10f64.powi(n) {
        r /= 10.0;
        e += 1;
    }
    if r < 10f64.powi(n - 1) && r != 0.0 {
        // Rounding carried the other way (defensive; scaled was
        // computed from a possibly-imprecise log10).
        r *= 10.0;
        e -= 1;
    }
    let digits = format!("{}", r as i64);
    let digits = if digits.len() < n as usize {
        format!("{}{}", "0".repeat(n as usize - digits.len()), digits)
    } else {
        digits
    };
    // Scientific form when plain notation would hide the count:
    // integer part longer than the digit count, or exponent < -4.
    if e >= n || e < -4 {
        let mantissa = if n == 1 {
            digits.clone()
        } else {
            format!("{}.{}", &digits[..1], &digits[1..])
        };
        return format!("{sign}{mantissa}e{e}");
    }
    if e >= 0 {
        let int_len = (e + 1) as usize;
        if int_len >= digits.len() {
            format!("{sign}{digits}")
        } else {
            format!("{sign}{}.{}", &digits[..int_len], &digits[int_len..])
        }
    } else {
        format!("{sign}0.{}{}", "0".repeat((-e - 1) as usize), digits)
    }
}

/// Scan a text for numbers and return (start, end) byte ranges.
/// Numbers touching `/` or a digit-flanked `-` (date separators in
/// "04/23/1988" / "2025-10-08") are skipped, so sig-fig annotation
/// never mangles a date.
fn number_spans(text: &str) -> Vec<(usize, usize)> {
    let b = text.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let c = b[i] as char;
        let starts = c.is_ascii_digit()
            || (c == '.' && i + 1 < b.len() && (b[i + 1] as char).is_ascii_digit())
            || (c == '-'
                && i + 1 < b.len()
                && (b[i + 1] as char).is_ascii_digit()
                && (i == 0 || !(b[i - 1] as char).is_ascii_digit()));
        if !starts {
            i += 1;
            continue;
        }
        let start = i;
        let mut j = i;
        if b[j] == b'-' {
            j += 1;
        }
        let mut seen_dot = false;
        let mut seen_exp = false;
        while j < b.len() {
            let ch = b[j] as char;
            if ch.is_ascii_digit() {
                j += 1;
            } else if ch == '.' && !seen_dot && !seen_exp {
                seen_dot = true;
                j += 1;
            } else if (ch == 'e' || ch == 'E') && !seen_exp && j > start {
                // Exponent only if digits follow (possibly signed).
                let mut k = j + 1;
                if k < b.len() && (b[k] == b'-' || b[k] == b'+') {
                    k += 1;
                }
                if k < b.len() && (b[k] as char).is_ascii_digit() {
                    seen_exp = true;
                    j = k;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        // Skip date fragments: touching a '/', or a '-' that is
        // flanked by digits on the other side (ISO dates 2025-10-08).
        let before = if start == 0 {
            None
        } else {
            Some(b[start - 1] as char)
        };
        let after = if j >= b.len() {
            None
        } else {
            Some(b[j] as char)
        };
        let after_dash_digit =
            after == Some('-') && j + 1 < b.len() && (b[j + 1] as char).is_ascii_digit();
        let before_dash_digit =
            before == Some('-') && start >= 2 && (b[start - 2] as char).is_ascii_digit();
        if before != Some('/') && after != Some('/') && !after_dash_digit && !before_dash_digit {
            spans.push((start, j));
        }
        i = j.max(start + 1);
    }
    spans
}

/// The first line of a result (its headline).
pub fn headline_of(output: &str) -> &str {
    output.lines().next().unwrap_or("")
}

/// The headline with every number re-shown to `n` significant
/// figures (item #1). The full-precision text is untouched; this
/// produces the ADDITIONAL rounded showing.
pub fn sig_figs_headline(output: &str, n: u8) -> String {
    let head = headline_of(output);
    let spans = number_spans(head);
    if spans.is_empty() {
        return head.to_string();
    }
    let mut out = String::new();
    let mut pos = 0usize;
    for (s, e) in spans {
        out.push_str(&head[pos..s]);
        let tok = &head[s..e];
        match tok.parse::<f64>() {
            Ok(v) => out.push_str(&format_sig_figs(v, n)),
            Err(_) => out.push_str(tok),
        }
        pos = e;
    }
    out.push_str(&head[pos..]);
    out
}

/// The full result plus its sig-figs headline line, for the steps
/// footer / CLI `--sig-figs` output.
pub fn annotate_sig_figs(output: &str, n: u8) -> String {
    format!(
        "{output}\n[To {n} significant figures: {}]",
        sig_figs_headline(output, n)
    )
}

// ---------- #16 vary-one-input sweep ----------

#[derive(Debug)]
pub struct VaryRow {
    pub value: f64,
    pub result: Result<String, String>,
}

pub const VARY_CAP: usize = 200;

/// Run `calc` with input `field` swept from `start` to `end` by
/// `step` (inclusive), all other inputs taken from `base`. Pure
/// engine shared by the GUI VARY section and the CLI `--vary`.
pub fn vary_run(
    calc: &Calc,
    base: &[f64],
    field: usize,
    start: f64,
    end: f64,
    step: f64,
) -> Result<Vec<VaryRow>, String> {
    if calc.list {
        return Err(
            "VARY works on calculators with labelled inputs — this one takes a list, so there is no single input to sweep."
                .into(),
        );
    }
    if field >= calc.inputs.len() {
        return Err(format!(
            "There is no input number {} — '{}' has {} inputs (numbered from 0).",
            field + 1,
            calc.name,
            calc.inputs.len()
        ));
    }
    if base.len() != calc.inputs.len() {
        return Err(format!(
            "'{}' needs {} numbers for the fixed inputs, got {}.",
            calc.name,
            calc.inputs.len(),
            base.len()
        ));
    }
    if step == 0.0 {
        return Err("The step cannot be 0 — the sweep would never move.".into());
    }
    if (end - start) * step.signum() < 0.0 {
        return Err(
            "The step moves away from the end value — check the signs of start, end and step."
                .into(),
        );
    }
    let span = (end - start) / step;
    let count = span.floor() as i64 + 1;
    if count < 1 {
        return Err("That sweep produces no rows — check start, end and step.".into());
    }
    if count as usize > VARY_CAP {
        return Err(format!(
            "That sweep would make {count} rows — the cap is {VARY_CAP}. Use a bigger step or a smaller range."
        ));
    }
    let mut rows = Vec::new();
    for k in 0..count {
        let v = start + step * k as f64;
        let mut args = base.to_vec();
        args[field] = v;
        rows.push(VaryRow {
            value: v,
            result: run_calc(calc.id, &args),
        });
    }
    Ok(rows)
}

// ---------- #31 CSV parsing + batch ----------

/// A small hand-written CSV parser (no new dependency — DECISIONS):
/// handles quoted fields, `""` escapes, commas and newlines inside
/// quotes, and CRLF line endings.
pub fn parse_csv(text: &str) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    let mut field_started = false;
    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(ch);
            }
        } else if ch == '"' && !field_started {
            in_quotes = true;
            field_started = true;
        } else if ch == ',' {
            row.push(std::mem::take(&mut field));
            field_started = false;
        } else if ch == '\r' {
            // swallow; \n (if present) ends the row
        } else if ch == '\n' {
            row.push(std::mem::take(&mut field));
            field_started = false;
            rows.push(std::mem::take(&mut row));
        } else {
            field.push(ch);
            field_started = true;
        }
    }
    if field_started || !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    // Drop fully-empty trailing rows.
    while rows
        .last()
        .map(|r| r.iter().all(|c| c.trim().is_empty()))
        .unwrap_or(false)
    {
        rows.pop();
    }
    rows
}

/// Quote one CSV field if it needs it (contains a comma, quote or
/// newline); quotes inside are doubled.
pub fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn csv_row(fields: &[String]) -> String {
    fields
        .iter()
        .map(|f| csv_field(f))
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Debug)]
pub struct BatchOutcome {
    pub output_csv: String,
    pub ok: usize,
    pub errors: usize,
}

/// Run one calculator over CSV text (item #31). The header row must
/// name the calculator's display fields in order (see
/// `meta::display_fields`); for list calculators the header is the
/// single list label and each data row's cells are the list values.
/// Each data row yields its inputs plus the headline result, or
/// `ERROR: <message>` — one bad row never stops the batch.
pub fn batch_run(calc: &Calc, input_csv: &str) -> Result<BatchOutcome, String> {
    let rows = parse_csv(input_csv);
    if rows.is_empty() {
        return Err("The input CSV is empty — it needs a header row naming the fields, then one row per calculation.".into());
    }
    let expected = crate::meta::display_fields(calc);
    let header: Vec<String> = rows[0].iter().map(|h| h.trim().to_string()).collect();
    if !calc.list && header != expected {
        return Err(format!(
            "The header row does not match '{}'. Expected: {} — got: {}.",
            calc.name,
            expected.join(", "),
            header.join(", ")
        ));
    }
    if calc.list && (header.len() != 1 || header[0] != expected[0]) {
        return Err(format!(
            "The header row does not match '{}'. Expected the single header: {} — got: {}.",
            calc.name,
            expected[0],
            header.join(", ")
        ));
    }
    let mut out_lines: Vec<String> = Vec::new();
    let mut out_header = expected.clone();
    out_header.push("Result".to_string());
    out_lines.push(csv_row(&out_header));
    let (mut ok, mut errors) = (0usize, 0usize);
    for row in &rows[1..] {
        if row.iter().all(|c| c.trim().is_empty()) {
            continue;
        }
        let cells: Vec<String> = row.iter().map(|c| c.trim().to_string()).collect();
        let result = (|| -> Result<String, String> {
            let args = if calc.list {
                let mut v = Vec::new();
                for cell in &cells {
                    match cell.parse::<f64>() {
                        Ok(x) => v.push(x),
                        Err(_) => return Err(format!("'{cell}' is not a number.")),
                    }
                }
                v
            } else {
                expand_args(calc, &cells)?
            };
            let out = run_calc(calc.id, &args)?;
            Ok(headline_of(&out).to_string())
        })();
        let mut out_row: Vec<String> = Vec::new();
        // Pad/trim the echoed inputs to the expected field count for
        // fixed calculators; list rows echo as entered.
        if calc.list {
            out_row.extend(cells.iter().cloned());
        } else {
            for i in 0..expected.len() {
                out_row.push(cells.get(i).cloned().unwrap_or_default());
            }
        }
        match result {
            Ok(head) => {
                ok += 1;
                out_row.push(head);
            }
            Err(e) => {
                errors += 1;
                out_row.push(format!("ERROR: {e}"));
            }
        }
        out_lines.push(csv_row(&out_row));
    }
    Ok(BatchOutcome {
        output_csv: out_lines.join("\n") + "\n",
        ok,
        errors,
    })
}

// ---------- #34 linear regression + calibration ----------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Regression {
    pub n: usize,
    pub slope: f64,
    pub intercept: f64,
    pub r: f64,
    pub r2: f64,
    pub mean_x: f64,
    pub mean_y: f64,
    pub sum_x: f64,
    pub sum_y: f64,
    pub sum_xy: f64,
    pub sum_xx: f64,
}

/// Least-squares linear regression y = slope*x + intercept, with the
/// substituted sums exposed so steps can show the working.
pub fn linear_regression(xs: &[f64], ys: &[f64]) -> Result<Regression, String> {
    if xs.len() != ys.len() {
        return Err("The x and y lists must have the same number of values.".into());
    }
    if xs.len() < 2 {
        return Err("Linear regression needs at least 2 points.".into());
    }
    let n = xs.len() as f64;
    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xy: f64 = xs.iter().zip(ys.iter()).map(|(x, y)| x * y).sum();
    let sum_xx: f64 = xs.iter().map(|x| x * x).sum();
    let sum_yy: f64 = ys.iter().map(|y| y * y).sum();
    let mean_x = sum_x / n;
    let mean_y = sum_y / n;
    let sxx = sum_xx - sum_x * sum_x / n;
    let syy = sum_yy - sum_y * sum_y / n;
    let sxy = sum_xy - sum_x * sum_y / n;
    if sxx == 0.0 {
        return Err("All the x values are the same — a regression line needs x to vary.".into());
    }
    let slope = sxy / sxx;
    let intercept = mean_y - slope * mean_x;
    let r = if syy == 0.0 {
        0.0
    } else {
        sxy / (sxx * syy).sqrt()
    };
    Ok(Regression {
        n: xs.len(),
        slope,
        intercept,
        r,
        r2: r * r,
        mean_x,
        mean_y,
        sum_x,
        sum_y,
        sum_xy,
        sum_xx,
    })
}

/// Calibration: predict x from a measured y, x = (y - b) / m.
pub fn regression_predict_x(reg: &Regression, y: f64) -> Result<f64, String> {
    if reg.slope == 0.0 {
        return Err(
            "The slope is 0 — the line is flat, so a measured y cannot be turned back into an x."
                .into(),
        );
    }
    Ok((y - reg.intercept) / reg.slope)
}

// ---------- #47 synonym search ----------

/// Synonym pairs (item #47): a query containing one side matches
/// calculators whose name or id contains the other side. At least 40
/// pairs covering the everyday terms across categories.
pub static SYNONYMS: &[(&str, &str)] = &[
    ("home loan", "mortgage"),
    ("mortgage", "home loan"),
    ("bmi", "body mass"),
    ("body mass", "bmi"),
    ("temp", "temperature"),
    ("temperature", "temp"),
    ("interest", "compound"),
    ("savings", "interest"),
    ("paycheck", "salary"),
    ("wage", "salary"),
    ("salary", "hourly"),
    ("hourly", "salary"),
    ("weight loss", "calorie"),
    ("calories", "calorie"),
    ("running", "pace"),
    ("pace", "running"),
    ("loan", "payment"),
    ("borrow", "loan"),
    ("rent", "buy"),
    ("house", "mortgage"),
    ("car loan", "auto"),
    ("auto", "car"),
    ("tax", "sales"),
    ("discount", "percent off"),
    ("sale price", "discount"),
    ("tip", "gratuity"),
    ("gratuity", "tip"),
    ("average", "mean"),
    ("mean", "average"),
    ("middle", "median"),
    ("spread", "standard deviation"),
    ("std", "standard deviation"),
    ("chance", "probability"),
    ("odds", "probability"),
    ("area", "square footage"),
    ("square footage", "area"),
    ("paint", "room"),
    ("concrete", "slab"),
    ("stairs", "steps"),
    ("steps", "stairs"),
    ("gas", "fuel"),
    ("fuel", "mpg"),
    ("mpg", "fuel economy"),
    ("speed", "velocity"),
    ("velocity", "speed"),
    ("force", "newton"),
    ("energy", "kinetic"),
    ("voltage", "ohms"),
    ("current", "ohms"),
    ("acid", "ph"),
    ("ph", "acid"),
    ("moles", "molarity"),
    ("dilute", "dilution"),
    ("age", "birthday"),
    ("birthday", "age"),
    ("days between", "date"),
    ("countdown", "days"),
    ("dog years", "dog age"),
    ("cat years", "cat age"),
    ("grade", "gpa"),
    ("gpa", "grade"),
    ("bill", "split"),
    ("split", "bill"),
    ("recipe", "scale"),
    ("cooking", "oven"),
    ("oven", "temperature"),
    ("carbon", "co2"),
    ("solar", "panel"),
    ("pool", "swimming"),
    ("heart rate", "pulse"),
    ("pulse", "heart"),
    ("pregnancy", "due date"),
    ("due date", "pregnancy"),
    ("alcohol", "bac"),
    ("bac", "blood alcohol"),
    ("retirement", "savings"),
    ("invest", "roi"),
    ("profit", "margin"),
    ("margin", "profit"),
];

fn norm(s: &str) -> String {
    s.to_lowercase().replace(['-', '_'], " ")
}

/// Score one calculator against a query: exact name (100) > id
/// (90) > synonym (70) > name contains (60) > category (40) >
/// description contains (30). 0 = no match.
pub fn search_score(c: &Calc, query: &str) -> i32 {
    let q = norm(query).trim().to_string();
    if q.is_empty() {
        return 0;
    }
    let name = norm(c.name);
    let id = norm(c.id);
    if name == q {
        return 100;
    }
    if id == q {
        return 90;
    }
    for (a, b) in SYNONYMS {
        if q.contains(a) && (name.contains(b) || id.contains(b)) {
            return 70;
        }
        if q.contains(b) && (name.contains(a) || id.contains(a)) {
            return 70;
        }
    }
    if name.contains(&q) {
        return 60;
    }
    if id.contains(&q) {
        return 55;
    }
    if norm(c.category).contains(&q) {
        return 40;
    }
    if norm(c.desc).contains(&q) {
        return 30;
    }
    0
}

/// Ranked search (item #47): indices into `calcs`, best first,
/// registry order breaking ties. Empty query = everything.
pub fn search_ranked(calcs: &[Calc], query: &str) -> Vec<usize> {
    if query.trim().is_empty() {
        return (0..calcs.len()).collect();
    }
    let mut scored: Vec<(i32, usize)> = calcs
        .iter()
        .enumerate()
        .map(|(i, c)| (search_score(c, query), i))
        .filter(|(s, _)| *s > 0)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored.into_iter().map(|(_, i)| i).collect()
}
