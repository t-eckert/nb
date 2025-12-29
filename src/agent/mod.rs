pub mod claude;
pub mod mock;
pub mod types;

use types::{AgentError, AgentResponse, Message};

/// Core trait that all AI agents must implement
pub trait Agent: Send + Sync {
    /// Sends a conversation to the agent and gets a response
    ///
    /// # Arguments
    /// * `messages` - Conversation history including system, user, and assistant messages
    ///
    /// # Returns
    /// The agent's response or an error
    fn chat(&self, messages: &[Message]) -> Result<AgentResponse, AgentError>;

    /// Gets the name/identifier of this agent
    fn name(&self) -> &str;

    /// Gets the model being used by this agent
    fn model(&self) -> &str;
}

/// Factory for creating agents based on provider type
pub struct AgentFactory;

impl AgentFactory {
    /// Creates an agent based on provider type and model
    ///
    /// # Supported Providers
    /// - `mock` - Mock agent for testing
    /// - `claude` - Claude API (requires api_key)
    /// - `ollama` - Ollama local models (not yet implemented)
    ///
    /// # Arguments
    /// * `provider` - The agent provider type
    /// * `model` - The model to use
    /// * `api_key` - Optional API key (required for Claude)
    pub fn create(
        provider: &str,
        model: &str,
        api_key: Option<String>,
    ) -> Result<Box<dyn Agent>, AgentError> {
        match provider.to_lowercase().as_str() {
            "mock" => Ok(Box::new(mock::MockAgent::new("mock", model))),
            "claude" => {
                let key = api_key.ok_or_else(|| {
                    AgentError::Configuration(
                        "API key required for Claude provider".to_string(),
                    )
                })?;
                let agent = claude::ClaudeAgent::new(key, model.to_string())?;
                Ok(Box::new(agent))
            }
            "ollama" => Err(AgentError::Configuration(
                "Ollama provider not yet implemented".to_string(),
            )),
            _ => Err(AgentError::Configuration(format!(
                "Unknown agent provider: {}",
                provider
            ))),
        }
    }

    /// Creates an agent from a Config
    ///
    /// This is a convenience method that extracts provider, model, and API key from Config
    pub fn from_config(config: &crate::config::Config) -> Result<Box<dyn Agent>, AgentError> {
        let provider = config.get_agent_provider();
        let model = config.get_agent_model();
        let api_key = config.get_agent_api_key();

        Self::create(&provider, &model, api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_factory_mock() {
        let agent = AgentFactory::create("mock", "test-model", None).unwrap();
        assert_eq!(agent.name(), "mock");
        assert_eq!(agent.model(), "test-model");
    }

    #[test]
    fn test_agent_factory_claude() {
        let agent = AgentFactory::create(
            "claude",
            "claude-3-5-sonnet-20241022",
            Some("test-api-key".to_string()),
        )
        .unwrap();
        assert_eq!(agent.name(), "claude");
        assert_eq!(agent.model(), "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_agent_factory_claude_missing_api_key() {
        let result = AgentFactory::create("claude", "claude-sonnet-4", None);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("API key required"));
        }
    }

    #[test]
    fn test_agent_factory_ollama_not_implemented() {
        let result = AgentFactory::create("ollama", "llama3", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_factory_unknown_provider() {
        let result = AgentFactory::create("unknown", "model", None);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Unknown agent provider"));
        }
    }

    #[test]
    fn test_agent_factory_from_config() {
        let mut config = crate::config::Config::new();
        config.agent.provider = Some("mock".to_string());
        config.agent.model = Some("test-model".to_string());

        let agent = AgentFactory::from_config(&config).unwrap();
        assert_eq!(agent.name(), "mock");
        assert_eq!(agent.model(), "test-model");
    }
}
