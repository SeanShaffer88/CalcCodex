//! Students-batch engines (v0.5.0, 50-enhancement program items
//! #12, #13, #14, #15, #17, #18; #19's data lives in meta.rs).
//! Everything here is a pure library operation: the GUI and the
//! CLI both call these functions, so no feature is window-only.
//!
//! Honesty notes (also in DECISIONS.md):
//! - Check-my-answer compares against the FIRST result value a
//!   calculator prints (documented wherever the feature appears).
//! - Slip messages are phrased as "a common cause is..." — they
//!   name the arithmetic relationship that is exactly true (the
//!   student's answer IS 4x the correct one) and offer the usual
//!   cause; they never claim to know what the student did.
//! - Exact mode covers exactly three scopes: fraction
//!   calculators (already exact), the pi geometry family (exact
//!   forms in pi), and the Expression Calculator (exact rational
//!   arithmetic for + - * / and whole-number powers over
//!   integers/fractions). Everything else gets a fallback line
//!   saying the decimal is the answer.

use crate::meta::{formula_for, glossary_for};
use crate::rigour::AngleMode;
use crate::{display_tokens, n, run_calc, Calc};

// ---------- exact rational arithmetic (#14) ----------

fn gcd128(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

/// A normalised rational number: denominator always positive,
/// numerator/denominator always coprime. i128 headroom is large;
/// every operation is checked and reports overflow as None so the
/// caller can fall back to decimal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rational {
    pub num: i128,
    pub den: i128,
}

// The arithmetic methods return Option (checked i128 ops), so
// the std::ops traits — which cannot fail — do not fit; the
// names stay add/sub/mul/div by design.
#[allow(clippy::should_implement_trait)]
impl Rational {
    pub fn new(num: i128, den: i128) -> Option<Rational> {
        if den == 0 {
            return None;
        }
        let g = gcd128(num, den);
        let (mut n, mut d) = (num / g, den / g);
        if d < 0 {
            n = -n;
            d = -d;
        }
        Some(Rational { num: n, den: d })
    }
    pub fn int(v: i128) -> Rational {
        Rational { num: v, den: 1 }
    }
    /// Parse an exact decimal string ("-1.25", "0.5", "12") — the
    /// decimal IS the exact value (no binary float detour).
    pub fn from_decimal_str(s: &str) -> Option<Rational> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        let (neg, s) = match s.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, s.strip_prefix('+').unwrap_or(s)),
        };
        let (int_part, frac_part) = match s.split_once('.') {
            Some((a, b)) => (a, b),
            None => (s, ""),
        };
        if int_part.is_empty() && frac_part.is_empty() {
            return None;
        }
        if !int_part.chars().all(|c| c.is_ascii_digit())
            || !frac_part.chars().all(|c| c.is_ascii_digit())
        {
            return None;
        }
        if frac_part.len() > 18 {
            return None; // beyond exact i128 comfort; fall back
        }
        let scale = 10i128.checked_pow(frac_part.len() as u32)?;
        let ip: i128 = if int_part.is_empty() {
            0
        } else {
            int_part.parse().ok()?
        };
        let fp: i128 = if frac_part.is_empty() {
            0
        } else {
            frac_part.parse().ok()?
        };
        let num = ip.checked_mul(scale)?.checked_add(fp)?;
        Rational::new(if neg { -num } else { num }, scale)
    }
    /// From an f64 whose printed form (crate::n, up to 10 decimal
    /// places) is taken as the intended value — used for the pi
    /// coefficients of the geometry family.
    pub fn from_f64_printed(v: f64) -> Option<Rational> {
        if !v.is_finite() {
            return None;
        }
        Rational::from_decimal_str(&n(v))
    }
    pub fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }
    pub fn is_int(self) -> bool {
        self.den == 1
    }
    pub fn add(self, o: Rational) -> Option<Rational> {
        let num = self
            .num
            .checked_mul(o.den)?
            .checked_add(o.num.checked_mul(self.den)?)?;
        Rational::new(num, self.den.checked_mul(o.den)?)
    }
    pub fn sub(self, o: Rational) -> Option<Rational> {
        let num = self
            .num
            .checked_mul(o.den)?
            .checked_sub(o.num.checked_mul(self.den)?)?;
        Rational::new(num, self.den.checked_mul(o.den)?)
    }
    pub fn mul(self, o: Rational) -> Option<Rational> {
        Rational::new(self.num.checked_mul(o.num)?, self.den.checked_mul(o.den)?)
    }
    pub fn div(self, o: Rational) -> Option<Rational> {
        if o.num == 0 {
            return None;
        }
        Rational::new(self.num.checked_mul(o.den)?, self.den.checked_mul(o.num)?)
    }
    pub fn neg(self) -> Rational {
        Rational {
            num: -self.num,
            den: self.den,
        }
    }
    pub fn pow(self, e: i32) -> Option<Rational> {
        if e == 0 {
            return Some(Rational::int(1));
        }
        if self.num == 0 {
            return if e > 0 { Some(Rational::int(0)) } else { None };
        }
        let ae = e.unsigned_abs();
        if ae > 64 {
            return None;
        }
        let num = self.num.checked_pow(ae)?;
        let den = self.den.checked_pow(ae)?;
        if e > 0 {
            Rational::new(num, den)
        } else {
            Rational::new(den, num)
        }
    }
    /// "3/4", "-2", "0" — integers print without the denominator.
    pub fn show(self) -> String {
        if self.den == 1 {
            format!("{}", self.num)
        } else {
            format!("{}/{}", self.num, self.den)
        }
    }
}

