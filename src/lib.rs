//! CalcDex core library.
//!
//! Every calculator is a pure operation `(numbers) -> Result<text>` held in
//! ONE registry. The GUI, the CLI and the tests all dispatch through this
//! same registry — there are no GUI-only calculators.
//!
//! Signature notes:
//! - Fixed-input calculators declare labelled inputs; the slice holds
//!   exactly one number per label, in order.
//! - List calculators (`list = true`) accept any number of values. In the
//!   GUI they are typed into one field separated by commas/spaces; on the
//!   CLI each value is its own argument.
//! - Pair-list calculators (`pairs = true`) take values in repeating pairs
//!   (documented per calculator, e.g. value,probability,value,probability).

// Guard-clause validation style (consecutive `if { return Err }`) is used
// throughout the calculator modules on purpose.
#![allow(clippy::possible_missing_else)]

pub mod calcs;

pub type CalcFn = Box<dyn Fn(&[f64]) -> Result<String, String> + Sync + Send>;

pub struct Calc {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    /// Plain-words description including the standard formula used.
    pub desc: &'static str,
    /// Labelled inputs, in order (for list calculators: one label
    /// describing the list).
    pub inputs: &'static [&'static str],
    pub list: bool,
    pub pairs: bool,
    /// Date groups: input triples (year, month, day starting at `start`)
    /// that the GUI/CLI treat as ONE date field labelled `label`, entered
    /// as MM/DD/YYYY. Empty for non-date calculators.
    pub date_groups: &'static [DateGroup],
    pub run: CalcFn,
}

/// One merged date field: the inputs at `start..start+3` are
/// (year, month, day) for a date the user enters as a single MM/DD/YYYY
/// value under the display `label` (e.g. "Birth Date").
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DateGroup {
    pub start: usize,
    pub label: &'static str,
}

pub const fn dg(start: usize, label: &'static str) -> DateGroup {
    DateGroup { start, label }
}

impl Calc {
    #[allow(clippy::too_many_arguments)]
    pub fn new<F>(
        id: &'static str,
        name: &'static str,
        category: &'static str,
        desc: &'static str,
        inputs: &'static [&'static str],
        run: F,
    ) -> Calc
    where
        F: Fn(&[f64]) -> Result<String, String> + Sync + Send + 'static,
    {
        Calc {
            id,
            name,
            category,
            desc,
            inputs,
            list: false,
            pairs: false,
            date_groups: &[],
            run: Box::new(run),
        }
    }
    pub fn with_dates(mut self, groups: &'static [DateGroup]) -> Calc {
        self.date_groups = groups;
        self
    }
    pub fn list_mode(mut self) -> Calc {
        self.list = true;
        self
    }
    pub fn pair_mode(mut self) -> Calc {
        self.list = true;
        self.pairs = true;
        self
    }
}

/// Build the full registry (all category modules register here).
pub fn all_calcs() -> Vec<Calc> {
    let mut v: Vec<Calc> = Vec::new();
    calcs::math::register(&mut v);
    calcs::finance::register(&mut v);
    calcs::health::register(&mut v);
    calcs::physics::register(&mut v);
    calcs::chemistry::register(&mut v);
    calcs::construction::register(&mut v);
    calcs::conversion::register(&mut v);
    calcs::stats::register(&mut v);
    calcs::everyday::register(&mut v);
    calcs::sports_food_eco::register(&mut v);
    v
}

pub fn find_calc(id: &str) -> Option<Calc> {
    all_calcs().into_iter().find(|c| c.id == id)
}

/// Run a calculator by id against already-parsed numbers.
pub fn run_calc(id: &str, args: &[f64]) -> Result<String, String> {
    match find_calc(id) {
        Some(c) => {
            if !c.list && args.len() != c.inputs.len() {
                return Err(format!(
                    "'{}' needs {} number(s) ({}), got {}.",
                    c.name,
                    c.inputs.len(),
                    c.inputs.join(", "),
                    args.len()
                ));
            }
            if c.list && args.is_empty() {
                return Err("Enter at least one number.".into());
            }
            if c.pairs && args.len() % 2 != 0 {
                return Err(
                    "This calculator takes values in pairs — you gave an odd count of numbers."
                        .into(),
                );
            }
            (c.run)(args)
        }
        None => Err(format!("No calculator called '{id}'. Try: CalcDex --list")),
    }
}

// ---------- shared helpers for calculator modules ----------

/// Get argument i (registry guarantees count for fixed calculators).
pub fn a(x: &[f64], i: usize) -> f64 {
    x.get(i).copied().unwrap_or(0.0)
}

pub fn chk(cond: bool, msg: &str) -> Result<(), String> {
    if cond {
        Ok(())
    } else {
        Err(msg.to_string())
    }
}

