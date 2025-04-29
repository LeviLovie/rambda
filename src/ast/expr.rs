use super::RedType;
use std::{collections::HashSet, rc::Rc};

pub fn var(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn abs(param: &str, body: Expr) -> Expr {
    Expr::Abs(param.to_string(), Rc::new(body))
}

pub fn apl(e1: Expr, e2: Expr) -> Expr {
    Expr::Apl(Rc::new(e1), Rc::new(e2))
}

#[derive(Debug, Clone)]
pub enum Expr {
    // Variable: identified by a name
    Var(String),

    // Abstraction: λx.e (a function with parameter and body)
    Abs(String, Rc<Expr>),

    // Apllication: e1 e2 (function application)
    Apl(Rc<Expr>, Rc<Expr>),
}

impl Expr {
    pub fn fmt_with_config(&self, color: bool, ascii: bool, merge: bool) -> String {
        let reset = if color { "\x1b[0m" } else { "" };
        let lambda = if color { "\x1b[1m\x1b[38;5;2m" } else { "" };
        let var = if color { "\x1b[1m\x1b[38;5;4m" } else { "" };
        let gray = if color { "\x1b[0m\x1b[38;5;240m" } else { "" };

        match self {
            Expr::Var(name) => name.clone(),
            Expr::Abs(from, to) => {
                if merge {
                    let (params, body) = self.collect_abstractions();
                    format!(
                        "{}λ{}{}{}.{}{}",
                        lambda,
                        var,
                        params.join(","),
                        gray,
                        reset,
                        body.fmt_with_config(color, ascii, merge),
                    )
                } else {
                    format!(
                        "{}λ{}{}{}.{}{}",
                        lambda,
                        var,
                        from,
                        gray,
                        reset,
                        to.fmt_with_config(color, ascii, merge),
                    )
                }
            }
            Expr::Apl(expr, _) => match &**expr {
                Expr::Apl(_, _) | Expr::Abs(_, _) => {
                    format!(
                        "{}({}{}{}){}",
                        gray,
                        reset,
                        expr.fmt_with_config(color, ascii, merge),
                        gray,
                        reset
                    )
                }
                _ => expr.fmt_with_config(color, ascii, merge),
            },
        }
    }

    fn collect_abstractions(&self) -> (Vec<String>, &Expr) {
        let mut params = Vec::new();
        let mut current = self;

        while let Expr::Abs(param, body) = current {
            params.push(param.clone());
            current = body;
        }

        (params, current)
    }

    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Expr::Var(name) => [name.clone()].into_iter().collect(),
            Expr::Abs(param, body) => {
                let mut vars = body.free_vars();
                vars.remove(param);
                vars
            }
            Expr::Apl(e1, e2) => {
                let mut vars = e1.free_vars();
                vars.extend(e2.free_vars());
                vars
            }
        }
    }

    pub fn bound_vars(&self) -> HashSet<String> {
        match self {
            Expr::Var(_) => HashSet::new(),
            Expr::Abs(param, body) => {
                let mut set = body.bound_vars();
                set.insert(param.clone());
                set
            }
            Expr::Apl(e1, e2) => {
                let mut set = e1.bound_vars();
                set.extend(e2.bound_vars());
                set
            }
        }
    }
}

impl Expr {
    fn is_free_in(&self, var: &str) -> bool {
        match self {
            Expr::Var(name) => name == var,
            Expr::Abs(param, body) => param != var && body.is_free_in(var),
            Expr::Apl(e1, e2) => e1.is_free_in(var) || e2.is_free_in(var),
        }
    }

    fn fresh_var(&self, hint: &str) -> String {
        let mut new_name = hint.to_string();
        let mut counter = 0;

        while self.is_free_in(&new_name) {
            counter += 1;
            new_name = format!("{}_{}", hint, counter);
        }

        new_name
    }

    pub fn is_redex(&self) -> bool {
        matches!(self, Expr::Apl(e1, _) if matches!(**e1, Expr::Abs(_, _)))
    }

    pub fn is_normal_form(&self) -> bool {
        match self {
            Expr::Var(_) => true,
            Expr::Abs(_, body) => body.is_normal_form(),
            Expr::Apl(e1, e2) => !self.is_redex() && e1.is_normal_form() && e2.is_normal_form(),
        }
    }

    pub fn substitute(&self, var: &str, expr: &Expr) -> (Expr, Vec<RedType>) {
        let mut result = self.clone();
        let mut reds: Vec<RedType> = vec![];

        match self {
            Expr::Var(v) => {
                if v == var {
                    result = expr.clone();
                }
            }
            Expr::Abs(param, body) => {
                if expr.is_free_in(param) {
                    let fresh = body.fresh_var(param);
                    let renamed_body = body.substitute(param, &Expr::Var(fresh.clone())).0;
                    result = abs(&fresh, renamed_body);
                    reds.push(RedType::AlphaConversion(param.clone(), fresh));
                } else {
                    let (new_body, red) = body.substitute(var, expr);
                    result = abs(&param, new_body);
                    reds.extend(red);
                }
                if param != var {
                    let (body, reds1) = body.substitute(var, expr);
                    result = abs(&param, body);
                    reds.extend(reds1);
                }
            }
            Expr::Apl(e1, e2) => {
                let (new_e1, reds1) = e1.substitute(var, expr);
                let (new_e2, reds2) = e2.substitute(var, expr);
                reds.extend(reds1);
                reds.extend(reds2);
                result = apl(new_e1, new_e2);
            }
        };

        (result, reds)
    }

