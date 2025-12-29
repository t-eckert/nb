use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Path to the notebook directory
    pub notebook_path: Option<String>,
    /// Editor command to use
    pub editor: Option<String>,
    /// Agent configuration
    #[serde(default)]
    pub agent: AgentConfig,
}

/// Agent-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent provider (e.g., "claude", "ollama", "mock")
    pub provider: Option<String>,
    /// Model to use
    pub model: Option<String>,
    /// API key (for cloud providers)
    pub api_key: Option<String>,
    /// Custom endpoint URL (for local providers like Ollama)
    pub endpoint: Option<String>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            provider: None,
            model: None,
            api_key: None,
            endpoint: None,
        }
    }
}

impl Config {
    /// Creates a new default configuration
    pub fn new() -> Self {
        Self {
            notebook_path: None,
            editor: None,
            agent: AgentConfig::default(),
        }
    }

    /// Gets the path to the config file
    ///
    /// Returns the path where the config file should be stored:
    /// - Linux/macOS: `~/.config/nb/config.toml`
    /// - Windows: `%APPDATA%\nb\config.toml`
    pub fn config_path() -> Result<PathBuf, String> {
        let proj_dirs = ProjectDirs::from("", "", "nb")
            .ok_or_else(|| "Could not determine config directory".to_string())?;

        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    /// Loads configuration from the config file
    ///
    /// If the config file doesn't exist, returns a default configuration.
    /// If the config file exists but is invalid, returns an error.
    pub fn load() -> Result<Self, String> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::new());
        }

        let contents = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Saves the configuration to the config file
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::config_path()?;

        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, contents)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// Gets the notebook path with fallback priority:
    /// 1. Config file value
    /// 2. NOTEBOOK_PATH environment variable
    /// 3. ~/Notebook default
    pub fn get_notebook_path(&self) -> PathBuf {
        // Priority 1: Config file
        if let Some(path) = &self.notebook_path {
            return PathBuf::from(path);
        }

        // Priority 2: Environment variable
        if let Ok(path) = env::var("NOTEBOOK_PATH") {
            return PathBuf::from(path);
        }

        // Priority 3: Default
        let home = env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join("Notebook")
    }

    /// Gets the editor with fallback priority:
    /// 1. Config file value
    /// 2. EDITOR environment variable
    /// 3. nvim default
    pub fn get_editor(&self) -> String {
        // Priority 1: Config file
        if let Some(editor) = &self.editor {
            return editor.clone();
        }

        // Priority 2: Environment variable
        if let Ok(editor) = env::var("EDITOR") {
            return editor;
        }

        // Priority 3: Default
        "nvim".to_string()
    }

    /// Sets the notebook path
    pub fn set_notebook_path(&mut self, path: String) {
        self.notebook_path = Some(path);
    }

    /// Sets the editor
    pub fn set_editor(&mut self, editor: String) {
        self.editor = Some(editor);
    }

    /// Gets the agent provider with fallback priority:
    /// 1. Config file value
    /// 2. NB_AGENT_PROVIDER environment variable
    /// 3. "mock" default
    pub fn get_agent_provider(&self) -> String {
        // Priority 1: Config file
        if let Some(provider) = &self.agent.provider {
            return provider.clone();
        }

        // Priority 2: Environment variable
        if let Ok(provider) = env::var("NB_AGENT_PROVIDER") {
            return provider;
        }

        // Priority 3: Default
        "mock".to_string()
    }

    /// Gets the agent model with fallback priority:
    /// 1. Config file value
    /// 2. NB_AGENT_MODEL environment variable
    /// 3. Provider-specific default
    pub fn get_agent_model(&self) -> String {
        // Priority 1: Config file
        if let Some(model) = &self.agent.model {
            return model.clone();
        }

        // Priority 2: Environment variable
        if let Ok(model) = env::var("NB_AGENT_MODEL") {
            return model;
        }

        // Priority 3: Provider-specific defaults
        match self.get_agent_provider().as_str() {
            "claude" => "claude-sonnet-4".to_string(),
            "ollama" => "llama3".to_string(),
            _ => "mock-model".to_string(),
        }
    }

    /// Gets the agent API key with fallback to environment variable
    pub fn get_agent_api_key(&self) -> Option<String> {
        // Priority 1: Config file
        if let Some(key) = &self.agent.api_key {
            return Some(key.clone());
        }

        // Priority 2: ANTHROPIC_API_KEY for Claude
        if self.get_agent_provider() == "claude" {
            if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
                return Some(key);
            }
        }

        None
    }

    /// Gets the agent endpoint with fallback to environment variable
    pub fn get_agent_endpoint(&self) -> Option<String> {
        // Priority 1: Config file
        if let Some(endpoint) = &self.agent.endpoint {
            return Some(endpoint.clone());
        }

        // Priority 2: Environment variable
        if let Ok(endpoint) = env::var("NB_AGENT_ENDPOINT") {
            return Some(endpoint);
        }

        // Priority 3: Provider defaults
        match self.get_agent_provider().as_str() {
            "ollama" => Some("http://localhost:11434".to_string()),
            _ => None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_new_config() {
        let config = Config::new();
        assert!(config.notebook_path.is_none());
        assert!(config.editor.is_none());
    }

    #[test]
    fn test_config_path() {
        let path = Config::config_path().unwrap();
        assert!(path.to_str().unwrap().contains("nb"));
        assert!(path.to_str().unwrap().ends_with("config.toml"));
    }

    #[test]
    #[serial]
    fn test_get_notebook_path_priority() {
        unsafe {
            env::remove_var("NOTEBOOK_PATH");
        }

        // Test default
        let config = Config::new();
        let path = config.get_notebook_path();
        assert!(path.to_str().unwrap().ends_with("Notebook"));

        // Test environment variable
        unsafe {
            env::set_var("NOTEBOOK_PATH", "/env/notebook");
        }
        let config = Config::new();
        let path = config.get_notebook_path();
        assert_eq!(path.to_str().unwrap(), "/env/notebook");

        // Test config file (highest priority)
        let mut config = Config::new();
        config.set_notebook_path("/config/notebook".to_string());
        let path = config.get_notebook_path();
        assert_eq!(path.to_str().unwrap(), "/config/notebook");

        unsafe {
            env::remove_var("NOTEBOOK_PATH");
        }
    }

    #[test]
    #[serial]
    fn test_get_editor_priority() {
        unsafe {
            env::remove_var("EDITOR");
        }

        // Test default
        let config = Config::new();
        let editor = config.get_editor();
        assert_eq!(editor, "nvim");

        // Test environment variable
        unsafe {
            env::set_var("EDITOR", "vim");
        }
        let config = Config::new();
        let editor = config.get_editor();
        assert_eq!(editor, "vim");

        // Test config file (highest priority)
        let mut config = Config::new();
        config.set_editor("emacs".to_string());
        let editor = config.get_editor();
        assert_eq!(editor, "emacs");

        unsafe {
            env::remove_var("EDITOR");
        }
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut config = Config::new();
        config.set_notebook_path("/test/notebook".to_string());
        config.set_editor("vim".to_string());

        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.notebook_path, Some("/test/notebook".to_string()));
        assert_eq!(parsed.editor, Some("vim".to_string()));
    }

    #[test]
    fn test_load_nonexistent_config() {
        // This should return default config without error
        let config = Config::load().unwrap();
        assert!(config.notebook_path.is_none());
        assert!(config.editor.is_none());
    }

    #[test]
    fn test_save_and_load_config() {
        // Create a temporary directory for config
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create and save config
        let mut config = Config::new();
        config.set_notebook_path("/test/path".to_string());
        config.set_editor("nano".to_string());

        // Manually write to temp location for testing
        let contents = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, contents).unwrap();

        // Load and verify
        let loaded_contents = fs::read_to_string(&config_path).unwrap();
        let loaded_config: Config = toml::from_str(&loaded_contents).unwrap();

        assert_eq!(loaded_config.notebook_path, Some("/test/path".to_string()));
        assert_eq!(loaded_config.editor, Some("nano".to_string()));
    }
}
