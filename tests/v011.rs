//! v0.1.1 known-answer tests: date strings, CLI date expansion,
//! calculation log, and the renamed "end date" wording.
use calcdex::{
    append_entry_to, civil_from_days, days_from_civil, expand_args, find_calc, format_date,
    load_history_from, make_entry, parse_date_str, run_calc, save_history_to, LogEntry,
};

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn date_parse_format_round_trip() {
    for (text, ymd) in [
        ("04/23/1988", (1988, 4, 23)),
        ("4/23/1988", (1988, 4, 23)),  // no leading zeros accepted
        ("02/29/2024", (2024, 2, 29)), // leap year
        ("12/31/1999", (1999, 12, 31)),
        ("01/01/2000", (2000, 1, 1)),
    ] {
        let parsed = parse_date_str(text).unwrap_or_else(|e| panic!("{text}: {e}"));
        assert_eq!(parsed, ymd, "{text}");
        assert_eq!(
            format_date(ymd.0, ymd.1, ymd.2),
            format!("{:02}/{:02}/{:04}", ymd.1, ymd.2, ymd.0)
        );
        // and back again
        assert_eq!(
            parse_date_str(&format_date(ymd.0, ymd.1, ymd.2)).unwrap(),
            ymd
        );
    }
}

#[test]
fn date_parse_teaching_errors() {
    let bad_month = parse_date_str("13/01/2024").unwrap_err();
    assert!(bad_month.contains("month must be 1 to 12"), "{bad_month}");
    let bad_day = parse_date_str("04/40/1988").unwrap_err();
    assert!(bad_day.contains("day must be 1 to 31"), "{bad_day}");
    let feb29 = parse_date_str("02/29/2025").unwrap_err(); // 2025 is not a leap year
    assert!(feb29.contains("not a real date"), "{feb29}");
    assert!(parse_date_str("02/29/2024").is_ok());
    let wrong_shape = parse_date_str("1988-04-23").unwrap_err();
    assert!(wrong_shape.contains("MM/DD/YYYY"), "{wrong_shape}");
    let words = parse_date_str("April 23 1988").unwrap_err();
    assert!(words.contains("MM/DD/YYYY"), "{words}");
}

#[test]
fn civil_from_days_is_the_inverse() {
    for (y, m, d) in [
        (1970, 1, 1),
        (1969, 12, 31),
        (1988, 4, 23),
        (2024, 2, 29),
        (2000, 2, 29),
        (1900, 3, 1), // 1900 was not a leap year
        (2026, 10, 25),
        (1, 1, 1),
        (9999, 12, 31),
    ] {
        assert_eq!(
            civil_from_days(days_from_civil(y, m, d)),
            (y, m, d),
            "{y}-{m}-{d}"
        );
    }
    // A solid sweep of days round-trips too.
    let mut z = -100_000;
    while z < 100_000 {
        let (y, m, d) = civil_from_days(z);
        assert_eq!(days_from_civil(y, m, d), z, "day {z}");
        z += 977;
    }
}

#[test]
fn cli_date_args_equal_old_number_form() {
    let age = find_calc("age").unwrap();
    let via_dates = expand_args(&age, &s(&["04/23/1988", "10/25/2026"])).unwrap();
    let via_numbers = expand_args(&age, &s(&["1988", "4", "23", "2026", "10", "25"])).unwrap();
    assert_eq!(via_dates, via_numbers);
    assert_eq!(via_dates, vec![1988.0, 4.0, 23.0, 2026.0, 10.0, 25.0]);
    let out = run_calc("age", &via_dates).unwrap();
    // SEAN's field-test values, independently verified.
    assert!(out.contains("Age: 38 years"), "{out}");
    assert!(out.contains("Total days: 14064"), "{out}");
    assert!(out.contains("2009 weeks"), "{out}");
    assert!(out.contains("end year"), "{out}");
    assert!(!out.contains("'on'"), "{out}");

    // Mixed form: one date arg + one old-form triple, and a date calc
    // with a trailing plain number (date-plus-days).
    let dpd = find_calc("date-plus-days").unwrap();
    assert_eq!(
        expand_args(&dpd, &s(&["01/31/2024", "1"])).unwrap(),
        vec![2024.0, 1.0, 31.0, 1.0]
    );
    // Bad dates give the teaching error, not a panic.
    assert!(expand_args(&age, &s(&["02/29/2025", "10/25/2026"])).is_err());
    // Non-date calculators still expand exactly as before.
    let ci = find_calc("compound-interest").unwrap();
    assert_eq!(
        expand_args(&ci, &s(&["1000", "5", "10"])).unwrap(),
        vec![1000.0, 5.0, 10.0]
    );
}