pub fn nz(v: f64, what: &str) -> Result<f64, String> {
    if v == 0.0 {
        Err(format!(
            "{what} cannot be zero (that would divide by zero)."
        ))
    } else {
        Ok(v)
    }
}

pub fn finite(v: f64) -> Result<f64, String> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(
            "That calculation produced a number too big (or not a number). Check the inputs."
                .into(),
        )
    }
}

/// Money format, 2 decimal places.
pub fn m(x: f64) -> String {
    format!("{x:.2}")
}

/// General number format: integers without decimals, otherwise up to
/// 10 decimal places, trailing zeros trimmed. Full precision is kept —
/// nothing is rounded away silently for the academic/professional
/// audience; calculators that present money or estimates say so.
pub fn n(x: f64) -> String {
    if !x.is_finite() {
        return x.to_string();
    }
    if x == x.trunc() && x.abs() < 1e15 {
        return format!("{}", x as i64);
    }
    let s = format!("{x:.10}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    s.to_string()
}

/// Scientific notation, 6 decimal places (e.g. 6.022141e23).
pub fn sci(x: f64) -> String {
    format!("{x:.6e}")
}

/// n with a fixed number of decimals, trimmed.
pub fn nd(x: f64, dp: usize) -> String {
    let s = format!("{x:.dp$}");
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

// ---------- dates (Howard Hinnant's days-from-civil algorithm) ----------

/// Days since 1970-01-01 for a Gregorian (year, month, day).
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y2 = if m <= 2 { y - 1 } else { y };
    let era = if y2 >= 0 { y2 } else { y2 - 399 } / 400;
    let yoe = y2 - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Inverse of days_from_civil: (year, month, day).
pub fn civil_from_days(z0: i64) -> (i64, i64, i64) {
    let z = z0 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn valid_date(y: f64, m: f64, d: f64) -> Result<(i64, i64, i64), String> {
    let (y, m, d) = (y as i64, m as i64, d as i64);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || y < 1 {
        return Err("Enter a real date: year, month 1-12, day 1-31.".into());
    }
    // Round-trip check catches e.g. Feb 30.
    let back = civil_from_days(days_from_civil(y, m, d));
    if back != (y, m, d) {
        return Err("That is not a real calendar date (check the day of the month).".into());
    }
    Ok((y, m, d))
}

// ---------- date strings (MM/DD/YYYY), system-clock today, CLI expansion ----------

/// The one format hint shown under every date field, in plain words.
pub const DATE_HINT: &str =
    "Format: MM/DD/YYYY \u{2014} for example 04/23/1988 means April 23, 1988.";

/// Format a (year, month, day) as MM/DD/YYYY.
pub fn format_date(y: i64, m: i64, d: i64) -> String {
    format!("{m:02}/{d:02}/{y:04}")
}

/// Parse a user-typed date in MM/DD/YYYY form (M/D/YYYY without leading
/// zeros is also accepted). Errors teach what is wrong, per the audience
/// (students/teachers checking their work).
pub fn parse_date_str(raw: &str) -> Result<(i64, i64, i64), String> {
    let t = raw.trim();
    let parts: Vec<&str> = t.split('/').collect();
    if parts.len() != 3 || parts.iter().any(|p| p.trim().is_empty()) {
        return Err(format!(
            "'{t}' is not in the date format this field needs. {DATE_HINT}"
        ));
    }
    let nums: Result<Vec<i64>, _> = parts.iter().map(|p| p.trim().parse::<i64>()).collect();
    let nums = match nums {
        Ok(v) => v,
        Err(_) => {
            return Err(format!(
                "'{t}' is not a real date \u{2014} a date is three numbers separated by slashes. {DATE_HINT}"
            ))
        }
    };
    let (m, d, y) = (nums[0], nums[1], nums[2]);
    if !(1..=12).contains(&m) {
        return Err(format!(
            "'{t}' is not a real date \u{2014} the month must be 1 to 12, but the first number is {m}. {DATE_HINT}"
        ));
    }
    if !(1..=31).contains(&d) {
        return Err(format!(
            "'{t}' is not a real date \u{2014} the day must be 1 to 31, but the second number is {d}. {DATE_HINT}"
        ));
    }
    if y < 1 {
        return Err(format!(
            "'{t}' is not a real date \u{2014} the year must be 1 or later. {DATE_HINT}"
        ));
    }
    // Round-trip check catches e.g. Feb 30 and Feb 29 in a non-leap year.
    if civil_from_days(days_from_civil(y, m, d)) != (y, m, d) {
        return Err(format!(
            "'{t}' is not a real date \u{2014} that month does not have a day {d} in year {y} (watch February: it has 29 days only in a leap year)."
        ));
    }
    Ok((y, m, d))
}

/// Days since the Unix epoch from the system clock (UTC day boundary).
pub fn today_days() -> i64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    secs.div_euclid(86_400)
}

/// Today's date as (year, month, day), from the system clock.
pub fn today_ymd() -> (i64, i64, i64) {
    civil_from_days(today_days())
}

/// Today's date in MM/DD/YYYY form.
pub fn today_str() -> String {
    let (y, m, d) = today_ymd();
    format_date(y, m, d)
}

/// Current timestamp "YYYY-MM-DD HH:MM" from the system clock (UTC;
/// no timezone database is bundled, so log times are UTC).
pub fn now_stamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02}",
        sod / 3600,
        (sod % 3600) / 60
    )
}