/// Parse a student's answer: a plain number ("12.5", "-3"), a
/// fraction ("3/4", "-7/2"), or a mixed number ("1 1/2", "-1 1/2"
/// — a negative whole part makes the fraction part negative too,
/// the same convention as the fraction-to-decimal calculator).
pub fn parse_answer(text: &str) -> Result<Rational, String> {
    let t = text.trim();
    if t.is_empty() {
        return Err("Type your answer first — a number like 12.5, or a fraction like 3/4.".into());
    }
    let parts: Vec<&str> = t.split_whitespace().collect();
    let parse_frac = |s: &str| -> Result<Rational, String> {
        match s.split_once('/') {
            Some((a, b)) => {
                let (Ok(p), Ok(q)) = (a.trim().parse::<i128>(), b.trim().parse::<i128>()) else {
                    return Err(format!("'{s}' is not a fraction — write it like 3/4."));
                };
                if q == 0 {
                    return Err("A fraction's denominator cannot be 0.".into());
                }
                Rational::new(p, q)
                    .ok_or_else(|| format!("'{s}' is not a fraction — write it like 3/4."))
            }
            None => Rational::from_decimal_str(s).ok_or_else(|| {
                format!("'{s}' is not a number — write a number like 12.5, or a fraction like 3/4.")
            }),
        }
    };
    match parts.len() {
        1 => parse_frac(parts[0]),
        2 => {
            // Mixed number: whole part then a fraction part.
            let whole: i128 = parts[0]
                .parse()
                .map_err(|_| format!("'{}' is not a mixed number — write it like 1 1/2.", t))?;
            let frac = parse_frac(parts[1])?;
            if frac.den == 1 {
                return Err(format!(
                    "'{}' is not a mixed number — the second part must be a fraction, like the 1/2 in 1 1/2.",
                    t
                ));
            }
            let w = Rational::int(whole);
            if whole < 0 {
                w.sub(frac)
            } else {
                w.add(frac)
            }
            .ok_or_else(|| "That answer is too big to check exactly.".to_string())
        }
        _ => Err(format!(
            "'{}' has too many parts — write one number, one fraction like 3/4, or one mixed number like 1 1/2.",
            t
        )),
    }
}

// ---------- expression AST (shared by #14 exact + #17 plot) ----------

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    Num(String),
    Const(&'static str),
    Var,
    Neg(Box<Ast>),
    Bin(char, Box<Ast>, Box<Ast>),
    Func(String, Box<Ast>),
}

#[derive(Clone, Debug)]
enum ATok {
    Num(String),
    Ident(String),
    Op(char),
    LParen,
    RParen,
}

