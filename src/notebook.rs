use chrono::{DateTime, Duration, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::document::Document;
use crate::editor::Editor;

fn get_date_offset(
    yesterday: bool,
    tomorrow: bool,
    date_str: Option<&str>,
) -> Result<NaiveDate, String> {
    // If a specific date is provided, use it
    if let Some(date) = date_str {
        return NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| format!("Invalid date format: {}. Expected YYYY-MM-DD", date));
    }

    let today = Local::now().date_naive();

    if yesterday {
        Ok(today - Duration::days(1))
    } else if tomorrow {
        Ok(today + Duration::days(1))
    } else {
        Ok(today)
    }
}

fn get_log_path(config: &Config, date: NaiveDate) -> PathBuf {
    let notebook_path = config.get_notebook_path();
    let filename = format!("{}.md", date.format("%Y-%m-%d"));
    notebook_path.join("Log").join(filename)
}

fn create_log_from_template(
    config: &Config,
    log_path: &PathBuf,
    date: NaiveDate,
) -> Result<(), String> {
    let notebook_path = config.get_notebook_path();
    let template_path = notebook_path.join("+Templates").join("Daily Note.md");

    // Read the template as a Document
    let mut template_doc = Document::from_file(&template_path)?;

    // Replace template placeholders with actual date
    // Format: "Sat 27 December 2025"
    let date_str = date.format("%a %-d %B %Y").to_string();
    template_doc.content = template_doc
        .content
        .replace("{{date:ddd D MMMM YYYY}}", &date_str)
        .replace(
            "{{date:D MMMM YYYY}}",
            &date.format("%-d %B %Y").to_string(),
        );

    // Ensure the Log directory exists
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create Log directory: {}", e))?;
    }

    // Write the document
    template_doc.to_file(log_path)?;

    Ok(())
}

pub fn edit_log(
    config: &Config,
    yesterday: bool,
    tomorrow: bool,
    date_str: Option<&str>,
) -> Result<(), String> {
    let date = get_date_offset(yesterday, tomorrow, date_str)?;
    let log_path = get_log_path(config, date);

    // Create the log file if it doesn't exist
    if !log_path.exists() {
        println!("Creating new log for {}", date.format("%a %-d %B %Y"));
        create_log_from_template(config, &log_path, date)?;
    }

    // Open the log in the user's editor
    let editor = Editor::from_config(config);
    editor.open(&log_path)?;

    Ok(())
}

pub fn view_log(
    config: &Config,
    yesterday: bool,
    tomorrow: bool,
    date_str: Option<&str>,
) -> Result<(), String> {
    let date = get_date_offset(yesterday, tomorrow, date_str)?;
    let log_path = get_log_path(config, date);

    // Check if the log file exists
    if !log_path.exists() {
        return Err(format!(
            "Log for {} does not exist: {}",
            date.format("%a %-d %B %Y"),
            log_path.display()
        ));
    }

    // Read and display the log
    let doc = Document::from_file(&log_path)?;

    // Print the full document (with frontmatter if present)
    print!("{}", doc.to_string());

    Ok(())
}

fn find_unchecked_todos(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim_start().starts_with("- [ ]"))
        .map(|line| line.to_string())
        .collect()
}

pub fn rollover_todos(config: &Config) -> Result<(), String> {
    let today = Local::now().date_naive();
    let tomorrow = today + Duration::days(1);

    let today_path = get_log_path(config, today);
    let tomorrow_path = get_log_path(config, tomorrow);

    // Check if today's log exists
    if !today_path.exists() {
        return Err(format!(
            "Today's log does not exist: {}",
            today_path.display()
        ));
    }

    // Read today's log
    let today_doc = Document::from_file(&today_path)?;

    // Find unchecked TODOs
    let unchecked_todos = find_unchecked_todos(&today_doc.content);

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
        create_log_from_template(config, &tomorrow_path, tomorrow)?;
    }

    // Read tomorrow's log
    let mut tomorrow_doc = Document::from_file(&tomorrow_path)?;

    // Find the ## Personal section and insert TODOs after it
    let mut lines: Vec<String> = tomorrow_doc.content.lines().map(|s| s.to_string()).collect();
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

    // Update the document content and write it
    tomorrow_doc.content = lines.join("\n") + "\n";
    tomorrow_doc.to_file(&tomorrow_path)?;

    println!(
        "\nSuccessfully rolled over {} TODO(s) to {}",
        unchecked_todos.len(),
        tomorrow.format("%a %-d %B %Y")
    );

    Ok(())
}

