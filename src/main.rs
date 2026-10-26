#![cfg_attr(windows, windows_subsystem = "windows")]
//! CalcDex — GUI shell + CLI. All real work lives in the library's
//! calculator registry, dispatched identically here, so there are no
//! GUI-only calculators.

use calcdex::{
    all_calcs, append_history, clear_history, date_group_at, expand_args, load_history, make_entry,
    pad_apply, pad_key_allowed, run_calc, today_str, Calc, FieldKind, LogEntry, PadKey, DATE_HINT,
};
use std::collections::BTreeSet;

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        std::process::exit(cli(&args));
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CalcDex — every kind of calculation, offline")
            .with_icon(std::sync::Arc::new(egui::IconData {
                rgba: include_bytes!("../assets/icon-64.rgba").to_vec(),
                width: 64,
                height: 64,
            }))
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CalcDex",
        options,
        Box::new(|cc| {
            configure_style(&cc.egui_ctx);
            configure_fonts(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}

/// CLI parity with the GUI:
///   CalcDex --list
///   CalcDex --search loan
///   CalcDex compound-interest 1000 5 10 12
///   CalcDex age 04/23/1988 10/25/2026      (dates as ONE MM/DD/YYYY argument)
///   CalcDex --log / --clear-log
fn cli(args: &[String]) -> i32 {
    let calcs = all_calcs();
    if args[0] == "--list" || args[0] == "list" {
        for c in &calcs {
            println!("{:<38} [{}] {}", c.id, c.category, c.name);
        }
        println!("\n{} calculators total.", calcs.len());
        return 0;
    }
    if args[0] == "--search" || args[0] == "search" {
        let q = args.get(1).map(|s| s.to_lowercase()).unwrap_or_default();
        for c in &calcs {
            if c.name.to_lowercase().contains(&q)
                || c.id.contains(&q)
                || c.desc.to_lowercase().contains(&q)
                || c.category.to_lowercase().contains(&q)
            {
                println!("{:<38} [{}] {}", c.id, c.category, c.name);
            }
        }
        return 0;
    }
    if args[0] == "--log" || args[0] == "log" {
        let entries = load_history();
        if entries.is_empty() {
            println!("The calculation log is empty. Run any calculator and its result is saved here automatically.");
            return 0;
        }
        println!("Calculation log ({} entries, newest first):", entries.len());
        for e in entries.iter().rev() {
            println!("\n{}  {} ({})", e.timestamp, e.name, e.id);
            println!("  Inputs: {}", e.inputs);
            println!("  Result: {}", e.result);
        }
        return 0;
    }
    if args[0] == "--clear-log" {
        return match clear_history() {
            Ok(()) => {
                println!("Calculation log cleared.");
                0
            }
            Err(e) => {
                eprintln!("Could not clear the log: {e}");
                1
            }
        };
    }
    if args[0] == "--help" || args[0] == "help" || args[0] == "-h" {
        println!("CalcDex \u{2014} every kind of calculation, offline.\n\nNo arguments: opens the window.\n--list                       list all calculators\n--search <word>              find calculators\n--log                        print the calculation log (every successful run, window or command line, is saved there)\n--clear-log                  empty the calculation log\n<calculator-id> <numbers...> run one calculator; numbers in the order of its labelled inputs\n                             (list calculators: one argument per value; pair calculators: values in pairs)\n\nDates: a date is entered as ONE argument in MM/DD/YYYY form \u{2014} for example\n  CalcDex age 04/23/1988 10/25/2026\nThe older three-number form still works: CalcDex age 1988 4 23 2026 10 25\n\nNumbers may use scientific notation (e.g. 6.022e23, 1.5e-3). Results show the formula, substituted values, and full precision.\n\nExample: CalcDex compound-interest 1000 5 10");
        return 0;
    }
    let id = &args[0];
    let Some(calc) = calcs.iter().find(|c| c.id == id.as_str()) else {
        eprintln!("No calculator called '{id}'. Try: CalcDex --list");
        return 2;
    };
    let nums = match expand_args(calc, &args[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}");
            return 2;
        }
    };
    match run_calc(id, &nums) {
        Ok(out) => {
            println!("{out}");
            // Every successful CLI run is saved to the same log the window uses.
            let entry = make_entry(calc, &nums, &out);
            let _ = append_history(entry);
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

// ---------- Terminal theme (Purgeon exact values) ----------
// --bg #0c0c0c, --panel #121212, --raised #181818, --border #262626,
// --text #d0d0d0, --muted #7a7a7a, --green #4ade80, --green-sel #163322,
// --amber #e5c07b, --cyan #56b6c2, --violet #b294f0, --red #f26d6d, radius 0.
fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    for font in style.text_styles.values_mut() {
        font.family = egui::FontFamily::Monospace;
    }
    ctx.set_style(style);
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(0x12, 0x12, 0x12);
    visuals.window_fill = egui::Color32::from_rgb(0x0c, 0x0c, 0x0c);
    visuals.extreme_bg_color = egui::Color32::from_rgb(0x0c, 0x0c, 0x0c);
    visuals.faint_bg_color = egui::Color32::from_rgb(0x18, 0x18, 0x18);
    visuals.selection.bg_fill = egui::Color32::from_rgb(0x16, 0x33, 0x22);
    visuals.selection.stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0x4a, 0xde, 0x80));
    visuals.hyperlink_color = egui::Color32::from_rgb(0x56, 0xb6, 0xc2);
    // Muted grey is reserved for true secondary text: field hint /
    // placeholder text (egui's "weak" colour), never for list content.
    visuals.weak_text_color = Some(MUTED);
    let zero = egui::CornerRadius::ZERO;
    for w in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        w.corner_radius = zero;
        w.bg_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0x26, 0x26, 0x26));
    }
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(0x18, 0x18, 0x18);
    // Hover must be clearly visible (v0.1.1 used #1a1a1a, barely
    // distinguishable from the #121212 panel); pressed gets green-dim.
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0x2a, 0x2a, 0x2a);
    visuals.widgets.hovered.fg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0xd0, 0xd0, 0xd0));
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0x1d, 0x3b, 0x27);
    visuals.widgets.active.fg_stroke =
        egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0x4a, 0xde, 0x80));
    ctx.set_visuals(visuals);
    // Slightly larger base text (13 pt, top of the Terminal range) and a
    // consistent spacing rhythm. Sizes are in egui points, so they scale
    // together when the user zooms the UI (Ctrl and + / -).
    ctx.style_mut(|style| {
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::monospace(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, egui::FontId::monospace(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Monospace, egui::FontId::monospace(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Small, egui::FontId::monospace(11.5));
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(10.0, 5.0);
        style.spacing.interact_size = egui::vec2(48.0, 26.0);
    });
}

fn configure_fonts(ctx: &egui::Context) {
    // Prefer the real Cascadia Mono / Cascadia Code / Consolas font files
    // when the machine has them (Windows ships Cascadia with Windows
    // Terminal and Consolas always); load the file as font data, like
    // DevTextTools loads Segoe fonts. Otherwise egui's bundled monospace
    // is used — the family stays Monospace either way.
    let candidates = [
        ("cascadia", "C:\\Windows\\Fonts\\CascadiaMono.ttf"),
        ("cascadia", "C:\\Windows\\Fonts\\CascadiaCode.ttf"),
        ("consolas", "C:\\Windows\\Fonts\\consola.ttf"),
        (
            "dejavu-mono",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        ),
    ];
    let mut fonts = egui::FontDefinitions::default();
    for (name, path) in candidates {
        if fonts.font_data.contains_key(name) {
            continue;
        }
        if let Ok(bytes) = std::fs::read(path) {
            fonts.font_data.insert(
                name.to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(bytes)),
            );
            for family in [egui::FontFamily::Monospace, egui::FontFamily::Proportional] {
                if let Some(list) = fonts.families.get_mut(&family) {
                    list.insert(0, name.to_owned());
                }
            }
            break;
        }
    }
    ctx.set_fonts(fonts);
}

