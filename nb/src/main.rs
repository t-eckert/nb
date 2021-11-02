#[macro_use]
extern crate clap;

use clap::Parser;
use config::Config;
use subcmd::{init, log, open, serve};

mod config;
mod editor;
mod subcmd;

#[derive(Parser)]
#[clap(about=crate_description!(), version=crate_version!(), author=crate_authors!())]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(about = "Create a new notebook in the current directory.")]
    Init(init::Init),

    #[clap(about = "Open today's log in your editor.")]
    Log(log::Log),

    #[clap(about = "Open the notebook in your editor.")]
    Open(open::Open),

    #[clap(about = "Serve the notebook in the browser.")]
    Serve(serve::Serve),
}

fn main() {
    let opts: Opts = Opts::parse();

    let config = Config {
        editor_cmd: "nvim".to_string(),
        ..Config::default()
    };

    match opts.subcmd {
        SubCommand::Init(args) => init::run(args, &config),
        SubCommand::Log(args) => subcmd::log::run(args, &config),
        SubCommand::Open(args) => subcmd::open::run(args, &config),
        SubCommand::Serve(args) => subcmd::serve::run(args, &config),
    }
}
