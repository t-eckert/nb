use crate::config::Config;
use crate::editor;

#[derive(Parser)]
pub struct Open {}

pub fn run(args: Open, config: &Config) {
    editor::open(&config.editor_cmd, &config.nb_dir);
}
