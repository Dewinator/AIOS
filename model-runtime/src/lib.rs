// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! AIOS Model Runtime Manager
//!
//! Manages local and optional remote AI models. Handles model selection,
//! quantization profiles, hardware acceleration (GPU/NPU/CPU), offline
//! fallback, context window management, and resource budgets.

use serde::{Deserialize, Serialize};

/// Types of models available in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    /// Small model for intent classification
    IntentParser,
    /// On-device speech recognition
    Asr,
    /// On-device text-to-speech
    Tts,
    /// Medium model for task planning
    Planner,
    /// Vision model for UI/file/camera understanding
    Vision,
    /// Large remote model for complex reasoning (optional)
    RemoteReasoning,
}

/// Hardware target for model execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "uppercase")]
pub enum HardwareTarget {
    Cpu,
    Gpu,
    Npu,
    Dsp,
}

/// Model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_type: ModelType,
    pub model_path: Option<String>,
    pub quantization: Option<String>,
    pub hardware_target: HardwareTarget,
    pub max_tokens: usize,
    pub is_local: bool,
}

/// Resource budget for model execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    pub max_ram_mb: usize,
    pub max_inference_time_ms: u64,
    pub max_battery_percent_per_hour: f32,
}

/// Model Runtime Manager handles model lifecycle.
pub struct ModelRuntimeManager {
    models: Vec<ModelConfig>,
    budget: ResourceBudget,
}

impl ModelRuntimeManager {
    pub fn new(budget: ResourceBudget) -> Self {
        Self {
            models: Vec::new(),
            budget,
        }
    }

    pub fn register_model(&mut self, config: ModelConfig) {
        self.models.push(config);
    }

    /// Select the best model for a given task type.
    pub fn select_model(&self, model_type: &ModelType) -> Option<&ModelConfig> {
        self.models
            .iter()
            .filter(|m| std::mem::discriminant(&m.model_type) == std::mem::discriminant(model_type))
            .filter(|m| m.is_local) // Prefer local models
            .next()
            .or_else(|| {
                // Fall back to remote if no local model available
                self.models
                    .iter()
                    .filter(|m| std::mem::discriminant(&m.model_type) == std::mem::discriminant(model_type))
                    .next()
            })
    }

    pub fn available_models(&self) -> &[ModelConfig] {
        &self.models
    }
}
