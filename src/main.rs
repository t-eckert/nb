mod cli;
mod notebook;

use clap::Parser;
use cli::{Cli, Commands, LogAction};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Log { action } => match action {
            LogAction::Edit {
                yesterday,
                tomorrow,
            } => notebook::edit_log(yesterday, tomorrow),
            LogAction::Rollover => notebook::rollover_todos(),
            LogAction::List { days } => notebook::list_logs(days),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
