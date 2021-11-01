use crate::config::Config;
use crate::editor;

#[derive(Parser)]
pub struct Open {
	#[clap(short, long)]
	verbose: bool,
}

pub fn run(args: Open, config: &Config) {
	editor::open(&config.editor_cmd, &config.nb_dir);
}
