//! Health calculators (Omni: Health; Nutilz: BMI etc). All estimates —
//! standard published formulas, labelled as estimates, not medical advice.
use crate::{a, chk, dg, n, nd, nz, Calc, DateGroup};

fn reg(v: &mut Vec<Calc>, c: Calc) {
    v.push(c);
}

static LMP_DATE: &[DateGroup] = &[dg(0, "First Day of Last Period")];

pub fn register(v: &mut Vec<Calc>) {
    reg(v, Calc::new("bmi", "BMI (Body Mass Index)", "Health",
        "BMI = weight kg / (height m)^2. Categories (WHO): under 18.5 underweight, 18.5-24.9 healthy, 25-29.9 overweight, 30+ obese. ESTIMATE only.", &["Weight (kg)", "Height (cm)"],
        |x| { chk(a(x,0)>0.0 && a(x,1)>0.0,"Weight and height must be over 0.")?; let h=a(x,1)/100.0; let bmi=a(x,0)/(h*h); let cat=if bmi<18.5 {"underweight"} else if bmi<25.0 {"healthy range"} else if bmi<30.0 {"overweight"} else {"obese"}; Ok(format!("BMI: {} — {}.\nSteps: {} / {}^2 = {}. Healthy weight for this height: {} to {} kg.", nd(bmi,2), cat, n(a(x,0)), n(h), nd(bmi,2), nd(18.5*h*h,1), nd(24.9*h*h,1))) }));
    reg(v, Calc::new("bmi-imperial", "BMI (Pounds & Inches)", "Health",
        "BMI = 703 x lb / in^2. Same WHO categories as the metric BMI calculator. ESTIMATE only.", &["Weight (lb)", "Height (inches)"],
        |x| { chk(a(x,0)>0.0 && a(x,1)>0.0,"Weight and height must be over 0.")?; let bmi=703.0*a(x,0)/(a(x,1)*a(x,1)); Ok(format!("BMI: {}.", nd(bmi,2))) }));
    reg(v, Calc::new("bmr-male", "BMR — Male (Mifflin-St Jeor)", "Health",
        "BMR (men) = 10 x kg + 6.25 x cm - 5 x age + 5. Calories your body burns at complete rest. ESTIMATE.", &["Weight (kg)", "Height (cm)", "Age (years)"],
        |x| Ok(format!("BMR: {} calories/day at rest.", nd(10.0*a(x,0)+6.25*a(x,1)-5.0*a(x,2)+5.0,0)))));
    reg(
        v,
        Calc::new(
            "bmr-female",
            "BMR — Female (Mifflin-St Jeor)",
            "Health",
            "BMR (women) = 10 x kg + 6.25 x cm - 5 x age - 161. ESTIMATE.",
            &["Weight (kg)", "Height (cm)", "Age (years)"],
            |x| {
                Ok(format!(
                    "BMR: {} calories/day at rest.",
                    nd(10.0 * a(x, 0) + 6.25 * a(x, 1) - 5.0 * a(x, 2) - 161.0, 0)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "bmr-harris-male",
            "BMR — Male (Harris-Benedict, Original)",
            "Health",
            "BMR (men, 1919 formula) = 66.5 + 13.75 x kg + 5.003 x cm - 6.75 x age. ESTIMATE.",
            &["Weight (kg)", "Height (cm)", "Age (years)"],
            |x| {
                Ok(format!(
                    "BMR: {} calories/day at rest.",
                    nd(66.5 + 13.75 * a(x, 0) + 5.003 * a(x, 1) - 6.75 * a(x, 2), 0)
                ))
            },
        ),
    );
    reg(
        v,
        Calc::new(
            "bmr-harris-female",
            "BMR — Female (Harris-Benedict, Original)",
            "Health",
            "BMR (women, 1919 formula) = 655.1 + 9.563 x kg + 1.850 x cm - 4.676 x age. ESTIMATE.",
            &["Weight (kg)", "Height (cm)", "Age (years)"],
            |x| {
                Ok(format!(
                    "BMR: {} calories/day at rest.",
                    nd(
                        655.1 + 9.563 * a(x, 0) + 1.85 * a(x, 1) - 4.676 * a(x, 2),
                        0
                    )
                ))
            },
        ),
    );
    reg(v, Calc::new("tdee", "TDEE (Daily Calories With Activity)", "Health",
        "TDEE = BMR x activity factor: 1.2 sedentary, 1.375 light, 1.55 moderate, 1.725 very active, 1.9 athlete. Enter your BMR (see BMR calculators) and the factor. ESTIMATE.", &["BMR (cal/day)", "Activity factor (1.2 - 1.9)"],
        |x| Ok(format!("TDEE (maintenance calories): {} cal/day. To lose ~0.5 kg/week eat about {} cal/day; to gain, about {}.", nd(a(x,0)*a(x,1),0), nd(a(x,0)*a(x,1)-500.0,0), nd(a(x,0)*a(x,1)+500.0,0)))));
    reg(v, Calc::new("calorie-target", "Calorie Target for a Weight Goal", "Health",
        "Uses 7,700 cal ≈ 1 kg of body fat. Daily adjustment = (kg to change x 7700) / days. Target = TDEE - adjustment (to lose) — direction follows the sign of kg (negative = lose). ESTIMATE, not medical advice.", &["TDEE (cal/day)", "Weight change wanted (kg, + gain / - lose)", "Days to do it in"],
        |x| { nz(a(x,2),"Days")?; let adj=a(x,1)*7700.0/a(x,2); Ok(format!("Eat about {} cal/day ({} cal/day {} your TDEE).", nd(a(x,0)+adj,0), nd(adj.abs(),0), if adj>=0.0 {"above"} else {"below"})) }));
    reg(v, Calc::new("body-fat-navy-male", "Body Fat % — Male (US Navy)", "Health",
        "US Navy method (men): BF% = 495 / (1.0324 - 0.19077 log10(waist-neck) + 0.15456 log10(height)) - 450. All cm. ESTIMATE.", &["Waist (cm, at navel)", "Neck (cm)", "Height (cm)"],
        |x| { chk(a(x,0)>a(x,1),"Waist must be bigger than neck for this formula.")?; let bf=495.0/(1.0324-0.19077*(a(x,0)-a(x,1)).log10()+0.15456*a(x,2).log10())-450.0; Ok(format!("Body fat: {}%. Fat mass: not shown — multiply your weight by {}%. Lean mass share: {}%.", nd(bf,2), nd(bf,1), nd(100.0-bf,1))) }));
    reg(v, Calc::new("body-fat-navy-female", "Body Fat % — Female (US Navy)", "Health",
        "US Navy method (women): BF% = 495 / (1.29579 - 0.35004 log10(waist+hip-neck) + 0.22100 log10(height)) - 450. All cm. ESTIMATE.", &["Waist (cm)", "Hip (cm)", "Neck (cm)", "Height (cm)"],
        |x| { chk(a(x,0)+a(x,1)>a(x,2),"Waist + hip must be bigger than neck for this formula.")?; let bf=495.0/(1.29579-0.35004*(a(x,0)+a(x,1)-a(x,2)).log10()+0.22100*a(x,3).log10())-450.0; Ok(format!("Body fat: {}%. Lean mass share: {}%.", nd(bf,2), nd(100.0-bf,1))) }));
    reg(v, Calc::new("ideal-weight-devine", "Ideal Weight (Devine Formula)", "Health",
        "Devine: men 50 kg + 2.3 kg for each inch over 5 ft (minus 2.3 kg for each inch under 5 ft); women 45.5 kg +/- the same 2.3 kg per inch. Enter sex code 1=male, 2=female, and height in cm. ESTIMATE.", &["Sex code (1 male, 2 female)", "Height (cm)"],
        |x| { let inches=a(x,1)/2.54; let over=inches-60.0; let iw=if a(x,0)==1.0 { 50.0+2.3*over } else if a(x,0)==2.0 { 45.5+2.3*over } else { return Err("Sex code must be 1 (male) or 2 (female).".into()) }; Ok(format!("Ideal weight (Devine): {} kg. (Height {} inches; {} inches {} 5 ft.)", nd(iw,1), nd(inches,1), nd(over.abs(),1), if over>=0.0 {"over"} else {"under"})) }));
    reg(v, Calc::new("water-intake", "Daily Water Intake Estimate", "Health",
        "Common guide: about 35 ml per kg of body weight per day (about 0.5 oz per lb). ESTIMATE — thirst, weather and activity matter more.", &["Weight (kg)"],
        |x| Ok(format!("About {} litres per day ({} ml). In US cups (240 ml): about {}.", nd(a(x,0)*35.0/1000.0,2), nd(a(x,0)*35.0,0), nd(a(x,0)*35.0/240.0,1)))));
    reg(v, Calc::new("target-heart-rate", "Target Heart Rate Zones", "Health",
        "Max HR ≈ 220 - age. Zones are % of max: fat burn ~60-70%, cardio ~70-80%, peak ~80-90%. ESTIMATE.", &["Age (years)"],
        |x| { let mx=220.0-a(x,0); Ok(format!("Max heart rate ≈ {} bpm.\nRecovery 50-60%: {}-{}\nFat burn 60-70%: {}-{}\nCardio 70-80%: {}-{}\nPeak 80-90%: {}-{}", nd(mx,0), nd(mx*0.5,0), nd(mx*0.6,0), nd(mx*0.6,0), nd(mx*0.7,0), nd(mx*0.7,0), nd(mx*0.8,0), nd(mx*0.8,0), nd(mx*0.9,0))) }));
    reg(v, Calc::new("karvonen", "Karvonen Heart Rate Target", "Health",
        "Target = resting HR + intensity% x (max HR - resting HR). Max HR = 220 - age. Enter intensity as % (e.g. 70). ESTIMATE.", &["Age (years)", "Resting heart rate (bpm)", "Intensity %"],
        |x| { let mx=220.0-a(x,0); let t=a(x,1)+a(x,2)/100.0*(mx-a(x,1)); Ok(format!("Target heart rate at {}% (Karvonen): {} bpm. Heart-rate reserve: {} bpm.", n(a(x,2)), nd(t,0), nd(mx-a(x,1),0))) }));
    reg(v, Calc::new("pace", "Running/Walking Pace", "Health",
        "Pace = time / distance. Speed = distance / time. Enter distance in km and time in minutes.", &["Distance (km)", "Time (minutes)"],
        |x| { nz(a(x,0),"Distance")?; nz(a(x,1),"Time")?; Ok(format!("Pace: {} min/km ({} min/mile). Speed: {} km/h ({} mph).", nd(a(x,1)/a(x,0),2), nd(a(x,1)/(a(x,0)*0.621371),2), nd(a(x,0)/(a(x,1)/60.0),2), nd(a(x,0)*0.621371/(a(x,1)/60.0),2))) }));
    reg(
        v,
        Calc::new(
            "calories-burned-walk",
            "Calories Burned Walking (Estimate)",
            "Health",
            "Uses a MET of about 3.5 for brisk walking: calories = MET x kg x hours. ESTIMATE.",
            &["Weight (kg)", "Minutes walked"],
            |x| {
                Ok(format!(
                    "About {} calories.",
                    nd(3.5 * a(x, 0) * (a(x, 1) / 60.0), 0)
                ))
            },
        ),
    );
    reg(v, Calc::new("calories-burned-run", "Calories Burned Running (Estimate)", "Health",
        "Rough guide: about 1.05 calories per kg per km run. calories = 1.05 x kg x km. ESTIMATE.", &["Weight (kg)", "Distance (km)"],
        |x| Ok(format!("About {} calories.", nd(1.05*a(x,0)*a(x,1),0)))));
    reg(v, Calc::new("steps-distance", "Steps to Distance & Calories", "Health",
        "Distance = steps x stride length (average stride ≈ 0.762 m). Calories ≈ 0.04 per step at 70 kg, scaled by weight. ESTIMATE.", &["Steps", "Weight (kg)", "Stride length (m, 0.762 avg)"],
        |x| Ok(format!("Distance: {} km ({} miles). About {} calories.", nd(a(x,0)*a(x,2)/1000.0,3), nd(a(x,0)*a(x,2)/1609.34,3), nd(a(x,0)*0.04*a(x,1)/70.0,0)))));
    reg(v, Calc::new("due-date", "Pregnancy Due Date (Naegele's Rule)", "Health",
        "Due date = first day of last period (LMP) + 280 days. Enter the first day of the last period as one MM/DD/YYYY date. ESTIMATE — only a due window, not medical advice.", &["LMP year", "LMP month", "LMP day"],
        |x| { let (y,mo,d)=crate::valid_date(a(x,0),a(x,1),a(x,2))?; let days=crate::days_from_civil(y,mo,d)+280; let (yy,mm,dd)=crate::civil_from_days(days); Ok(format!("Estimated due date: {yy}-{mm:02}-{dd:02} (LMP + 280 days). Gestational age today is counted from the LMP.")) }).with_dates(LMP_DATE));
    reg(v, Calc::new("ovulation", "Ovulation & Fertile Window Estimate", "Health",
        "Ovulation ≈ next period minus 14 days; next period = LMP + cycle length. Fertile window ≈ ovulation minus 5 days to ovulation plus 1. Enter the first day of the last period as one MM/DD/YYYY date, plus the cycle length in days. ESTIMATE.", &["LMP year", "LMP month", "LMP day", "Cycle length (days)"],
        |x| { let (y,mo,d)=crate::valid_date(a(x,0),a(x,1),a(x,2))?; let base=crate::days_from_civil(y,mo,d); let ov=base+a(x,3) as i64-14; let (yy,mm,dd)=crate::civil_from_days(ov); let (fy,fm,fd)=crate::civil_from_days(ov-5); let (ey,em,ed)=crate::civil_from_days(ov+1); Ok(format!("Estimated ovulation: {yy}-{mm:02}-{dd:02}. Fertile window: {fy}-{fm:02}-{fd:02} to {ey}-{em:02}-{ed:02}.")) }).with_dates(LMP_DATE));
    reg(v, Calc::new("bac-widmark", "Blood Alcohol Estimate (Widmark)", "Health",
        "Widmark: BAC% ≈ (grams of alcohol / (body weight g x r)) x 100 - 0.015 x hours. r=0.68 men, 0.55 women. Enter sex code 1=male 2=female. ESTIMATE ONLY — never use this to decide to drive.", &["Pure alcohol (grams)", "Weight (kg)", "Sex code (1 male, 2 female)", "Hours since drinking"],
        |x| { let r=if a(x,2)==1.0 {0.68} else if a(x,2)==2.0 {0.55} else { return Err("Sex code must be 1 or 2.".into()) }; let bac=(a(x,0)/(a(x,1)*1000.0*r))*100.0-0.015*a(x,3); Ok(format!("Estimated BAC: {}% (floored at 0: {}%). ESTIMATE ONLY.", nd(bac,3), nd(bac.max(0.0),3))) }));
    reg(v, Calc::new("sleep-cycles", "Sleep Cycles (Wake / Sleep Times)", "Health",
        "Sleep runs in ~90-minute cycles. Given hours from now until you must wake, this shows the best full-cycle sleep lengths (4, 5 or 6 cycles = 6h, 7.5h, 9h) plus ~14 min to fall asleep. Guide only.", &["Hours until you must wake up"],
        |x| { let h=a(x,0); let fits=(h*60.0/90.0).floor(); Ok(format!("Full 90-min cycles that fit: {} ({} hours of sleep). Best options: sleep {} h (4 cycles), {} h (5 cycles) or {} h (6 cycles) — plus about 14 minutes to fall asleep.", fits, nd(fits*1.5,1), 6.0, 7.5, 9.0)) }));
    reg(v, Calc::new("macro-split", "Macro Split (Protein/Carbs/Fat Grams)", "Health",
        "Splits daily calories by % into grams: protein & carbs = 4 cal/g, fat = 9 cal/g. Percentages should add to 100.", &["Daily calories", "Protein %", "Carbs %", "Fat %"],
        |x| { let tot=a(x,1)+a(x,2)+a(x,3); chk((tot-100.0).abs()<0.01,"The three percentages must add to 100 (yours add to a different total).")?; Ok(format!("Protein: {} g. Carbs: {} g. Fat: {} g.", nd(a(x,0)*a(x,1)/100.0/4.0,0), nd(a(x,0)*a(x,2)/100.0/4.0,0), nd(a(x,0)*a(x,3)/100.0/9.0,0))) }));
    reg(v, Calc::new("protein-needs", "Daily Protein Needs Estimate", "Health",
        "Guides: sedentary ≈ 0.8 g/kg, active ≈ 1.2-1.6 g/kg, muscle building ≈ 1.6-2.2 g/kg (enter your chosen factor). ESTIMATE.", &["Weight (kg)", "Protein factor (g per kg)"],
        |x| Ok(format!("Daily protein target: {} g.", nd(a(x,0)*a(x,1),0)))));
}
