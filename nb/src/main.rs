use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Thomas Eckert")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(version = "0.1.0")]
    Log(Log),
}

#[derive(Parser)]
struct Log {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    println!("Hello, world!");
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Log(_) => {
            println!("Hello")
        }
    }
}
