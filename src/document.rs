use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a markdown document with optional YAML frontmatter
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// YAML frontmatter as a map of key-value pairs
    pub frontmatter: HashMap<String, Value>,
    /// The markdown content (everything after frontmatter)
    pub content: String,
}

impl Document {
    /// Creates a new empty document
    pub fn new() -> Self {
        Self {
            frontmatter: HashMap::new(),
            content: String::new(),
        }
    }

    /// Creates a document with content but no frontmatter
    pub fn with_content(content: String) -> Self {
        Self {
            frontmatter: HashMap::new(),
            content,
        }
    }

    /// Parses a markdown string with optional YAML frontmatter
    ///
    /// Frontmatter must be at the start of the file, delimited by `---`
    ///
    /// # Example
    /// ```markdown
    /// ---
    /// title: My Note
    /// tags: [work, important]
    /// ---
    ///
    /// # Content here
    /// ```
    pub fn parse(text: &str) -> Result<Self, String> {
        // Check if the document starts with frontmatter delimiter
        if !text.starts_with("---\n") && !text.starts_with("---\r\n") {
            // No frontmatter, treat entire text as content
            return Ok(Self::with_content(text.to_string()));
        }

        // Find the closing frontmatter delimiter
        let after_first_delimiter = if text.starts_with("---\r\n") {
            &text[5..]
        } else {
            &text[4..]
        };

        // Check if the closing delimiter is at the very start (empty frontmatter)
        let (frontmatter_str, content) = if after_first_delimiter.starts_with("---\n")
            || after_first_delimiter.starts_with("---\r\n") {
            // Empty frontmatter case
            let skip = if after_first_delimiter.starts_with("---\r\n") { 5 } else { 4 };
            ("", after_first_delimiter[skip..].trim_start())
        } else {
            // Look for the closing delimiter
            let end_delimiter_pos = after_first_delimiter
                .find("\n---\n")
                .or_else(|| after_first_delimiter.find("\r\n---\r\n"))
                .or_else(|| after_first_delimiter.find("\n---\r\n"))
                .or_else(|| after_first_delimiter.find("\r\n---\n"));

            match end_delimiter_pos {
                Some(pos) => {
                    let frontmatter = &after_first_delimiter[..pos];
                    let content_start = pos + 5; // Skip past "\n---\n"
                    let content = if content_start < after_first_delimiter.len() {
                        &after_first_delimiter[content_start..]
                    } else {
                        ""
                    };
                    (frontmatter, content.trim_start())
                }
                None => {
                    // No closing delimiter found, treat as no frontmatter
                    return Ok(Self::with_content(text.to_string()));
                }
            }
        };

        // Parse the YAML frontmatter (handle empty case)
        let frontmatter: HashMap<String, Value> = if frontmatter_str.trim().is_empty() {
            HashMap::new()
        } else {
            serde_yaml::from_str(frontmatter_str)
                .map_err(|e| format!("Failed to parse frontmatter: {}", e))?
        };

        Ok(Self {
            frontmatter,
            content: content.to_string(),
        })
    }

