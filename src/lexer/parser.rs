use crate::ast::{abs, apl, var, Expr};

use super::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Some(Token::Lambda) => {
                self.advance(); // Consume λ

                let mut params = Vec::new();

                // Parse parameters separated by commas
                while let Some(Token::Identifier(name)) = self.peek() {
                    let param = name.clone();
                    self.advance();
                    params.push(param);

                    // Check for a comma or dot
                    match self.peek() {
                        Some(Token::Comma) => {
                            self.advance(); // Consume comma
                        }
                        Some(Token::Dot) => {
                            self.advance(); // Consume dot
                            break;
                        }
                        _ => return None, // Expect comma or dot, otherwise error
                    }
                }

                // Parse body of the lambda
                let body = self.parse_expr()?;
                let mut lambda_expr = body;

                // If we have multiple parameters, apply them in sequence
                for param in params.into_iter().rev() {
                    lambda_expr = abs(&param, lambda_expr);
                }

                Some(lambda_expr)
            }
            Some(Token::Identifier(name)) => {
                let result = var(&name);
                self.advance();
                Some(result)
            }
            Some(Token::LeftParen) => {
                self.advance(); // Consume (
                let expr = self.parse_expr()?;

                match self.peek() {
                    Some(Token::RightParen) => {
                        self.advance();
                        Some(expr)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn parse_application(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        // Parse application (left-to-right associativity)
        while let Some(Token::LeftParen) | Some(Token::Identifier(_)) = self.peek() {
            let arg = self.parse_primary()?;
            expr = apl(expr, arg);
        }

        Some(expr)
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_application()
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let expr = self.parse_expr();

        if self.position == self.tokens.len() {
            expr
        } else {
            None
        }
    }
}
