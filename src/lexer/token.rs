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
