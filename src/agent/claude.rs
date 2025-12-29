use super::types::{AgentError, AgentResponse, Message, ResponseMetadata, Role};
use super::Agent;
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_MAX_TOKENS: usize = 8192;

/// Claude AI agent using the Anthropic Messages API
pub struct ClaudeAgent {
    api_key: String,
    model: String,
    max_tokens: usize,
    client: reqwest::blocking::Client,
}

impl ClaudeAgent {
    /// Creates a new Claude agent
    ///
    /// # Arguments
    /// * `api_key` - Anthropic API key
    /// * `model` - Model to use (e.g., "claude-sonnet-4-20250514")
    pub fn new(api_key: String, model: String) -> Result<Self, AgentError> {
        if api_key.is_empty() {
            return Err(AgentError::Configuration(
                "API key cannot be empty".to_string(),
            ));
        }

        let client = reqwest::blocking::Client::new();

        Ok(Self {
            api_key,
            model,
            max_tokens: DEFAULT_MAX_TOKENS,
            client,
        })
    }

    /// Sets the maximum number of tokens to generate
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Converts our messages to Anthropic API format
    fn convert_messages(&self, messages: &[Message]) -> (Option<String>, Vec<AnthropicMessage>) {
        let mut system_prompt = None;
        let mut api_messages = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    // Anthropic uses a separate system parameter
                    system_prompt = Some(msg.content.clone());
                }
                Role::User | Role::Assistant => {
                    api_messages.push(AnthropicMessage {
                        role: match msg.role {
                            Role::User => "user".to_string(),
                            Role::Assistant => "assistant".to_string(),
                            _ => unreachable!(),
                        },
                        content: msg.content.clone(),
                    });
                }
            }
        }

        (system_prompt, api_messages)
    }
}

impl Agent for ClaudeAgent {
    fn chat(&self, messages: &[Message]) -> Result<AgentResponse, AgentError> {
        if messages.is_empty() {
            return Err(AgentError::InvalidRequest(
                "No messages provided".to_string(),
            ));
        }

        // Convert messages to Anthropic format
        let (system_prompt, api_messages) = self.convert_messages(messages);

        if api_messages.is_empty() {
            return Err(AgentError::InvalidRequest(
                "At least one user or assistant message is required".to_string(),
            ));
        }

        // Build request
        let request_body = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            messages: api_messages,
            system: system_prompt,
        };

        // Make API request
        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| AgentError::Communication(format!("Failed to send request: {}", e)))?;

        // Handle HTTP errors
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(match status.as_u16() {
                429 => AgentError::RateLimit(format!("Rate limit exceeded: {}", error_text)),
                401 => AgentError::Configuration(format!("Invalid API key: {}", error_text)),
                400 => AgentError::InvalidRequest(format!("Bad request: {}", error_text)),
                _ => AgentError::ApiError(format!("HTTP {}: {}", status, error_text)),
            });
        }

        // Parse response
        let api_response: AnthropicResponse = response
            .json()
            .map_err(|e| AgentError::Communication(format!("Failed to parse response: {}", e)))?;

        // Extract content
        let content = api_response
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        // Build metadata
        let metadata = ResponseMetadata {
            input_tokens: Some(api_response.usage.input_tokens),
            output_tokens: Some(api_response.usage.output_tokens),
            model: Some(api_response.model),
            stop_reason: api_response.stop_reason,
        };

        Ok(AgentResponse::with_metadata(content, metadata))
    }

    fn name(&self) -> &str {
        "claude"
    }

    fn model(&self) -> &str {
        &self.model
    }
}

// Anthropic API types

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: usize,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    #[serde(rename = "type")]
    response_type: String,
    role: String,
    content: Vec<ContentBlock>,
    model: String,
    stop_reason: Option<String>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    input_tokens: usize,
    output_tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_agent_creation() {
        let agent = ClaudeAgent::new("test-key".to_string(), "claude-3-5-sonnet-20241022".to_string()).unwrap();
        assert_eq!(agent.name(), "claude");
        assert_eq!(agent.model(), "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_claude_agent_empty_api_key() {
        let result = ClaudeAgent::new("".to_string(), "model".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_claude_agent_with_max_tokens() {
        let agent = ClaudeAgent::new("test-key".to_string(), "model".to_string())
            .unwrap()
            .with_max_tokens(4096);
        assert_eq!(agent.max_tokens, 4096);
    }

    #[test]
    fn test_convert_messages() {
        let agent = ClaudeAgent::new("test-key".to_string(), "model".to_string()).unwrap();

        let messages = vec![
            Message::system("You are helpful"),
            Message::user("Hello"),
            Message::assistant("Hi there"),
        ];

        let (system, api_messages) = agent.convert_messages(&messages);
        assert_eq!(system, Some("You are helpful".to_string()));
        assert_eq!(api_messages.len(), 2);
        assert_eq!(api_messages[0].role, "user");
        assert_eq!(api_messages[0].content, "Hello");
        assert_eq!(api_messages[1].role, "assistant");
        assert_eq!(api_messages[1].content, "Hi there");
    }

    #[test]
    fn test_chat_empty_messages() {
        let agent = ClaudeAgent::new("test-key".to_string(), "model".to_string()).unwrap();
        let result = agent.chat(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_chat_only_system_message() {
        let agent = ClaudeAgent::new("test-key".to_string(), "model".to_string()).unwrap();
        let messages = vec![Message::system("System only")];
        let result = agent.chat(&messages);
        assert!(result.is_err());
    }
}
