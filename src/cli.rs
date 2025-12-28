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
        #[arg(long)]
        yesterday: bool,
        /// Edit tomorrow's log
        #[arg(long)]
        tomorrow: bool,
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
