use std::env;
use std::path::Path;
use std::process::Command;

/// Encapsulates an editor executable that can be used to edit files
pub struct Editor {
    executable: String,
}

impl Editor {
    /// Creates a new Editor instance
    ///
    /// The editor executable is determined by:
    /// 1. $EDITOR environment variable
    /// 2. Falls back to "nvim" if not set
    pub fn new() -> Self {
        Self {
            executable: env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string()),
        }
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
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use tempfile::NamedTempFile;

    #[test]
    #[serial]
    fn test_editor_new_with_env_var() {
        unsafe {
            env::set_var("EDITOR", "vim");
        }
        let editor = Editor::new();
        unsafe {
            env::remove_var("EDITOR");
        }

        assert_eq!(editor.executable, "vim");
    }

    #[test]
    #[serial]
    fn test_editor_new_default() {
        unsafe {
            env::remove_var("EDITOR");
        }
        let editor = Editor::new();

        assert_eq!(editor.executable, "nvim");
    }

    #[test]
    #[serial]
    fn test_editor_new_with_custom_editor() {
        unsafe {
            env::set_var("EDITOR", "emacs");
        }
        let editor = Editor::new();
        unsafe {
            env::remove_var("EDITOR");
        }

        assert_eq!(editor.executable, "emacs");
    }

    #[test]
    fn test_editor_default_trait() {
        unsafe {
            env::remove_var("EDITOR");
        }
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
