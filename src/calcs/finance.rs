//! Finance calculators (Omni: Finance; Nutilz: loan / savings / tax).
use crate::{a, chk, m, n, nd, nz, Calc};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

/// Monthly payment for principal P, annual % rate, years.
fn pmt(p: f64, annual_pct: f64, years: f64) -> Result<f64, String> {
    chk(
        p >= 0.0 && years > 0.0,
        "Principal must be 0 or more and the term must be over 0 years.",
    )?;
    let r = annual_pct / 100.0 / 12.0;
    let nn = years * 12.0;
    if r == 0.0 {
        return Ok(p / nn);
    }
    Ok(p * r / (1.0 - (1.0 + r).powf(-nn)))
}

pub fn register(v: &mut Vec<Calc>) {
    reg(v, Calc::new("compound-interest", "Compound Interest (Yearly)", "Finance",
        "Future value = P (1 + r)^t, compounded once a year. P=principal, r=annual rate %, t=years. Interest = FV - P. (For monthly compounding with deposits, see the monthly-deposit calculator.)", &["Principal P", "Annual rate %", "Years"],
        |x| { chk(a(x,0)>=0.0 && a(x,2)>=0.0,"Principal and years cannot be negative.")?; let fv=a(x,0)*(1.0+a(x,1)/100.0).powf(a(x,2)); Ok(format!("Future value: {}. Interest earned: {}.\nSteps: {} x (1 + {}/100)^{} = {}.", m(fv), m(fv-a(x,0)), m(a(x,0)), n(a(x,1)), n(a(x,2)), m(fv))) }));
    reg(v, Calc::new("compound-interest-monthly-deposit", "Compound Interest With Monthly Deposit", "Finance",
        "FV = P(1+r/12)^(12t) + D x [((1+r/12)^(12t)-1)/(r/12)], monthly compounding. D=monthly deposit.", &["Principal P", "Annual rate %", "Years", "Monthly deposit D"],
        |x| { let r=a(x,1)/100.0/12.0; let nn=12.0*a(x,2); let g=(1.0+r).powf(nn); let fv=if r==0.0 { a(x,0)+a(x,3)*nn } else { a(x,0)*g + a(x,3)*((g-1.0)/r) }; Ok(format!("Future value: {}. Total you put in: {}. Interest earned: {}.", m(fv), m(a(x,0)+a(x,3)*nn), m(fv-a(x,0)-a(x,3)*nn))) }));
    reg(
        v,
        Calc::new(
            "simple-interest",
            "Simple Interest",
            "Finance",
            "Interest = P x r x t (r as a %). Total = P + interest.",
            &["Principal P", "Annual rate %", "Years"],
            |x| {
                let i = a(x, 0) * a(x, 1) / 100.0 * a(x, 2);
                Ok(format!(
                    "Interest: {}. Total value: {}.",
                    m(i),
                    m(a(x, 0) + i)
                ))
            },
        ),
    );
    reg(v, Calc::new("loan-payment", "Loan Payment", "Finance",
        "Monthly payment = P x r / (1 - (1+r)^-n), r = annual%/12, n = years x 12. Also shows total paid and total interest.", &["Loan amount", "Annual rate %", "Years"],
        |x| { let pay=pmt(a(x,0),a(x,1),a(x,2))?; let total=pay*12.0*a(x,2); Ok(format!("Monthly payment: {}. Total paid: {}. Total interest: {}.", m(pay), m(total), m(total-a(x,0)))) }));
    reg(v, Calc::new("mortgage-payment", "Mortgage Payment (With Tax & Insurance)", "Finance",
        "Principal+interest uses the loan formula; then adds monthly property tax (annual tax/12), monthly insurance (annual/12) and monthly PMI if given.", &["Home price", "Down payment", "Annual rate %", "Years", "Annual property tax", "Annual insurance", "Monthly PMI (0 if none)"],
        |x| { let loan=a(x,0)-a(x,1); chk(loan>0.0,"The loan (price minus down payment) must be over 0.")?; let pi=pmt(loan,a(x,2),a(x,3))?; let tax=a(x,4)/12.0; let ins=a(x,5)/12.0; Ok(format!("Loan amount: {}. Principal & interest: {}/month. Tax: {}/month. Insurance: {}/month. PMI: {}/month.\nTOTAL monthly payment: {}.", m(loan), m(pi), m(tax), m(ins), m(a(x,6)), m(pi+tax+ins+a(x,6)))) }));
    reg(v, Calc::new("auto-loan", "Auto Loan Payment", "Finance",
        "Loan = price - down payment - trade-in + sales tax on (price - trade-in). Then the standard loan-payment formula.", &["Car price", "Down payment", "Trade-in value", "Sales tax %", "Annual rate %", "Years"],
        |x| { let taxed=(a(x,0)-a(x,2)).max(0.0); let loan=a(x,0)-a(x,1)-a(x,2)+taxed*a(x,3)/100.0; chk(loan>0.0,"The amount to finance must be over 0 — check price, down payment and trade-in.")?; let pay=pmt(loan,a(x,4),a(x,5))?; Ok(format!("Amount financed: {} (sales tax {}). Monthly payment: {}. Total interest: {}.", m(loan), m(taxed*a(x,3)/100.0), m(pay), m(pay*12.0*a(x,5)-loan))) }));
    reg(
        v,
        Calc::new(
            "loan-affordability",
            "Loan Amount From a Monthly Budget",
            "Finance",
            "How much you can borrow for a chosen monthly payment: P = pay x (1 - (1+r)^-n) / r.",
            &["Monthly payment you can afford", "Annual rate %", "Years"],
            |x| {
                chk(a(x, 2) > 0.0, "Years must be over 0.")?;
                let r = a(x, 1) / 100.0 / 12.0;
                let nn = 12.0 * a(x, 2);
                let p = if r == 0.0 {
                    a(x, 0) * nn
                } else {
                    a(x, 0) * (1.0 - (1.0 + r).powf(-nn)) / r
                };
                Ok(format!(
                    "You can borrow about: {}. Total paid: {}. Total interest: {}.",
                    m(p),
                    m(a(x, 0) * nn),
                    m(a(x, 0) * nn - p)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "present-value",
            "Present Value",
            "Finance",
            "PV = FV / (1 + r)^t. What a future amount is worth today.",
            &["Future value", "Annual rate %", "Years"],
            |x| {
                Ok(format!(
                    "Present value: {}.",
                    m(a(x, 0) / (1.0 + a(x, 1) / 100.0).powf(a(x, 2)))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "future-value",
            "Future Value (Single Sum)",
            "Finance",
            "FV = PV x (1 + r)^t, yearly compounding.",
            &["Present value", "Annual rate %", "Years"],
            |x| {
                Ok(format!(
                    "Future value: {}. Growth: {}.",
                    m(a(x, 0) * (1.0 + a(x, 1) / 100.0).powf(a(x, 2))),
                    m(a(x, 0) * (1.0 + a(x, 1) / 100.0).powf(a(x, 2)) - a(x, 0))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "annuity-pv",
            "Annuity Present Value",
            "Finance",
            "PV of a yearly payment: PV = pay x (1 - (1+r)^-t) / r.",
            &["Payment per year", "Annual rate %", "Years"],
            |x| {
                let r = a(x, 1) / 100.0;
                let pv = if r == 0.0 {
                    a(x, 0) * a(x, 2)
                } else {
                    a(x, 0) * (1.0 - (1.0 + r).powf(-a(x, 2))) / r
                };
                Ok(format!(
                    "Present value of the annuity: {}. Total payments: {}.",
                    m(pv),
                    m(a(x, 0) * a(x, 2))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "annuity-fv",
            "Annuity Future Value",
            "Finance",
            "FV of a yearly payment: FV = pay x ((1+r)^t - 1) / r.",
            &["Payment per year", "Annual rate %", "Years"],
            |x| {
                let r = a(x, 1) / 100.0;
                let fv = if r == 0.0 {
                    a(x, 0) * a(x, 2)
                } else {
                    a(x, 0) * (((1.0 + r).powf(a(x, 2))) - 1.0) / r
                };
                Ok(format!(
                    "Future value of the annuity: {}. Total paid in: {}. Interest: {}.",
                    m(fv),
                    m(a(x, 0) * a(x, 2)),
                    m(fv - a(x, 0) * a(x, 2))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "cagr",
            "CAGR (Compound Annual Growth Rate)",
            "Finance",
            "CAGR = (end/start)^(1/years) - 1, as a %.",
            &["Starting value", "Ending value", "Years"],
            |x| {
                chk(
                    a(x, 0) > 0.0 && a(x, 1) >= 0.0 && a(x, 2) > 0.0,
                    "Start must be over 0; years must be over 0.",
                )?;
                Ok(format!(
                    "CAGR: {}% per year.",
                    nd(((a(x, 1) / a(x, 0)).powf(1.0 / a(x, 2)) - 1.0) * 100.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "roi",
            "Return on Investment (ROI)",
            "Finance",
            "ROI = (gain - cost) / cost x 100.",
            &["Amount invested (cost)", "Final value (gain)"],
            |x| {
                nz(a(x, 0), "The amount invested")?;
                Ok(format!(
                    "ROI: {}%. Profit/loss: {}.",
                    nd((a(x, 1) - a(x, 0)) / a(x, 0) * 100.0, 4),
                    m(a(x, 1) - a(x, 0))
                ))
            },
        ),
    );
    reg(v, Calc::new("profit-margin", "Profit Margin & Markup", "Finance",
        "Margin = profit / revenue x 100. Markup = profit / cost x 100. Profit = revenue - cost.", &["Revenue (selling price total)", "Cost"],
        |x| { nz(a(x,0),"Revenue (for margin)")?; nz(a(x,1),"Cost (for markup)")?; let p=a(x,0)-a(x,1); Ok(format!("Profit: {}. Margin: {}%. Markup: {}%.", m(p), nd(p/a(x,0)*100.0,4), nd(p/a(x,1)*100.0,4))) }));
    reg(
        v,
        Calc::new(
            "price-from-margin",
            "Selling Price From a Target Margin",
            "Finance",
            "Price = cost / (1 - margin/100).",
            &["Cost", "Target margin %"],
            |x| {
                chk(a(x, 1) < 100.0, "Margin must be under 100%.")?;
                Ok(format!(
                    "Selling price: {}. Profit per sale: {}.",
                    m(a(x, 0) / (1.0 - a(x, 1) / 100.0)),
                    m(a(x, 0) / (1.0 - a(x, 1) / 100.0) - a(x, 0))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "price-from-markup",
            "Selling Price From a Markup",
            "Finance",
            "Price = cost x (1 + markup/100).",
            &["Cost", "Markup %"],
            |x| {
                Ok(format!(
                    "Selling price: {}. Margin at that price: {}%.",
                    m(a(x, 0) * (1.0 + a(x, 1) / 100.0)),
                    nd(a(x, 1) / (100.0 + a(x, 1)) * 100.0, 4)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "discount",
            "Discount Calculator",
            "Finance",
            "Sale price = price x (1 - discount/100). Savings = price - sale price.",
            &["Original price", "Discount %"],
            |x| {
                Ok(format!(
                    "Sale price: {}. You save: {}.",
                    m(a(x, 0) * (1.0 - a(x, 1) / 100.0)),
                    m(a(x, 0) * a(x, 1) / 100.0)
                ))
            },
        ),
    );
    reg(v, Calc::new("double-discount", "Double Discount (Two % Off in a Row)", "Finance",
        "Two discounts apply one after the other (not added): result = price x (1-d1/100) x (1-d2/100). Shows the single equivalent discount.", &["Original price", "First discount %", "Second discount %"],
        |x| { let r=a(x,0)*(1.0-a(x,1)/100.0)*(1.0-a(x,2)/100.0); Ok(format!("Final price: {}. Total saved: {}. Equivalent single discount: {}%.", m(r), m(a(x,0)-r), nd((1.0-r/a(x,0))*100.0,4))) }));
    reg(
        v,
        Calc::new(
            "sales-tax-add",
            "Sales Tax (Add Tax)",
            "Finance",
            "Total = price + price x tax%. Also used for VAT-add.",
            &["Price before tax", "Tax rate %"],
            |x| {
                Ok(format!(
                    "Tax amount: {}. Total with tax: {}.",
                    m(a(x, 0) * a(x, 1) / 100.0),
                    m(a(x, 0) * (1.0 + a(x, 1) / 100.0))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "sales-tax-extract",
            "Sales Tax / VAT (Extract From Total)",
            "Finance",
            "Price before tax = total / (1 + tax/100). Tax inside the total = total - that.",
            &["Total including tax", "Tax rate %"],
            |x| {
                let base = a(x, 0) / (1.0 + a(x, 1) / 100.0);
                Ok(format!(
                    "Price before tax: {}. Tax included in the total: {}.",
                    m(base),
                    m(a(x, 0) - base)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "tip",
            "Tip Calculator",
            "Finance",
            "Tip = bill x tip%. Total = bill + tip.",
            &["Bill amount", "Tip %"],
            |x| {
                Ok(format!(
                    "Tip: {}. Total: {}.",
                    m(a(x, 0) * a(x, 1) / 100.0),
                    m(a(x, 0) * (1.0 + a(x, 1) / 100.0))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "tip-split",
            "Tip & Split the Bill",
            "Finance",
            "Adds the tip, then splits the total evenly between people.",
            &["Bill amount", "Tip %", "Number of people"],
            |x| {
                nz(a(x, 2), "The number of people")?;
                let total = a(x, 0) * (1.0 + a(x, 1) / 100.0);
                Ok(format!(
                    "Tip: {}. Total: {}. Each person pays: {}.",
                    m(a(x, 0) * a(x, 1) / 100.0),
                    m(total),
                    m(total / a(x, 2))
                ))
            },
        ),
    );
    reg(v, Calc::new("salary-hourly", "Salary to Hourly", "Finance",
        "Hourly = yearly salary / hours per year. Uses 2080 hours (40h x 52 weeks) unless you change the hours input.", &["Yearly salary", "Hours per year (2080 standard)"],
        |x| { nz(a(x,1),"Hours per year")?; Ok(format!("Hourly wage: {}. Weekly pay: {}. Monthly pay: {}.", m(a(x,0)/a(x,1)), m(a(x,0)/52.0), m(a(x,0)/12.0))) }));
    reg(
        v,
        Calc::new(
            "hourly-salary",
            "Hourly to Salary (With Overtime)",
            "Finance",
            "Weekly = rate x regular hours + rate x 1.5 x overtime hours. Yearly = weekly x 52.",
            &[
                "Hourly rate",
                "Regular hours per week",
                "Overtime hours per week",
            ],
            |x| {
                let weekly = a(x, 0) * a(x, 1) + a(x, 0) * 1.5 * a(x, 2);
                Ok(format!(
                    "Weekly pay: {}. Monthly pay (avg): {}. Yearly salary: {}.",
                    m(weekly),
                    m(weekly * 52.0 / 12.0),
                    m(weekly * 52.0)
                ))
            },
        ),
    );
    reg(v, Calc::new("savings-goal", "Savings Goal (Monthly Deposit Needed)", "Finance",
        "Monthly deposit = goal x r / ((1+r)^n - 1), r = annual%/12, n = months. If rate is 0: goal/months.", &["Savings goal", "Annual rate %", "Months to save"],
        |x| { chk(a(x,2)>0.0,"Months must be over 0.")?; let r=a(x,1)/100.0/12.0; let dep=if r==0.0 { a(x,0)/a(x,2) } else { a(x,0)*r/((1.0+r).powf(a(x,2))-1.0) }; Ok(format!("Save {} per month. Total deposited: {}. Interest makes up the other {}.", m(dep), m(dep*a(x,2)), m(a(x,0)-dep*a(x,2)))) }));
    reg(
        v,
        Calc::new(
            "break-even",
            "Break-Even Point (Units)",
            "Finance",
            "Break-even units = fixed costs / (price per unit - variable cost per unit).",
            &["Fixed costs", "Price per unit", "Variable cost per unit"],
            |x| {
                let cm = a(x, 1) - a(x, 2);
                nz(cm, "Price minus variable cost")?;
                chk(
                    cm > 0.0,
                    "Price must be higher than variable cost, or you never break even.",
                )?;
                Ok(format!(
                    "Break-even: {} units. Revenue at break-even: {}.",
                    nd(a(x, 0) / cm, 2),
                    m(a(x, 0) / cm * a(x, 1))
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "effective-rate",
            "Effective Annual Rate (EAR)",
            "Finance",
            "EAR = (1 + nominal/n)^n - 1, as a %.",
            &["Nominal annual rate %", "Compounds per year n"],
            |x| {
                Ok(format!(
                    "Effective annual rate: {}%.",
                    nd(
                        ((1.0 + a(x, 0) / 100.0 / a(x, 1)).powf(a(x, 1)) - 1.0) * 100.0,
                        4
                    )
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "rule-of-72",
            "Rule of 72 (Doubling Time)",
            "Finance",
            "Years to double ≈ 72 / annual rate %. Also computes the exact value ln2/ln(1+r).",
            &["Annual rate %"],
            |x| {
                nz(a(x, 0), "The rate")?;
                Ok(format!(
                    "Rule of 72 estimate: {} years. Exact doubling time: {} years.",
                    nd(72.0 / a(x, 0), 3),
                    nd(std::f64::consts::LN_2 / (1.0 + a(x, 0) / 100.0).ln(), 3)
                ))
            },
        ),
    );
    reg(v, Calc::new("credit-card-payoff", "Credit Card Payoff Estimate", "Finance",
        "Months to pay off = -ln(1 - balance x r / payment) / ln(1 + r), r = APR/12. Payment must beat the monthly interest.", &["Balance", "APR %", "Monthly payment"],
        |x| { let r=a(x,1)/100.0/12.0; if r==0.0 { nz(a(x,2),"Monthly payment")?; return Ok(format!("Months to pay off (no interest): {}. Total paid: {}.", nd(a(x,0)/a(x,2),1), m(a(x,0)))); } chk(a(x,2)>a(x,0)*r,"That payment does not cover the monthly interest, so the balance would never fall.")?; let months=-(1.0-a(x,0)*r/a(x,2)).ln()/(1.0+r).ln(); Ok(format!("Months to pay off: {} (about {} years). Total paid: {}. Total interest: {}.", nd(months,1), nd(months/12.0,2), m(a(x,2)*months), m(a(x,2)*months-a(x,0)))) }));
    reg(v, Calc::new("inflation", "Inflation-Adjusted Value", "Finance",
        "Future price = amount x (1 + inflation)^years. Also today's buying power of that future amount.", &["Amount today", "Inflation % per year", "Years"],
        |x| { let f=a(x,0)*(1.0+a(x,1)/100.0).powf(a(x,2)); Ok(format!("Will cost in {} years: {}. What {} then buys in today's money: {}.", n(a(x,2)), m(f), m(f), m(a(x,0)))) }));
    reg(
        v,
        Calc::new(
            "net-margin",
            "Net Profit Margin (Business)",
            "Finance",
            "Net margin = net income / revenue x 100. Gross profit shown too (revenue - costs).",
            &["Revenue", "Total costs"],
            |x| {
                nz(a(x, 0), "Revenue")?;
                let net = a(x, 0) - a(x, 1);
                Ok(format!(
                    "Net income: {}. Net margin: {}%. Cost share of revenue: {}%.",
                    m(net),
                    nd(net / a(x, 0) * 100.0, 4),
                    nd(a(x, 1) / a(x, 0) * 100.0, 2)
                ))
            },
        ),
    );
    reg(v, Calc::new("payback-period", "Payback Period", "Finance",
        "Years to recover an investment = initial cost / yearly cash inflow (simple payback, no discounting).", &["Initial investment", "Cash inflow per year"],
        |x| { nz(a(x,1),"Yearly cash inflow")?; Ok(format!("Simple payback period: {} years.", nd(a(x,0)/a(x,1),3))) }));
    reg(v, Calc::new("depreciation-straight", "Straight-Line Depreciation", "Finance",
        "Yearly depreciation = (cost - salvage value) / useful life in years. Book value after Y years also shown — Y is the last input.", &["Cost", "Salvage value", "Useful life (years)", "Years from now Y"],
        |x| { nz(a(x,2),"Useful life")?; let dep=(a(x,0)-a(x,1))/a(x,2); let bv=(a(x,0)-dep*a(x,3)).max(a(x,1)); Ok(format!("Depreciation per year: {}. Book value after {} years: {} (not below salvage {}).", m(dep), n(a(x,3)), m(bv), m(a(x,1)))) }));
    reg(v, Calc::new("rent-vs-buy-simple", "Rent vs Buy (Simple Cost Compare)", "Finance",
        "Compares total rent over the years vs the interest + tax + insurance cost of buying (a rough guide only — ignores home value changes and equity). Labelled estimate.", &["Monthly rent", "Home price", "Mortgage rate %", "Years", "Yearly tax+insurance"],
        |x| { let rent_total=a(x,0)*12.0*a(x,3); let bal=pmt(a(x,1),a(x,2),a(x,3))?; let interest=bal*12.0*a(x,3)-a(x,1); let buy_cost=interest+a(x,4)*a(x,3); Ok(format!("Renting for {} years costs {}. Buying's interest+tax+insurance over that time: {} (but you also pay down {} of principal, building equity). ESTIMATE ONLY.", n(a(x,3)), m(rent_total), m(buy_cost), m(a(x,1)))) }));
}
