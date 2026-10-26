//! Date/Time & Everyday life + Other calculators (Omni: Everyday life,
//! Other, Ecology partly here). Dates use real calendar maths
//! (days-from-civil in lib.rs).
use crate::{
    a, chk, civil_from_days, days_from_civil, dg, m, n, nd, nz, valid_date, Calc, DateGroup,
};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

static START_END_DATES: &[DateGroup] = &[dg(0, "Start Date"), dg(3, "End Date")];
static START_DATE_ONLY: &[DateGroup] = &[dg(0, "Start Date")];
static BIRTH_END_DATES: &[DateGroup] = &[dg(0, "Birth Date"), dg(3, "End Date")];
static DATE_ONLY: &[DateGroup] = &[dg(0, "Date")];
static FROM_TARGET_DATES: &[DateGroup] = &[dg(0, "From Date"), dg(3, "Target Date")];

pub fn register(v: &mut Vec<Calc>) {
    reg(v, Calc::new("days-between-dates", "Days Between Two Dates", "Date & Time",
        "Real calendar count of days from the start date to the end date (leap years handled). Enter each date as one MM/DD/YYYY date.", &["Start year", "Start month", "Start day", "End year", "End month", "End day"],
        |x| { let (y1,m1,d1)=valid_date(a(x,0),a(x,1),a(x,2))?; let (y2,m2,d2)=valid_date(a(x,3),a(x,4),a(x,5))?; let diff=days_from_civil(y2,m2,d2)-days_from_civil(y1,m1,d1); Ok(format!("Days between: {} ({} weeks and {} days). {} direction.", diff.abs(), diff.abs()/7, diff.abs()%7, if diff>=0 {"the end date is after the start date"} else {"the end date is before the start date"})) }).with_dates(START_END_DATES));
    reg(
        v,
        Calc::new(
            "date-plus-days",
            "Date Plus / Minus Days",
            "Date & Time",
            "Adds N days to a date (use negative N to go backwards). Enter the start date as one MM/DD/YYYY date. Real calendar maths.",
            &["Year", "Month", "Day", "Days to add (can be negative)"],
            |x| {
                let (y, mo, d) = valid_date(a(x, 0), a(x, 1), a(x, 2))?;
                let (yy, mm, dd) = civil_from_days(days_from_civil(y, mo, d) + a(x, 3) as i64);
                Ok(format!("Result date: {yy}-{mm:02}-{dd:02}."))
            },
        )
        .with_dates(START_DATE_ONLY),
    );
    reg(v, Calc::new("age", "Age Calculator", "Date & Time",
        "Age from a birth date to an end date. Enter each date as one MM/DD/YYYY date (the End Date starts filled with today's date \u{2014} press Today? beside it to refill it). Also total days lived.", &["Birth year", "Birth month", "Birth day", "End year", "End month", "End day"],
        |x| { let (by,bm,bd)=valid_date(a(x,0),a(x,1),a(x,2))?; let (oy,om,od)=valid_date(a(x,3),a(x,4),a(x,5))?; let days=days_from_civil(oy,om,od)-days_from_civil(by,bm,bd); chk(days>=0,"The end date is before the birth date.")?; let mut years=oy-by; if (om,od)<(bm,bd) { years-=1; } Ok(format!("Age: {} years (birthday {} reached in the end year). Total days: {}. {} weeks approx.", years, if (om,od)>=(bm,bd) {"already"} else {"not yet"}, days, days/7)) }).with_dates(BIRTH_END_DATES));
    reg(
        v,
        Calc::new(
            "weekday",
            "Day of the Week for a Date",
            "Date & Time",
            "Uses the day count from 1970-01-01 (a Thursday) to name the weekday. Enter the date as one MM/DD/YYYY date.",
            &["Year", "Month", "Day"],
            |x| {
                let (y, mo, d) = valid_date(a(x, 0), a(x, 1), a(x, 2))?;
                let days = days_from_civil(y, mo, d);
                let names = [
                    "Thursday",
                    "Friday",
                    "Saturday",
                    "Sunday",
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                ];
                let idx = ((days % 7) + 7) % 7;
                Ok(format!("{y}-{mo:02}-{d:02} is a {}.", names[idx as usize]))
            },
        )
        .with_dates(DATE_ONLY),
    );
    reg(v, Calc::new("time-duration", "Time Duration Between Clock Times", "Date & Time",
        "Minutes between two same-day clock times (24h). Wraps past midnight if end < start (adds 24h).", &["Start hour (0-23)", "Start minute", "End hour (0-23)", "End minute"],
        |x| { let s=a(x,0)*60.0+a(x,1); let mut e=a(x,2)*60.0+a(x,3); if e<s { e+=1440.0; } let d=e-s; Ok(format!("Duration: {} minutes = {} hours {} minutes ({} h in decimal).", n(d), (d/60.0) as i64, (d%60.0) as i64, nd(d/60.0,4))) }));
    reg(
        v,
        Calc::new(
            "countdown-days",
            "Countdown to a Date (Days From a Date)",
            "Date & Time",
            "Days from a from date to a target date. Enter each date as one MM/DD/YYYY date (the From Date starts filled with today's date).",
            &[
                "From year",
                "From month",
                "From day",
                "Target year",
                "Target month",
                "Target day",
            ],
            |x| {
                let (y1, m1, d1) = valid_date(a(x, 0), a(x, 1), a(x, 2))?;
                let (y2, m2, d2) = valid_date(a(x, 3), a(x, 4), a(x, 5))?;
                let diff = days_from_civil(y2, m2, d2) - days_from_civil(y1, m1, d1);
                Ok(format!(
                    "Countdown: {} days ({}).",
                    diff,
                    if diff > 0 {
                        "to go"
                    } else if diff == 0 {
                        "today!"
                    } else {
                        "already passed"
                    }
                ))
            },
        )
        .with_dates(FROM_TARGET_DATES),
    );
    reg(v, Calc::new("business-days", "Working Days Between Dates", "Date & Time",
        "Counts the working days (Monday to Friday) from the start date to the end date exactly, day by day. Enter each date as one MM/DD/YYYY date.", &["Start year", "Start month", "Start day", "End year", "End month", "End day"],
        |x| { let (y1,m1,d1)=valid_date(a(x,0),a(x,1),a(x,2))?; let (y2,m2,d2)=valid_date(a(x,3),a(x,4),a(x,5))?; let start=days_from_civil(y1,m1,d1).min(days_from_civil(y2,m2,d2)); let total=(days_from_civil(y2,m2,d2)-days_from_civil(y1,m1,d1)).abs(); let exact=count_business_days(start, total); Ok(format!("Total days: {}. Business days (Mon-Fri, exact count, not counting the start day, counting the end day if a weekday): {}.", total, exact)) }).with_dates(START_END_DATES));
    reg(v, Calc::new("dog-age", "Dog Age to Human Years", "Everyday",
        "Modern guide (AVMA): first year ≈ 15 human years, second adds 9 (24 total), then +5 per year. Enter whole years and pick size code for the older-dog adjustment: 1 small, 2 medium, 3 large (+5/+5/+6 per extra year used). ESTIMATE.", &["Dog age (years)", "Size code (1 small, 2 medium, 3 large)"],
        |x| { chk(a(x,0)>0.0,"Age must be over 0.")?; chk(a(x,1)==1.0 || a(x,1)==2.0 || a(x,1)==3.0, "Size code must be 1 (small), 2 (medium) or 3 (large).")?; let age=a(x,0); let extra=if a(x,1)==3.0 {6.0} else {5.0}; let h=if age<=1.0 { 15.0*age } else if age<=2.0 { 15.0+9.0*(age-1.0) } else { 24.0+extra*(age-2.0) }; Ok(format!("About {} human years. ESTIMATE.", nd(h,0))) }));
    reg(
        v,
        Calc::new(
            "cat-age",
            "Cat Age to Human Years",
            "Everyday",
            "Common guide: first year ≈ 15, second adds 9 (24 total), then +4 per year. ESTIMATE.",
            &["Cat age (years)"],
            |x| {
                chk(a(x, 0) > 0.0, "Age must be over 0.")?;
                let age = a(x, 0);
                let h = if age <= 1.0 {
                    15.0 * age
                } else if age <= 2.0 {
                    15.0 + 9.0 * (age - 1.0)
                } else {
                    24.0 + 4.0 * (age - 2.0)
                };
                Ok(format!("About {} human years. ESTIMATE.", nd(h, 0)))
            },
        ),
    );
    reg(v, Calc::new("fuel-cost-trip", "Fuel Cost of a Trip", "Everyday",
        "Fuel used = distance / economy. Cost = fuel used x price per unit. Use consistent units: e.g. miles and mpg, price per gallon; or km and km-per-litre, price per litre.", &["Distance", "Fuel economy (distance per unit of fuel)", "Price per unit of fuel"],
        |x| { nz(a(x,1),"Fuel economy")?; let used=a(x,0)/a(x,1); Ok(format!("Fuel used: {}. Trip fuel cost: {}.", nd(used,3), m(used*a(x,2)))) }));
    reg(
        v,
        Calc::new(
            "mpg-trip",
            "Fuel Economy From a Fill-Up",
            "Everyday",
            "Economy = distance driven / fuel used.",
            &["Distance driven", "Fuel used"],
            |x| {
                nz(a(x, 1), "Fuel used")?;
                Ok(format!(
                    "Economy: {} (distance per fuel unit).",
                    nd(a(x, 0) / a(x, 1), 3)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "electricity-cost",
            "Electricity Cost (Appliance)",
            "Everyday",
            "Energy kWh = watts x hours / 1000. Cost = kWh x rate per kWh.",
            &["Watts", "Hours used", "Rate per kWh"],
            |x| {
                let kwh = a(x, 0) * a(x, 1) / 1000.0;
                Ok(format!(
                    "Energy: {} kWh. Cost: {}. For 30 days at this daily use: {}.",
                    nd(kwh, 4),
                    m(kwh * a(x, 2)),
                    m(kwh * a(x, 2) * 30.0)
                ))
            },
        ),
    );
    reg(v, Calc::new("download-time", "Download Time", "Everyday",
        "Time = file size (MB) x 8 / speed (Mbps). Uses decimal MB (1000-based) matching most speed tests.", &["File size (MB)", "Download speed (Mbps)"],
        |x| { nz(a(x,1),"Speed")?; let secs=a(x,0)*8.0/a(x,1); Ok(format!("Time: {} seconds = {} minutes (about {} h {} min). At 90% real-world efficiency: about {} minutes.", nd(secs,1), nd(secs/60.0,2), (secs/3600.0) as i64, ((secs%3600.0)/60.0) as i64, nd(secs/60.0/0.9,2))) }));
    reg(v, Calc::new("aspect-ratio", "Aspect Ratio & Missing Dimension", "Everyday",
        "Simplifies width:height by their greatest common divisor and computes the matching height for a new width (and the matching width for a new height — both shown).", &["Width", "Height", "New width"],
        |x| { let g=crate::gcd_i(a(x,0) as i64, a(x,1) as i64).max(1); nz(a(x,0),"Width")?; nz(a(x,1),"Height")?; let nh=a(x,2)*a(x,1)/a(x,0); Ok(format!("Ratio: {}:{} (decimal {}). At new width {}: height = {}. At new height {}: width = {}.", (a(x,0) as i64)/g, (a(x,1) as i64)/g, nd(a(x,0)/a(x,1),4), n(a(x,2)), nd(nh,3), n(a(x,2)), nd(a(x,2)*a(x,0)/a(x,1),3))) }));
    reg(v, Calc::new("screen-ppi", "Screen PPI & Size", "Everyday",
        "PPI = sqrt(w^2 + h^2) / diagonal inches. Physical width/height from the diagonal and ratio also shown.", &["Horizontal pixels", "Vertical pixels", "Diagonal (inches)"],
        |x| { nz(a(x,2),"Diagonal")?; let ppi=(a(x,0).powi(2)+a(x,1).powi(2)).sqrt()/a(x,2); let r=a(x,0)/a(x,1); let h=a(x,2)/(1.0+r*r).sqrt(); Ok(format!("PPI: {}. Physical size: about {} x {} inches.", nd(ppi,1), nd(h*r,2), nd(h,2))) }));
    reg(v, Calc::new("gpa", "GPA (Grade Points & Credits Pairs)", "Everyday",
        "Enter pairs: grade points for a course, its credits, repeated. GPA = sum(points x credits) / sum(credits). Use the 4.0 scale (A=4, B=3, C=2, D=1, F=0).", &["Pairs: grade points, credits, ... (list)"],
        |x| { let mut num=0.0; let mut den=0.0; let mut i=0; while i+1<x.len() { num+=x[i]*x[i+1]; den+=x[i+1]; i+=2; } nz(den,"Total credits")?; Ok(format!("GPA: {}. Total credits: {}.", nd(num/den,3), n(den))) }).pair_mode());
    reg(v, Calc::new("test-grade", "Test Grade % & Points Needed", "Everyday",
        "Grade % = correct / total x 100. Second part: points still needed to reach a target % overall is in the Attendance/grade planner notes — here you also get how many more correct answers would hit your target on THIS test.", &["Correct answers", "Total questions", "Target %"],
        |x| { nz(a(x,1),"Total questions")?; let cur=a(x,0)/a(x,1)*100.0; let need=(a(x,2)/100.0*a(x,1)-a(x,0)).ceil().max(0.0); Ok(format!("Score: {}%. To reach {}% on this test you would need {} more correct answers ({} total correct).", nd(cur,2), n(a(x,2)), need, a(x,0)+need)) }));
    reg(v, Calc::new("attendance", "Attendance Percentage", "Everyday",
        "Attendance % = attended / total sessions x 100. Also how many you can still miss while staying at or above a target %.", &["Sessions attended", "Total sessions", "Target attendance %"],
        |x| { nz(a(x,1),"Total sessions")?; let cur=a(x,0)/a(x,1)*100.0; let can_miss=(a(x,0)-a(x,2)/100.0*a(x,1)).floor().max(0.0); Ok(format!("Attendance: {}%. You can miss {} more session(s) and stay at or above {}% (assuming no new sessions are added).", nd(cur,2), can_miss, n(a(x,2)))) }));
    reg(
        v,
        Calc::new(
            "age-in-days",
            "Age in Total Days / Weeks / Hours",
            "Everyday",
            "Enter a birth date and an end date as MM/DD/YYYY dates; shows totals.",
            &[
                "Birth year",
                "Birth month",
                "Birth day",
                "End year",
                "End month",
                "End day",
            ],
            |x| {
                let (by, bm, bd) = valid_date(a(x, 0), a(x, 1), a(x, 2))?;
                let (oy, om, od) = valid_date(a(x, 3), a(x, 4), a(x, 5))?;
                let days = days_from_civil(oy, om, od) - days_from_civil(by, bm, bd);
                chk(days >= 0, "The end date is before the birth date.")?;
                Ok(format!(
                    "{} days = {} weeks = {} hours = {} minutes.",
                    days,
                    days / 7,
                    days * 24,
                    days * 24 * 60
                ))
            },
        )
        .with_dates(BIRTH_END_DATES),
    );
    reg(v, Calc::new("split-bill", "Split a Bill Evenly (With Tax & Tip)", "Everyday",
        "Total = amount x (1 + tax%) + amount x tip% — tip counted on the pre-tax amount, the common custom. Then split.", &["Bill amount", "Tax %", "Tip %", "People"],
        |x| { nz(a(x,3),"Number of people")?; let total=a(x,0)*(1.0+a(x,1)/100.0)+a(x,0)*a(x,2)/100.0; Ok(format!("Total: {}. Each of {} people pays: {}.", m(total), n(a(x,3)), m(total/a(x,3)))) }));
    reg(v, Calc::new("unit-price", "Unit Price Comparison (Better Buy)", "Everyday",
        "Unit price = price / quantity, for each of two products. The lower unit price is the better buy.", &["Product 1 price", "Product 1 quantity", "Product 2 price", "Product 2 quantity"],
        |x| { nz(a(x,1),"Product 1 quantity")?; nz(a(x,3),"Product 2 quantity")?; let u1=a(x,0)/a(x,1); let u2=a(x,2)/a(x,3); Ok(format!("Product 1: {} per unit. Product 2: {} per unit. Better buy: product {} (saves {} per unit).", m(u1), m(u2), if u1<=u2 {1} else {2}, m((u1-u2).abs()))) }));
    reg(
        v,
        Calc::new(
            "overtime-pay",
            "Overtime Pay (Time-and-a-Half)",
            "Everyday",
            "Weekly pay = rate x regular hours + rate x 1.5 x overtime hours.",
            &["Hourly rate", "Regular hours", "Overtime hours"],
            |x| {
                Ok(format!(
                    "Regular pay: {}. Overtime pay: {}. Total: {}.",
                    m(a(x, 0) * a(x, 1)),
                    m(a(x, 0) * 1.5 * a(x, 2)),
                    m(a(x, 0) * a(x, 1) + a(x, 0) * 1.5 * a(x, 2))
                ))
            },
        ),
    );
}

fn count_business_days(start_day: i64, total: i64) -> i64 {
    // 1970-01-01 (day 0) was a Thursday. weekday index: (days mod 7): 0 Thu,1 Fri,2 Sat,3 Sun,4 Mon,5 Tue,6 Wed.
    // Business days = Mon..Fri => indices {0(Thu),1(Fri),4,5,6}. Count days in (start, start+total].
    let mut count = 0i64;
    let mut i = 1i64;
    while i <= total {
        let wd = ((start_day + i) % 7 + 7) % 7;
        if matches!(wd, 0 | 1 | 4 | 5 | 6) {
            count += 1;
        }
        i += 1;
    }
    count
}
