#![cfg_attr(windows, windows_subsystem = "windows")]
//! CalcDex — GUI shell + CLI. All real work lives in the library's
//! calculator registry, dispatched identically here, so there are no
//! GUI-only calculators.

use calcdex::{
    all_calcs, append_history, clear_history, date_group_at, expand_args, load_history, make_entry,
    run_calc, today_str, Calc, LogEntry, DATE_HINT,
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
            .with_min_inner_size([880.0, 580.0]),
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
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0x1a, 0x1a, 0x1a);
    ctx.set_visuals(visuals);
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

struct App {
    calcs: Vec<Calc>,
    categories: Vec<String>,
    category: String, // "All" or a category name
    search: String,
    selected: Option<usize>, // index into calcs
    inputs: Vec<String>,
    /// One text per date group of the selected calculator (MM/DD/YYYY).
    date_texts: Vec<String>,
    list_input: String,
    output: String,
    show_log: bool,
    log_entries: Vec<LogEntry>,
}

impl App {
    fn new() -> App {
        let calcs = all_calcs();
        let mut set: BTreeSet<String> = BTreeSet::new();
        for c in &calcs {
            set.insert(c.category.to_string());
        }
        let mut categories = vec!["All".to_string()];
        categories.extend(set);
        App {
            calcs,
            categories,
            category: "All".to_string(),
            search: String::new(),
            selected: None,
            inputs: Vec::new(),
            date_texts: Vec::new(),
            list_input: String::new(),
            output: String::new(),
            show_log: false,
            log_entries: load_history(),
        }
    }

    fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
        self.inputs = vec![String::new(); self.calcs[idx].inputs.len()];
        self.list_input.clear();
        self.output.clear();
        self.show_log = false;
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
                // log view reads this same in-memory list, so it updates
                // immediately.
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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    .color(MUTED)
                    .size(11.0),
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
        egui::SidePanel::left("cats")
            .resizable(true)
            .default_width(215.0)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("SEARCH").color(MUTED).size(11.0));
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("filter calculators...")
                            .desired_width(150.0),
                    );
                    if ui.button("Clear").clicked() {
                        self.search.clear();
                    }
                });
                ui.add_space(6.0);
                ui.label(egui::RichText::new("CATEGORY").color(MUTED).size(11.0));
                egui::ScrollArea::vertical()
                    .id_salt("cat-scroll")
                    .max_height(180.0)
                    .show(ui, |ui| {
                        for cat in &self.categories {
                            let n = if cat == "All" {
                                self.calcs.len()
                            } else {
                                self.calcs.iter().filter(|c| c.category == cat).count()
                            };
                            let label = format!("{} ({})", cat.to_uppercase(), n);
                            if ui.selectable_label(self.category == *cat, label).clicked() {
                                self.category = cat.clone();
                            }
                        }
                    });
                ui.add_space(6.0);
                ui.separator();
                ui.label(egui::RichText::new("CALCULATORS").color(MUTED).size(11.0));
                let mut to_select: Option<usize> = None;
                egui::ScrollArea::vertical()
                    .id_salt("calc-scroll")
                    .show(ui, |ui| {
                        for (i, c) in self.calcs.iter().enumerate() {
                            if !self.visible(c) {
                                continue;
                            }
                            if ui
                                .selectable_label(self.selected == Some(i), c.name)
                                .clicked()
                            {
                                to_select = Some(i);
                            }
                        }
                    });
                if let Some(i) = to_select {
                    self.select(i);
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_log {
                ui.label(
                    egui::RichText::new("CALCULATION LOG")
                        .color(GREEN)
                        .strong()
                        .size(16.0),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{} saved calculations, newest first. Every successful calculation (window or command line) is saved here automatically. Click RE-OPEN to load an entry's calculator with its inputs restored.",
                        self.log_entries.len()
                    ))
                    .color(MUTED),
                );
                ui.add_space(6.0);
                let mut do_clear = false;
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("CLEAR LOG").color(RED),
                    ))
                    .clicked()
                {
                    do_clear = true;
                }
                if do_clear {
                    let _ = clear_history();
                    self.log_entries.clear();
                }
                ui.add_space(6.0);
                ui.separator();
                let mut to_restore: Option<LogEntry> = None;
                if self.log_entries.is_empty() {
                    ui.label(
                        egui::RichText::new("The log is empty. Run any calculator and its result appears here.")
                            .color(TEXT),
                    );
                }
                egui::ScrollArea::vertical()
                    .id_salt("log-scroll")
                    .show(ui, |ui| {
                        for e in self.log_entries.iter().rev() {
                            ui.group(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&e.timestamp)
                                            .color(MUTED)
                                            .size(11.0),
                                    );
                                    ui.label(
                                        egui::RichText::new(&e.name)
                                            .color(GREEN)
                                            .strong(),
                                    );
                                    if ui.button("RE-OPEN").clicked() {
                                        to_restore = Some(e.clone());
                                    }
                                });
                                ui.label(
                                    egui::RichText::new(format!("Inputs: {}", e.inputs))
                                        .color(TEXT),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Result: {}", e.result))
                                        .color(TEXT),
                                );
                            });
                            ui.add_space(4.0);
                        }
                    });
                if let Some(e) = to_restore {
                    self.restore(&e);
                }
                return;
            }
            let Some(idx) = self.selected else {
                ui.label(
                    egui::RichText::new("Pick a calculator on the left.")
                        .color(TEXT)
                        .size(14.0),
                );
                ui.label(
                    egui::RichText::new("Every calculator here also works from the command line — e.g. CalcDex compound-interest 1000 5 10 12")
                        .color(MUTED),
                );
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
                egui::RichText::new(name.to_uppercase()).color(GREEN).strong().size(16.0),
            );
            ui.label(egui::RichText::new(format!("[{category}]  id: {id}")).color(MUTED).size(11.0));
            ui.add_space(4.0);
            ui.label(egui::RichText::new(desc).color(TEXT));
            ui.add_space(8.0);
            if is_list {
                ui.label(egui::RichText::new("NUMBERS (COMMA OR SPACE SEPARATED)").color(MUTED).size(11.0));
                ui.add(
                    egui::TextEdit::multiline(&mut self.list_input)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );
            } else {
                let mut gi = 0usize;
                let mut i = 0usize;
                while i < inputs_labels.len() {
                    if let Some(g) = date_groups.iter().find(|g| g.start == i) {
                        ui.horizontal(|ui| {
                            ui.add_sized([300.0, 20.0], egui::Label::new(egui::RichText::new(g.label.to_uppercase()).color(MUTED).size(11.0)));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.date_texts[gi])
                                    .hint_text("MM/DD/YYYY")
                                    .desired_width(180.0),
                            );
                            if ui
                                .add(egui::Button::new(egui::RichText::new("Today?").color(GREEN)))
                                .clicked()
                            {
                                self.date_texts[gi] = today_str();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.add_sized([300.0, 20.0], egui::Label::new(""));
                            ui.label(egui::RichText::new(DATE_HINT).color(MUTED).size(11.0));
                        });
                        gi += 1;
                        i += 3;
                    } else {
                        let label = inputs_labels[i];
                        ui.horizontal(|ui| {
                            ui.add_sized([300.0, 20.0], egui::Label::new(egui::RichText::new(label.to_uppercase()).color(MUTED).size(11.0)));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.inputs[i]).desired_width(180.0),
                            );
                        });
                        i += 1;
                    }
                }
            }
            ui.add_space(8.0);
            if ui
                .add(egui::Button::new(egui::RichText::new("CALCULATE").color(GREEN).strong()))
                .clicked()
            {
                self.calculate();
            }
            ui.add_space(8.0);
            ui.separator();
            if !self.output.is_empty() {
                let is_error = self.output.starts_with("Error") || self.output.contains("is not a number");
                egui::ScrollArea::vertical()
                    .id_salt("out-scroll")
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&self.output)
                                .color(if is_error { RED } else { TEXT })
                                .size(13.0),
                        );
                    });
            }
        });
    }
}