// ---------- App ----------
//
// v0.2.0 layout — Design 3 (SEAN's pick from the 8 research mockups,
// Microsoft-Calculator-inspired):
//   * a slim MODE RAIL on the far left: the categories are the modes,
//     each with its Alt+ shortcut hint, plus the search box on top;
//   * the CENTRE: a chip strip of the active mode's calculators, the
//     selected calculator's form, a prominent DISPLAY for the result,
//     a big on-screen KEYPAD, and the worked steps as a footer block;
//   * a HISTORY rail docked on the right (the calculation log), which
//     collapses below the width threshold documented at
//     `HISTORY_RAIL_MIN_WIDTH` — there the header LOG toggle switches
//     the centre to the log view instead, as it did in v0.1.x.

/// One mode-rail entry: a category (or "All") as a mode.
struct Mode {
    /// Full category name ("All" or e.g. "Date & Time").
    name: String,
    /// Short label shown in the slim rail.
    short: &'static str,
    /// Shortcut hint printed on the row, e.g. "Alt+1".
    hint: String,
    /// The key that, with Alt held, selects this mode.
    key: egui::Key,
}

/// Curated mode order (the registry's own presentation order, not
/// alphabetical): general first, then the school/university subjects,
/// then the applied sets.
const MODE_ORDER: &[(&str, &str)] = &[
    ("All", "ALL"),
    ("Math", "MATH"),
    ("Finance", "FINANCE"),
    ("Health", "HEALTH"),
    ("Physics", "PHYSICS"),
    ("Chemistry", "CHEM"),
    ("Statistics", "STATS"),
    ("Conversion", "CONVERT"),
    ("Construction", "BUILD"),
    ("Date & Time", "DATE"),
    ("Everyday", "DAILY"),
    ("Sports", "SPORTS"),
    ("Food", "FOOD"),
    ("Ecology", "ECOLOGY"),
    ("Other", "OTHER"),
];

