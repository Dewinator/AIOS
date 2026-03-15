// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! Input Validation
//!
//! Validates tool call inputs against their registered JSON schemas
//! before passing them to the Policy Engine and Executor.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Tool '{0}' not found in registry")]
    ToolNotFound(String),

    #[error("Invalid input for tool '{tool_id}': {message}")]
    InvalidInput {
        tool_id: String,
        message: String,
    },

    #[error("Missing required field '{field}' for tool '{tool_id}'")]
    MissingField {
        tool_id: String,
        field: String,
    },
}

pub struct InputValidator;

impl InputValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate tool call input against the tool's JSON schema.
    pub fn validate(
        &self,
        tool_id: &str,
        input: &serde_json::Value,
        schema: &serde_json::Value,
    ) -> Result<(), ValidationError> {
        // TODO: Use jsonschema crate for full JSON Schema validation
        // For now, basic check that input is an object
        if !input.is_object() {
            return Err(ValidationError::InvalidInput {
                tool_id: tool_id.to_string(),
                message: "Input must be a JSON object".to_string(),
            });
        }

        // Check required fields from schema
        if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
            for field in required {
                if let Some(field_name) = field.as_str() {
                    if input.get(field_name).is_none() {
                        return Err(ValidationError::MissingField {
                            tool_id: tool_id.to_string(),
                            field: field_name.to_string(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_input() {
        let validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "required": ["path", "scope"]
        });
        let input = json!({
            "path": "Documents/note.txt",
            "scope": "documents"
        });

        assert!(validator.validate("system.files.read", &input, &schema).is_ok());
    }

    #[test]
    fn test_missing_required_field() {
        let validator = InputValidator::new();
        let schema = json!({
            "type": "object",
            "required": ["path", "scope"]
        });
        let input = json!({
            "path": "Documents/note.txt"
        });

        assert!(validator.validate("system.files.read", &input, &schema).is_err());
    }

    #[test]
    fn test_non_object_input() {
        let validator = InputValidator::new();
        let schema = json!({"type": "object"});
        let input = json!("not an object");

        assert!(validator.validate("test", &input, &schema).is_err());
    }
}
