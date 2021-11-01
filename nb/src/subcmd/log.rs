use crate::config::Config;
use crate::editor;

#[derive(Parser)]
pub struct Log {
	#[clap(short, long)]
	verbose: bool,
}

pub fn run(args: Log, config: &Config) {
	editor::open(&config.editor_cmd, &config.nb_dir);
}
