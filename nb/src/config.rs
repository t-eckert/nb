/// Configuration for the notabene CLI.
pub struct Config {
    /// editor_cmd is the command to use for opening the editor (e.g. `vi`).
    pub editor_cmd: String,

    /// nb_dir is the directory where notabene stores its notes.
    pub nb_dir: String,

    /// port is where user interface and files will be served.
    pub port: u16,

    /// log_template is the template for daily log files.
    pub log_template: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            editor_cmd: "vi".to_string(),
            nb_dir: "~/Notebook".to_string(),
            port: 8080,
            log_template: "## Tasks\n\n##Notes\n".to_string(),
        }
    }
}
