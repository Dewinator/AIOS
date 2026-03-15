// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

//! AIOS Tool Broker
//!
//! The Tool Broker is the only standardized bridge between AI agents and system
//! functions. It validates tool calls against their schemas, routes them through
//! the Policy Engine, and logs all actions via the Audit Service.

mod registry;
mod executor;
mod validation;

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("AIOS Tool Broker starting...");

    let registry = registry::ToolRegistry::new();
    info!("Tool registry initialized with {} tools", registry.tool_count());

    // TODO: Start IPC listener (Unix socket / Binder bridge)
    // TODO: Accept tool call requests from aiosd
    // TODO: Validate, check policy, execute, audit

    info!("AIOS Tool Broker ready");
}