pub fn list_logs(config: &Config, days: usize, show_unfinished: bool) -> Result<(), String> {
    let notebook_path = config.get_notebook_path();
    let log_dir = notebook_path.join("Log");

    // Check if Log directory exists
    if !log_dir.exists() {
        return Err(format!(
            "Log directory does not exist: {}",
            log_dir.display()
        ));
    }

    // Read all files in the Log directory
    let entries =
        fs::read_dir(&log_dir).map_err(|e| format!("Failed to read Log directory: {}", e))?;

    // Parse dates from filenames and store in a sorted map
    let mut logs: BTreeMap<NaiveDate, PathBuf> = BTreeMap::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
            // Try to parse YYYY-MM-DD.md format
            if filename.ends_with(".md") {
                let date_str = filename.trim_end_matches(".md");
                if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    logs.insert(date, path);
                }
            }
        }
    }

    // Calculate the cutoff date
    let today = Local::now().date_naive();
    let cutoff_date = today - Duration::days(days as i64 - 1);

    // Filter and display logs
    let recent_logs: Vec<_> = logs
        .iter()
        .filter(|(date, _)| **date >= cutoff_date && **date <= today)
        .rev()
        .collect();

    if recent_logs.is_empty() {
        println!("No logs found in the last {} day(s)", days);
        return Ok(());
    }

    println!("Logs from the last {} day(s):\n", days);

    for (date, path) in recent_logs {
        // Try to read the file and count TODOs
        let (total_todos, completed_todos, unchecked_todos) =
            if let Ok(doc) = Document::from_file(path) {
                let total = doc
                    .content
                    .lines()
                    .filter(|line| {
                        let trimmed = line.trim_start();
                        trimmed.starts_with("- [ ]") || trimmed.starts_with("- [x]")
                    })
                    .count();

                let completed = doc
                    .content
                    .lines()
                    .filter(|line| line.trim_start().starts_with("- [x]"))
                    .count();

                let unchecked: Vec<String> = if show_unfinished {
                    find_unchecked_todos(&doc.content)
                } else {
                    Vec::new()
                };

                (total, completed, unchecked)
            } else {
                (0, 0, Vec::new())
            };

        // Format the output
        let day_name = date.format("%a").to_string();
        let date_str = date.format("%Y-%m-%d").to_string();

        let todo_info = if total_todos > 0 {
            format!("  [{}/{}]", completed_todos, total_todos)
        } else {
            String::new()
        };

        println!("{} ({}){}", date_str, day_name, todo_info);

        // Show unfinished TODOs if requested
        if show_unfinished && !unchecked_todos.is_empty() {
            for todo in unchecked_todos {
                println!("  {}", todo.trim());
            }
        }
    }

    Ok(())
}

/// Print the notebook directory path to stdout
///
/// This is useful for shell integration. Users can use `cd $(nb open)` to change
/// to the notebook directory.
pub fn open(config: &Config) -> Result<(), String> {
    let notebook_path = config.get_notebook_path();
    println!("{}", notebook_path.display());
    Ok(())
}

// Weather-related structs for OpenWeather API
#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    daily: Vec<DailyForecast>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DailyForecast {
    dt: i64,
    temp: Temperature,
    weather: Vec<WeatherCondition>,
    pop: f64,
    #[serde(default)]
    rain: Option<f64>,
    #[serde(default)]
    snow: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Temperature {
    min: f64,
    max: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherCondition {
    description: String,
}

fn get_api_key() -> Result<String, String> {
    let output = Command::new("op")
        .args(&["read", "op://Private/Open Weather Map/Notebook API Key"])
        .output()
        .map_err(|e| format!("Failed to execute 1Password CLI: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to retrieve API key from 1Password: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let api_key = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in API key: {}", e))?
        .trim()
        .to_string();

    if api_key.is_empty() {
        return Err("API key is empty".to_string());
    }

    Ok(api_key)
}

fn fetch_weather(api_key: &str) -> Result<WeatherResponse, String> {
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat=45.4215&lon=-75.6972&exclude=minutely,hourly,alerts&units=metric&appid={}",
        api_key
    );

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch weather data: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Weather API returned error status: {}",
            response.status()
        ));
    }

    let weather: WeatherResponse = response
        .json()
        .map_err(|e| format!("Failed to parse weather response: {}", e))?;

    Ok(weather)
}

