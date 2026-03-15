# AIOS — AI-native Operating System

An Android-compatible agent operating system built on Linux/AOSP, where AI agents work as privileged, policy-controlled system services — not just apps.

**Core principle: AI-first, policy-driven, Android-compatible.**

## Vision

A mobile operating system for smartphones, tablets, and edge devices that:

- Runs on Linux / AOSP with full Android hardware support
- Integrates local AI models and agents as core system functionality
- Controls all system actions through a secure policy layer
- Lets users interact with an executing AI system, not just isolated apps
- Prefers local execution — cloud is optional

## Architecture

```
┌─────────────────────────────────────────┐
│  Experience Layer                       │
│  Conversational Shell · Agent UI        │
│  Task Feed · Approval Center · Logs     │
├─────────────────────────────────────────┤
│  AI Runtime Layer                       │
│  Model Runtime · Context Service        │
│  Tool Broker · Policy Engine            │
│  Memory Layer · Task Planner            │
│  Audit Service · Consent · Safety       │
├─────────────────────────────────────────┤
│  Android Core / System Services         │
│  init · Binder IPC · HAL               │
│  System Server · Package Manager        │
├─────────────────────────────────────────┤
│  Kernel / Low-Level                     │
│  Android Common Kernel · SELinux        │
│  Drivers · cgroups · namespaces         │
├─────────────────────────────────────────┤
│  Hardware                               │
│  ARM SoC · NPU/DSP/GPU · Sensors       │
│  Secure Element / TEE                   │
└─────────────────────────────────────────┘
```

## Key Components

| Component | Description | Language |
|-----------|-------------|----------|
| `aiosd` | Privileged AI system daemon | Kotlin/Java |
| `broker` | Tool Broker — bridge between AI and system | Rust |
| `policy-engine` | deny-by-default action gating | Rust |
| `safety-monitor` | Runtime anomaly detection | Rust |
| `memory-layer` | Scoped, encrypted agent memory | Rust |
| `model-runtime` | Local model management (GPU/NPU/CPU) | Rust |
| `shell-app` | Conversational Shell UI | Kotlin (Compose) |
| `audit` | Full action logging + viewer | Rust + Kotlin |
| `consent` | Multi-level user approval | Kotlin |

## Security Model

Every AI action follows this pipeline:

1. **Recognize** user intent
2. **Plan** the execution steps
3. **Check** permissions (Policy Engine)
4. **Assess** risk level
5. **Simulate** or explain the action
6. **Execute** only after approval
7. **Log** everything

The system uses capability-based rights (e.g., `contacts.read`, `calendar.write`) — never generic root access. Four trust zones separate Secure Core, Privileged AI, User-space Tools, and Untrusted Inputs.

## User Modes

| Mode | Description |
|------|-------------|
| **Assist** | Suggestions only, execute after confirmation |
| **Guided Autonomy** | Routine tasks in approved areas run independently |
| **Trusted Automation** | Extended agent rights in tightly scoped workspaces |

## Project Structure

```
AIOS/
├── docs/               # Architecture, threat model, specs
├── schemas/            # Tool and policy JSON/YAML schemas
├── aiosd/              # Privileged Android system service
├── broker/             # Tool Broker (Rust)
├── policy-engine/      # Policy Engine (Rust)
├── safety-monitor/     # Safety Monitor (Rust)
├── memory-layer/       # Memory Layer (Rust)
├── model-runtime/      # Model Runtime Manager (Rust)
├── tools/              # System tool implementations
├── shell-app/          # Conversational Shell (Jetpack Compose)
├── audit/              # Audit service + log viewer
├── consent/            # Consent service
├── aosp-overlay/       # SELinux policies, init scripts
└── tests/              # Integration & E2E tests
```

## Roadmap

- **Phase 0** — Research: Threat model, tool schema standard, policy design, model benchmarks
- **Phase 1** — MVP: Agent as privileged system service on custom Android build
- **Phase 2** — Deep AOSP integration: Roles, memory, multimodal input, app-intent bridge
- **Phase 3** — Developer platform: Tool SDK, certification, plugin store
- **Phase 4** — Standalone OS product: OEM partnerships, OTA, enterprise features

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Kernel | AOSP, Android Common Kernel, SELinux |
| System service | Kotlin/Java, Binder/AIDL |
| Core engines | Rust |
| Database | SQLite (encrypted) |
| AI Runtime | llama.cpp / ONNX Runtime / MediaPipe |
| UI | Jetpack Compose |
| Schemas | JSON Schema |
| Policies | YAML |

## Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

## License

This project is licensed under the Apache License 2.0 — see [LICENSE](LICENSE) for details.
