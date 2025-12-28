mod cli;
mod editor;
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
                date,
            } => notebook::edit_log(yesterday, tomorrow, date.as_deref()),
            LogAction::Rollover => notebook::rollover_todos(),
            LogAction::List { days, show_unfinished } => notebook::list_logs(days, show_unfinished),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
