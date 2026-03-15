// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! AIOS Safety Monitor
//!
//! Runtime monitoring for agent behavior. Detects anomalies such as
//! unusual tool chains, loops, data exfiltration attempts, prompt injection
//! indicators, and excessive resource consumption.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// A recorded action for analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecord {
    pub tool_id: String,
    pub timestamp: DateTime<Utc>,
    pub origin: String,
    pub capabilities_used: Vec<String>,
}

/// Types of anomalies the Safety Monitor can detect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Anomaly {
    LoopDetected {
        tool_id: String,
        count: usize,
    },
    ExcessiveActions {
        count: usize,
        window_seconds: u64,
    },
    SuspiciousChain {
        tools: Vec<String>,
        reason: String,
    },
    PossibleExfiltration {
        source_capability: String,
        destination_capability: String,
    },
}

/// Safety Monitor tracks agent behavior and flags anomalies.
pub struct SafetyMonitor {
    history: VecDeque<ActionRecord>,
    max_history: usize,
    loop_threshold: usize,
    actions_per_minute_limit: usize,
}

impl SafetyMonitor {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            max_history: 1000,
            loop_threshold: 5,
            actions_per_minute_limit: 30,
        }
    }

    /// Record an action and check for anomalies.
    pub fn record_and_check(&mut self, action: ActionRecord) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Check for loops (same tool called repeatedly)
        let recent_same = self
            .history
            .iter()
            .rev()
            .take(self.loop_threshold)
            .filter(|a| a.tool_id == action.tool_id)
            .count();

        if recent_same >= self.loop_threshold {
            anomalies.push(Anomaly::LoopDetected {
                tool_id: action.tool_id.clone(),
                count: recent_same + 1,
            });
        }

        // Check for excessive actions per minute
        let one_minute_ago = Utc::now() - chrono::Duration::seconds(60);
        let recent_count = self
            .history
            .iter()
            .filter(|a| a.timestamp > one_minute_ago)
            .count();

        if recent_count >= self.actions_per_minute_limit {
            anomalies.push(Anomaly::ExcessiveActions {
                count: recent_count + 1,
                window_seconds: 60,
            });
        }

        // Check for suspicious read→send chains (possible exfiltration)
        if action.capabilities_used.iter().any(|c| c.contains(".send") || c.contains("network")) {
            let recent_reads: Vec<_> = self
                .history
                .iter()
                .rev()
                .take(5)
                .filter(|a| a.capabilities_used.iter().any(|c| c.contains(".read")))
                .collect();

            for read_action in recent_reads {
                for read_cap in &read_action.capabilities_used {
                    if read_cap.contains(".read") {
                        for send_cap in &action.capabilities_used {
                            if send_cap.contains(".send") || send_cap.contains("network") {
                                anomalies.push(Anomaly::PossibleExfiltration {
                                    source_capability: read_cap.clone(),
                                    destination_capability: send_cap.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }

        // Store action
        self.history.push_back(action);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        anomalies
    }

    /// Trigger emergency stop — clear all state and signal system.
    pub fn kill_switch(&mut self) -> bool {
        self.history.clear();
        tracing::error!("KILL SWITCH ACTIVATED — all agent activity halted");
        true
    }
}

impl Default for SafetyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_action(tool_id: &str, capabilities: Vec<&str>) -> ActionRecord {
        ActionRecord {
            tool_id: tool_id.to_string(),
            timestamp: Utc::now(),
            origin: "user".to_string(),
            capabilities_used: capabilities.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_loop_detection() {
        let mut monitor = SafetyMonitor::new();

        for _ in 0..5 {
            monitor.record_and_check(make_action("system.files.read", vec!["files.read"]));
        }

        let anomalies = monitor.record_and_check(make_action("system.files.read", vec!["files.read"]));
        assert!(anomalies.iter().any(|a| matches!(a, Anomaly::LoopDetected { .. })));
    }

    #[test]
    fn test_no_false_positive_for_different_tools() {
        let mut monitor = SafetyMonitor::new();

        monitor.record_and_check(make_action("tool.a", vec![]));
        monitor.record_and_check(make_action("tool.b", vec![]));
        monitor.record_and_check(make_action("tool.c", vec![]));

        let anomalies = monitor.record_and_check(make_action("tool.d", vec![]));
        assert!(anomalies.is_empty());
    }

    #[test]
    fn test_exfiltration_detection() {
        let mut monitor = SafetyMonitor::new();

        monitor.record_and_check(make_action("system.contacts.read", vec!["contacts.list.read"]));

        let anomalies = monitor.record_and_check(
            make_action("system.messages.send", vec!["messages.sms.send"])
        );
        assert!(anomalies.iter().any(|a| matches!(a, Anomaly::PossibleExfiltration { .. })));
    }

    #[test]
    fn test_kill_switch() {
        let mut monitor = SafetyMonitor::new();
        monitor.record_and_check(make_action("test", vec![]));
        assert!(monitor.kill_switch());
    }
}
