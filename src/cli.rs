use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nb")]
#[command(about = "NotaBene - A CLI for managing your markdown notebook", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage daily logs
    Log {
        #[command(subcommand)]
        action: LogAction,
    },
}

#[derive(Subcommand)]
pub enum LogAction {
    /// Edit a daily log
    Edit {
        /// Edit yesterday's log
        #[arg(short, long)]
        yesterday: bool,
        /// Edit tomorrow's log
        #[arg(short, long)]
        tomorrow: bool,
        /// Edit log for a specific date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
    },
    /// View a daily log
    View {
        /// View yesterday's log
        #[arg(short, long)]
        yesterday: bool,
        /// View tomorrow's log
        #[arg(short, long)]
        tomorrow: bool,
        /// View log for a specific date (YYYY-MM-DD)
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Rollover unfinished TODOs to the next day
    Rollover,
    /// List recent logs
    List {
        /// Number of days to show (default: 7)
        #[arg(short, long, default_value = "7")]
        days: usize,
        /// Show unfinished TODOs under each day
        #[arg(long)]
        show_unfinished: bool,
    },
}
