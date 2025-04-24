use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RedType {
    // (λx.M) N → M[x := N]
    BetaReduction(String), // variable name for clarity

    // λx.M → λy.M[x := y] where y is not free in M
    AlphaConversion(String, String), // from, to

    // Reduction inside a subexpression
    ContextualReduction(String),

    // No reduction performed
    NoReduction,
}

impl fmt::Display for RedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedType::BetaReduction(var) => write!(f, "β({})", var),
            RedType::AlphaConversion(from, to) => write!(f, "α({} -> {})", from, to),
            RedType::ContextualReduction(var) => write!(f, "C({})", var),
            RedType::NoReduction => write!(f, "No reduction"),
        }
    }
}