fn format_weather(forecast: &DailyForecast) -> String {
    let high = forecast.temp.max.round() as i32;
    let low = forecast.temp.min.round() as i32;
    let description = capitalize_first_letter(&forecast.weather[0].description);

    let mut weather_str = format!("Weather: High {}°C, Low {}°C. {}", high, low, description);

    // Add cold weather alert
    if low < -10 {
        weather_str = format!("Weather: **Cold weather alert.** High {}°C, Low {}°C. {}", high, low, description);
    }

    // Add precipitation information
    if let Some(snow_mm) = forecast.snow {
        if snow_mm > 0.5 {
            let snow_cm = (snow_mm / 10.0).round() as i32;
            if snow_cm >= 1 {
                weather_str.push_str(&format!(" {}cm snow accumulation expected.", snow_cm));
            }
        }
    } else if let Some(rain_mm) = forecast.rain {
        if rain_mm > 0.5 {
            weather_str.push_str(&format!(" ({}% chance)", (forecast.pop * 100.0).round() as i32));
        }
    } else if forecast.pop > 0.2 {
        weather_str.push_str(&format!(" ({}% chance)", (forecast.pop * 100.0).round() as i32));
    }

    weather_str.push('.');
    weather_str
}

fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn insert_weather_into_log(log_path: &PathBuf, weather_line: &str) -> Result<bool, String> {
    // Read the log file
    let mut doc = Document::from_file(log_path)?;

    // Check if weather already exists
    if doc.content.contains("Weather:") {
        return Ok(false); // Weather already exists
    }

    // Split content into lines
    let mut lines: Vec<String> = doc.content.lines().map(|s| s.to_string()).collect();

    // Find the first heading line (starts with #)
    let heading_idx = lines.iter().position(|line| line.starts_with('#'));

    if let Some(idx) = heading_idx {
        // Insert weather after the heading, with a blank line
        lines.insert(idx + 1, String::new());
        lines.insert(idx + 2, weather_line.to_string());
    } else {
        // No heading found, insert at the beginning
        lines.insert(0, weather_line.to_string());
        lines.insert(1, String::new());
    }

    // Update the document content and write it
    doc.content = lines.join("\n") + "\n";
    doc.to_file(log_path)?;

    Ok(true) // Weather was added
}

