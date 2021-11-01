use crate::config::Config;

#[derive(Parser)]
pub struct Init {
	#[clap(short, long)]
	verbose: bool,
}

pub fn run(args: Init, config: &Config) {
	println!("Init not implemented")
}
