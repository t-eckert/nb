use std::fmt;

/// Represents a message in a conversation
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    /// Creates a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    /// Creates a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// Creates an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// Role of a message sender
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// System instruction or context
    System,
    /// User input
    User,
    /// Assistant response
    Assistant,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

/// Response from an agent
#[derive(Debug, Clone)]
pub struct AgentResponse {
    /// The text content of the response
    pub content: String,
    /// Optional metadata about the response
    pub metadata: ResponseMetadata,
}

impl AgentResponse {
    /// Creates a simple response with just content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            metadata: ResponseMetadata::default(),
        }
    }

    /// Creates a response with content and metadata
    pub fn with_metadata(content: impl Into<String>, metadata: ResponseMetadata) -> Self {
        Self {
            content: content.into(),
            metadata,
        }
    }
}

/// Metadata about an agent response
#[derive(Debug, Clone, Default)]
pub struct ResponseMetadata {
    /// Number of tokens in the prompt (if available)
    pub input_tokens: Option<usize>,
    /// Number of tokens in the response (if available)
    pub output_tokens: Option<usize>,
    /// Model that generated the response
    pub model: Option<String>,
    /// Stop reason (if available)
    pub stop_reason: Option<String>,
}

/// Errors that can occur when using agents
#[derive(Debug)]
pub enum AgentError {
    /// Configuration error (missing API key, invalid settings, etc.)
    Configuration(String),
    /// Network or API communication error
    Communication(String),
    /// API returned an error
    ApiError(String),
    /// Rate limit exceeded
    RateLimit(String),
    /// Invalid request
    InvalidRequest(String),
    /// Other errors
    Other(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            AgentError::Communication(msg) => write!(f, "Communication error: {}", msg),
            AgentError::ApiError(msg) => write!(f, "API error: {}", msg),
            AgentError::RateLimit(msg) => write!(f, "Rate limit error: {}", msg),
            AgentError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            AgentError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AgentError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_constructors() {
        let sys = Message::system("You are helpful");
        assert_eq!(sys.role, Role::System);
        assert_eq!(sys.content, "You are helpful");

        let user = Message::user("Hello");
        assert_eq!(user.role, Role::User);
        assert_eq!(user.content, "Hello");

        let assistant = Message::assistant("Hi there");
        assert_eq!(assistant.role, Role::Assistant);
        assert_eq!(assistant.content, "Hi there");
    }

    #[test]
    fn test_role_display() {
        assert_eq!(Role::System.to_string(), "system");
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::Assistant.to_string(), "assistant");
    }

    #[test]
    fn test_agent_response() {
        let response = AgentResponse::new("Hello world");
        assert_eq!(response.content, "Hello world");
        assert!(response.metadata.input_tokens.is_none());

        let metadata = ResponseMetadata {
            input_tokens: Some(10),
            output_tokens: Some(5),
            model: Some("test-model".to_string()),
            stop_reason: Some("end_turn".to_string()),
        };
        let response = AgentResponse::with_metadata("Test", metadata);
        assert_eq!(response.content, "Test");
        assert_eq!(response.metadata.input_tokens, Some(10));
        assert_eq!(response.metadata.output_tokens, Some(5));
    }

    #[test]
    fn test_agent_error_display() {
        let err = AgentError::Configuration("missing key".to_string());
        assert!(err.to_string().contains("Configuration error"));
        assert!(err.to_string().contains("missing key"));
    }
}
