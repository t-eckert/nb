/// Configuration for the notabene CLI.
pub struct Config {
	/// editor_cmd is the command to use for opening the editor (e.g. `vi`).
	pub editor_cmd: String,
	pub nb_dir: String,
	pub port: u16,
}
