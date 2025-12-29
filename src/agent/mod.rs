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
    /// - `claude` - Claude API (not yet implemented)
    /// - `ollama` - Ollama local models (not yet implemented)
    pub fn create(provider: &str, model: &str) -> Result<Box<dyn Agent>, AgentError> {
        match provider.to_lowercase().as_str() {
            "mock" => Ok(Box::new(mock::MockAgent::new("mock", model))),
            "claude" => Err(AgentError::Configuration(
                "Claude provider not yet implemented".to_string(),
            )),
            "ollama" => Err(AgentError::Configuration(
                "Ollama provider not yet implemented".to_string(),
            )),
            _ => Err(AgentError::Configuration(format!(
                "Unknown agent provider: {}",
                provider
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_factory_mock() {
        let agent = AgentFactory::create("mock", "test-model").unwrap();
        assert_eq!(agent.name(), "mock");
        assert_eq!(agent.model(), "test-model");
    }

    #[test]
    fn test_agent_factory_claude_not_implemented() {
        let result = AgentFactory::create("claude", "claude-sonnet-4");
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_factory_ollama_not_implemented() {
        let result = AgentFactory::create("ollama", "llama3");
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_factory_unknown_provider() {
        let result = AgentFactory::create("unknown", "model");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Unknown agent provider"));
        }
    }
}