    /// Reads and parses a document from a file
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
        Self::parse(&text)
    }

    /// Serializes the document back to a markdown string with frontmatter
    pub fn to_string(&self) -> String {
        if self.frontmatter.is_empty() {
            return self.content.clone();
        }

        let frontmatter_str = serde_yaml::to_string(&self.frontmatter)
            .unwrap_or_else(|_| String::from("{}"));

        format!("---\n{}---\n\n{}", frontmatter_str, self.content)
    }

    /// Writes the document to a file
    pub fn to_file(&self, path: &Path) -> Result<(), String> {
        let content = self.to_string();
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file {}: {}", path.display(), e))
    }

    /// Gets a frontmatter value by key
    pub fn get_frontmatter(&self, key: &str) -> Option<&Value> {
        self.frontmatter.get(key)
    }

    /// Gets a frontmatter string value by key
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.frontmatter.get(key).and_then(|v| v.as_str()).map(String::from)
    }

    /// Gets a frontmatter array of strings by key
    pub fn get_strings(&self, key: &str) -> Option<Vec<String>> {
        self.frontmatter.get(key).and_then(|v| {
            v.as_sequence().map(|seq| {
                seq.iter()
                    .filter_map(|item| item.as_str().map(String::from))
                    .collect()
            })
        })
    }

    /// Sets a frontmatter value
    pub fn set_frontmatter(&mut self, key: String, value: Value) {
        self.frontmatter.insert(key, value);
    }

    /// Sets a frontmatter string value
    pub fn set_string(&mut self, key: String, value: String) {
        self.frontmatter.insert(key, Value::String(value));
    }

    /// Sets a frontmatter array of strings
    pub fn set_strings(&mut self, key: String, values: Vec<String>) {
        let yaml_values: Vec<Value> = values.into_iter().map(Value::String).collect();
        self.frontmatter.insert(key, Value::Sequence(yaml_values));
    }

    /// Removes a frontmatter key
    pub fn remove_frontmatter(&mut self, key: &str) -> Option<Value> {
        self.frontmatter.remove(key)
    }

    /// Checks if frontmatter contains a key
    pub fn has_frontmatter(&self, key: &str) -> bool {
        self.frontmatter.contains_key(key)
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_document_without_frontmatter() {
        let text = "# Hello\n\nThis is content.";
        let doc = Document::parse(text).unwrap();

        assert!(doc.frontmatter.is_empty());
        assert_eq!(doc.content, text);
    }

    #[test]
    fn test_parse_document_with_frontmatter() {
        let text = "---\ntitle: My Note\ntags:\n  - work\n  - important\n---\n\n# Content\n\nHello!";
        let doc = Document::parse(text).unwrap();

        assert_eq!(doc.get_string("title"), Some("My Note".to_string()));
        assert_eq!(doc.get_strings("tags"), Some(vec!["work".to_string(), "important".to_string()]));
        assert_eq!(doc.content, "# Content\n\nHello!");
    }

    #[test]
    fn test_parse_document_with_empty_frontmatter() {
        let text = "---\n---\n\n# Content";
        let doc = Document::parse(text).unwrap();

        assert!(doc.frontmatter.is_empty());
        assert_eq!(doc.content, "# Content");
    }

    #[test]
    fn test_to_string_without_frontmatter() {
        let doc = Document::with_content("# Hello\n\nContent here.".to_string());
        let result = doc.to_string();

        assert_eq!(result, "# Hello\n\nContent here.");
    }

    #[test]
    fn test_to_string_with_frontmatter() {
        let mut doc = Document::with_content("# Content".to_string());
        doc.set_string("title".to_string(), "My Note".to_string());
        doc.set_strings("tags".to_string(), vec!["work".to_string(), "personal".to_string()]);

        let result = doc.to_string();

        assert!(result.starts_with("---\n"));
        assert!(result.contains("title: My Note"));
        assert!(result.contains("tags:"));
        assert!(result.contains("# Content"));
    }

    #[test]
    fn test_roundtrip_parse_and_serialize() {
        let original = "---\ntitle: Test\ndate: 2025-12-28\n---\n\n# Hello World";
        let doc = Document::parse(original).unwrap();
        let serialized = doc.to_string();
        let reparsed = Document::parse(&serialized).unwrap();

        assert_eq!(doc.get_string("title"), reparsed.get_string("title"));
        assert_eq!(doc.get_string("date"), reparsed.get_string("date"));
        assert_eq!(doc.content, reparsed.content);
    }

    #[test]
    fn test_get_and_set_frontmatter() {
        let mut doc = Document::new();

        assert_eq!(doc.get_string("title"), None);

        doc.set_string("title".to_string(), "My Title".to_string());
        assert_eq!(doc.get_string("title"), Some("My Title".to_string()));

        doc.set_strings("tags".to_string(), vec!["a".to_string(), "b".to_string()]);
        assert_eq!(doc.get_strings("tags"), Some(vec!["a".to_string(), "b".to_string()]));
    }

    #[test]
    fn test_remove_frontmatter() {
        let mut doc = Document::new();
        doc.set_string("title".to_string(), "Test".to_string());

        assert!(doc.has_frontmatter("title"));

        let removed = doc.remove_frontmatter("title");
        assert!(removed.is_some());
        assert!(!doc.has_frontmatter("title"));
    }

    #[test]
    fn test_from_file_and_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let mut original_doc = Document::with_content("# Test Content".to_string());
        original_doc.set_string("title".to_string(), "File Test".to_string());

        original_doc.to_file(path).unwrap();

        let loaded_doc = Document::from_file(path).unwrap();

        assert_eq!(loaded_doc.get_string("title"), Some("File Test".to_string()));
        assert_eq!(loaded_doc.content, "# Test Content");
    }

    #[test]
    fn test_parse_crlf_frontmatter() {
        let text = "---\r\ntitle: Windows\r\n---\r\n\r\n# Content";
        let doc = Document::parse(text).unwrap();

        assert_eq!(doc.get_string("title"), Some("Windows".to_string()));
        assert_eq!(doc.content, "# Content");
    }
}