/// If input index `i` starts a date group in this calculator, return it.
pub fn date_group_at(c: &Calc, i: usize) -> Option<&DateGroup> {
    c.date_groups.iter().find(|g| g.start == i)
}

/// How the inputs of a run are shown to the user (and stored in the log):
/// date triples collapse to their MM/DD/YYYY form, other numbers are
/// shown one per token. Returns one display token per field.
pub fn display_tokens(c: &Calc, args: &[f64]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if c.list {
        return args.iter().map(|v| n(*v)).collect();
    }
    let mut i = 0usize;
    while i < args.len() {
        if let Some(_g) = date_group_at(c, i) {
            if i + 2 < args.len() {
                out.push(format_date(
                    args[i] as i64,
                    args[i + 1] as i64,
                    args[i + 2] as i64,
                ));
                i += 3;
                continue;
            }
        }
        out.push(n(args[i]));
        i += 1;
    }
    out
}

/// Expand CLI-style string arguments into the registry's numeric input
/// vector for one calculator. A date group accepts EITHER one argument
/// in MM/DD/YYYY form OR the old three separate numbers, so
/// `CalcDex age 04/23/1988 10/25/2026` and
/// `CalcDex age 1988 4 23 2026 10 25` are equivalent. Non-date tokens
/// may also be comma-separated groups (previous behaviour).
pub fn expand_args(c: &Calc, toks: &[String]) -> Result<Vec<f64>, String> {
    // Flatten: a token containing '/' is kept whole (it is a date);
    // other tokens are split on commas like before.
    let mut flat: Vec<String> = Vec::new();
    for tok in toks {
        if tok.contains('/') {
            flat.push(tok.clone());
        } else {
            for part in tok.split(',') {
                if !part.is_empty() {
                    flat.push(part.to_string());
                }
            }
        }
    }
    if c.list || c.date_groups.is_empty() {
        let mut nums = Vec::new();
        for t in &flat {
            match t.trim().parse::<f64>() {
                Ok(v) => nums.push(v),
                Err(_) => return Err(format!("'{t}' is not a number.")),
            }
        }
        return Ok(nums);
    }
    let mut nums: Vec<f64> = Vec::new();
    let mut pos = 0usize; // position in flat
    let mut i = 0usize; // input index
    while i < c.inputs.len() {
        if date_group_at(c, i).is_some() {
            let Some(tok) = flat.get(pos) else {
                return Err(format!(
                    "'{}' needs a {} date ({}).",
                    c.name,
                    date_group_at(c, i).map(|g| g.label).unwrap_or("date"),
                    DATE_HINT
                ));
            };
            if tok.contains('/') {
                let (y, m, d) = parse_date_str(tok)?;
                nums.push(y as f64);
                nums.push(m as f64);
                nums.push(d as f64);
                pos += 1;
            } else {
                // Old three-number form.
                for k in 0..3 {
                    let Some(t) = flat.get(pos + k) else {
                        return Err(format!("'{}' needs more numbers.", c.name));
                    };
                    match t.trim().parse::<f64>() {
                        Ok(v) => nums.push(v),
                        Err(_) => return Err(format!("'{t}' is not a number.")),
                    }
                }
                pos += 3;
            }
            i += 3;
        } else {
            let Some(tok) = flat.get(pos) else {
                return Err(format!("'{}' needs more numbers.", c.name));
            };
            match tok.trim().parse::<f64>() {
                Ok(v) => nums.push(v),
                Err(_) => return Err(format!("'{tok}' is not a number.")),
            }
            pos += 1;
            i += 1;
        }
    }
    if pos < flat.len() {
        return Err(format!(
            "'{}' got extra input starting at '{}'.",
            c.name, flat[pos]
        ));
    }
    Ok(nums)
}

// ---------- calculation log (persistent history, GUI + CLI shared) ----------

