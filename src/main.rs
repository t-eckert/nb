mod agent;
mod cli;
mod config;
mod document;
mod editor;
mod notebook;

use clap::Parser;
use cli::{Cli, Commands, LogAction};
use config::Config;

fn main() {
    let cli = Cli::parse();

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Log { action } => match action {
            LogAction::Edit {
                yesterday,
                tomorrow,
                date,
            } => notebook::edit_log(&config, yesterday, tomorrow, date.as_deref()),
            LogAction::Rollover => notebook::rollover_todos(&config),
            LogAction::List {
                days,
                show_unfinished,
            } => notebook::list_logs(&config, days, show_unfinished),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
