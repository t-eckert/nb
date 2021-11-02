use chrono::{DateTime, Local, TimeZone};

use crate::config::Config;
use crate::editor;

#[derive(Parser)]
pub struct Log {
    #[clap(short, long)]
    offset: Option<i64>,
}

pub fn run(args: Log, config: &Config) {
    let offset = args.offset.unwrap_or(0);
    println!("{}", offset);

    let log_file_path = format_file_path(&config.nb_dir, Local::now());

    if !does_exist(&log_file_path) {}

    editor::open(&config.editor_cmd, &log_file_path);
}

fn format_file_path<T: TimeZone>(nb_dir: &str, date: DateTime<T>) -> String {
    format!(
        "{}/Log/{}.md",
        nb_dir,
        date.naive_local().format("%Y-%m-%d")
    )
}

fn does_exist(path: &str) -> bool {
    false
}

fn write_to_file(path: &str, content: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_path() {
        let nb_dir = "/home/user/notebook";
        let date =
            DateTime::parse_from_str("2020-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z").unwrap();

        assert_eq!(
            format_file_path(nb_dir, date),
            "/home/user/notebook/Log/2020-01-01.md"
        );
    }
}
