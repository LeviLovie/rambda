use super::Token;

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
