// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! Tool Registry
//!
//! Manages the catalog of available system tools. Each tool is defined by a
//! JSON schema that specifies its interface, permissions, risk class, and
//! audit requirements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
    pub capabilities: Vec<String>,
    pub risk_class: RiskClass,
    pub side_effects: Vec<String>,
    pub rollback: RollbackInfo,
    pub audit: AuditConfig,
    pub consent_level: ConsentLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskClass {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackInfo {
    pub supported: bool,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub log_input: bool,
    pub log_output: bool,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsentLevel {
    None,
    Once,
    Persistent,
    Strong,
    Biometric,
}

pub struct ToolRegistry {
    tools: HashMap<String, ToolDefinition>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: ToolDefinition) {
        self.tools.insert(tool.id.clone(), tool);
    }

    pub fn get(&self, tool_id: &str) -> Option<&ToolDefinition> {
        self.tools.get(tool_id)
    }

    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }

    pub fn list_tools(&self) -> Vec<&ToolDefinition> {
        self.tools.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_retrieve_tool() {
        let mut registry = ToolRegistry::new();
        let tool = ToolDefinition {
            id: "system.test.example".to_string(),
            name: "Test Tool".to_string(),
            description: "A test tool".to_string(),
            version: "0.1.0".to_string(),
            category: "test".to_string(),
            input: serde_json::json!({"type": "object"}),
            output: serde_json::json!({"type": "object"}),
            capabilities: vec!["test.read".to_string()],
            risk_class: RiskClass::Low,
            side_effects: vec![],
            rollback: RollbackInfo {
                supported: false,
                method: "n/a".to_string(),
            },
            audit: AuditConfig {
                log_input: true,
                log_output: false,
                retention_days: 30,
            },
            consent_level: ConsentLevel::None,
        };

        registry.register(tool);
        assert_eq!(registry.tool_count(), 1);
        assert!(registry.get("system.test.example").is_some());
        assert!(registry.get("nonexistent").is_none());
    }
}