/// Alt+ keys in mode order: digits first (1..9, 0), then Q W E R T —
/// fifteen modes total. These are the hints printed on the rail, and
/// they are the set the handler below implements — nothing else is
/// printed anywhere.
const MODE_KEYS: &[egui::Key] = &[
    egui::Key::Num1,
    egui::Key::Num2,
    egui::Key::Num3,
    egui::Key::Num4,
    egui::Key::Num5,
    egui::Key::Num6,
    egui::Key::Num7,
    egui::Key::Num8,
    egui::Key::Num9,
    egui::Key::Num0,
    egui::Key::Q,
    egui::Key::W,
    egui::Key::E,
    egui::Key::R,
    egui::Key::T,
];

fn mode_hint(key: egui::Key) -> String {
    let name = match key {
        egui::Key::Num0 => "0",
        egui::Key::Num1 => "1",
        egui::Key::Num2 => "2",
        egui::Key::Num3 => "3",
        egui::Key::Num4 => "4",
        egui::Key::Num5 => "5",
        egui::Key::Num6 => "6",
        egui::Key::Num7 => "7",
        egui::Key::Num8 => "8",
        egui::Key::Num9 => "9",
        egui::Key::Q => "Q",
        egui::Key::W => "W",
        egui::Key::E => "E",
        egui::Key::R => "R",
        egui::Key::T => "T",
        _ => "?",
    };
    format!("Alt+{name}")
}

fn build_modes(present: &BTreeSet<String>) -> Vec<Mode> {
    let mut modes = Vec::new();
    let mut keys = MODE_KEYS.iter();
    for (name, short) in MODE_ORDER {
        if *name != "All" && !present.contains(*name) {
            continue;
        }
        let Some(key) = keys.next() else { break };
        modes.push(Mode {
            name: name.to_string(),
            short,
            hint: mode_hint(*key),
            key: *key,
        });
    }
    modes
}

/// Which input field the on-screen keypad currently targets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FieldRef {
    /// Index into the calculator's labelled inputs.
    Input(usize),
    /// Index into the calculator's date groups.
    Date(usize),
    /// The single list-input field of a list calculator.
    List,
}

struct App {
    calcs: Vec<Calc>,
    modes: Vec<Mode>,
    category: String, // "All" or a category name
    search: String,
    selected: Option<usize>, // index into calcs
    inputs: Vec<String>,
    /// One text per date group of the selected calculator (MM/DD/YYYY).
    date_texts: Vec<String>,
    list_input: String,
    output: String,
    /// History rail visible (wide windows) / log view shown (narrow).
    show_log: bool,
    log_entries: Vec<LogEntry>,
    /// Last field the user clicked or typed in (keypad target).
    focus: Option<FieldRef>,
    /// After a keypad press, give keyboard focus back to the target
    /// field on the next frame so typing and the pad interleave.
    refocus: bool,
}

impl App {
    fn new() -> App {
        let calcs = all_calcs();
        let mut set: BTreeSet<String> = BTreeSet::new();
        for c in &calcs {
            set.insert(c.category.to_string());
        }
        App {
            modes: build_modes(&set),
            calcs,
            category: "All".to_string(),
            search: String::new(),
            selected: None,
            inputs: Vec::new(),
            date_texts: Vec::new(),
            list_input: String::new(),
            output: String::new(),
            show_log: true,
            log_entries: load_history(),
            focus: None,
            refocus: false,
        }
    }

    fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
        self.inputs = vec![String::new(); self.calcs[idx].inputs.len()];
        self.list_input.clear();
        self.output.clear();
        self.focus = None;
        self.refocus = false;
        // One merged text field per date group.
        self.date_texts = vec![String::new(); self.calcs[idx].date_groups.len()];
        // Sensible defaults: the Age calculator's End Date and the
        // Countdown's From Date start as today's date.
        let today = today_str();
        for (gi, g) in self.calcs[idx].date_groups.iter().enumerate() {
            let prefill = (self.calcs[idx].id == "age" && g.label == "End Date")
                || (self.calcs[idx].id == "countdown-days" && g.label == "From Date");
            if prefill {
                self.date_texts[gi] = today.clone();
            }
        }
    }

    /// Re-open a calculator from a log entry, with its inputs restored
    /// (date fields refilled in their merged MM/DD/YYYY form).
    fn restore(&mut self, entry: &LogEntry) {
        let Some(idx) = self.calcs.iter().position(|c| c.id == entry.id) else {
            return;
        };
        self.select(idx);
        let toks: Vec<&str> = if entry.restore.is_empty() {
            Vec::new()
        } else {
            entry.restore.split(";;").collect()
        };
        if self.calcs[idx].list {
            self.list_input = toks.join(", ");
            return;
        }
        let calc = &self.calcs[idx];
        let mut ti = 0usize;
        let mut gi = 0usize;
        for i in 0..calc.inputs.len() {
            if date_group_at(calc, i).is_some() {
                if let Some(t) = toks.get(ti) {
                    if gi < self.date_texts.len() {
                        self.date_texts[gi] = t.to_string();
                    }
                }
                gi += 1;
                ti += 1;
            } else if calc
                .date_groups
                .iter()
                .any(|g| i > g.start && i < g.start + 3)
            {
                // inside a date triple: nothing to fill
            } else {
                if let Some(t) = toks.get(ti) {
                    self.inputs[i] = t.to_string();
                }
                ti += 1;
            }
        }
    }

    fn visible(&self, c: &Calc) -> bool {
        if self.category != "All" && c.category != self.category {
            return false;
        }
        if self.search.trim().is_empty() {
            return true;
        }
        let q = self.search.to_lowercase();
        c.name.to_lowercase().contains(&q)
            || c.id.contains(&q)
            || c.desc.to_lowercase().contains(&q)
    }

    /// The field the keypad will type into: the last field the user
    /// touched, if it still exists on the selected calculator, else
    /// the calculator's first field.
    fn effective_focus(&self) -> Option<FieldRef> {
        let idx = self.selected?;
        let calc = &self.calcs[idx];
        if calc.list {
            return Some(FieldRef::List);
        }
        if let Some(f) = self.focus {
            let ok = match f {
                FieldRef::List => false,
                FieldRef::Date(gi) => gi < calc.date_groups.len(),
                FieldRef::Input(i) => {
                    i < calc.inputs.len()
                        && date_group_at(calc, i).is_none()
                        && !calc
                            .date_groups
                            .iter()
                            .any(|g| i > g.start && i < g.start + 3)
                }
            };
            if ok {
                return Some(f);
            }
        }
        if date_group_at(calc, 0).is_some() {
            Some(FieldRef::Date(0))
        } else if calc.inputs.is_empty() {
            None
        } else {
            Some(FieldRef::Input(0))
        }
    }

    fn focus_kind(&self) -> FieldKind {
        match self.effective_focus() {
            Some(FieldRef::Date(_)) => FieldKind::Date,
            Some(FieldRef::List) => FieldKind::List,
            _ => FieldKind::Numeric,
        }
    }

    /// Apply one on-screen keypad press to the target field's text.
    fn pad_press(&mut self, key: PadKey) {
        let Some(target) = self.effective_focus() else {
            return;
        };
        match target {
            FieldRef::Input(i) => {
                if let Some(s) = self.inputs.get_mut(i) {
                    *s = pad_apply(s, key);
                }
            }
            FieldRef::Date(gi) => {
                if let Some(s) = self.date_texts.get_mut(gi) {
                    *s = pad_apply(s, key);
                }
            }
            FieldRef::List => {
                self.list_input = pad_apply(&self.list_input.clone(), key);
            }
        }
        self.focus = Some(target);
        self.refocus = true;
    }

    fn calculate(&mut self) {
        let Some(idx) = self.selected else {
            return;
        };
        let calc = &self.calcs[idx];
        let mut nums: Vec<f64> = Vec::new();
        if calc.list {
            for tok in self
                .list_input
                .split(|ch: char| ch == ',' || ch == ';' || ch.is_whitespace())
            {
                if tok.is_empty() {
                    continue;
                }
                match tok.parse::<f64>() {
                    Ok(v) => nums.push(v),
                    Err(_) => {
                        self.output = format!("'{tok}' is not a number.");
                        return;
                    }
                }
            }
        } else {
            let mut gi = 0usize;
            let mut i = 0usize;
            while i < calc.inputs.len() {
                if let Some(g) = date_group_at(calc, i) {
                    let raw = self.date_texts.get(gi).cloned().unwrap_or_default();
                    match calcdex::parse_date_str(&raw) {
                        Ok((y, m, d)) => {
                            nums.push(y as f64);
                            nums.push(m as f64);
                            nums.push(d as f64);
                        }
                        Err(e) => {
                            self.output = format!("Error in {}: {e}", g.label);
                            return;
                        }
                    }
                    gi += 1;
                    i += 3;
                } else {
                    let raw = &self.inputs[i];
                    match raw.trim().parse::<f64>() {
                        Ok(v) => nums.push(v),
                        Err(_) => {
                            self.output = format!(
                                "'{}' needs a number.",
                                calc.inputs.get(i).copied().unwrap_or("Input")
                            );
                            return;
                        }
                    }
                    i += 1;
                }
            }
        }
        match run_calc(calc.id, &nums) {
            Ok(out) => {
                // Auto-save every successful calculation to the log; the
                // history rail reads this same in-memory list, so the new
                // entry appears there immediately.
                let entry = make_entry(calc, &nums, &out);
                if let Ok(entries) = append_history(entry) {
                    self.log_entries = entries;
                } else {
                    self.log_entries.push(make_entry(calc, &nums, &out));
                }
                self.output = out;
            }
            Err(e) => {
                self.output = format!("Error: {e}");
            }
        }
    }
}

