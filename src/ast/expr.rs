use super::RedType;
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Expr {
    // Variable: identified by a name
    Var(String),

    // Abstraction: λx.e (a function with parameter and body)
    Abs(String, Rc<Expr>),

    // Apllication: e1 e2 (function application)
    Apl(Rc<Expr>, Rc<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Abs(param, body) => write!(f, "λ{}.{}", param, body),
            Expr::Apl(e1, e2) => match **e2 {
                Expr::Apl(_, _) => write!(f, "{} ({})", e1, e2),
                _ => write!(f, "{} {}", e1, e2),
            },
        }
    }
}

impl Expr {
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

    fn substitute_once(&self, var: &str, replacement: &Expr) -> (Expr, RedType) {
        match self {
            Expr::Var(name) if name == var => {
                (replacement.clone(), RedType::BetaReduction(var.to_string()))
            }
            Expr::Var(_) => (self.clone(), RedType::NoReduction),
            Expr::Abs(param, body) => {
                if param == var {
                    (self.clone(), RedType::NoReduction)
                } else if replacement.is_free_in(param) {
                    let fresh = body.fresh_var(param);
                    let renamed_body = body.substitute_once(param, &Expr::Var(fresh.clone())).0;
                    (
                        Expr::Abs(fresh.clone(), Rc::new(renamed_body)),
                        RedType::AlphaConversion(param.clone(), fresh),
                    )
                } else {
                    let (new_body, red) = body.substitute_once(var, replacement);
                    (Expr::Abs(param.clone(), Rc::new(new_body)), red)
                }
            }
            Expr::Apl(e1, e2) => {
                let (new_e1, red1) = e1.substitute_once(var, replacement);
                if red1 != RedType::NoReduction {
                    (Expr::Apl(Rc::new(new_e1), e2.clone()), red1)
                } else {
                    let (new_e2, red2) = e2.substitute_once(var, replacement);
                    if red2 != RedType::NoReduction {
                        (Expr::Apl(e1.clone(), Rc::new(new_e2)), red2)
                    } else {
                        (self.clone(), RedType::NoReduction)
                    }
                }
            }
        }
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

    pub fn eval_step(&self) -> (Expr, RedType) {
        match self {
            Expr::Apl(e1, e2) => {
                if let Expr::Abs(param, body) = &**e1 {
                    return body.substitute_once(param, &**e2);
                }

                let (reduced_e1, red1) = e1.eval_step();
                if red1 != RedType::NoReduction {
                    return (
                        Expr::Apl(Rc::new(reduced_e1), e2.clone()),
                        RedType::ContextualReduction("left".to_string()),
                    );
                }

                let (reduced_e2, red2) = e2.eval_step();
                if red2 != RedType::NoReduction {
                    return (
                        Expr::Apl(e1.clone(), Rc::new(reduced_e2)),
                        RedType::ContextualReduction("right".to_string()),
                    );
                }

                (self.clone(), RedType::NoReduction)
            }
            Expr::Abs(param, body) => {
                let (reduced_body, red) = body.eval_step();
                if red != RedType::NoReduction {
                    return (Expr::Abs(param.clone(), Rc::new(reduced_body)), red);
                }
                (self.clone(), RedType::NoReduction)
            }
            Expr::Var(_) => (self.clone(), RedType::NoReduction),
        }
    }

    pub fn eval_full(&self) -> (Expr, Vec<RedType>) {
        let mut reductions = Vec::new();
        let mut expr = self.clone();
        while !expr.is_normal_form() {
            let (next_expr, reduction_type) = expr.eval_step();
            if reduction_type == RedType::NoReduction {
                break;
            }
            expr = next_expr;
            reductions.push(reduction_type);
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

pub fn var(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn abs(param: &str, body: Expr) -> Expr {
    Expr::Abs(param.to_string(), Rc::new(body))
}

pub fn apl(e1: Expr, e2: Expr) -> Expr {
    Expr::Apl(Rc::new(e1), Rc::new(e2))
}