pub fn add_weather(config: &Config, days: usize) -> Result<(), String> {
    println!("Fetching weather forecast from OpenWeather API...");

    // Get API key from 1Password
    let api_key = get_api_key()?;

    // Fetch weather data
    let weather = fetch_weather(&api_key)?;

    let today = Local::now().date_naive();
    let mut added_count = 0;
    let mut skipped_count = 0;
    let mut summary: Vec<String> = Vec::new();

    // Process each day's forecast
    for i in 0..days.min(weather.daily.len()) {
        let forecast = &weather.daily[i];
        let forecast_datetime = DateTime::from_timestamp(forecast.dt, 0)
            .ok_or("Invalid timestamp in weather data")?;
        let forecast_date = forecast_datetime.date_naive();

        // Only process dates from today onwards
        if forecast_date < today {
            continue;
        }

        let log_path = get_log_path(config, forecast_date);

        // Create log if it doesn't exist
        if !log_path.exists() {
            create_log_from_template(config, &log_path, forecast_date)?;
        }

        // Format and insert weather
        let weather_line = format_weather(forecast);
        match insert_weather_into_log(&log_path, &weather_line) {
            Ok(true) => {
                added_count += 1;
                summary.push(format!(
                    "  {} ({}): {}",
                    forecast_date.format("%Y-%m-%d"),
                    forecast_date.format("%a"),
                    weather_line.trim_start_matches("Weather: ")
                ));
            }
            Ok(false) => {
                skipped_count += 1;
            }
            Err(e) => {
                eprintln!(
                    "Warning: Failed to add weather for {}: {}",
                    forecast_date.format("%Y-%m-%d"),
                    e
                );
            }
        }
    }

    // Print summary
    println!("\nWeather forecast summary:");
    println!("Added weather to {} day(s):", added_count);
    for line in summary {
        println!("{}", line);
    }

    if skipped_count > 0 {
        println!("\nSkipped {} day(s) that already had weather.", skipped_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_log(dir: &PathBuf, date: &str, content: &str) -> std::io::Result<()> {
        let log_path = dir.join(format!("{}.md", date));
        fs::write(log_path, content)
    }

    fn create_test_config(notebook_path: &str) -> Config {
        let mut config = Config::new();
        config.set_notebook_path(notebook_path.to_string());
        config
    }

    #[test]
    fn test_find_unchecked_todos() {
        let content = r#"# Test Log

## Personal

- [x] Completed task
- [ ] Unchecked task 1
- [ ] Unchecked task 2
- [x] Another completed task

## Notes
"#;

        let todos = find_unchecked_todos(content);
        assert_eq!(todos.len(), 2);
        assert!(todos[0].contains("Unchecked task 1"));
        assert!(todos[1].contains("Unchecked task 2"));
    }

    #[test]
    fn test_find_unchecked_todos_with_indentation() {
        let content = r#"
  - [ ] Indented unchecked
    - [ ] More indented
- [x] Completed
"#;

        let todos = find_unchecked_todos(content);
        assert_eq!(todos.len(), 2);
    }

    #[test]
    fn test_list_logs_with_todos() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        // Create test logs
        let today = Local::now().date_naive();
        let yesterday = today - Duration::days(1);

        create_test_log(
            &log_dir,
            &today.format("%Y-%m-%d").to_string(),
            "# Today\n- [x] Done\n- [ ] Not done\n",
        )
        .unwrap();

        create_test_log(
            &log_dir,
            &yesterday.format("%Y-%m-%d").to_string(),
            "# Yesterday\n- [x] Task 1\n- [x] Task 2\n",
        )
        .unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = list_logs(&config, 7, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_list_logs_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = list_logs(&config, 7, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_list_logs_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = list_logs(&config, 7, false);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Log directory does not exist"));
    }

    #[test]
    fn test_rollover_todos_basic() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        let template_dir = temp_dir.path().join("+Templates");
        fs::create_dir_all(&log_dir).unwrap();
        fs::create_dir_all(&template_dir).unwrap();

        // Create template
        let template_path = template_dir.join("Daily Note.md");
        fs::write(
            template_path,
            "# {{date:ddd D MMMM YYYY}}\n\n## Personal\n\n## Notes\n",
        )
        .unwrap();

        // Create today's log with unchecked TODOs
        let today = Local::now().date_naive();
        create_test_log(
            &log_dir,
            &today.format("%Y-%m-%d").to_string(),
            "# Today\n\n## Personal\n\n- [x] Done task\n- [ ] Todo 1\n- [ ] Todo 2\n",
        )
        .unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = rollover_todos(&config);

        assert!(result.is_ok());

        // Check that tomorrow's log was created
        let tomorrow = today + Duration::days(1);
        let tomorrow_path = log_dir.join(format!("{}.md", tomorrow.format("%Y-%m-%d")));
        assert!(tomorrow_path.exists());

        // Check that todos were rolled over
        let tomorrow_content = fs::read_to_string(tomorrow_path).unwrap();
        assert!(tomorrow_content.contains("Todo 1"));
        assert!(tomorrow_content.contains("Todo 2"));
        assert!(tomorrow_content.contains("Rolled over from"));
    }

    #[test]
    fn test_rollover_todos_no_unchecked() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        // Create today's log with only completed TODOs
        let today = Local::now().date_naive();
        create_test_log(
            &log_dir,
            &today.format("%Y-%m-%d").to_string(),
            "# Today\n\n- [x] All done\n- [x] Everything complete\n",
        )
        .unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = rollover_todos(&config);

        assert!(result.is_ok());
    }

    #[test]
    fn test_rollover_todos_missing_today_log() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = rollover_todos(&config);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_view_log_existing() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        let today = Local::now().date_naive();
        let content = "# Test Log\n\n## Notes\n\nThis is a test.";
        create_test_log(
            &log_dir,
            &today.format("%Y-%m-%d").to_string(),
            content,
        )
        .unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = view_log(&config, false, false, None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = view_log(&config, false, false, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_view_log_yesterday() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        let yesterday = Local::now().date_naive() - Duration::days(1);
        let content = "# Yesterday's Log";
        create_test_log(
            &log_dir,
            &yesterday.format("%Y-%m-%d").to_string(),
            content,
        )
        .unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = view_log(&config, true, false, None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_specific_date() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("Log");
        fs::create_dir_all(&log_dir).unwrap();

        let content = "# Specific Date Log";
        create_test_log(&log_dir, "2025-12-25", content).unwrap();

        let config = create_test_config(temp_dir.path().to_str().unwrap());
        let result = view_log(&config, false, false, Some("2025-12-25"));

        assert!(result.is_ok());
    }
}
