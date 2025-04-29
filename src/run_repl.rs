use super::State;
use anyhow::Result;
use rambda::ast::Expr;
use std::io::{self, Write};

pub fn run_repl() -> Result<()> {
    let mut state = State::new()?;
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            // EOF (Ctrl-D or Ctrl-Z)
            break;
        }

        let line = line.trim();
        if line == "exit" || line == "quit" {
            break;
        }

        state.exec(line.to_string());
        println!("{}", state.history.join("\n"));
        state.history.clear();
    }

    Ok(())
}
