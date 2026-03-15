// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! AIOS Policy Engine
//!
//! The most critical security component. Evaluates every tool call request
//! against the loaded policy rules and returns a decision.
//!
//! Principle: deny-by-default. No action is allowed unless an explicit rule permits it.

use serde::{Deserialize, Serialize};

/// The possible decisions the Policy Engine can make.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Decision {
    Allow,
    AllowWithLog,
    RequireConfirmation,
    Deny,
    Quarantine,
}

/// Context provided with each policy evaluation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyContext {
    pub tool_id: String,
    pub risk_class: String,
    pub capabilities: Vec<String>,
    pub side_effects: Vec<String>,
    pub device_state: DeviceState,
    pub user_role: String,
    pub origin: RequestOrigin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    pub locked: bool,
    pub network: NetworkState,
    pub battery_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkState {
    Wifi,
    Cellular,
    Roaming,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestOrigin {
    User,
    Agent,
    Plugin,
    System,
}

/// Result of a policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    pub decision: Decision,
    pub consent_required: Option<String>,
    pub reason: String,
    pub matched_rule: Option<String>,
}

/// The Policy Engine evaluates tool call requests against loaded rules.
pub struct PolicyEngine {
    default_decision: Decision,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            default_decision: Decision::Deny,
        }
    }

    /// Evaluate a tool call request against the policy.
    pub fn evaluate(&self, context: &PolicyContext) -> PolicyResult {
        // Check device-locked restriction
        if context.device_state.locked {
            if context.risk_class != "low" {
                return PolicyResult {
                    decision: Decision::Deny,
                    consent_required: None,
                    reason: "Device is locked — only low-risk actions allowed".to_string(),
                    matched_rule: Some("locked-device-restrictions".to_string()),
                };
            }
        }

        // Check roaming restriction
        if matches!(context.device_state.network, NetworkState::Roaming) {
            if context.capabilities.iter().any(|c| c.starts_with("network.")) {
                return PolicyResult {
                    decision: Decision::Deny,
                    consent_required: None,
                    reason: "Network actions blocked while roaming".to_string(),
                    matched_rule: Some("roaming-restrictions".to_string()),
                };
            }
        }

        // Risk-based evaluation
        match context.risk_class.as_str() {
            "low" => {
                if context.side_effects.is_empty() {
                    PolicyResult {
                        decision: Decision::Allow,
                        consent_required: None,
                        reason: "Low-risk, no side effects".to_string(),
                        matched_rule: Some("allow-low-risk-local".to_string()),
                    }
                } else {
                    PolicyResult {
                        decision: Decision::AllowWithLog,
                        consent_required: None,
                        reason: "Low-risk with side effects".to_string(),
                        matched_rule: Some("allow-low-risk-with-effects".to_string()),
                    }
                }
            }
            "medium" => PolicyResult {
                decision: Decision::RequireConfirmation,
                consent_required: Some("once".to_string()),
                reason: "Medium-risk action requires confirmation".to_string(),
                matched_rule: Some("confirm-medium-risk".to_string()),
            },
            "high" => PolicyResult {
                decision: Decision::RequireConfirmation,
                consent_required: Some("strong".to_string()),
                reason: "High-risk action requires strong confirmation".to_string(),
                matched_rule: Some("confirm-high-risk".to_string()),
            },
            "critical" => PolicyResult {
                decision: Decision::RequireConfirmation,
                consent_required: Some("biometric".to_string()),
                reason: "Critical action requires biometric confirmation".to_string(),
                matched_rule: Some("confirm-critical".to_string()),
            },
            _ => PolicyResult {
                decision: self.default_decision.clone(),
                consent_required: None,
                reason: "Unknown risk class — denied by default".to_string(),
                matched_rule: None,
            },
        }
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_context(risk_class: &str, side_effects: Vec<String>) -> PolicyContext {
        PolicyContext {
            tool_id: "test.tool".to_string(),
            risk_class: risk_class.to_string(),
            capabilities: vec![],
            side_effects,
            device_state: DeviceState {
                locked: false,
                network: NetworkState::Wifi,
                battery_level: 80,
            },
            user_role: "owner".to_string(),
            origin: RequestOrigin::User,
        }
    }

    #[test]
    fn test_low_risk_no_side_effects_allowed() {
        let engine = PolicyEngine::new();
        let ctx = make_context("low", vec![]);
        let result = engine.evaluate(&ctx);
        assert_eq!(result.decision, Decision::Allow);
    }

    #[test]
    fn test_low_risk_with_side_effects_logged() {
        let engine = PolicyEngine::new();
        let ctx = make_context("low", vec!["modifies_settings".to_string()]);
        let result = engine.evaluate(&ctx);
        assert_eq!(result.decision, Decision::AllowWithLog);
    }

    #[test]
    fn test_high_risk_requires_confirmation() {
        let engine = PolicyEngine::new();
        let ctx = make_context("high", vec![]);
        let result = engine.evaluate(&ctx);
        assert_eq!(result.decision, Decision::RequireConfirmation);
        assert_eq!(result.consent_required, Some("strong".to_string()));
    }

    #[test]
    fn test_locked_device_denies_non_low_risk() {
        let engine = PolicyEngine::new();
        let mut ctx = make_context("medium", vec![]);
        ctx.device_state.locked = true;
        let result = engine.evaluate(&ctx);
        assert_eq!(result.decision, Decision::Deny);
    }

    #[test]
    fn test_unknown_risk_class_denied() {
        let engine = PolicyEngine::new();
        let ctx = make_context("unknown", vec![]);
        let result = engine.evaluate(&ctx);
        assert_eq!(result.decision, Decision::Deny);
    }
}
