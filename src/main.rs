mod run_file;
mod run_repl;
mod run_tui;
mod state;

pub use state::State;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Tui,
    Repl,
    File(FileArgs),
}

#[derive(Args, Debug, Clone)]
struct FileArgs {
    #[arg()]
    pub path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    match args.command {
        Commands::Tui => {
            run_tui::run_tui()?;
        }
        Commands::File(args) => {
            run_file::run_file(args.path)?;
        }
        Commands::Repl => {
            run_repl::run_repl()?;
        }
    }

    println!("Bye, and thanks for using Rambda! <3");
    Ok(())
}
