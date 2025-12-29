use std::path::Path;
use std::process::Command;

use crate::config::Config;

/// Encapsulates an editor executable that can be used to edit files
pub struct Editor {
    executable: String,
}

impl Editor {
    /// Creates an Editor from a Config
    ///
    /// The editor executable is determined by Config.get_editor() which follows:
    /// 1. Config file value
    /// 2. $EDITOR environment variable
    /// 3. Falls back to "nvim"
    pub fn from_config(config: &Config) -> Self {
        Self {
            executable: config.get_editor(),
        }
    }

    /// Creates a new Editor instance with a specific executable
    pub fn with_executable(executable: String) -> Self {
        Self { executable }
    }

    /// Opens the specified file in the editor
    ///
    /// # Arguments
    /// * `path` - Path to the file to edit
    ///
    /// # Returns
    /// * `Ok(())` if the editor was opened successfully
    /// * `Err(String)` if the editor failed to open
    pub fn open(&self, path: &Path) -> Result<(), String> {
        Command::new(&self.executable)
            .arg(path)
            .status()
            .map_err(|e| format!("Failed to open editor: {}", e))?;
        Ok(())
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::with_executable("nvim".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_editor_from_config() {
        let mut config = Config::new();
        config.set_editor("vim".to_string());

        let editor = Editor::from_config(&config);
        assert_eq!(editor.executable, "vim");
    }

    #[test]
    fn test_editor_from_config_default() {
        let config = Config::new();
        let editor = Editor::from_config(&config);

        // Should use default from config (which may be env var or nvim)
        assert!(!editor.executable.is_empty());
    }

    #[test]
    fn test_editor_with_executable() {
        let editor = Editor::with_executable("emacs".to_string());
        assert_eq!(editor.executable, "emacs");
    }

    #[test]
    fn test_editor_default_trait() {
        let editor = Editor::default();
        assert_eq!(editor.executable, "nvim");
    }

    #[test]
    fn test_editor_open_with_nonexistent_editor() {
        let editor = Editor {
            executable: "nonexistent-editor-12345".to_string(),
        };

        let temp_file = NamedTempFile::new().unwrap();
        let result = editor.open(temp_file.path());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to open editor"));
    }

    #[test]
    fn test_editor_open_with_nonexistent_file() {
        // Using 'true' as the editor - it's a command that exists and exits successfully
        // This tests that we can call open() without actually launching an interactive editor
        let editor = Editor {
            executable: "true".to_string(),
        };

        let nonexistent_path = std::path::Path::new("/tmp/nonexistent-file-12345.txt");
        let result = editor.open(nonexistent_path);

        // 'true' command ignores arguments and exits successfully
        // Some editors might fail on nonexistent files, but 'true' won't
        assert!(result.is_ok());
    }
}