/// One saved calculation. `inputs` is the human-readable form (dates as
/// MM/DD/YYYY), `restore` holds one entered token per field separated by
/// ";;" so the GUI can refill the calculator exactly as it was run.
#[derive(Clone, PartialEq, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub id: String,
    pub name: String,
    pub inputs: String,
    pub result: String,
    pub restore: String,
}

pub const LOG_CAP: usize = 500;

fn log_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\p")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn log_unescape(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('p') => out.push('|'),
                Some(other) => out.push(other),
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/// One record per line: timestamp | id | name | inputs | restore | result
/// (every field escaped, result may contain anything once escaped).
pub fn serialize_entry(e: &LogEntry) -> String {
    [
        e.timestamp.as_str(),
        e.id.as_str(),
        e.name.as_str(),
        e.inputs.as_str(),
        e.restore.as_str(),
        e.result.as_str(),
    ]
    .iter()
    .map(|f| log_escape(f))
    .collect::<Vec<_>>()
    .join("|")
}

pub fn parse_entry(line: &str) -> Option<LogEntry> {
    // Split on unescaped '|'.
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            cur.push(ch);
            if let Some(n) = chars.next() {
                cur.push(n);
            }
        } else if ch == '|' {
            fields.push(cur.clone());
            cur.clear();
        } else {
            cur.push(ch);
        }
        let _ = chars.peek();
    }
    fields.push(cur);
    if fields.len() != 6 {
        return None;
    }
    let mut it = fields.into_iter().map(|f| log_unescape(&f));
    Some(LogEntry {
        timestamp: it.next()?,
        id: it.next()?,
        name: it.next()?,
        inputs: it.next()?,
        restore: it.next()?,
        result: it.next()?,
    })
}

/// Where the log lives: %APPDATA%\CalcDex\history.txt when APPDATA is
/// set, else beside the executable, else the current folder.
pub fn history_path() -> std::path::PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        if !appdata.is_empty() {
            return std::path::Path::new(&appdata)
                .join("CalcDex")
                .join("history.txt");
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.join("history.txt");
        }
    }
    std::path::PathBuf::from("history.txt")
}

pub fn load_history_from(path: &std::path::Path) -> Vec<LogEntry> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    text.lines().filter_map(parse_entry).collect()
}

pub fn load_history() -> Vec<LogEntry> {
    load_history_from(&history_path())
}

pub fn save_history_to(path: &std::path::Path, entries: &[LogEntry]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut text = String::new();
    for e in entries {
        text.push_str(&serialize_entry(e));
        text.push('\n');
    }
    std::fs::write(path, text)
}

/// Append one entry to the log at `path`, keeping only the most recent
/// LOG_CAP entries. Returns the full updated list (oldest first).
pub fn append_entry_to(path: &std::path::Path, entry: LogEntry) -> std::io::Result<Vec<LogEntry>> {
    let mut entries = load_history_from(path);
    entries.push(entry);
    if entries.len() > LOG_CAP {
        let drop = entries.len() - LOG_CAP;
        entries.drain(0..drop);
    }
    save_history_to(path, &entries)?;
    Ok(entries)
}

/// Append one entry to the default log file.
pub fn append_history(entry: LogEntry) -> std::io::Result<Vec<LogEntry>> {
    append_entry_to(&history_path(), entry)
}

pub fn clear_history() -> std::io::Result<()> {
    save_history_to(&history_path(), &[])
}

/// Build a log entry for a successful run.
pub fn make_entry(c: &Calc, args: &[f64], result: &str) -> LogEntry {
    let toks = display_tokens(c, args);
    LogEntry {
        timestamp: now_stamp(),
        id: c.id.to_string(),
        name: c.name.to_string(),
        inputs: toks.join(", "),
        result: result.to_string(),
        restore: toks.join(";;"),
    }
}

// ---------- statistics helpers ----------

/// Error function (Abramowitz & Stegun 7.1.26, |error| <= 1.5e-7).
pub fn erf(x: f64) -> f64 {
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-x * x).exp();
    s * y
}

/// Standard normal CDF Phi(z).
pub fn norm_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2))
}

/// Two-sided p-value for a z statistic.
pub fn p_from_z(z: f64) -> f64 {
    2.0 * (1.0 - norm_cdf(z.abs()))
}

/// Factorial for non-negative integers up to 170.
pub fn factorial(x: f64) -> Result<f64, String> {
    if x < 0.0 || x != x.trunc() {
        return Err("Factorial needs a whole number, 0 or bigger.".into());
    }
    if x > 170.0 {
        return Err("Factorial is only supported up to 170 (bigger answers overflow).".into());
    }
    let mut r = 1.0f64;
    let mut i = 2.0;
    while i <= x {
        r *= i;
        i += 1.0;
    }
    Ok(r)
}

pub fn gcd_i(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
