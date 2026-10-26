//! v0.1.2 audit tests — Finance.
//! Expected values computed independently (Python/hand) from the
//! standard published formulas, never copied from program output.
use calcdex::run_calc;

fn out(id: &str, args: &[f64]) -> String {
    run_calc(id, args).unwrap_or_else(|e| panic!("{id} {args:?} errored: {e}"))
}
fn has(id: &str, args: &[f64], needle: &str) {
    let o = out(id, args);
    assert!(o.contains(needle), "{id} {args:?} missing {needle:?}:\n{o}");
}
fn err(id: &str, args: &[f64], needle: &str) {
    let e = run_calc(id, args).unwrap_err();
    assert!(
        e.contains(needle),
        "{id} {args:?} wrong error {e:?}, want {needle:?}"
    );
}

#[test]
fn growth_and_value() {
    // P=1000 at 5% for 1 year plus 100/month deposited at month end:
    // growth 1051.16 + deposits compounded 1227.89 -> FV 2279.05
    // (independently summed month by month).
    has(
        "compound-interest-monthly-deposit",
        &[1000.0, 5.0, 1.0, 100.0],
        "2279.05",
    );
    // PV of 1100 due in 1 year at 10% = 1000.00.
    has("present-value", &[1100.0, 10.0, 1.0], "1000");
    // FV of 1000 at 10% for 2 years = 1210.00.
    has("future-value", &[1000.0, 10.0, 2.0], "1210");
    // PV of annuity 1000/yr, 10%, 2 years = 1000/1.1 + 1000/1.21 = 1735.54.
    has("annuity-pv", &[1000.0, 10.0, 2.0], "1735.54");
    // FV of annuity 1000/yr, 10%, 2 years = 1000*1.1 + 1000 = 2100.00.
    has("annuity-fv", &[1000.0, 10.0, 2.0], "2100");
    // EAR of 12% nominal compounded monthly = (1.01)^12 - 1 = 12.6825%.
    has("effective-rate", &[12.0, 12.0], "12.6825");
    // BUG PIN (v0.1.2 fix): n = 0 used to produce NaN/garbage; now a
    // teaching error.
    err("effective-rate", &[12.0, 0.0], "at least 1");
    // 100 in 10 years at 3% inflation is worth 134.39 in future money.
    has("inflation", &[100.0, 3.0, 10.0], "134.39");
}

#[test]
fn loans() {
    // Car 20000, 2000 down, 10% tax on price, 6% APR, 5 years:
    // amount financed 20000, payment = 386.66 (standard amortization).
    has(
        "auto-loan",
        &[20000.0, 2000.0, 0.0, 10.0, 6.0, 5.0],
        "386.66",
    );
    // 500/month at 6% for 10 years affords 45036.73.
    has("loan-affordability", &[500.0, 6.0, 10.0], "45036.73");
    // 1200 balance at 12% APR paying 100/month: months =
    // -ln(1 - rP/A)/ln(1+r) = 12.847148, so total paid = 1284.71
    // and interest = 84.71 (the program displays months to 1 dp).
    has("credit-card-payoff", &[1200.0, 12.0, 100.0], "1284.71");
    has("credit-card-payoff", &[1200.0, 12.0, 100.0], "84.71");
    // Rent 1000/mo for 10 years = 120000 total. Buying at 0% interest:
    // interest+tax+insurance cost = 2000/yr x 10 = 20000.
    has(
        "rent-vs-buy-simple",
        &[1000.0, 300000.0, 0.0, 10.0, 2000.0],
        "120000",
    );
    has(
        "rent-vs-buy-simple",
        &[1000.0, 300000.0, 0.0, 10.0, 2000.0],
        "20000",
    );
}

#[test]
fn business() {
    // Buy 100, sell 150 -> ROI 50%.
    has("roi", &[100.0, 150.0], "50");
    // Revenue 1000, costs 600 -> margin 40%, markup 66.6667%.
    has("profit-margin", &[1000.0, 600.0], "40");
    has("profit-margin", &[1000.0, 600.0], "66.6667");
    // Cost 80 at 20% margin -> price 100.00.
    has("price-from-margin", &[80.0, 20.0], "100");
    // Cost 50 at 50% markup -> price 75.00, margin 33.3333%.
    has("price-from-markup", &[50.0, 50.0], "75");
    has("price-from-markup", &[50.0, 50.0], "33.3333");
    // Net margin: profit 300 on revenue 1000 = 30%.
    has("net-margin", &[1000.0, 700.0], "30");
    // Fixed 1000, price 25, variable 15 -> break-even 100 units.
    has("break-even", &[1000.0, 25.0, 15.0], "100");
    // Payback: 10000 / 2500 per year = 4 years.
    has("payback-period", &[10000.0, 2500.0], "4");
    // Straight-line: (10000-1000)/9 = 1000/yr; book after 3y = 7000.
    has(
        "depreciation-straight",
        &[10000.0, 1000.0, 9.0, 3.0],
        "1000",
    );
    has(
        "depreciation-straight",
        &[10000.0, 1000.0, 9.0, 3.0],
        "7000",
    );
}

#[test]
fn everyday_money() {
    // 100 with two stacked 10% discounts = 81; equal to one 19% discount.
    has("double-discount", &[100.0, 10.0, 10.0], "81");
    has("double-discount", &[100.0, 10.0, 10.0], "19");
    // BUG PIN (v0.1.2 fix): price 0 used to print "Equivalent single
    // discount: NaN%"; now it explains there is nothing to compute.
    has(
        "double-discount",
        &[0.0, 10.0, 10.0],
        "no equivalent discount",
    );
    // 100 at 8% tax: tax 8, total 108.
    has("sales-tax-add", &[100.0, 8.0], "108");
    // 108 including 8% tax: pre-tax base 100.
    has("sales-tax-extract", &[108.0, 8.0], "100");
    // Tip 15% on 80 = 12, total 92.
    has("tip", &[80.0, 15.0], "12");
    has("tip", &[80.0, 15.0], "92");
    // 20/h x 40 h + 5 h overtime at 1.5x = 950/week, 49400/year.
    has("hourly-salary", &[20.0, 40.0, 5.0], "950");
    has("hourly-salary", &[20.0, 40.0, 5.0], "49400");
    // Save 1200 in 12 months at 0% -> 100/month.
    has("savings-goal", &[1200.0, 0.0, 12.0], "100");
    // BUG PIN (v0.1.2 fix): a negative rate used to print a negative
    // "exact doubling time". Money never doubles at <= 0%.
    err("rule-of-72", &[-5.0], "over 0%");
    has("rule-of-72", &[8.0], "9");
}
