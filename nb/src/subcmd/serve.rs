use crate::config::Config;

#[derive(Parser)]
pub struct Serve {
	#[clap(short, long)]
	verbose: bool,
}

pub fn run(args: Serve, config: &Config) {
	println!("Serve not implemented")
}