const GREEN: egui::Color32 = egui::Color32::from_rgb(0x4a, 0xde, 0x80);
const MUTED: egui::Color32 = egui::Color32::from_rgb(0x7a, 0x7a, 0x7a);
const TEXT: egui::Color32 = egui::Color32::from_rgb(0xd0, 0xd0, 0xd0);
const RED: egui::Color32 = egui::Color32::from_rgb(0xf2, 0x6d, 0x6d);
/// Selection background for rows/chips (Purgeon green-sel #163322).
const SEL_BG: egui::Color32 = egui::Color32::from_rgb(0x16, 0x33, 0x22);
/// Hover background for rows — clearly visible on #121212.
const HOVER_BG: egui::Color32 = egui::Color32::from_rgb(0x26, 0x26, 0x26);
/// Shortcut-hint grey on the mode rail: #9a9a9a measures 6.66:1 on the
/// panel (WCAG AA asks 4.5:1) — brighter than the theme muted grey,
/// which is only for large/secondary print. See docs/AUDIT.md.
const HINT: egui::Color32 = egui::Color32::from_rgb(0x9a, 0x9a, 0x9a);
/// Display background (theme bg #0c0c0c, inset against the panel).
const DISPLAY_BG: egui::Color32 = egui::Color32::from_rgb(0x0c, 0x0c, 0x0c);
/// Raised fill for keypad buttons (#181818).
const RAISED: egui::Color32 = egui::Color32::from_rgb(0x18, 0x18, 0x18);
/// Equals-key fill (theme green-dim #1d3b27).
const GREEN_DIM: egui::Color32 = egui::Color32::from_rgb(0x1d, 0x3b, 0x27);
/// Below this window width (points) the history rail collapses: the
/// header LOG toggle then switches the centre to the log view, as in
/// v0.1.x. At the 760 pt minimum window the rail is always collapsed.
const HISTORY_RAIL_MIN_WIDTH: f32 = 1100.0;

/// One mode-rail row: short label on the left, shortcut hint on the
/// right, full category name in the hover tooltip. Selected styling
/// matches the v0.1.2 list rows (selection background + green text +
/// 3 pt marker bar), so selection is never colour-alone subtle.
fn mode_row(ui: &mut egui::Ui, mode: &Mode, count: usize, selected: bool) -> bool {
    let width = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(width, 30.0), egui::Sense::click());
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, 0.0, SEL_BG);
        painter.rect_filled(
            egui::Rect::from_min_size(rect.min, egui::vec2(3.0, rect.height())),
            0.0,
            GREEN,
        );
    } else if resp.hovered() {
        painter.rect_filled(rect, 0.0, HOVER_BG);
    }
    let cy = rect.center().y;
    painter.text(
        egui::pos2(rect.min.x + 10.0, cy),
        egui::Align2::LEFT_CENTER,
        mode.short,
        egui::FontId::monospace(13.0),
        if selected { GREEN } else { TEXT },
    );
    painter.text(
        egui::pos2(rect.max.x - 8.0, cy),
        egui::Align2::RIGHT_CENTER,
        &mode.hint,
        egui::FontId::monospace(10.5),
        HINT,
    );
    let clicked = resp.clicked();
    resp.on_hover_text(format!("{} ({count} calculators)", mode.name));
    clicked
}

/// The history (calculation log) content, shared by the docked right
/// rail and the narrow-window centre log view. Returns
/// (entry to re-open, clear-log clicked).
fn history_ui(ui: &mut egui::Ui, entries: &[LogEntry]) -> (Option<LogEntry>, bool) {
    let mut to_restore: Option<LogEntry> = None;
    ui.label(
        egui::RichText::new(format!("HISTORY ({})", entries.len()))
            .color(GREEN)
            .strong()
            .size(14.0),
    );
    ui.label(
        egui::RichText::new(
            "Saved calculations, newest first. Every successful calculation \
             (window or command line) is saved automatically. RE-OPEN loads \
             an entry's calculator with its inputs restored.",
        )
        .color(TEXT),
    );
    ui.add_space(8.0);
    let mut do_clear = false;
    if ui
        .add(egui::Button::new(
            egui::RichText::new("CLEAR LOG").color(RED),
        ))
        .clicked()
    {
        do_clear = true;
    }
    ui.add_space(8.0);
    ui.separator();
    if entries.is_empty() {
        ui.label(
            egui::RichText::new(
                "The log is empty. Run any calculator and its result appears here.",
            )
            .color(TEXT),
        );
    }
    egui::ScrollArea::vertical()
        .id_salt("history-scroll")
        .show(ui, |ui| {
            for e in entries.iter().rev() {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new(&e.timestamp).color(TEXT).size(11.5));
                        ui.label(egui::RichText::new(&e.name).color(GREEN).strong());
                        if ui.button("RE-OPEN").clicked() {
                            to_restore = Some(e.clone());
                        }
                    });
                    ui.label(egui::RichText::new(format!("Inputs: {}", e.inputs)).color(TEXT));
                    ui.label(egui::RichText::new(format!("Result: {}", e.result)).color(TEXT));
                });
                ui.add_space(4.0);
            }
        });
    (to_restore, do_clear)
}

/// One keypad key: an exact-size button inside an enabled/disabled
/// scope, so keys that make no sense for the focused field are visibly
/// greyed out and cannot be pressed. Returns true when clicked.
fn pad_key(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    label: &str,
    allowed: bool,
    fill: egui::Color32,
    color: egui::Color32,
) -> bool {
    let mut clicked = false;
    ui.add_enabled_ui(allowed, |ui| {
        clicked = ui
            .add_sized(
                size,
                egui::Button::new(egui::RichText::new(label).color(color).size(16.0).strong())
                    .fill(fill),
            )
            .clicked();
    });
    clicked
}