    pub fn eval_step(&self) -> (Expr, Vec<RedType>) {
        let mut result = self.clone();
        let mut reds = Vec::new();

        match self {
            Expr::Apl(e1, e2) => {
                println!("Eval apl");
                if let Expr::Abs(param, body) = &**e1 {
                    println!("Found abs");
                    let (body, reds1) = body.substitute(param, &**e2);
                    result = body;
                    reds.extend(reds1);
                    return (result, reds);
                }

                let (new_e1, reds1) = e1.eval_step();
                if !reds1.is_empty() {
                    println!("Evaluated e1");
                    reds.extend(reds1);
                    result = apl(new_e1, (**e2).clone());
                    reds.push(RedType::ContextualReduction("l".to_string()));
                    return (result, reds);
                }

                let (new_e2, reds2) = e2.eval_step();
                if !reds1.is_empty() {
                    println!("Evaluated e2");
                    reds.extend(reds2);
                    result = apl((**e1).clone(), new_e2);
                    reds.push(RedType::ContextualReduction("r".to_string()));
                    return (result, reds);
                }
            }
            Expr::Abs(param, body) => {
                println!("Eval abs");

                let (body, reds1) = body.eval_step();
                result = abs(param, body);
                reds.extend(reds1);
                return (result, reds);
            }
            Expr::Var(name) => {
                println!("Eval var");
                return (result, reds);
            }
        };

        (result, reds)
    }

    pub fn eval_full(&self) -> (Expr, Vec<RedType>) {
        let mut reductions = Vec::new();
        let mut expr = self.clone();
        while !expr.is_normal_form() {
            let (next_expr, reds) = expr.eval_step();
            expr = next_expr;
            reductions.extend(reds);
        }
        (expr, reductions)
    }

    fn split_name_number(name: &str) -> Option<(String, u32)> {
        let mut chars = name.chars().rev().peekable();
        let mut number = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        if number.is_empty() {
            return None;
        }

        let base: String = chars.collect::<Vec<_>>().into_iter().rev().collect();
        let num = number
            .chars()
            .rev()
            .collect::<String>()
            .parse::<u32>()
            .ok()?;
        Some((base, num))
    }

    fn rename_var(&self, from: &str, to: &str) -> Expr {
        match self {
            Expr::Var(name) if name == from => Expr::Var(to.to_string()),
            Expr::Var(_) => self.clone(),
            Expr::Abs(param, _) if param == from => self.clone(), // shadowed, stop
            Expr::Abs(param, body) => Expr::Abs(param.clone(), Rc::new(body.rename_var(from, to))),
            Expr::Apl(e1, e2) => Expr::Apl(
                Rc::new(e1.rename_var(from, to)),
                Rc::new(e2.rename_var(from, to)),
            ),
        }
    }

    pub fn simplify_numbered_vars(&self) -> (Expr, Vec<RedType>) {
        let mut reductions = Vec::new();
        let mut expr = self.clone();
        match self {
            Expr::Var(name) => {
                if let Some((base, num)) = Self::split_name_number(name) {
                    if num == 1 {
                        let base = base.trim_end_matches('_').to_string();
                        expr = Expr::Var(base.clone());
                        reductions.push(RedType::AlphaConversion(name.clone(), base));
                    } else {
                        let new_name = format!("{}_{}", base, num - 1);
                        if self.is_free_in(&new_name) {
                            expr = Expr::Var(new_name.clone());
                            reductions.push(RedType::AlphaConversion(name.clone(), new_name));
                        }
                    }
                }
            }
            Expr::Abs(param, body) => {
                let mut param = param.clone();
                let mut body = body.clone();

                // Try simplifying the bound var name
                if let Some((base, num)) = Self::split_name_number(&param) {
                    if num == 1 {
                        let base = base.trim_end_matches('_').to_string();
                        if !body.is_free_in(&base) {
                            // Rename both the parameter and its uses in the body
                            body = body.rename_var(&param, &base).into();
                            reductions.push(RedType::AlphaConversion(param.clone(), base.clone()));
                            param = base;
                        }
                    }
                }

                let (new_body, reds) = body.simplify_numbered_vars();
                expr = Expr::Abs(param, Rc::new(new_body));
                reductions.extend(reds);
            }
            Expr::Apl(e1, e2) => {
                let (new_e1, reds_e1) = e1.simplify_numbered_vars();
                let (new_e2, reds_e2) = e2.simplify_numbered_vars();
                expr = Expr::Apl(Rc::new(new_e1), Rc::new(new_e2));
                reductions.extend(reds_e1);
                reductions.extend(reds_e2);
            }
        };

        (expr, reductions)
    }
}
