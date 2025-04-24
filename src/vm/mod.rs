use crate::{
    ast::{Expr, RedType},
    lexer::{Lexer, Parser},
};
use anyhow::{anyhow, Result};

pub struct Vm {
    current_expr: Option<Expr>,
    lexer: Lexer,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            current_expr: None,
            lexer: Lexer::new(),
        }
    }

    pub fn parse_expr(&mut self, input: &str) -> Result<()> {
        self.lexer.reload(input);
        let tokens = self.lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let parsed = parser
            .parse()
            .ok_or(anyhow!("Failed to parse expression"))?;
        self.current_expr = Some(parsed);

        Ok(())
    }

    pub fn get_expr(&self) -> Option<&Expr> {
        self.current_expr.as_ref()
    }

    pub fn eval(&mut self) -> Result<Vec<(RedType, Expr)>> {
        let mut steps: Vec<(RedType, Expr)> = vec![];

        let mut is_normal_form = false;
        while !is_normal_form {
            if let Some(expr) = &self.current_expr {
                let (next_expr, reduction_type) = expr.eval_step();
                if reduction_type != RedType::NoReduction {
                    steps.push((reduction_type, next_expr.clone()));
                }
                self.current_expr = Some(next_expr);
            } else {
                return Err(anyhow!("No expression to evaluate"));
            }
            is_normal_form = self.current_expr.as_ref().unwrap().is_normal_form();
        }

        let (simplified, reductions) = self.current_expr.as_ref().unwrap().simplify_numbered_vars();
        for reduction in reductions {
            if let RedType::AlphaConversion(old, new) = reduction {
                steps.push((RedType::AlphaConversion(old, new), simplified.clone()));
            }
        }

        self.current_expr = Some(simplified);
        Ok(steps)
    }
}
