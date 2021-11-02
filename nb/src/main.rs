#[macro_use]
extern crate clap;

use clap::Parser;
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
    #[clap(
        version = "0.1.0",
        about = "Create a new notebook in the current directory."
    )]
    Init(init::Init),

    #[clap(version = "0.1.0", about = "Open today's log in your editor.")]
    Log(log::Log),

    #[clap(version = "0.1.0", about = "Open the notebook in your editor.")]
    Open(open::Open),

    #[clap(version = "0.1.0", about = "Serve the notebook in the browser.")]
    Serve(serve::Serve),
}

fn main() {
    let opts: Opts = Opts::parse();

    let config = config::Config {
        editor_cmd: String::from("nvim"),
        nb_dir: String::from("~/Notebook"),
        port: 4000,
    };

    match opts.subcmd {
        SubCommand::Init(args) => init::run(args, &config),
        SubCommand::Log(args) => subcmd::log::run(args, &config),
        SubCommand::Open(args) => subcmd::open::run(args, &config),
        SubCommand::Serve(args) => subcmd::serve::run(args, &config),
    }
}