#[test]
fn date_calculators_carry_date_group_metadata() {
    for (id, labels) in [
        ("age", vec!["Birth Date", "End Date"]),
        ("days-between-dates", vec!["Start Date", "End Date"]),
        ("date-plus-days", vec!["Start Date"]),
        ("weekday", vec!["Date"]),
        ("countdown-days", vec!["From Date", "Target Date"]),
        ("business-days", vec!["Start Date", "End Date"]),
        ("due-date", vec!["First Day of Last Period"]),
        ("ovulation", vec!["First Day of Last Period"]),
        ("age-in-days", vec!["Birth Date", "End Date"]),
    ] {
        let c = find_calc(id).unwrap_or_else(|| panic!("missing {id}"));
        let got: Vec<&str> = c.date_groups.iter().map(|g| g.label).collect();
        assert_eq!(got, labels, "{id}");
        for g in c.date_groups {
            assert!(g.start + 2 < c.inputs.len(), "{id} group out of range");
        }
    }
}

#[test]
fn log_append_read_cap_and_restore_round_trip() {
    let dir = std::env::temp_dir().join(format!("calcdex-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("history.txt");

    let age = find_calc("age").unwrap();
    let nums = vec![1988.0, 4.0, 23.0, 2026.0, 10.0, 25.0];
    let result = run_calc("age", &nums).unwrap();
    let entry = make_entry(&age, &nums, &result);
    // Display + restore forms collapse date triples to MM/DD/YYYY.
    assert_eq!(entry.inputs, "04/23/1988, 10/25/2026");
    assert_eq!(entry.restore, "04/23/1988;;10/25/2026");

    let entries = append_entry_to(&path, entry.clone()).unwrap();
    assert_eq!(entries.len(), 1);
    let read_back = load_history_from(&path);
    assert_eq!(read_back, vec![entry.clone()]);
    // Restore tokens expand back to the exact same numbers.
    let toks: Vec<String> = entry.restore.split(";;").map(|t| t.to_string()).collect();
    assert_eq!(expand_args(&age, &toks).unwrap(), nums);

    // Tricky characters survive a round trip (pipes, newlines, backslashes).
    let tricky = LogEntry {
        timestamp: "2026-10-25 21:14".into(),
        id: "percent-of".into(),
        name: "Percent | Of".into(),
        inputs: "25, 200".into(),
        result: "line one\nline two \\ with | pipes".into(),
        restore: "25;;200".into(),
    };
    append_entry_to(&path, tricky.clone()).unwrap();
    let read_back = load_history_from(&path);
    assert_eq!(read_back.len(), 2);
    assert_eq!(read_back[1], tricky);

    // Cap: only the most recent 500 entries are kept.
    let big = dir.join("big-history.txt");
    let mut many: Vec<LogEntry> = Vec::new();
    for i in 0..510 {
        many.push(LogEntry {
            timestamp: format!("2026-01-01 00:{:02}", i % 60),
            id: "percent-of".into(),
            name: format!("Entry {i}"),
            inputs: "1, 2".into(),
            result: "ok".into(),
            restore: "1;;2".into(),
        });
    }
    save_history_to(&big, &many[..500]).unwrap();
    let mut last = many[500].clone();
    for extra in many.iter().skip(501) {
        append_entry_to(&big, extra.clone()).unwrap();
        last = extra.clone();
    }
    let capped = load_history_from(&big);
    assert_eq!(capped.len(), 500);
    assert_eq!(capped.last().unwrap(), &last);
    assert_ne!(
        capped.first().unwrap().name,
        "Entry 0",
        "oldest entries must be dropped"
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn date_outputs_use_end_date_wording() {
    let out = run_calc("age-in-days", &[1988.0, 4.0, 23.0, 2026.0, 10.0, 25.0]).unwrap();
    assert!(out.contains("14064 days"), "{out}");
    let err = run_calc("age", &[2026.0, 1.0, 1.0, 1988.0, 4.0, 23.0]).unwrap_err();
    assert!(err.contains("end date is before the birth date"), "{err}");
    // Due date: LMP 04/23/2026 + 280 days = 2027-01-28.
    let due = run_calc("due-date", &[2026.0, 4.0, 23.0]).unwrap();
    assert!(due.contains("2027-01-28"), "{due}");
}
