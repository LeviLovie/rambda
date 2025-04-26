#[derive(Debug, Clone, PartialEq)]
pub enum RedType {
    // (λx.M) N → M[x := N]
    BetaReduction(String), // variable name for clarity

    // λx.M → λy.M[x := y] where y is not free in M
    AlphaConversion(String, String), // from, to

    // Reduction inside a subexpression
    ContextualReduction(String),

    // Simplification of a term
    Simplification(String),

    // No reduction performed
    NoReduction,
}

impl RedType {
    pub fn fmt_with_config(&self, color: bool, utf8: bool) -> String {
        let reset = if color { "\x1b[0m" } else { "" };
        let type_ = if color { "\x1b[1m\x1b[38;5;3m" } else { "" };
        let gray = if color { "\x1b[0m\x1b[38;5;240m" } else { "" };
        let alpha = if utf8 { "α" } else { "A" };
        let beta = if utf8 { "β" } else { "B" };
        let gamma = if utf8 { "γ" } else { "C" };

        match self {
            RedType::AlphaConversion(from, to) => {
                format!(
                    "{}->{}{}{}({}{}{} > {}{}{}){}",
                    gray, type_, alpha, gray, reset, from, gray, reset, to, gray, reset
                )
            }
            RedType::BetaReduction(var) => {
                format!(
                    "{}->{}{}{}({}{}{}){}",
                    gray, type_, beta, gray, reset, var, gray, reset
                )
            }
            RedType::ContextualReduction(var) => {
                format!(
                    "{}->{}{}{}({}{}{}){}",
                    gray, type_, gamma, gray, reset, var, gray, reset
                )
            }
            RedType::Simplification(var) => {
                format!(
                    "{}->{}{}{}({}{}{}){}",
                    gray, type_, gamma, gray, reset, var, gray, reset
                )
            }
            RedType::NoReduction => "No reduction".into(),
        }
    }
}