impl App {
    /// The Design 3 on-screen keypad: big buttons typing into the
    /// focused field (see `pad_press`). The separator cell is `/` for
    /// date fields and `,` for the list field; keys that make no sense
    /// for the focused field are greyed out via `pad_key_allowed`.
    /// `=` runs the calculation, exactly like CALCULATE / Enter.
    fn keypad_ui(&mut self, ui: &mut egui::Ui) {
        let kind = self.focus_kind();
        let pad_w = ui.available_width().min(520.0);
        let btn = egui::vec2((pad_w - 24.0) / 4.0, 48.0);
        let (sep_label, sep_key) = if kind == FieldKind::List {
            (",", PadKey::Comma)
        } else {
            ("/", PadKey::Slash)
        };
        ui.horizontal(|ui| {
            if pad_key(ui, btn, "7", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('7'));
            }
            if pad_key(ui, btn, "8", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('8'));
            }
            if pad_key(ui, btn, "9", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('9'));
            }
            if pad_key(ui, btn, "DEL", true, RAISED, TEXT) {
                self.pad_press(PadKey::Backspace);
            }
        });
        ui.horizontal(|ui| {
            if pad_key(ui, btn, "4", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('4'));
            }
            if pad_key(ui, btn, "5", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('5'));
            }
            if pad_key(ui, btn, "6", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('6'));
            }
            if pad_key(ui, btn, "C", true, RAISED, RED) {
                self.pad_press(PadKey::Clear);
            }
        });
        ui.horizontal(|ui| {
            if pad_key(ui, btn, "1", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('1'));
            }
            if pad_key(ui, btn, "2", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('2'));
            }
            if pad_key(ui, btn, "3", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('3'));
            }
            if pad_key(
                ui,
                btn,
                sep_label,
                pad_key_allowed(kind, sep_key),
                RAISED,
                TEXT,
            ) {
                self.pad_press(sep_key);
            }
        });
        ui.horizontal(|ui| {
            if pad_key(
                ui,
                btn,
                "+/-",
                pad_key_allowed(kind, PadKey::Negate),
                RAISED,
                TEXT,
            ) {
                self.pad_press(PadKey::Negate);
            }
            if pad_key(ui, btn, "0", true, RAISED, TEXT) {
                self.pad_press(PadKey::Digit('0'));
            }
            if pad_key(
                ui,
                btn,
                ".",
                pad_key_allowed(kind, PadKey::Dot),
                RAISED,
                TEXT,
            ) {
                self.pad_press(PadKey::Dot);
            }
            if pad_key(ui, btn, "=", true, GREEN_DIM, GREEN) {
                self.calculate();
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Alt+ mode shortcuts — the exact set printed on the rail.
        let mut mode_pick: Option<String> = None;
        ctx.input(|i| {
            if i.modifiers.alt {
                for m in &self.modes {
                    if i.key_pressed(m.key) {
                        mode_pick = Some(m.name.clone());
                    }
                }
            }
        });
        if let Some(name) = mode_pick {
            self.category = name;
        }
        // Enter = Calculate (single-line fields; the list box keeps
        // Enter for separating values).
        let enter_calc = ctx.input(|i| i.key_pressed(egui::Key::Enter))
            && self.selected.is_some()
            && self.focus_kind() != FieldKind::List;

        let wide = ctx.content_rect().width() >= HISTORY_RAIL_MIN_WIDTH;
        let mut hist_restore: Option<LogEntry> = None;
        let mut hist_clear = false;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("CALCDEX")
                        .color(GREEN)
                        .strong()
                        .size(18.0),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{} CALCULATORS - OFFLINE - NO ADS",
                        self.calcs.len()
                    ))
                    .color(TEXT)
                    .size(11.5),
                );
                ui.separator();
                let log_label = format!("LOG ({})", self.log_entries.len());
                if ui
                    .selectable_label(
                        self.show_log,
                        egui::RichText::new(log_label).color(GREEN).size(12.0),
                    )
                    .clicked()
                {
                    self.show_log = !self.show_log;
                    if self.show_log {
                        self.log_entries = load_history();
                    }
                }
            });
        });

        // ---- Mode rail (far left) ----
        egui::SidePanel::left("modes")
            .resizable(true)
            .default_width(178.0)
            .width_range(140.0..=240.0)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("SEARCH")
                        .color(GREEN)
                        .strong()
                        .size(12.0),
                );
                let search_w = (ui.available_width() - 58.0).max(80.0);
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("filter calculators...")
                            .desired_width(search_w),
                    );
                    if ui.button("Clear").clicked() {
                        self.search.clear();
                    }
                });
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("MODES")
                        .color(GREEN)
                        .strong()
                        .size(12.0),
                );
                egui::ScrollArea::vertical()
                    .id_salt("mode-scroll")
                    .show(ui, |ui| {
                        let mut picked: Option<String> = None;
                        for m in &self.modes {
                            let n = if m.name == "All" {
                                self.calcs.len()
                            } else {
                                self.calcs.iter().filter(|c| c.category == m.name).count()
                            };
                            if mode_row(ui, m, n, self.category == m.name) {
                                picked = Some(m.name.clone());
                            }
                        }
                        if let Some(name) = picked {
                            self.category = name;
                        }
                    });
                ui.add_space(8.0);
                ui.separator();
                ui.label(
                    egui::RichText::new(format!(
                        "{} calculators in {} modes. Alt+ a shown key picks a mode.",
                        self.calcs.len(),
                        self.modes.len() - 1
                    ))
                    .color(HINT)
                    .size(11.5),
                );
            });

        // ---- History rail (right; wide windows only) ----
        if self.show_log && wide {
            egui::SidePanel::right("history")
                .resizable(true)
                .default_width(300.0)
                .width_range(240.0..=420.0)
                .show(ctx, |ui| {
                    let (restore, clear) = history_ui(ui, &self.log_entries);
                    if restore.is_some() {
                        hist_restore = restore;
                    }
                    hist_clear = hist_clear || clear;
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Narrow window + log on: the centre becomes the log view
            // (the v0.1.x behaviour the LOG toggle has always had).
            if self.show_log && !wide {
                let (restore, clear) = history_ui(ui, &self.log_entries);
                if restore.is_some() {
                    hist_restore = restore;
                }
                hist_clear = hist_clear || clear;
                return;
            }
            egui::ScrollArea::vertical()
                .id_salt("detail-scroll")
                .show(ui, |ui| {
                    self.centre_ui(ui, enter_calc);
                });
        });

        if hist_clear {
            let _ = clear_history();
            self.log_entries.clear();
        }
        if let Some(e) = hist_restore {
            self.restore(&e);
        }
    }
}

