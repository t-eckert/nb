use chrono::{Duration, Local, NaiveDate};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn get_notebook_path() -> PathBuf {
    env::var("NOTEBOOK_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME environment variable not set");
            PathBuf::from(home).join("Notebook")
        })
}

fn get_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
}

fn get_date_offset(yesterday: bool, tomorrow: bool) -> NaiveDate {
    let today = Local::now().date_naive();

    if yesterday {
        today - Duration::days(1)
    } else if tomorrow {
        today + Duration::days(1)
    } else {
        today
    }
}

fn get_log_path(date: NaiveDate) -> PathBuf {
    let notebook_path = get_notebook_path();
    let filename = format!("{}.md", date.format("%Y-%m-%d"));
    notebook_path.join("Log").join(filename)
}

fn create_log_from_template(log_path: &PathBuf, date: NaiveDate) -> Result<(), String> {
    let notebook_path = get_notebook_path();
    let template_path = notebook_path.join("+Templates").join("Daily Note.md");

    // Read the template
    let template = fs::read_to_string(&template_path)
        .map_err(|e| format!("Failed to read template: {}", e))?;

    // Replace template placeholders with actual date
    // Format: "Sat 27 December 2025"
    let date_str = date.format("%a %-d %B %Y").to_string();
    let content = template
        .replace("{{date:ddd D MMMM YYYY}}", &date_str)
        .replace(
            "{{date:D MMMM YYYY}}",
            &date.format("%-d %B %Y").to_string(),
        );

    // Ensure the Log directory exists
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create Log directory: {}", e))?;
    }

    // Write the file
    fs::write(log_path, content).map_err(|e| format!("Failed to write log file: {}", e))?;

    Ok(())
}

pub fn edit_log(yesterday: bool, tomorrow: bool) -> Result<(), String> {
    let date = get_date_offset(yesterday, tomorrow);
    let log_path = get_log_path(date);

    // Create the log file if it doesn't exist
    if !log_path.exists() {
        println!("Creating new log for {}", date.format("%a %-d %B %Y"));
        create_log_from_template(&log_path, date)?;
    }

    // Open the log in the user's editor
    let editor = get_editor();
    let status = Command::new(&editor)
        .arg(&log_path)
        .status()
        .map_err(|e| format!("Failed to open editor '{}': {}", editor, e))?;

    if !status.success() {
        return Err(format!("Editor exited with non-zero status"));
    }

    Ok(())
}

fn find_unchecked_todos(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim_start().starts_with("- [ ]"))
        .map(|line| line.to_string())
        .collect()
}

pub fn rollover_todos() -> Result<(), String> {
    let today = Local::now().date_naive();
    let tomorrow = today + Duration::days(1);

    let today_path = get_log_path(today);
    let tomorrow_path = get_log_path(tomorrow);

    // Check if today's log exists
    if !today_path.exists() {
        return Err(format!(
            "Today's log does not exist: {}",
            today_path.display()
        ));
    }

    // Read today's log
    let today_content = fs::read_to_string(&today_path)
        .map_err(|e| format!("Failed to read today's log: {}", e))?;

    // Find unchecked TODOs
    let unchecked_todos = find_unchecked_todos(&today_content);

    if unchecked_todos.is_empty() {
        println!("No unchecked TODOs to roll over!");
        return Ok(());
    }

    println!(
        "Found {} unchecked TODO(s) to roll over:",
        unchecked_todos.len()
    );
    for todo in &unchecked_todos {
        println!("  {}", todo.trim());
    }

    // Create tomorrow's log if it doesn't exist
    if !tomorrow_path.exists() {
        println!(
            "\nCreating tomorrow's log for {}",
            tomorrow.format("%a %-d %B %Y")
        );
        create_log_from_template(&tomorrow_path, tomorrow)?;
    }

    // Read tomorrow's log
    let tomorrow_content = fs::read_to_string(&tomorrow_path)
        .map_err(|e| format!("Failed to read tomorrow's log: {}", e))?;

    // Find the ## Personal section and insert TODOs after it
    let mut lines: Vec<String> = tomorrow_content.lines().map(|s| s.to_string()).collect();
    let personal_idx = lines.iter().position(|line| line.trim() == "## Personal");

    let insert_idx = match personal_idx {
        Some(idx) => {
            // Find the next non-empty line after ## Personal or the end of the section
            let mut insert_pos = idx + 1;

            // Skip any existing content in the Personal section to add at the end
            while insert_pos < lines.len() {
                let line = lines[insert_pos].trim();
                if line.starts_with("##") {
                    // Found the next section, insert before it
                    break;
                }
                insert_pos += 1;
            }
            insert_pos
        }
        None => {
            // No Personal section found, append at the end
            lines.len()
        }
    };

    // Insert a header comment and the unchecked TODOs
    lines.insert(insert_idx, "".to_string());
    lines.insert(
        insert_idx + 1,
        format!("### Rolled over from {}", today.format("%a %-d %B %Y")),
    );
    lines.insert(insert_idx + 2, "".to_string());

    for (i, todo) in unchecked_todos.iter().enumerate() {
        lines.insert(insert_idx + 3 + i, todo.clone());
    }

    // Write the updated tomorrow's log
    let new_content = lines.join("\n") + "\n";
    fs::write(&tomorrow_path, new_content)
        .map_err(|e| format!("Failed to write tomorrow's log: {}", e))?;

    println!(
        "\nSuccessfully rolled over {} TODO(s) to {}",
        unchecked_todos.len(),
        tomorrow.format("%a %-d %B %Y")
    );

    Ok(())
}