fn ast_tokenize(src: &str) -> Result<Vec<(ATok, usize)>, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut toks = Vec::new();
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        let pos = i + 1;
        if ch.is_whitespace() {
            i += 1;
            continue;
        }
        if ch.is_ascii_digit()
            || (ch == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            let start = i;
            let mut seen_dot = false;
            let mut seen_exp = false;
            while i < chars.len() {
                let c = chars[i];
                if c.is_ascii_digit() {
                    i += 1;
                } else if c == '.' && !seen_dot && !seen_exp {
                    seen_dot = true;
                    i += 1;
                } else if (c == 'e' || c == 'E') && !seen_exp {
                    // 'e' is also Euler's number: only treat as an
                    // exponent when digits (optionally signed)
                    // follow immediately.
                    let mut k = i + 1;
                    if k < chars.len() && (chars[k] == '-' || chars[k] == '+') {
                        k += 1;
                    }
                    if k < chars.len() && chars[k].is_ascii_digit() {
                        seen_exp = true;
                        i = k;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            let text: String = chars[start..i].iter().collect();
            if text.parse::<f64>().is_err() {
                return Err(format!(
                    "Parse error at position {pos}: '{text}' is not a number."
                ));
            }
            toks.push((ATok::Num(text), pos));
            continue;
        }
        if ch.is_alphabetic() {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let name: String = chars[start..i].iter().collect();
            toks.push((ATok::Ident(name.to_ascii_lowercase()), pos));
            continue;
        }
        match ch {
            '+' | '-' | '*' | '/' | '^' => toks.push((ATok::Op(ch), pos)),
            '(' => toks.push((ATok::LParen, pos)),
            ')' => toks.push((ATok::RParen, pos)),
            _ => {
                return Err(format!(
                    "Parse error at position {pos}: '{ch}' is not part of an expression — use numbers, x, + - * / ^, brackets, and function names like sqrt."
                ))
            }
        }
        i += 1;
    }
    Ok(toks)
}

struct AstParser {
    toks: Vec<(ATok, usize)>,
    i: usize,
    src_len: usize,
}

impl AstParser {
    fn peek(&self) -> Option<&ATok> {
        self.toks.get(self.i).map(|t| &t.0)
    }
    fn err_expected(&self, what: &str) -> String {
        match self.toks.get(self.i) {
            Some((_, pos)) => format!("Parse error at position {}: expected {what} here.", pos),
            None => format!(
                "Parse error at position {}: expected {what}, but the expression ended.",
                self.src_len + 1
            ),
        }
    }
    fn parse_expr(&mut self) -> Result<Ast, String> {
        let mut v = self.parse_term()?;
        while let Some(ATok::Op(op)) = self.peek() {
            if *op != '+' && *op != '-' {
                break;
            }
            let op = *op;
            self.i += 1;
            let rhs = self.parse_term()?;
            v = Ast::Bin(op, Box::new(v), Box::new(rhs));
        }
        Ok(v)
    }
    fn parse_term(&mut self) -> Result<Ast, String> {
        let mut v = self.parse_unary()?;
        while let Some(ATok::Op(op)) = self.peek() {
            if *op != '*' && *op != '/' {
                break;
            }
            let op = *op;
            self.i += 1;
            let rhs = self.parse_unary()?;
            v = Ast::Bin(op, Box::new(v), Box::new(rhs));
        }
        Ok(v)
    }
    fn parse_unary(&mut self) -> Result<Ast, String> {
        if let Some(ATok::Op('-')) = self.peek() {
            self.i += 1;
            return Ok(Ast::Neg(Box::new(self.parse_unary()?)));
        }
        if let Some(ATok::Op('+')) = self.peek() {
            self.i += 1;
            return self.parse_unary();
        }
        self.parse_power()
    }
    fn parse_power(&mut self) -> Result<Ast, String> {
        let base = self.parse_atom()?;
        if let Some(ATok::Op('^')) = self.peek() {
            self.i += 1;
            let exp = self.parse_unary()?;
            return Ok(Ast::Bin('^', Box::new(base), Box::new(exp)));
        }
        Ok(base)
    }
    fn parse_atom(&mut self) -> Result<Ast, String> {
        let Some((tok, pos)) = self.toks.get(self.i).cloned() else {
            return Err(self.err_expected("a number, x, a constant, or '('"));
        };
        match tok {
            ATok::Num(text) => {
                self.i += 1;
                Ok(Ast::Num(text))
            }
            ATok::LParen => {
                self.i += 1;
                let v = self.parse_expr()?;
                match self.peek() {
                    Some(ATok::RParen) => {
                        self.i += 1;
                        Ok(v)
                    }
                    _ => Err(self.err_expected("')'")),
                }
            }
            ATok::Ident(name) => {
                self.i += 1;
                match name.as_str() {
                    "x" => Ok(Ast::Var),
                    "pi" => Ok(Ast::Const("pi")),
                    "e" => Ok(Ast::Const("e")),
                    "tau" => Ok(Ast::Const("tau")),
                    "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sqrt" | "ln"
                    | "log10" | "exp" | "abs" => {
                        match self.peek() {
                            Some(ATok::LParen) => {
                                self.i += 1;
                            }
                            _ => {
                                return Err(self.err_expected(&format!(
                                    "'(' after the function name '{name}'"
                                )))
                            }
                        }
                        let arg = self.parse_expr()?;
                        match self.peek() {
                            Some(ATok::RParen) => {
                                self.i += 1;
                            }
                            _ => return Err(self.err_expected("')'")),
                        }
                        Ok(Ast::Func(name, Box::new(arg)))
                    }
                    _ => Err(format!(
                        "Parse error at position {}: '{name}' is not a known constant, variable or function. Variable: x. Constants: pi, e, tau. Functions: sin, cos, tan, asin, acos, atan, sqrt, ln, log10, exp, abs.",
                        pos
                    )),
                }
            }
            _ => Err(self.err_expected("a number, x, a constant, or '('")),
        }
    }
}

/// Parse an expression into an AST (grammar identical to the
/// v0.4.0 expression parser, plus the variable x).
pub fn parse_ast(src: &str) -> Result<Ast, String> {
    let toks = ast_tokenize(src)?;
    if toks.is_empty() {
        return Err("The expression is empty — type something like 2*(3+4)^2 or x^2.".into());
    }
    let mut p = AstParser {
        toks,
        i: 0,
        src_len: src.chars().count(),
    };
    let v = p.parse_expr()?;
    if let Some((_, pos)) = p.toks.get(p.i) {
        return Err(format!(
            "Parse error at position {}: the expression was complete, but more input follows — check for a missing operator.",
            pos
        ));
    }
    Ok(v)
}

fn ast_has_var(a: &Ast) -> bool {
    match a {
        Ast::Var => true,
        Ast::Neg(b) => ast_has_var(b),
        Ast::Bin(_, l, r) => ast_has_var(l) || ast_has_var(r),
        Ast::Func(_, b) => ast_has_var(b),
        _ => false,
    }
}

/// Evaluate the AST at a given x (plot sampling + the exact
/// path's decimal cross-check). Mirrors rigour::eval_expression
/// semantics; any domain error or non-finite value is Err.
pub fn ast_eval(a: &Ast, x: f64, mode: AngleMode) -> Result<f64, String> {
    let deg = mode == AngleMode::Degrees;
    let v = match a {
        Ast::Num(text) => text
            .parse::<f64>()
            .map_err(|_| format!("'{text}' is not a number."))?,
        Ast::Const(name) => match *name {
            "pi" => std::f64::consts::PI,
            "tau" => std::f64::consts::TAU,
            _ => std::f64::consts::E,
        },
        Ast::Var => x,
        Ast::Neg(b) => -ast_eval(b, x, mode)?,
        Ast::Bin(op, l, r) => {
            let lv = ast_eval(l, x, mode)?;
            let rv = ast_eval(r, x, mode)?;
            match op {
                '+' => lv + rv,
                '-' => lv - rv,
                '*' => lv * rv,
                '/' => {
                    if rv == 0.0 {
                        return Err("Division by zero has no answer.".into());
                    }
                    lv / rv
                }
                '^' => lv.powf(rv),
                _ => return Err("Unknown operator.".into()),
            }
        }
        Ast::Func(name, arg) => {
            let v = ast_eval(arg, x, mode)?;
            let to_rad = |q: f64| {
                if deg {
                    q * std::f64::consts::PI / 180.0
                } else {
                    q
                }
            };
            let from_rad = |q: f64| {
                if deg {
                    q * 180.0 / std::f64::consts::PI
                } else {
                    q
                }
            };
            match name.as_str() {
                "sin" => to_rad(v).sin(),
                "cos" => to_rad(v).cos(),
                "tan" => to_rad(v).tan(),
                "asin" => {
                    if !(-1.0..=1.0).contains(&v) {
                        return Err("asin needs a value between -1 and 1.".into());
                    }
                    from_rad(v.asin())
                }
                "acos" => {
                    if !(-1.0..=1.0).contains(&v) {
                        return Err("acos needs a value between -1 and 1.".into());
                    }
                    from_rad(v.acos())
                }
                "atan" => from_rad(v.atan()),
                "sqrt" => {
                    if v < 0.0 {
                        return Err("sqrt of a negative number has no real answer.".into());
                    }
                    v.sqrt()
                }
                "ln" => {
                    if v <= 0.0 {
                        return Err("ln needs a value over 0.".into());
                    }
                    v.ln()
                }
                "log10" => {
                    if v <= 0.0 {
                        return Err("log10 needs a value over 0.".into());
                    }
                    v.log10()
                }
                "exp" => v.exp(),
                "abs" => v.abs(),
                _ => return Err(format!("Unknown function '{name}'.")),
            }
        }
    };
    if !v.is_finite() {
        return Err("That produced a number too big (or not a number).".into());
    }
    Ok(v)
}

/// Exact rational evaluation (#14 scope c): Some(value) when the
/// whole expression stays inside + - * / and whole-number powers
/// over integer/fraction literals; None as soon as anything else
/// appears (x, functions, pi/e/tau, a non-whole exponent, or i128
/// overflow) — the caller then falls back to decimal and says so.
pub fn ast_eval_rational(a: &Ast) -> Option<Rational> {
    match a {
        Ast::Num(text) => Rational::from_decimal_str(text),
        Ast::Neg(b) => ast_eval_rational(b).map(|r| r.neg()),
        Ast::Bin(op, l, r) => {
            let lv = ast_eval_rational(l)?;
            let rv = ast_eval_rational(r)?;
            match op {
                '+' => lv.add(rv),
                '-' => lv.sub(rv),
                '*' => lv.mul(rv),
                '/' => lv.div(rv),
                '^' => {
                    if !rv.is_int() || rv.num.abs() > 64 {
                        return None;
                    }
                    lv.pow(rv.num as i32)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub struct ExactOutcome {
    pub decimal: f64,
    pub exact: Option<Rational>,
}

/// Evaluate an expression for Exact mode: decimal value plus the
/// exact rational value when the expression qualifies.
pub fn exact_expression(src: &str, mode: AngleMode) -> Result<ExactOutcome, String> {
    let ast = parse_ast(src)?;
    if ast_has_var(&ast) {
        return Err(
            "This expression uses x — it belongs in the PLOT section, which samples it over a range."
                .into(),
        );
    }
    let decimal = ast_eval(&ast, 0.0, mode)?;
    let exact = ast_eval_rational(&ast);
    Ok(ExactOutcome { decimal, exact })
}

// ---------- #17 plot sampling ----------

/// Sample an expression in x over [from, to] at `count` evenly
/// spaced points (count+1 values). Undefined / non-finite points
/// are None — a gap — in both the GUI plot and the CLI CSV.
pub fn sample_function(
    src: &str,
    from: f64,
    to: f64,
    count: usize,
    mode: AngleMode,
) -> Result<Vec<(f64, Option<f64>)>, String> {
    if count == 0 {
        return Err("A plot needs at least one interval.".into());
    }
    if !from.is_finite() || !to.is_finite() || from == to {
        return Err("The plot range needs two different numbers (x from, then x to).".into());
    }
    let ast = parse_ast(src)?;
    let mut out = Vec::with_capacity(count + 1);
    for i in 0..=count {
        let x = from + (to - from) * (i as f64) / (count as f64);
        let y = ast_eval(&ast, x, mode).ok().filter(|v| v.is_finite());
        out.push((x, y));
    }
    Ok(out)
}

/// The CLI form of the plot (#17): CSV of x,y sample points;
/// y is empty where the function is undefined (a gap).
pub fn plot_csv(
    src: &str,
    from: f64,
    to: f64,
    count: usize,
    mode: AngleMode,
) -> Result<String, String> {
    let pts = sample_function(src, from, to, count, mode)?;
    let mut out = String::from("x,y\n");
    for (x, y) in pts {
        match y {
            Some(v) => out.push_str(&format!("{},{}\n", n(x), n(v))),
            None => out.push_str(&format!("{},\n", n(x))),
        }
    }
    Ok(out)
}

// ---------- result value extraction (shared by #13) ----------

/// The first result value in a calculator's output (before any
/// "Steps:" block): the first number after the first ':' on the
/// first line that has one after a colon, else after the last '=',
/// else the first number — the same reading the spec check uses.
/// This is THE value check-my-answer compares against (documented).
pub fn first_result_value(output: &str) -> Option<f64> {
    for line in output.lines() {
        if line.trim_start().starts_with("Steps:") {
            break;
        }
        let spans = crate::rigour::number_spans(line);
        if spans.is_empty() {
            continue;
        }
        let pick = |idx: usize| line[spans[idx].0..spans[idx].1].parse::<f64>().ok();
        if let Some(colon) = line.find(':') {
            if let Some(idx) = spans.iter().position(|(s, _)| *s > colon) {
                return pick(idx);
            }
        }
        if let Some(eq) = line.find('=') {
            // The FIRST value assigned in the headline is the
            // calculator's main answer ("Area = 78.54. Circumference
            // = 31.42. ..." checks against the area).
            if let Some(idx) = spans.iter().position(|(s, _)| *s > eq) {
                return pick(idx);
            }
        }
        return pick(0);
    }
    None
}

/// Calculators whose headline answer IS an exact fraction.
pub const FRACTION_EXACT_IDS: &[&str] = &[
    "fraction-simplify",
    "fraction-add",
    "fraction-subtract",
    "fraction-multiply",
    "fraction-divide",
    "decimal-to-fraction",
];

/// Extract the exact fraction a fraction-family calculator
/// printed (so a student's fraction answer can be checked
/// exactly, not by tolerance).
pub fn exact_fraction_of_output(calc: &Calc, output: &str) -> Option<Rational> {
    let head = output.lines().next()?;
    let tok_after = |marker: &str| -> Option<Rational> {
        let idx = head.find(marker)? + marker.len();
        let tok: String = head[idx..]
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '/' || *c == '-')
            .collect();
        let (p, q) = tok.split_once('/')?;
        Rational::new(p.parse().ok()?, q.parse().ok()?)
    };
    if calc.id == "fraction-simplify" {
        return tok_after("simplifies to ");
    }
    // frac_op lines end "= p/q (decimal ...)"; decimal-to-fraction
    // prints "0.75 = 3/4." — the last "= " owns the answer either way.
    let idx = head.rfind("= ")? + 2;
    let tok: String = head[idx..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '/' || *c == '-')
        .collect();
    let (p, q) = tok.split_once('/')?;
    Rational::new(p.parse().ok()?, q.parse().ok()?)
}

// ---------- #13 check my answer ----------

#[derive(Clone, Debug, PartialEq)]
pub struct CheckOutcome {
    pub correct: bool,
    /// True when the comparison was exact (fraction family).
    pub exact: bool,
    pub student: f64,
    pub expected: f64,
    pub message: String,
}

/// The generic slip detectors (#13): pure relationships between
/// the student's answer and the correct one. Returns a plain-words
/// note naming the relationship (which is exactly true) and the
/// common cause — or None when no known pattern fits.
pub fn detect_slip(student: f64, expected: f64) -> Option<String> {
    if expected == 0.0 || student == expected {
        return None;
    }
    // The student's value is usually rounded, so a slip counts
    // as matching when the relationship holds to within 0.5% —
    // the same tolerance the check itself uses. (A merely close
    // answer never reaches here: it was already marked correct,
    // so a loose match cannot mislabel a right answer's near
    // miss — slips are only reported for wrong answers.)
    let close = |a: f64, b: f64| b != 0.0 && ((a - b) / b).abs() <= 0.005;
    if student == -expected {
        return Some(
            "Your answer is the correct value with the sign flipped — check for a dropped minus sign or a subtraction done the other way round."
                .into(),
        );
    }
    let ratio = student / expected;
    for (r, note) in [
        (100.0, "Your answer is 100x the correct one — a common cause is a percent used as a whole number: divide the percent by 100 first (5% = 0.05)."),
        (0.01, "Your answer is the correct one divided by 100 — a common cause is dividing by 100 one time too many when a percent was involved."),
        (4.0, "Your answer is 4x the correct one — a common cause is using the diameter where the formula needs the radius (with r = d/2, areas come out 4x too big)."),
        (0.25, "Your answer is a quarter of the correct one — a common cause is halving something twice, e.g. using the radius where the formula needs the diameter."),
        (2.0, "Your answer is 2x the correct one — a common cause is a factor of 2 dropped or doubled: a forgotten 1/2 in the formula, a diameter used as a radius, or something counted twice."),
        (0.5, "Your answer is half the correct one — a common cause is a 1/2 applied that the formula does not have, or a radius used where the formula needs the diameter."),
    ] {
        if close(ratio, r) {
            return Some(note.into());
        }
    }
    if expected.abs() > 1.0 && close(student, expected * expected) {
        return Some(
            "Your answer is the correct value squared — a square was taken that the formula does not ask for (or an extra squaring crept in)."
                .into(),
        );
    }
    if expected > 0.0 && student > 0.0 && close(student * student, expected) {
        return Some(
            "Your answer is the square root of the correct value — a common cause is a square missed out: the formula squares something (like r in pi r^2) that was left unsquared."
                .into(),
        );
    }
    None
}

/// Run check-my-answer (#13) for a completed run: parse the
/// student's answer, compare with the calculator's first result
/// value — exactly for the fraction family, within 0.5% relative
/// otherwise (0.005 absolute when the correct answer is 0).
pub fn check_answer(calc: &Calc, output: &str, answer_text: &str) -> Result<CheckOutcome, String> {
    let student_r = parse_answer(answer_text)?;
    let student = student_r.to_f64();
    if FRACTION_EXACT_IDS.contains(&calc.id) {
        if let Some(expected_r) = exact_fraction_of_output(calc, output) {
            let expected = expected_r.to_f64();
            let correct = student_r == expected_r;
            let message = if correct {
                format!(
                    "Correct — {} is exactly right. (Checked exactly: fraction answers are compared as fractions, with no rounding tolerance.)",
                    student_r.show()
                )
            } else {
                let mut m = format!(
                    "Not yet. Your answer: {}. The correct answer: {}. Compare your working with the steps below to find where it changed.",
                    student_r.show(),
                    expected_r.show()
                );
                if let Some(slip) = detect_slip(student, expected) {
                    m.push_str(&format!("\nLikely slip: {slip}"));
                }
                m
            };
            return Ok(CheckOutcome {
                correct,
                exact: true,
                student,
                expected,
                message,
            });
        }
    }
    let expected = first_result_value(output).ok_or_else(|| {
        "This result has no single number to check against — read the worked steps instead."
            .to_string()
    })?;
    let tol = if expected == 0.0 {
        0.005
    } else {
        0.005 * expected.abs()
    };
    let correct = (student - expected).abs() <= tol;
    let message = if correct {
        format!(
            "Correct. Your answer {} matches the calculator's {} (checked within 0.5% — small rounding differences in your working are allowed).",
            n(student),
            n(expected)
        )
    } else {
        let mut m = format!(
            "Not yet. Your answer: {}. The correct answer: {}. Compare your working with the steps below to find where it changed.",
            n(student),
            n(expected)
        );
        if let Some(slip) = detect_slip(student, expected) {
            m.push_str(&format!("\nLikely slip: {slip}"));
        }
        m
    };
    Ok(CheckOutcome {
        correct,
        exact: false,
        student,
        expected,
        message,
    })
}

// ---------- #12 deeper steps + #18 export ----------

/// Stage 2 of the deeper steps: the formula with the user's own
/// values in place of the symbols. Symbols are replaced longest
/// first, on word boundaries, using the glossary order (which is
/// written in input order). Date calculators substitute their
/// merged date fields instead (symbol replacement across a date
/// triple would be nonsense).
pub fn substituted_formula(calc: &Calc, args: &[f64]) -> String {
    let formula = formula_for(calc);
    if calc.list || !calc.date_groups.is_empty() || calc.text_run.is_some() {
        let fields = crate::meta::display_fields(calc);
        let toks = display_tokens(calc, args);
        let pairs: Vec<String> = fields
            .iter()
            .zip(toks.iter())
            .map(|(f, t)| format!("{f} = {t}"))
            .collect();
        return format!("{}   (with {})", formula, pairs.join(", "));
    }
    let mut out = formula.to_string();
    let glossary = glossary_for(calc);
    let mut pairs: Vec<(&str, f64)> = glossary
        .iter()
        .enumerate()
        .filter_map(|(i, (sym, _))| args.get(i).map(|v| (*sym, *v)))
        .collect();
    pairs.sort_by_key(|(sym, _)| std::cmp::Reverse(sym.len()));
    for (sym, val) in pairs {
        out = replace_symbol(&out, sym, &n(val));
    }
    out
}

fn replace_symbol(text: &str, sym: &str, val: &str) -> String {
    if sym.is_empty() || !sym.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return text.to_string();
    }
    let mut out = String::new();
    let bytes: Vec<char> = text.chars().collect();
    let symv: Vec<char> = sym.chars().collect();
    let mut i = 0usize;
    while i < bytes.len() {
        if i + symv.len() <= bytes.len() && bytes[i..i + symv.len()] == symv[..] {
            let before_ok =
                i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == '_');
            let after = i + symv.len();
            let after_ok = after >= bytes.len()
                || !(bytes[after].is_ascii_alphanumeric() || bytes[after] == '_');
            if before_ok && after_ok {
                out.push_str(val);
                i = after;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    out
}

/// The deeper-steps text (#12): four labelled stages — formula,
/// formula with your values, the calculator's own working exactly
/// as the standard view prints it, and the result line. Working is
/// never invented: stage 3 is the run's own output, verbatim.
pub fn deeper_steps(calc: &Calc, args: &[f64], output: &str) -> String {
    let headline = output.lines().next().unwrap_or("");
    format!(
        "Step 1 — the formula:\n  {}\nStep 2 — the formula with your values in:\n  {}\nStep 3 — the working:\n{}\nStep 4 — the result:\n  {}",
        formula_for(calc),
        substituted_formula(calc, args),
        output,
        headline
    )
}

/// The plain-text worked solution written by EXPORT SOLUTION
/// (#18) — the same text from the window button and --export.
pub fn solution_text(calc: &Calc, args: &[f64], output: &str, deeper: bool) -> String {
    let fields = crate::meta::display_fields(calc);
    let toks = display_tokens(calc, args);
    let mut s = String::new();
    s.push_str(&format!("{} — worked solution (CalcDex)\n\n", calc.name));
    s.push_str(&format!("Formula: {}\n", formula_for(calc)));
    let glossary = glossary_for(calc);
    if !glossary.is_empty() {
        s.push_str("Symbols: ");
        s.push_str(
            &glossary
                .iter()
                .map(|(sym, mean)| format!("{sym} = {mean}"))
                .collect::<Vec<_>>()
                .join("; "),
        );
        s.push('\n');
    }
    s.push_str("\nInputs:\n");
    for (f, t) in fields.iter().zip(toks.iter()) {
        s.push_str(&format!("  {f}: {t}\n"));
    }
    s.push_str("\nWorking and result:\n");
    if deeper {
        s.push_str(&deeper_steps(calc, args, output));
    } else {
        s.push_str(output);
    }
    s.push('\n');
    s
}

/// The worked solution for a text (expression) run (#18).
pub fn solution_text_for_text(name: &str, text: &str, output: &str) -> String {
    format!(
        "{name} — worked solution (CalcDex)\n\nExpression: {text}\n\nWorking and result:\n{output}\n"
    )
}

/// The prefilled export path (#18): the user's Documents folder
/// when USERPROFILE is known (Windows), else their home folder.
pub fn default_export_path(calc: &Calc) -> String {
    let file = format!("CalcDex-{}-solution.txt", calc.id);
    if let Ok(up) = std::env::var("USERPROFILE") {
        if !up.is_empty() {
            return format!("{up}\\Documents\\{file}");
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return format!("{home}/Documents/{file}");
        }
    }
    file
}

// ---------- #14 exact-mode annotation lines ----------

fn pi_coeff(r: Rational) -> String {
    if r.num == 0 {
        return "0".to_string();
    }
    if r.num == 1 && r.den == 1 {
        return "pi".to_string();
    }
    if r.den == 1 {
        return format!("{}*pi", r.num);
    }
    format!("{}*pi", r.show())
}

fn rat(v: f64) -> Option<Rational> {
    Rational::from_f64_printed(v)
}

/// The exact form of a pi-family result, coefficients as exact
/// rationals of the printed inputs. Some(joined forms) for the
/// pi geometry family; None outside it.
pub fn pi_exact_forms(calc: &Calc, args: &[f64]) -> Option<String> {
    let g = |i: usize| args.get(i).copied().unwrap_or(0.0);
    let forms: Vec<String> = match calc.id {
        "circle" => {
            let r = rat(g(0))?;
            vec![
                format!("Area = {}", pi_coeff(r.mul(r)?)),
                format!("Circumference = {}", pi_coeff(r.mul(Rational::int(2))?)),
            ]
        }
        "circumference" => {
            let d = rat(g(0))?;
            vec![format!("Circumference = {}", pi_coeff(d))]
        }
        "circle-from-area" => {
            let a = g(0);
            vec![
                format!("Radius = sqrt({}/pi)", n(a)),
                format!("Circumference = 2*sqrt({}*pi)", n(a)),
            ]
        }
        "cylinder" => {
            let r = rat(g(0))?;
            let h = rat(g(1))?;
            vec![
                format!("Volume = {}", pi_coeff(r.mul(r)?.mul(h)?)),
                format!(
                    "Surface area = {}",
                    pi_coeff(r.mul(Rational::int(2))?.mul(r.add(h)?)?)
                ),
            ]
        }
        "sphere" => {
            let r = rat(g(0))?;
            vec![
                format!(
                    "Volume = {}",
                    pi_coeff(r.mul(r)?.mul(r)?.mul(Rational::new(4, 3)?)?)
                ),
                format!(
                    "Surface area = {}",
                    pi_coeff(r.mul(r)?.mul(Rational::int(4))?)
                ),
            ]
        }
        "cone" => {
            let r = rat(g(0))?;
            let h = rat(g(1))?;
            let vol = r.mul(r)?.mul(h)?.mul(Rational::new(1, 3)?)?;
            vec![
                format!("Volume = {}", pi_coeff(vol)),
                format!(
                    "Surface area = {}*pi*({} + sqrt({}))",
                    n(g(0)),
                    n(g(0)),
                    n(g(0) * g(0) + g(1) * g(1))
                ),
            ]
        }
        "sector" => {
            let r = rat(g(0))?;
            let ang = rat(g(1))?;
            let frac = ang.div(Rational::int(360))?;
            vec![
                format!(
                    "Arc length = {}",
                    pi_coeff(r.mul(Rational::int(2))?.mul(frac)?)
                ),
                format!("Sector area = {}", pi_coeff(r.mul(r)?.mul(frac)?)),
            ]
        }
        "ellipse" => {
            let a = rat(g(0))?;
            let b = rat(g(1))?;
            vec![
                format!("Area = {}", pi_coeff(a.mul(b)?)),
                "Perimeter = (no exact form — it is Ramanujan's approximation)".to_string(),
            ]
        }
        "frustum" => {
            let big = rat(g(0))?;
            let small = rat(g(1))?;
            let h = rat(g(2))?;
            let sum = big.mul(big)?.add(big.mul(small)?)?.add(small.mul(small)?)?;
            vec![format!(
                "Volume = {}",
                pi_coeff(h.mul(sum)?.mul(Rational::new(1, 3)?)?)
            )]
        }
        "square-footage-circle" => {
            let d = rat(g(0))?;
            let r = d.div(Rational::int(2))?;
            vec![format!("Area = {}", pi_coeff(r.mul(r)?))]
        }
        _ => return None,
    };
    Some(forms.join("; "))
}

/// The one Exact-mode annotation line for a numeric run (#14):
/// exact labelling for fractions, exact pi forms for the pi
/// family, or the honest fallback line.
pub fn exact_line(calc: &Calc, args: &[f64]) -> String {
    if FRACTION_EXACT_IDS.contains(&calc.id) {
        return "[Exact mode: this calculator works in exact fractions — no rounding was used anywhere.]"
            .to_string();
    }
    if let Some(forms) = pi_exact_forms(calc, args) {
        return format!("[Exact form: {forms}]");
    }
    "[Exact mode: this calculator has no exact form in Exact mode's scope (fractions, the pi geometry family, rational expressions) — the decimal result above is the answer.]"
        .to_string()
}

/// The Exact-mode annotation line for an expression run (#14).
pub fn exact_line_for_expression(src: &str, mode: AngleMode) -> String {
    match exact_expression(src, mode) {
        Ok(out) => match out.exact {
            Some(r) => format!("[Exact: {} = {}]", src.trim(), r.show()),
            None => "[Exact mode: this expression has no exact rational form (it uses x, a function, pi/e/tau, or a non-whole power) — the decimal result above is the answer.]".to_string(),
        },
        Err(_) => "[Exact mode: the expression could not be evaluated.]".to_string(),
    }
}

// ---------- #15 practice mode ----------

/// A small seeded RNG (xorshift64*) — no dependency. Same seed =
/// same sequence, on every machine.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        // Avoid the zero state (xorshift would stick at 0).
        Rng {
            state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed },
        }
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    /// Uniform in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Uniform in [lo, hi).
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }
}

/// Calculators practice mode never generates: unit-code and
/// code-field calculators, chart lookups, and the fill-one-of-two
/// kitchen converters (a generated pair would usually be invalid
/// input rather than a problem). Date calculators are excluded by
/// their date groups; list/pair/text calculators by their shape.
/// The exclusion is stated wherever practice mode is described.
/// Said wherever practice mode is offered, so the exclusion is
/// never silent (brief wording).
pub const PRACTICE_EXCLUSION_NOTE: &str = "Practice problems are not made for calculators whose inputs are unit codes, lookup tables, fill-one-of-two pairs, dates or lists — random values would not be sensible questions for them.";

pub const PRACTICE_EXCLUDED: &[&str] = &[
    "angle-convert",
    "convert-length",
    "convert-mass",
    "convert-area",
    "convert-volume",
    "convert-speed",
    "convert-time",
    "convert-data",
    "convert-pressure",
    "convert-energy",
    "convert-power-units",
    "convert-force-units",
    "convert-torque-units",
    "convert-density-units",
    "convert-frequency",
    "convert-flow",
    "convert-illuminance",
    "convert-temperature",
    "convert-fuel-economy",
    "compound-density",
    "compound-fuel-consumption",
    "compound-data-rate",
    "compound-concentration",
    "compound-energy-density",
    "grams-cups-flour",
    "grams-cups-sugar",
    "grams-cups-brown-sugar",
    "grams-cups-butter",
    "grams-cups-cocoa",
    "grams-cups-rice",
    "grams-cups-oats",
    "grams-cups-honey",
    "grams-cups-milk",
    "grams-cups-water",
    "grams-cups-oil",
    "grams-cups-powdered-sugar",
    "shoe-size-men",
    "ideal-weight-devine",
    "bac-widmark",
    "dog-age",
    "dice-probability",
];

pub fn practice_eligible(calc: &Calc) -> bool {
    !calc.list
        && !calc.pairs
        && calc.text_run.is_none()
        && calc.date_groups.is_empty()
        && !PRACTICE_EXCLUDED.contains(&calc.id)
}

/// The calculator ids practice can pick (for RANDOM mode).
pub fn practice_pool(calcs: &[Calc]) -> Vec<usize> {
    calcs
        .iter()
        .enumerate()
        .filter(|(_, c)| practice_eligible(c))
        .map(|(i, _)| i)
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct PracticeProblem {
    pub calc_id: String,
    pub args: Vec<f64>,
    /// The question text: calculator name + inputs, answer hidden.
    pub question: String,
}

/// Generate one practice problem for `calc` from `rng`: each
/// example value is varied by a random factor in [0.5, 2) (a zero
/// example becomes a fresh 1–10 draw; whole-number examples stay
/// whole). Candidates that the calculator itself rejects are
/// retried (up to 60 draws), so a generated problem always runs.
pub fn practice_problem(calc: &Calc, rng: &mut Rng) -> Option<PracticeProblem> {
    if !practice_eligible(calc) {
        return None;
    }
    let example = crate::meta::example_for(calc);
    if example.is_empty() {
        return None;
    }
    for _ in 0..60 {
        let mut args = Vec::with_capacity(example.len());
        for v in example {
            let varied = if *v == 0.0 {
                rng.range(1.0, 10.0)
            } else if v.abs() >= 50.0 {
                // Large example values (heights, prices, weights)
                // stay in a plausible band around the example.
                v * rng.range(0.8, 1.25)
            } else {
                v * rng.range(0.5, 2.0)
            };
            let varied = if *v == v.trunc() && v.abs() < 1e6 {
                varied.round()
            } else {
                (varied * 100.0).round() / 100.0
            };
            args.push(varied);
        }
        if run_calc(calc.id, &args).is_ok() {
            let fields = crate::meta::display_fields(calc);
            let toks = display_tokens(calc, &args);
            let listed: Vec<String> = fields
                .iter()
                .zip(toks.iter())
                .map(|(f, t)| format!("{f} = {t}"))
                .collect();
            return Some(PracticeProblem {
                calc_id: calc.id.to_string(),
                args,
                question: format!(
                    "{} with {}. Work out the first answer it gives.",
                    calc.name,
                    listed.join(", ")
                ),
            });
        }
    }
    None
}

/// Mix a seed with a calculator id so `--seed N` gives each
/// calculator its own (still fully reproducible) sequence.
fn seed_for(seed: u64, id: &str) -> u64 {
    let mut h = seed ^ 0xA0761D6478BD642F;
    for b in id.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0xE7037ED1A0B428DB);
        h ^= h >> 32;
    }
    h
}

/// The first problem of a seeded sequence (the CLI --practice
/// form: the problem depends only on the calculator and the seed,
/// so --answer can recompute exactly the same problem).
pub fn practice_problem_seeded(calc: &Calc, seed: u64) -> Option<PracticeProblem> {
    let mut rng = Rng::new(seed_for(seed, calc.id));
    practice_problem(calc, &mut rng)
}

/// Pick a random eligible calculator and generate its problem
/// (GUI RANDOM practice).
pub fn practice_random(calcs: &[Calc], rng: &mut Rng) -> Option<(usize, PracticeProblem)> {
    let pool = practice_pool(calcs);
    if pool.is_empty() {
        return None;
    }
    for _ in 0..40 {
        let idx = pool[(rng.next_u64() % pool.len() as u64) as usize];
        if let Some(p) = practice_problem(&calcs[idx], rng) {
            return Some((idx, p));
        }
    }
    None
}