impl App {
    /// The centre column: calculator chip strip, title + description,
    /// display, form, Calculate + keypad, steps footer.
    fn centre_ui(&mut self, ui: &mut egui::Ui, enter_calc: bool) {
        // ---- Calculator strip for the active mode ----
        ui.label(
            egui::RichText::new(format!("CALCULATORS — {}", self.category.to_uppercase()))
                .color(GREEN)
                .strong()
                .size(12.0),
        );
        let mut to_select: Option<usize> = None;
        egui::ScrollArea::vertical()
            .id_salt("strip-scroll")
            .max_height(112.0)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut any = false;
                    for (i, c) in self.calcs.iter().enumerate() {
                        if !self.visible(c) {
                            continue;
                        }
                        any = true;
                        let selected = self.selected == Some(i);
                        let btn = egui::Button::new(
                            egui::RichText::new(c.name)
                                .color(if selected { GREEN } else { TEXT })
                                .size(12.5),
                        )
                        .fill(if selected { SEL_BG } else { RAISED })
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            if selected {
                                GREEN
                            } else {
                                egui::Color32::from_rgb(0x26, 0x26, 0x26)
                            },
                        ));
                        let resp = ui.add(btn);
                        if resp.clicked() {
                            to_select = Some(i);
                        }
                        resp.on_hover_text(c.name);
                    }
                    if !any {
                        ui.label(
                            egui::RichText::new(
                                "No calculators match — clear the search or pick another mode.",
                            )
                            .color(TEXT),
                        );
                    }
                });
            });
        if let Some(i) = to_select {
            self.select(i);
        }
        ui.add_space(8.0);
        ui.separator();

        let Some(idx) = self.selected else {
            egui::Frame::new()
                .fill(DISPLAY_BG)
                .inner_margin(egui::Margin::same(10))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(0x26, 0x26, 0x26),
                ))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("PICK A CALCULATOR")
                            .color(GREEN)
                            .strong()
                            .size(14.0),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(
                            "Choose one from the strip above — or filter by name with                              the Search box in the rail. Modes switch with Alt+ the                              key printed on the rail.",
                        )
                        .color(TEXT),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Every calculator here also works from the command line — e.g. CalcDex compound-interest 1000 5 10 12")
                            .color(HINT),
                    );
                });
            return;
        };
        // Borrow fields separately to satisfy the borrow checker.
        let (id, name, category, desc, inputs_labels, is_list, date_groups) = {
            let c = &self.calcs[idx];
            (
                c.id,
                c.name,
                c.category,
                c.desc,
                c.inputs,
                c.list,
                c.date_groups,
            )
        };
        ui.label(
            egui::RichText::new(name.to_uppercase())
                .color(GREEN)
                .strong()
                .size(16.0),
        );
        ui.label(
            egui::RichText::new(format!("[{category}]  id: {id}"))
                .color(TEXT)
                .size(11.5),
        );
        ui.add_space(4.0);
        ui.label(egui::RichText::new(desc).color(TEXT));
        ui.add_space(8.0);

        // ---- Display (Design 3: prominent result zone) ----
        let is_error = self.output.starts_with("Error")
            || self.output.contains("is not a number")
            || self.output.contains("needs a number");
        egui::Frame::new()
            .fill(DISPLAY_BG)
            .inner_margin(egui::Margin::same(10))
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_rgb(0x26, 0x26, 0x26),
            ))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("DISPLAY — {}", name.to_uppercase()))
                        .color(HINT)
                        .size(11.5),
                );
                if self.output.is_empty() {
                    ui.label(
                        egui::RichText::new(
                            "Enter the values below, then press CALCULATE (or = on the keypad).",
                        )
                        .color(HINT)
                        .size(14.0),
                    );
                } else if is_error {
                    ui.label(egui::RichText::new(&self.output).color(RED).size(16.0));
                } else {
                    let headline = self.output.lines().next().unwrap_or("");
                    ui.label(
                        egui::RichText::new(headline)
                            .color(GREEN)
                            .strong()
                            .size(24.0),
                    );
                }
            });
        ui.add_space(8.0);

        // ---- Form ----
        // Input rows: the label column takes 38% of the pane width
        // (clamped 130-300 pt) and the field takes what is left
        // (clamped 150-380 pt), recomputed every frame so resizing or
        // zooming reflows instead of clipping.
        let avail = ui.available_width();
        let label_w = (avail * 0.38).clamp(130.0, 300.0);
        let field_w = (avail - label_w - 16.0).clamp(150.0, 380.0);
        let eff_focus = self.effective_focus();
        let mut pending_focus: Option<FieldRef> = None;
        let mut refocus_pending = self.refocus;
        if is_list {
            ui.label(
                egui::RichText::new("NUMBERS (COMMA OR SPACE SEPARATED)")
                    .color(TEXT)
                    .size(12.0),
            );
            let resp = ui.add(
                egui::TextEdit::multiline(&mut self.list_input)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY),
            );
            if resp.clicked() || resp.has_focus() {
                pending_focus = Some(FieldRef::List);
            }
            if refocus_pending && eff_focus == Some(FieldRef::List) {
                resp.request_focus();
                refocus_pending = false;
            }
        } else {
            let mut gi = 0usize;
            let mut i = 0usize;
            while i < inputs_labels.len() {
                if let Some(g) = date_groups.iter().find(|g| g.start == i) {
                    ui.horizontal_wrapped(|ui| {
                        ui.add_sized(
                            [label_w, 20.0],
                            egui::Label::new(
                                egui::RichText::new(g.label.to_uppercase())
                                    .color(TEXT)
                                    .size(12.0),
                            ),
                        );
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut self.date_texts[gi])
                                .hint_text("MM/DD/YYYY")
                                .desired_width(170.0),
                        );
                        if resp.clicked() || resp.has_focus() {
                            pending_focus = Some(FieldRef::Date(gi));
                        }
                        if refocus_pending && eff_focus == Some(FieldRef::Date(gi)) {
                            resp.request_focus();
                            refocus_pending = false;
                        }
                        if ui
                            .add(egui::Button::new(
                                egui::RichText::new("Today?").color(GREEN),
                            ))
                            .clicked()
                        {
                            self.date_texts[gi] = today_str();
                            pending_focus = Some(FieldRef::Date(gi));
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.add_sized([label_w, 20.0], egui::Label::new(""));
                        ui.label(egui::RichText::new(DATE_HINT).color(TEXT).size(11.5));
                    });
                    gi += 1;
                    i += 3;
                } else {
                    let label = inputs_labels[i];
                    ui.horizontal_wrapped(|ui| {
                        ui.add_sized(
                            [label_w, 20.0],
                            egui::Label::new(
                                egui::RichText::new(label.to_uppercase())
                                    .color(TEXT)
                                    .size(12.0),
                            ),
                        );
                        let resp = ui.add(
                            egui::TextEdit::singleline(&mut self.inputs[i]).desired_width(field_w),
                        );
                        if resp.clicked() || resp.has_focus() {
                            pending_focus = Some(FieldRef::Input(i));
                        }
                        if refocus_pending && eff_focus == Some(FieldRef::Input(i)) {
                            resp.request_focus();
                            refocus_pending = false;
                        }
                    });
                    i += 1;
                }
            }
        }
        if let Some(f) = pending_focus {
            self.focus = Some(f);
        }
        self.refocus = refocus_pending;

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui
                .add_sized(
                    [180.0, 36.0],
                    egui::Button::new(egui::RichText::new("CALCULATE").color(GREEN).strong()),
                )
                .clicked()
            {
                self.calculate();
            }
            ui.label(
                egui::RichText::new("Enter = Calculate")
                    .color(HINT)
                    .size(11.5),
            );
        });

        // ---- Keypad ----
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("KEYPAD")
                .color(GREEN)
                .strong()
                .size(12.0),
        );
        ui.label(
            egui::RichText::new(
                "The keypad types into the last field you clicked — a whole \
                 calculation can be run with the mouse alone.",
            )
            .color(HINT)
            .size(11.5),
        );
        self.keypad_ui(ui);

        if enter_calc {
            self.calculate();
        }

        // ---- Steps footer: the FULL result text, never truncated ----
        if !self.output.is_empty() && !is_error {
            ui.add_space(8.0);
            ui.separator();
            ui.label(
                egui::RichText::new("STEPS / FULL RESULT")
                    .color(GREEN)
                    .strong()
                    .size(12.0),
            );
            ui.label(egui::RichText::new(&self.output).color(TEXT).size(13.0));
        }
    }
}
