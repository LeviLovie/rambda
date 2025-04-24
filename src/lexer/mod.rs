use crate::{abs, apl, var, Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lambda,             // λ or \
    Dot,                // .
    Comma,              // ,
    Identifier(String), // Variable names
    LeftParen,          // (
    RightParen,         // )
    Whitespace,         // Spaces, tabs, etc.
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            input: "".chars().collect(),
            position: 0,
        }
    }

    pub fn load(&mut self, input: &str) {
        self.input = input.chars().collect();
        self.position = 0;
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.position = 0;
    }

    pub fn reload(&mut self, input: &str) {
        self.clear();
        self.load(input);
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> Option<Token> {
        // Skip whitespace
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            if c != '\n' {
                break;
            }
            self.advance();
        }

        let current = self.peek()?;

        match current {
            'λ' | '\\' => {
                self.advance();
                Some(Token::Lambda)
            }
            '.' => {
                self.advance();
                Some(Token::Dot)
            }
            ',' => {
                self.advance();
                Some(Token::Comma)
            }
            '(' => {
                self.advance();
                Some(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Some(Token::RightParen)
            }
            c if c.is_alphanumeric() || c == '_' => {
                let mut identifier = String::new();

                while let Some(c) = self.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                Some(Token::Identifier(identifier))
            }
            _ => {
                self.advance();
                // TODO: Output warning here
                self.next_token()
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}

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
