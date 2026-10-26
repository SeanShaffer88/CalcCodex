#![cfg_attr(windows, windows_subsystem = "windows")]
//! CalcDex — GUI shell + CLI. All real work lives in the library's
//! calculator registry, dispatched identically here, so there are no
//! GUI-only calculators.

use calcdex::features::{
    annotate_sig_figs, batch_run, csv_row, headline_of, search_ranked, sig_figs_headline, vary_run,
};
use calcdex::meta::{defaults_for, display_fields, hint_for};
use calcdex::settings::{load_settings, save_settings as save_settings_file, Settings};
use calcdex::{
    all_calcs, append_history, clear_history, date_group_at, display_tokens, expand_args,
    load_history, make_entry, run_calc, today_str, Calc, LogEntry, DATE_HINT,
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
    // Global --sig-figs N (item #1): may appear anywhere; it only
    // ADDS a rounded showing of the headline — the full-precision
    // result (and the log) are never altered.
    let mut sig_figs: Option<u8> = None;
    let mut args: Vec<String> = args.to_vec();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--sig-figs" {
            match args.get(i + 1).and_then(|v| v.parse::<u8>().ok()) {
                Some(n) if (1..=10).contains(&n) => {
                    sig_figs = Some(n);
                    args.drain(i..=i + 1);
                }
                _ => {
                    eprintln!("--sig-figs needs a whole number from 1 to 10, e.g. --sig-figs 3.");
                    return 2;
                }
            }
        } else {
            i += 1;
        }
    }
    if args.is_empty() {
        // Only --sig-figs was given: nothing to run.
        eprintln!("Nothing to run — give a calculator id or --help.");
        return 2;
    }
    let args = &args[..];
    if args[0] == "--list" || args[0] == "list" {
        for c in &calcs {
            println!("{:<38} [{}] {}", c.id, c.category, c.name);
        }
        println!("\n{} calculators total.", calcs.len());
        return 0;
    }
    if args[0] == "--search" || args[0] == "search" {
        // Ranked search with synonyms (item #47).
        let q = args.get(1).cloned().unwrap_or_default();
        for idx in search_ranked(&calcs, &q) {
            let c = &calcs[idx];
            println!("{:<38} [{}] {}", c.id, c.category, c.name);
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
    if args[0] == "--describe" {
        let Some(id) = args.get(1) else {
            eprintln!("--describe needs a calculator id, e.g. CalcDex --describe bmi");
            return 2;
        };
        let Some(calc) = calcs.iter().find(|c| c.id == id.as_str()) else {
            eprintln!("No calculator called '{id}'. Try: CalcDex --list");
            return 2;
        };
        println!("{} [{}]  id: {}", calc.name, calc.category, calc.id);
        println!("{}", calc.desc);
        println!("Fields (numbered from 0):");
        for (fi, (label, hint)) in describe_fields(calc).iter().enumerate() {
            println!("  {fi}. {label} — {hint}");
        }
        println!(
            "Example: CalcDex {} --example   (inputs: {})",
            calc.id,
            display_tokens(calc, calc.example).join(", ")
        );
        return 0;
    }
    if args[0] == "--batch" {
        // Batch calculation from CSV (item #31).
        let (Some(id), Some(input_path), Some(output_path)) =
            (args.get(1), args.get(2), args.get(3))
        else {
            eprintln!("Usage: CalcDex --batch <calculator-id> <input.csv> <output.csv>");
            return 2;
        };
        let Some(calc) = calcs.iter().find(|c| c.id == id.as_str()) else {
            eprintln!("No calculator called '{id}'. Try: CalcDex --list");
            return 2;
        };
        let text = match std::fs::read_to_string(input_path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Could not read {input_path}: {e}");
                return 1;
            }
        };
        return match batch_run(calc, &text) {
            Ok(outcome) => {
                if let Err(e) = std::fs::write(output_path, &outcome.output_csv) {
                    eprintln!("Could not write {output_path}: {e}");
                    return 1;
                }
                println!(
                    "{} rows: {} ok, {} errors — see {}",
                    outcome.ok + outcome.errors,
                    outcome.ok,
                    outcome.errors,
                    output_path
                );
                0
            }
            Err(e) => {
                eprintln!("Error: {e}");
                1
            }
        };
    }
    if args[0] == "--favourites" {
        // Favourites + recent from the SAME settings file the window
        // uses — it is the shared source of truth for both.
        let valid: BTreeSet<String> = calcs.iter().map(|c| c.id.to_string()).collect();
        let settings = load_settings(&valid);
        println!(
            "Settings file: {}",
            calcdex::settings::settings_path().display()
        );
        if settings.favourites.is_empty() {
            println!("No favourites yet. Add one: CalcDex --favourite <calculator-id>");
        } else {
            println!("Favourites ({}):", settings.favourites.len());
            for id in &settings.favourites {
                let name = calcs
                    .iter()
                    .find(|c| c.id == id.as_str())
                    .map(|c| c.name)
                    .unwrap_or("");
                println!("  {:<38} {}", id, name);
            }
        }
        if settings.recent.is_empty() {
            println!("No recent calculators yet — run any calculator and it is remembered here.");
        } else {
            println!("Recent (most recent first):");
            for id in &settings.recent {
                let name = calcs
                    .iter()
                    .find(|c| c.id == id.as_str())
                    .map(|c| c.name)
                    .unwrap_or("");
                println!("  {:<38} {}", id, name);
            }
        }
        println!(
            "Remembered inputs: {} calculator(s) have saved inputs in the settings file.",
            settings.remembered.len()
        );
        return 0;
    }
    if args[0] == "--favourite" || args[0] == "--unfavourite" {
        let add = args[0] == "--favourite";
        let Some(id) = args.get(1) else {
            eprintln!("Usage: CalcDex {} <calculator-id>", args[0]);
            return 2;
        };
        let Some(calc) = calcs.iter().find(|c| c.id == id.as_str()) else {
            eprintln!("No calculator called '{id}'. Try: CalcDex --list");
            return 2;
        };
        let valid: BTreeSet<String> = calcs.iter().map(|c| c.id.to_string()).collect();
        let mut settings = load_settings(&valid);
        if add {
            settings.favourites.insert(id.clone());
        } else {
            settings.favourites.remove(id);
        }
        return match save_settings_file(&settings) {
            Ok(()) => {
                println!(
                    "{} {} the favourites.",
                    calc.name,
                    if add { "added to" } else { "removed from" }
                );
                0
            }
            Err(e) => {
                eprintln!("Could not save settings: {e}");
                1
            }
        };
    }
    if args[0] == "--help" || args[0] == "help" || args[0] == "-h" {
        println!("CalcDex \u{2014} every kind of calculation, offline.\n\nNo arguments: opens the window.\n--list                       list all calculators\n--search <words>             find calculators (ranked, with synonyms: 'home loan' finds the mortgage calculator)\n--describe <id>              show a calculator's fields, hints and example\n--log                        print the calculation log (every successful run, window or command line, is saved there)\n--clear-log                  empty the calculation log\n--favourites                 list favourites + recent (from the shared settings file)\n--favourite <id>             add a calculator to the favourites in the settings file\n--unfavourite <id>           remove it again\n--batch <id> <in.csv> <out.csv>  run one calculator over a CSV file (header row names the fields; see README)\n--sig-figs <n>               also show the headline rounded to n significant figures (display only; may appear anywhere)\n<calculator-id> <numbers...> run one calculator; numbers in the order of its labelled inputs\n<calculator-id> --example    run the calculator's built-in teaching example\n<calculator-id> --vary <field> <start> <end> <step> <other numbers...>\n                             sweep one input (field numbered from 0, as in --describe);\n                             the other numbers are the remaining inputs in order\n                             (list calculators: one argument per value; pair calculators: values in pairs)\n\nDates: a date is entered as ONE argument in MM/DD/YYYY form \u{2014} for example\n  CalcDex age 04/23/1988 10/25/2026\nThe older three-number form still works: CalcDex age 1988 4 23 2026 10 25\n\nNumbers may use scientific notation (e.g. 6.022e23, 1.5e-3). Results show the formula, substituted values, and full precision.\n\nExample: CalcDex compound-interest 1000 5 10");
        return 0;
    }
    let id = &args[0];
    let Some(calc) = calcs.iter().find(|c| c.id == id.as_str()) else {
        eprintln!("No calculator called '{id}'. Try: CalcDex --list");
        return 2;
    };
    // <id> --example (item #11)
    if args.get(1).map(|s| s.as_str()) == Some("--example") {
        let nums = calc.example.to_vec();
        return match run_calc(id, &nums) {
            Ok(out) => {
                println!(
                    "Example — {} with inputs: {}",
                    calc.name,
                    display_tokens(calc, &nums).join(", ")
                );
                print_result(&out, sig_figs);
                let entry = make_entry(calc, &nums, &out);
                let _ = append_history(entry);
                cli_note_run(calc, &nums);
                0
            }
            Err(e) => {
                eprintln!("Error: {e}");
                1
            }
        };
    }
    // <id> --vary <field> <start> <end> <step> <other numbers...> (item #16)
    if args.get(1).map(|s| s.as_str()) == Some("--vary") {
        let spec: Vec<&String> = args.iter().skip(2).collect();
        if spec.len() < 4 {
            eprintln!("Usage: CalcDex {id} --vary <field> <start> <end> <step> <other numbers...>");
            return 2;
        }
        let parse = |s: &String| s.parse::<f64>();
        let (Ok(field), Ok(start), Ok(end), Ok(step)) = (
            spec[0].parse::<usize>(),
            parse(spec[1]),
            parse(spec[2]),
            parse(spec[3]),
        ) else {
            eprintln!("--vary needs: a field number, then start, end and step as numbers.");
            return 2;
        };
        let mut others: Vec<f64> = Vec::new();
        for s in &spec[4..] {
            match s.parse::<f64>() {
                Ok(v) => others.push(v),
                Err(_) => {
                    eprintln!("Error: '{}' is not a number.", s);
                    return 2;
                }
            }
        }
        if calc.list || others.len() + 1 != calc.inputs.len() {
            eprintln!(
                "Error: '{}' needs {} other number(s) around the varied field.",
                calc.name,
                calc.inputs.len().saturating_sub(1)
            );
            return 2;
        }
        let mut base = others.clone();
        if field > base.len() {
            eprintln!("Error: there is no field {field} on '{}'.", calc.name);
            return 2;
        }
        base.insert(field, 0.0);
        return match vary_run(calc, &base, field, start, end, step) {
            Ok(rows) => {
                println!("{}", csv_row(&["value".to_string(), "result".to_string()]));
                for r in &rows {
                    let cell = match &r.result {
                        Ok(out) => headline_of(out).to_string(),
                        Err(e) => format!("ERROR: {e}"),
                    };
                    println!("{}", csv_row(&[calcdex::n(r.value), cell]));
                }
                0
            }
            Err(e) => {
                eprintln!("Error: {e}");
                1
            }
        };
    }
    let nums = match expand_args(calc, &args[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}");
            return 2;
        }
    };
    match run_calc(id, &nums) {
        Ok(out) => {
            print_result(&out, sig_figs);
            // Every successful CLI run is saved to the same log the window uses.
            let entry = make_entry(calc, &nums, &out);
            let _ = append_history(entry);
            cli_note_run(calc, &nums);
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

/// Print a result, adding the sig-figs headline showing when the
/// global --sig-figs flag is on (item #1). The log always stores
/// the un-annotated full-precision text.
/// A successful CLI run updates the SAME settings file the window
/// uses: the calculator joins Recent and its inputs are remembered
/// (items #46/#48 — the settings file is the shared source of truth).
/// Best-effort: a settings write failure never fails the calculation.
fn cli_note_run(calc: &Calc, nums: &[f64]) {
    let valid: BTreeSet<String> = all_calcs().iter().map(|c| c.id.to_string()).collect();
    let mut settings = load_settings(&valid);
    settings.note_run(calc.id, display_tokens(calc, nums));
    let _ = save_settings_file(&settings);
}

fn print_result(out: &str, sig_figs: Option<u8>) {
    match sig_figs {
        Some(n) => println!("{}", annotate_sig_figs(out, n)),
        None => println!("{out}"),
    }
}

/// (label, hint) per user-visible field, for --describe: date groups
/// collapse to their merged label + the date hint.
fn describe_fields(calc: &Calc) -> Vec<(String, String)> {
    if calc.list {
        return vec![(
            calc.inputs
                .first()
                .copied()
                .unwrap_or("Numbers")
                .to_string(),
            hint_for(calc, 0),
        )];
    }
    let fields = display_fields(calc);
    let mut hints: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < calc.inputs.len() {
        if date_group_at(calc, i).is_some() {
            hints.push(hint_for(calc, i));
            i += 3;
        } else if calc
            .date_groups
            .iter()
            .any(|g| i > g.start && i < g.start + 3)
        {
            i += 1;
        } else {
            hints.push(hint_for(calc, i));
            i += 1;
        }
    }
    fields.into_iter().zip(hints).collect()
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
//     and the worked steps as a footer block;
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
    /// v0.3.0: persistent settings (favourites, recent, remembered
    /// inputs, sig-figs) — items #1/#46/#48.
    settings: Settings,
    /// Ctrl+K (or `/`) asked for the search box to take focus.
    focus_search: bool,
    /// Batch panel (item #31) replaces the centre when true.
    show_batch: bool,
    batch_in: String,
    batch_out: String,
    batch_summary: String,
    /// Vary-one-input state (item #16).
    vary_field: usize,
    vary_start: String,
    vary_end: String,
    vary_step: String,
    vary_rows: Vec<(f64, Result<String, String>)>,
    vary_error: String,
}

impl App {
    fn new() -> App {
        let calcs = all_calcs();
        let mut set: BTreeSet<String> = BTreeSet::new();
        for c in &calcs {
            set.insert(c.category.to_string());
        }
        // Corrupt/missing settings -> defaults inside load_settings;
        // startup can never be broken by the settings file.
        let settings = load_settings(&set);
        App {
            settings,
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
            focus_search: false,
            show_batch: false,
            batch_in: String::new(),
            batch_out: String::new(),
            batch_summary: String::new(),
            vary_field: 0,
            vary_start: String::new(),
            vary_end: String::new(),
            vary_step: String::new(),
            vary_rows: Vec::new(),
            vary_error: String::new(),
        }
    }

    fn persist_settings(&self) {
        let _ = save_settings_file(&self.settings);
    }

    fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
        self.inputs = vec![String::new(); self.calcs[idx].inputs.len()];
        self.list_input.clear();
        self.output.clear();
        self.vary_rows.clear();
        self.vary_error.clear();
        self.vary_field = 0;
        self.vary_start.clear();
        self.vary_end.clear();
        self.vary_step.clear();
        // One merged text field per date group.
        self.date_texts = vec![String::new(); self.calcs[idx].date_groups.len()];
        // Item #48: reopening a calculator restores the values last
        // used in it. Item #50: otherwise the fields open on the
        // smart defaults — visible and editable, never hidden.
        let toks: Vec<String> = match self.settings.remembered.get(self.calcs[idx].id) {
            Some(t) => t.clone(),
            None => display_tokens(&self.calcs[idx], defaults_for(&self.calcs[idx])),
        };
        self.fill_from_tokens(&toks);
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

    /// Fill the form from display tokens (one per user-visible
    /// field; dates as MM/DD/YYYY). Shared by select (#48/#50),
    /// log restore and TRY AN EXAMPLE (#11).
    fn fill_from_tokens(&mut self, toks: &[String]) {
        let Some(idx) = self.selected else { return };
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
                        self.date_texts[gi] = t.clone();
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
                    self.inputs[i] = t.clone();
                }
                ti += 1;
            }
        }
    }

    /// TRY AN EXAMPLE (item #11): fill the built-in teaching example
    /// and run it immediately — a complete worked example in 1 click.
    fn run_example(&mut self) {
        let Some(idx) = self.selected else { return };
        let toks = display_tokens(&self.calcs[idx], self.calcs[idx].example);
        self.fill_from_tokens(&toks);
        self.calculate();
    }

    /// Reset the form to the smart defaults (items #48/#50).
    fn reset_defaults(&mut self) {
        let Some(idx) = self.selected else { return };
        let toks = display_tokens(&self.calcs[idx], defaults_for(&self.calcs[idx]));
        self.fill_from_tokens(&toks);
        self.output.clear();
        // Re-apply the today prefills, as on select.
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
        let toks: Vec<String> = if entry.restore.is_empty() {
            Vec::new()
        } else {
            entry.restore.split(";;").map(|t| t.to_string()).collect()
        };
        self.fill_from_tokens(&toks);
    }

    fn visible(&self, c: &Calc) -> bool {
        if self.category == "Favourites" {
            if !self.settings.favourites.contains(c.id) {
                return false;
            }
        } else if self.category != "All" && c.category != self.category {
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

    /// Parse the form into the registry's numeric vector. Error
    /// strings are exactly the ones the GUI has always shown.
    fn parse_inputs(&mut self) -> Result<Vec<f64>, String> {
        let Some(idx) = self.selected else {
            return Err("No calculator selected.".into());
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
                        return Err(format!("'{tok}' is not a number."));
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
                            return Err(format!("Error in {}: {e}", g.label));
                        }
                    }
                    gi += 1;
                    i += 3;
                } else {
                    let raw = &self.inputs[i];
                    match raw.trim().parse::<f64>() {
                        Ok(v) => nums.push(v),
                        Err(_) => {
                            return Err(format!(
                                "'{}' needs a number.",
                                calc.inputs.get(i).copied().unwrap_or("Input")
                            ));
                        }
                    }
                    i += 1;
                }
            }
        }
        Ok(nums)
    }

    fn calculate(&mut self) {
        let Some(idx) = self.selected else {
            return;
        };
        let nums = match self.parse_inputs() {
            Ok(v) => v,
            Err(e) => {
                self.output = e;
                return;
            }
        };
        let calc = &self.calcs[idx];
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
                // Items #46/#48: remember this calculator's inputs and
                // its place in the Recent list.
                let toks = display_tokens(calc, &nums);
                self.settings.note_run(calc.id, toks);
                self.persist_settings();
                self.output = out;
            }
            Err(e) => {
                self.output = format!("Error: {e}");
            }
        }
    }

    /// VARY (item #16): sweep one input across start/end/step using
    /// the values currently in the form for the other inputs.
    fn do_vary(&mut self) {
        self.vary_rows.clear();
        self.vary_error.clear();
        let Some(idx) = self.selected else { return };
        let (Ok(start), Ok(end), Ok(step)) = (
            self.vary_start.trim().parse::<f64>(),
            self.vary_end.trim().parse::<f64>(),
            self.vary_step.trim().parse::<f64>(),
        ) else {
            self.vary_error =
                "VARY needs start, end and step as numbers — e.g. start 0, end 10, step 2."
                    .to_string();
            return;
        };
        let base = match self.parse_inputs() {
            Ok(v) => v,
            Err(e) => {
                self.vary_error = e;
                return;
            }
        };
        let calc = &self.calcs[idx];
        match vary_run(calc, &base, self.vary_field, start, end, step) {
            Ok(rows) => {
                self.vary_rows = rows.into_iter().map(|r| (r.value, r.result)).collect();
            }
            Err(e) => {
                self.vary_error = e;
            }
        }
    }

    /// BATCH (item #31): run the selected calculator over the CSV
    /// file at batch_in and write the results file at batch_out.
    fn do_batch(&mut self) {
        let Some(idx) = self.selected else {
            self.batch_summary = "Pick a calculator first (mode rail + chip strip).".to_string();
            return;
        };
        let calc = &self.calcs[idx];
        let text = match std::fs::read_to_string(self.batch_in.trim()) {
            Ok(t) => t,
            Err(e) => {
                self.batch_summary = format!("Could not read {}: {e}", self.batch_in.trim());
                return;
            }
        };
        match batch_run(calc, &text) {
            Ok(outcome) => match std::fs::write(self.batch_out.trim(), &outcome.output_csv) {
                Ok(()) => {
                    self.batch_summary = format!(
                        "{} rows: {} ok, {} errors — see output file",
                        outcome.ok + outcome.errors,
                        outcome.ok,
                        outcome.errors
                    );
                }
                Err(e) => {
                    self.batch_summary = format!("Could not write {}: {e}", self.batch_out.trim());
                }
            },
            Err(e) => {
                self.batch_summary = format!("Error: {e}");
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
/// Raised fill for chips and raised controls (#181818).
const RAISED: egui::Color32 = egui::Color32::from_rgb(0x18, 0x18, 0x18);
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
    if tip_btn(
        ui,
        egui::RichText::new("CLEAR LOG").color(RED),
        "Empty the calculation log (the saved history file is cleared too)",
    )
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
                        if tip_btn(
                            ui,
                            egui::RichText::new("RE-OPEN"),
                            "Open this entry's calculator with its inputs restored",
                        )
                        .clicked()
                        {
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

// ---------- Tooltip rule (SEAN standing rule, applied v0.3.0) ----------
//
// EVERY interactive control in the window carries a tooltip, no
// exceptions. The enforceable pattern: buttons and selectable
// labels are ONLY built through the helpers below, each of which
// REQUIRES a tooltip argument — a call site cannot forget it.
// Text edits and other widgets get `.on_hover_text(..)` on their
// response at the call site. tests/tooltips.rs scans this file and
// fails if a bare `ui.button(` / `ui.selectable_label(` /
// `egui::Button::new` appears outside these helpers.

/// A plain button with a mandatory tooltip.
fn tip_btn(ui: &mut egui::Ui, text: egui::RichText, tooltip: &str) -> egui::Response {
    ui.add(egui::Button::new(text)).on_hover_text(tooltip)
}

/// A fixed-size button with a mandatory tooltip.
fn tip_btn_sized(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    text: egui::RichText,
    tooltip: &str,
) -> egui::Response {
    ui.add_sized(size, egui::Button::new(text))
        .on_hover_text(tooltip)
}

/// A selectable label (toggle) with a mandatory tooltip.
fn tip_sel(
    ui: &mut egui::Ui,
    selected: bool,
    text: egui::RichText,
    tooltip: &str,
) -> egui::Response {
    ui.selectable_label(selected, text).on_hover_text(tooltip)
}

/// A selectable value (combo-box option) with a mandatory tooltip.
fn tip_sel_value(
    ui: &mut egui::Ui,
    current: &mut usize,
    value: usize,
    label: &str,
    tooltip: &str,
) -> egui::Response {
    ui.selectable_value(current, value, label)
        .on_hover_text(tooltip)
}

/// A calculator chip in the centre strip (custom fill/stroke), with
/// a mandatory tooltip.
fn tip_chip(ui: &mut egui::Ui, name: &str, selected: bool) -> egui::Response {
    let btn = egui::Button::new(
        egui::RichText::new(name)
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
    ui.add(btn).on_hover_text(format!("Open {name}"))
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
        // Item #47: Ctrl+K focuses search from anywhere; `/` does
        // too when no field has focus (so it never fights typing).
        // Item #46: Alt+F opens the Favourites mode.
        ctx.input(|i| {
            if i.modifiers.ctrl && i.key_pressed(egui::Key::K) {
                self.focus_search = true;
            }
            if i.modifiers.alt && i.key_pressed(egui::Key::F) {
                self.category = "Favourites".to_string();
            }
            if i.key_pressed(egui::Key::Slash) && !i.modifiers.ctrl && !i.modifiers.alt {
                self.focus_search = true;
            }
        });
        if self.focus_search {
            // If a text field is actually being edited, `/` belongs
            // to it — egui gives the key to the focused widget first;
            // requesting focus here only moves focus to search, which
            // is Ctrl+K's job. For `/`, only steal when nothing is
            // focused:
            let any_focused = ctx.memory(|m| m.focused().is_some());
            if any_focused && !ctx.input(|i| i.modifiers.ctrl) {
                self.focus_search = false;
            }
        }
        // Enter = Calculate (single-line fields; the list box keeps
        // Enter for separating values).
        let enter_calc = ctx.input(|i| i.key_pressed(egui::Key::Enter))
            && self
                .selected
                .map(|idx| !self.calcs[idx].list)
                .unwrap_or(false);

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
                if tip_sel(
                    ui,
                    self.show_batch,
                    egui::RichText::new("BATCH").color(GREEN).size(12.0),
                    "Run one calculator over a CSV file — many rows in one go",
                )
                .clicked()
                {
                    self.show_batch = !self.show_batch;
                }
                ui.separator();
                ui.label(egui::RichText::new("SIG FIGS:").color(HINT).size(11.5));
                let mut sf_pick: Option<Option<u8>> = None;
                if tip_sel(
                    ui,
                    self.settings.sig_figs.is_none(),
                    egui::RichText::new("Off").size(12.0),
                    "Show full precision (the stored result is never rounded)",
                )
                .clicked()
                {
                    sf_pick = Some(None);
                }
                for n in [2u8, 3, 4, 5, 6] {
                    if tip_sel(
                        ui,
                        self.settings.sig_figs == Some(n),
                        egui::RichText::new(format!("{n}")).size(12.0),
                        "Also show the headline rounded to this many significant figures",
                    )
                    .clicked()
                    {
                        sf_pick = Some(Some(n));
                    }
                }
                if let Some(pick) = sf_pick {
                    self.settings.sig_figs = pick;
                    self.persist_settings();
                }
                ui.separator();
                let log_label = format!("LOG ({})", self.log_entries.len());
                if tip_sel(
                    ui,
                    self.show_log,
                    egui::RichText::new(log_label).color(GREEN).size(12.0),
                    "Show or hide the history rail — every saved calculation, newest first",
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
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("filter calculators... (Ctrl+K)")
                            .desired_width(search_w),
                    );
                    let resp = resp.on_hover_text(
                        "Type to filter calculators by name — synonyms work too ('home loan' finds the mortgage calculator). Ctrl+K focuses this box.",
                    );
                    if self.focus_search {
                        resp.request_focus();
                        self.focus_search = false;
                    }
                    if tip_btn(
                        ui,
                        egui::RichText::new("Clear"),
                        "Clear the search text",
                    )
                    .clicked()
                    {
                        self.search.clear();
                    }
                });
                ui.add_space(8.0);
                ui.add_space(8.0);
                let fav_n = self.settings.favourites.len();
                if tip_sel(
                    ui,
                    self.category == "Favourites",
                    egui::RichText::new(format!("\u{2605} FAVOURITES ({fav_n})"))
                        .color(if self.category == "Favourites" {
                            GREEN
                        } else {
                            TEXT
                        })
                        .size(12.5),
                    "Your starred calculators plus the recent list (Alt+F)",
                )
                .clicked()
                {
                    self.category = "Favourites".to_string();
                }
                ui.add_space(4.0);
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
            if self.show_batch {
                self.batch_ui(ui);
                return;
            }
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
    /// display, form, Calculate, steps footer, VARY section.
    fn centre_ui(&mut self, ui: &mut egui::Ui, enter_calc: bool) {
        // ---- Calculator strip for the active mode ----
        ui.label(
            egui::RichText::new(format!("CALCULATORS — {}", self.category.to_uppercase()))
                .color(GREEN)
                .strong()
                .size(12.0),
        );
        // Recent strip (item #46): in Favourites mode the last 8
        // calculators actually run sit above the strip as chips.
        if self.category == "Favourites" && !self.settings.recent.is_empty() {
            ui.label(egui::RichText::new("RECENT").color(HINT).size(11.5));
            let mut recent_pick: Option<usize> = None;
            ui.horizontal_wrapped(|ui| {
                for id in self.settings.recent.clone() {
                    if let Some(i) = self.calcs.iter().position(|c| c.id == id) {
                        if tip_btn(
                            ui,
                            egui::RichText::new(self.calcs[i].name)
                                .color(TEXT)
                                .size(12.0),
                            "Recently used — click to open it",
                        )
                        .clicked()
                        {
                            recent_pick = Some(i);
                        }
                    }
                }
            });
            if let Some(i) = recent_pick {
                self.select(i);
            }
            ui.add_space(4.0);
        }
        let mut to_select: Option<usize> = None;
        egui::ScrollArea::vertical()
            .id_salt("strip-scroll")
            .max_height(112.0)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut any = false;
                    // Item #47: while searching, chips are ranked
                    // (exact name > synonym > description).
                    let order: Vec<usize> = if self.search.trim().is_empty() {
                        (0..self.calcs.len()).collect()
                    } else {
                        search_ranked(&self.calcs, &self.search)
                    };
                    for i in order {
                        let c = &self.calcs[i];
                        if !self.visible(c) {
                            continue;
                        }
                        any = true;
                        let selected = self.selected == Some(i);
                        let resp = tip_chip(ui, c.name, selected);
                        if resp.clicked() {
                            to_select = Some(i);
                        }
                    }
                    if !any {
                        ui.label(
                            egui::RichText::new(if self.category == "Favourites" {
                                "No favourites yet — open a calculator and press its \u{2606} FAVOURITE button. Your recent calculators are listed above."
                            } else {
                                "No calculators match — clear the search or pick another mode."
                            })
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
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(name.to_uppercase())
                    .color(GREEN)
                    .strong()
                    .size(16.0),
            );
            let is_fav = self.settings.favourites.contains(id);
            let star = if is_fav {
                "\u{2605} FAVOURITE"
            } else {
                "\u{2606} FAVOURITE"
            };
            if tip_btn(
                ui,
                egui::RichText::new(star).color(if is_fav { GREEN } else { TEXT }),
                "Star this calculator — it joins the FAVOURITES mode at the top of the rail (click again to unstar)",
            )
            .clicked()
            {
                self.settings.toggle_favourite(id);
                self.persist_settings();
            }
        });
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
                            "Enter the values below, then press CALCULATE (or Enter).",
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
                    if let Some(n) = self.settings.sig_figs {
                        ui.label(
                            egui::RichText::new(format!(
                                "\u{2248} to {n} significant figures: {}",
                                sig_figs_headline(&self.output, n)
                            ))
                            .color(TEXT)
                            .size(15.0),
                        );
                    }
                }
            });
        ui.add_space(8.0);

        // ---- Form ----
        // Input rows: the label column takes 38% of the pane width
        // (clamped 130-300 pt) and the field takes what is left
        // (clamped 150-480 pt), recomputed every frame so resizing or
        // zooming reflows instead of clipping. Every centre block —
        // display, form, steps, VARY — spans the full centre width,
        // so the column reads evenly at any size.
        let avail = ui.available_width();
        let label_w = (avail * 0.38).clamp(130.0, 300.0);
        let field_w = (avail - label_w - 16.0).clamp(150.0, 480.0);
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
            let _ = resp.on_hover_text("Type the numbers here, separated by commas or spaces");
            ui.label(
                egui::RichText::new(hint_for(&self.calcs[idx], 0))
                    .color(HINT)
                    .size(11.5),
            );
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
                        let _ = resp.on_hover_text(
                            "Type the date as MM/DD/YYYY (the Today? button fills in today)",
                        );
                        if tip_btn(
                            ui,
                            egui::RichText::new("Today?").color(GREEN),
                            "Fill in today's date",
                        )
                        .clicked()
                        {
                            self.date_texts[gi] = today_str();
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
                        let _ =
                            resp.on_hover_text(format!("Type a value for {}", inputs_labels[i]));
                    });
                    ui.horizontal(|ui| {
                        ui.add_sized([label_w, 20.0], egui::Label::new(""));
                        ui.label(
                            egui::RichText::new(hint_for(&self.calcs[idx], i))
                                .color(HINT)
                                .size(11.5),
                        );
                    });
                    i += 1;
                }
            }
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if tip_btn_sized(
                ui,
                egui::vec2(180.0, 36.0),
                egui::RichText::new("CALCULATE").color(GREEN).strong(),
                "Run the calculation with the values above (or press Enter)",
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
            if tip_btn(
                ui,
                egui::RichText::new("TRY AN EXAMPLE").color(TEXT),
                "Fill this calculator's built-in teaching example and run it at once",
            )
            .clicked()
            {
                self.run_example();
            }
            if tip_btn(
                ui,
                egui::RichText::new("RESET").color(TEXT),
                "Put the smart default values back in every field",
            )
            .clicked()
            {
                self.reset_defaults();
            }
        });

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
            let footer = match self.settings.sig_figs {
                Some(n) => annotate_sig_figs(&self.output, n),
                None => self.output.clone(),
            };
            ui.label(egui::RichText::new(footer).color(TEXT).size(13.0));
        }
        self.vary_ui(ui);
    }

    /// VARY ONE INPUT (item #16): sweep one numeric input across a
    /// range with the form's other values fixed; the table lands
    /// under the steps footer. Fixed-input calculators only.
    fn vary_ui(&mut self, ui: &mut egui::Ui) {
        let Some(idx) = self.selected else { return };
        if self.calcs[idx].list {
            return;
        }
        let labels: Vec<&str> = self.calcs[idx].inputs.to_vec();
        if self.vary_field >= labels.len() {
            self.vary_field = 0;
        }
        ui.add_space(8.0);
        ui.separator();
        ui.label(
            egui::RichText::new("VARY ONE INPUT")
                .color(GREEN)
                .strong()
                .size(12.0),
        );
        ui.label(
            egui::RichText::new(
                "Hold the other inputs as they are and run this calculator across a range of one input (at most 200 rows).",
            )
            .color(HINT)
            .size(11.5),
        );
        let mut run = false;
        ui.horizontal_wrapped(|ui| {
            let combo = egui::ComboBox::from_label("Input")
                .selected_text(labels[self.vary_field])
                .show_ui(ui, |ui| {
                    for (fi, label) in labels.iter().enumerate() {
                        tip_sel_value(
                            ui,
                            &mut self.vary_field,
                            fi,
                            label,
                            "Sweep this input across the range",
                        );
                    }
                });
            combo
                .response
                .on_hover_text("Pick which input the sweep changes");
            ui.add(
                egui::TextEdit::singleline(&mut self.vary_start)
                    .hint_text("start")
                    .desired_width(70.0),
            )
            .on_hover_text("The first value of the sweep");
            ui.add(
                egui::TextEdit::singleline(&mut self.vary_end)
                    .hint_text("end")
                    .desired_width(70.0),
            )
            .on_hover_text("The last value of the sweep");
            ui.add(
                egui::TextEdit::singleline(&mut self.vary_step)
                    .hint_text("step")
                    .desired_width(70.0),
            )
            .on_hover_text("How much the swept input grows by each row");
            if tip_btn(
                ui,
                egui::RichText::new("RUN VARY").color(GREEN),
                "Run the sweep and show the table below",
            )
            .clicked()
            {
                run = true;
            }
        });
        if run {
            self.do_vary();
        }
        if !self.vary_error.is_empty() {
            ui.label(egui::RichText::new(&self.vary_error).color(RED).size(12.5));
        }
        if !self.vary_rows.is_empty() {
            let field_label = labels[self.vary_field];
            egui::ScrollArea::vertical()
                .id_salt("vary-scroll")
                .max_height(180.0)
                .show(ui, |ui| {
                    for (value, result) in &self.vary_rows {
                        let cell = match result {
                            Ok(out) => headline_of(out).to_string(),
                            Err(e) => format!("ERROR: {e}"),
                        };
                        ui.label(
                            egui::RichText::new(format!(
                                "{field_label} = {}  ->  {cell}",
                                calcdex::n(*value)
                            ))
                            .color(TEXT)
                            .size(12.5),
                        );
                    }
                });
        }
    }

    /// BATCH FROM CSV (item #31): path fields + RUN BATCH + summary.
    /// egui has no native file-picker dialogs, so paths are typed —
    /// stated honestly here and in the README.
    fn batch_ui(&mut self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new("BATCH FROM CSV")
                .color(GREEN)
                .strong()
                .size(16.0),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(
                "Run ONE calculator over many rows of a CSV file. The file's header row must name the calculator's fields in order; every row produces its inputs plus the result (or an ERROR note) in the output file. Paths are typed — this toolkit has no file-picker dialog.",
            )
            .color(TEXT),
        );
        ui.add_space(8.0);
        match self.selected {
            Some(idx) => {
                let c = &self.calcs[idx];
                ui.label(
                    egui::RichText::new(format!("Calculator: {} ({})", c.name, c.id))
                        .color(GREEN)
                        .size(13.0),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "Expected header: {}",
                        display_fields(c).join(", ")
                    ))
                    .color(HINT)
                    .size(11.5),
                );
            }
            None => {
                ui.label(
                    egui::RichText::new(
                        "Pick a calculator first — close BATCH, pick one from the chip strip, then open BATCH again.",
                    )
                    .color(TEXT),
                );
            }
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_sized(
                [130.0, 20.0],
                egui::Label::new(egui::RichText::new("INPUT CSV").color(TEXT).size(12.0)),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.batch_in)
                    .hint_text("e.g. C:\\Users\\you\\rates.csv")
                    .desired_width(380.0),
            )
            .on_hover_text("Type the full path of the input CSV file");
        });
        ui.horizontal(|ui| {
            ui.add_sized(
                [130.0, 20.0],
                egui::Label::new(egui::RichText::new("OUTPUT CSV").color(TEXT).size(12.0)),
            );
            ui.add(
                egui::TextEdit::singleline(&mut self.batch_out)
                    .hint_text("e.g. C:\\Users\\you\\results.csv")
                    .desired_width(380.0),
            )
            .on_hover_text("Type the full path the output CSV should be written to");
        });
        ui.add_space(8.0);
        let mut run = false;
        if tip_btn_sized(
            ui,
            egui::vec2(180.0, 36.0),
            egui::RichText::new("RUN BATCH").color(GREEN).strong(),
            "Run the batch and write the output CSV",
        )
        .clicked()
        {
            run = true;
        }
        if run {
            self.do_batch();
        }
        if !self.batch_summary.is_empty() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(&self.batch_summary)
                    .color(TEXT)
                    .size(13.0),
            );
        }
    }
}
