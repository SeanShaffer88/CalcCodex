//! Persistent settings (v0.3.0, items #1 / #46 / #48): favourites,
//! recent calculators, per-calculator remembered inputs, and the
//! sig-figs choice — one simple documented text file shared by all
//! three, at %APPDATA%\CalcDex\settings.txt.
//!
//! Format (one record per line, `#` comments allowed):
//!   sig_figs=3            (or sig_figs=off)
//!   favourite=<calc id>
//!   recent=<id>,<id>,...  (most recent first, max 8)
//!   inputs.<calc id>=<token>;;<token>;;...
//! Input tokens are the display tokens of a run (dates as
//! MM/DD/YYYY), separated by ";;" exactly like the calculation log's
//! restore field; backslash escapes `\\`, `\;` and `\n` inside a
//! token. Unknown ids and malformed lines are ignored on load — a
//! corrupt or missing file NEVER breaks startup; the app falls back
//! to defaults (see `load_settings`).

use std::collections::{BTreeSet, HashMap};

pub const RECENT_CAP: usize = 8;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
    /// Significant-figures display choice (None = off).
    pub sig_figs: Option<u8>,
    pub favourites: BTreeSet<String>,
    /// Most recent first.
    pub recent: Vec<String>,
    /// calc id -> display tokens of its last run (item #48).
    pub remembered: HashMap<String, Vec<String>>,
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
    match s.sig_figs {
        Some(n) => out.push_str(&format!("sig_figs={n}\n")),
        None => out.push_str("sig_figs=off\n"),
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
        if let Some(v) = line.strip_prefix("sig_figs=") {
            s.sig_figs = if v == "off" {
                None
            } else {
                v.parse::<u8>().ok().filter(|n| (1..=10).contains(n))
            };
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
