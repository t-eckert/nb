use super::types::{AgentError, AgentResponse, Message, ResponseMetadata};
use super::Agent;

/// A mock agent for testing purposes
///
/// Returns predefined responses or echoes user input
pub struct MockAgent {
    name: String,
    model: String,
    /// Optional fixed response to return
    fixed_response: Option<String>,
}

impl MockAgent {
    /// Creates a new mock agent
    pub fn new(name: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            model: model.into(),
            fixed_response: None,
        }
    }

    /// Creates a mock agent that always returns a fixed response
    pub fn with_fixed_response(
        name: impl Into<String>,
        model: impl Into<String>,
        response: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            model: model.into(),
            fixed_response: Some(response.into()),
        }
    }
}

impl Agent for MockAgent {
    fn chat(&self, messages: &[Message]) -> Result<AgentResponse, AgentError> {
        if messages.is_empty() {
            return Err(AgentError::InvalidRequest(
                "No messages provided".to_string(),
            ));
        }

        let content = if let Some(ref response) = self.fixed_response {
            response.clone()
        } else {
            // Echo the last user message
            let last_user_msg = messages
                .iter()
                .rev()
                .find(|m| matches!(m.role, super::types::Role::User))
                .map(|m| m.content.clone())
                .unwrap_or_else(|| "No user message found".to_string());

            format!("Mock response to: {}", last_user_msg)
        };

        let metadata = ResponseMetadata {
            input_tokens: Some(messages.iter().map(|m| m.content.len() / 4).sum()),
            output_tokens: Some(content.len() / 4),
            model: Some(self.model.clone()),
            stop_reason: Some("end_turn".to_string()),
        };

        Ok(AgentResponse::with_metadata(content, metadata))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::types::Message;

    #[test]
    fn test_mock_agent_echo() {
        let agent = MockAgent::new("test-agent", "mock-model");

        let messages = vec![
            Message::system("You are helpful"),
            Message::user("Hello there"),
        ];

        let response = agent.chat(&messages).unwrap();
        assert!(response.content.contains("Hello there"));
        assert_eq!(agent.name(), "test-agent");
        assert_eq!(agent.model(), "mock-model");
    }

    #[test]
    fn test_mock_agent_fixed_response() {
        let agent = MockAgent::with_fixed_response("test", "model", "Fixed response");

        let messages = vec![Message::user("Any message")];

        let response = agent.chat(&messages).unwrap();
        assert_eq!(response.content, "Fixed response");
    }

    #[test]
    fn test_mock_agent_empty_messages() {
        let agent = MockAgent::new("test", "model");
        let result = agent.chat(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_agent_metadata() {
        let agent = MockAgent::new("test", "model");
        let messages = vec![Message::user("Test")];

        let response = agent.chat(&messages).unwrap();
        assert!(response.metadata.input_tokens.is_some());
        assert!(response.metadata.output_tokens.is_some());
        assert_eq!(response.metadata.model, Some("model".to_string()));
    }
}
