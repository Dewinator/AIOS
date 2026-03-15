// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! AIOS Memory Layer
//!
//! Provides controlled, scoped memory for AI agents. Memory is separated into
//! distinct scopes with different lifecycles and encryption requirements.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Memory scopes with different lifecycles and access rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryScope {
    /// Short-lived, cleared after conversation ends
    Session,
    /// Persists across sessions, user-deletable
    Personal,
    /// Tied to specific app or workspace
    AppWorkspace { app_id: String },
    /// Encrypted, biometric-protected
    SecureVault,
}

/// A single memory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub scope: MemoryScope,
    pub key: String,
    pub value: serde_json::Value,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Memory Layer manages agent memory across scopes.
pub struct MemoryLayer {
    // TODO: Replace with SQLite-backed encrypted storage
    entries: Vec<MemoryEntry>,
}

impl MemoryLayer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn store(&mut self, entry: MemoryEntry) {
        self.entries.push(entry);
    }

    pub fn recall(&self, scope: &MemoryScope, key: &str) -> Option<&MemoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.key == key)
            .filter(|e| Self::scope_matches(&e.scope, scope))
            .last()
    }

    pub fn forget(&mut self, id: &str) -> bool {
        let len_before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < len_before
    }

    pub fn clear_scope(&mut self, scope: &MemoryScope) {
        self.entries.retain(|e| !Self::scope_matches(&e.scope, scope));
    }

    fn scope_matches(a: &MemoryScope, b: &MemoryScope) -> bool {
        matches!(
            (a, b),
            (MemoryScope::Session, MemoryScope::Session)
                | (MemoryScope::Personal, MemoryScope::Personal)
                | (MemoryScope::SecureVault, MemoryScope::SecureVault)
                | (
                    MemoryScope::AppWorkspace { app_id: ref a_id },
                    MemoryScope::AppWorkspace { app_id: ref b_id }
                ) if a_id == b_id
        )
    }
}

impl Default for MemoryLayer {
    fn default() -> Self {
        Self::new()
    }
}
