//! Persistent settings (v0.3.0, items #1 / #46 / #48): favourites,
//! recent calculators, per-calculator remembered inputs, and the
//! sig-figs choice — one simple documented text file shared by all
//! three, at %APPDATA%\CalcDex\settings.txt.
//!
//! Format (one record per line, `#` comments allowed):
//!   display_format=sig-figs:3   (off | sig-figs:N | decimals:N |
//!                                engineering | auto; v0.4.0 — the
//!                                old sig_figs= line still loads)
//!   rounding=half-up      (half-up | bankers | truncate)
//!   update_check_startup=off    (offer-only update check; off
//!                                unless the user switches it on)
//!   steps=standard        (standard | deeper; v0.5.0 #12)
//!   projection=off        (on | off; v0.6.0 #21 classroom
//!                          projection mode: larger text + higher
//!                          contrast in the window)
//!   favourite=<calc id>
//!   recent=<id>,<id>,...  (most recent first, max 8)
//!   inputs.<calc id>=<token>;;<token>;;...
//!   constant.<name>=<value>   (v0.7.0 #38: a user-defined
//!                                constant for the Expression
//!                                Calculator; names are validated
//!                                by rigour::validate_constant_name,
//!                                so a line naming a built-in
//!                                constant or function never loads)
//! Input tokens are the display tokens of a run (dates as
//! MM/DD/YYYY), separated by ";;" exactly like the calculation log's
//! restore field; backslash escapes `\\`, `\;` and `\n` inside a
//! token. Unknown ids and malformed lines are ignored on load — a
//! corrupt or missing file NEVER breaks startup; the app falls back
//! to defaults (see `load_settings`).

use crate::rigour::{DisplayFormat, DisplayMode, RoundingMode};
use std::collections::{BTreeSet, HashMap};

pub const RECENT_CAP: usize = 8;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
    /// Unified display format (v0.4.0 #1/#2/#3/#37): mode,
    /// figure count, and the rounding mode (#9).
    pub display: DisplayFormat,
    /// Offer-only update check at startup (default OFF — the
    /// program makes no network connection unless this is on or
    /// the user presses Check for updates).
    pub update_check_startup: bool,
    /// Deeper steps mode (v0.5.0, program #12): the steps area
    /// shows the four labelled stages instead of the raw output.
    pub steps_deeper: bool,
    /// Classroom projection mode (v0.6.0, program #21): the
    /// window renders at ~1.5x zoom with brightened muted/hint
    /// colours so the back of a classroom can read it.
    pub projection: bool,
    pub favourites: BTreeSet<String>,
    /// Most recent first.
    pub recent: Vec<String>,
    /// calc id -> display tokens of its last run (item #48).
    pub remembered: HashMap<String, Vec<String>>,
    /// User-defined constants (v0.7.0 #38): (name, value) pairs
    /// in definition order; names are stored already validated +
    /// lowercased by rigour::validate_constant_name.
    pub constants: Vec<(String, f64)>,
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn unesc(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some(other) => out.push(other),
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

/// Split a token list on unescaped ";;".
fn split_tokens(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let b: Vec<char> = s.chars().collect();
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == '\\' && i + 1 < b.len() {
            cur.push(b[i]);
            cur.push(b[i + 1]);
            i += 2;
        } else if b[i] == ';' && i + 1 < b.len() && b[i + 1] == ';' {
            out.push(unesc(&cur));
            cur.clear();
            i += 2;
        } else {
            cur.push(b[i]);
            i += 1;
        }
    }
    out.push(unesc(&cur));
    out
}

pub fn serialize_settings(s: &Settings) -> String {
    let mut out = String::new();
    out.push_str("# CalcDex settings — see settings.rs for the format.\n");
    out.push_str(&format!("display_format={}\n", s.display.as_setting()));
    out.push_str(&format!("rounding={}\n", s.display.rounding.as_setting()));
    out.push_str(&format!(
        "update_check_startup={}\n",
        if s.update_check_startup { "on" } else { "off" }
    ));
    out.push_str(if s.steps_deeper {
        "steps=deeper\n"
    } else {
        "steps=standard\n"
    });
    out.push_str(if s.projection {
        "projection=on\n"
    } else {
        "projection=off\n"
    });
    for (name, value) in &s.constants {
        out.push_str(&format!("constant.{name}={value}\n"));
    }
    for f in &s.favourites {
        out.push_str(&format!("favourite={f}\n"));
    }
    if !s.recent.is_empty() {
        out.push_str(&format!("recent={}\n", s.recent.join(",")));
    }
    let mut ids: Vec<&String> = s.remembered.keys().collect();
    ids.sort();
    for id in ids {
        let toks: Vec<String> = s.remembered[id].iter().map(|t| esc(t)).collect();
        out.push_str(&format!("inputs.{id}={}\n", toks.join(";;")));
    }
    out
}

