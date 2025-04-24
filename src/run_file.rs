use super::State;
use anyhow::{anyhow, Result};

pub fn run_file(file_name: String) -> Result<()> {
    let mut state = State::new();
    if !std::path::Path::new(&file_name).exists() {
        return Err(anyhow!("File not found"));
    }
    let contents = std::fs::read_to_string(file_name)?;
    let lines = contents.lines();
    for line in lines {
        state.exec(line.to_string());
        if state.exit {
            break;
        }
    }

    let history = state.history.join("\n");
    println!("{}", history);

    Ok(())
}