/// Parse settings text. Anything unrecognised is skipped; ids that
/// are not in `valid_ids` are dropped (a settings file from another
/// version can never resurrect a removed calculator).
pub fn parse_settings(text: &str, valid_ids: &BTreeSet<String>) -> Settings {
    let mut s = Settings::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(v) = line.strip_prefix("display_format=") {
            if let Ok(f) = DisplayFormat::parse(v) {
                // Keep an already-read rounding choice.
                let rounding = s.display.rounding;
                s.display = DisplayFormat { rounding, ..f };
            }
        } else if let Some(v) = line.strip_prefix("rounding=") {
            if let Some(r) = RoundingMode::parse(v) {
                s.display.rounding = r;
            }
        } else if let Some(v) = line.strip_prefix("update_check_startup=") {
            s.update_check_startup = v == "on" || v == "true";
        } else if let Some(v) = line.strip_prefix("steps=") {
            s.steps_deeper = v.trim() == "deeper";
        } else if let Some(v) = line.strip_prefix("projection=") {
            s.projection = v.trim() == "on" || v.trim() == "true";
        } else if let Some(v) = line.strip_prefix("sig_figs=") {
            // Legacy (v0.3.0) line: only applies when no explicit
            // display_format line has set a mode.
            if s.display.mode == DisplayMode::Off && v != "off" {
                if let Some(n) = v.parse::<u8>().ok().filter(|n| (1..=10).contains(n)) {
                    s.display.mode = DisplayMode::SigFigs;
                    s.display.n = Some(n);
                }
            }
        } else if let Some(v) = line.strip_prefix("favourite=") {
            if valid_ids.contains(v) {
                s.favourites.insert(v.to_string());
            }
        } else if let Some(v) = line.strip_prefix("recent=") {
            s.recent = v
                .split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| valid_ids.contains(x))
                .take(RECENT_CAP)
                .collect();
        } else if let Some(rest) = line.strip_prefix("constant.") {
            // User-defined constants (#38): a line loads only if
            // the name still validates (no built-in collision, no
            // function name) and the value is a finite number —
            // anything else is skipped like any other corrupt line.
            if let Some((name, value)) = rest.split_once('=') {
                if let (Ok(name), Ok(v)) = (
                    crate::rigour::validate_constant_name(name),
                    value.trim().parse::<f64>(),
                ) {
                    if v.is_finite() {
                        s.constants.retain(|(n, _)| n != &name);
                        s.constants.push((name, v));
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("inputs.") {
            if let Some((id, toks)) = rest.split_once('=') {
                if valid_ids.contains(id) && !toks.is_empty() {
                    s.remembered.insert(id.to_string(), split_tokens(toks));
                }
            }
        }
    }
    s
}

/// Where the settings live: %APPDATA%\CalcDex\settings.txt when
/// APPDATA is set, else beside the executable, else the current
/// folder (same rule as the history log).
pub fn settings_path() -> std::path::PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        if !appdata.is_empty() {
            return std::path::Path::new(&appdata)
                .join("CalcDex")
                .join("settings.txt");
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.join("settings.txt");
        }
    }
    std::path::PathBuf::from("settings.txt")
}

/// Load settings from `path`; missing/corrupt -> defaults. Never
/// fails: persistence problems must never break startup.
pub fn load_settings_from(path: &std::path::Path, valid_ids: &BTreeSet<String>) -> Settings {
    match std::fs::read_to_string(path) {
        Ok(text) => parse_settings(&text, valid_ids),
        Err(_) => Settings::default(),
    }
}

pub fn load_settings(valid_ids: &BTreeSet<String>) -> Settings {
    load_settings_from(&settings_path(), valid_ids)
}

pub fn save_settings_to(path: &std::path::Path, s: &Settings) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, serialize_settings(s))
}

pub fn save_settings(s: &Settings) -> std::io::Result<()> {
    save_settings_to(&settings_path(), s)
}

impl Settings {
    /// Record a run of `id`: it becomes the most-recent calculator
    /// and its display tokens are remembered (item #48).
    pub fn note_run(&mut self, id: &str, tokens: Vec<String>) {
        self.recent.retain(|r| r != id);
        self.recent.insert(0, id.to_string());
        self.recent.truncate(RECENT_CAP);
        self.remembered.insert(id.to_string(), tokens);
    }

    pub fn toggle_favourite(&mut self, id: &str) -> bool {
        if self.favourites.contains(id) {
            self.favourites.remove(id);
            false
        } else {
            self.favourites.insert(id.to_string());
            true
        }
    }
}
