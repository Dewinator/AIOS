# AIOS — The AI Operating System

> An open-source **AI operating system** built on Android/AOSP where AI agents run as privileged system services — not just apps. On-device LLM inference, policy-controlled tool execution, and a conversational shell that replaces the traditional launcher.

**AIOS is an AI OS** — a complete AI-native operating system for smartphones, tablets, and edge devices. It combines on-device large language models with a secure tool execution pipeline, making the AI a first-class citizen of the operating system rather than an app running on top of it.

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

---

## What Makes This an AI Operating System?

Most "AI assistants" are apps that sit in a sandbox with limited system access. **AIOS is different** — the AI agent operates at the system level with:

- **44 system tools** — WiFi, Bluetooth, brightness, focus mode, calendar, contacts, messaging, file management, app control, alarms, and more
- **On-device LLM inference** — run GGUF models locally via llama.cpp (no cloud required)
- **Multi-backend AI** — seamlessly switch between local models, Claude/OpenAI APIs, or a remote MLX server
- **Policy engine** — every AI action is gated by a deny-by-default security policy
- **Tool broker** — validates, executes, and audits every system action
- **Conversational shell** — the AI *is* the launcher, not an app inside one

## Current Status: Working MVP

The AIOS shell app is a functional Android application that you can build and run today:

- Conversational AI interface with animated avatar
- On-device LLM via llama.cpp (download GGUF models from HuggingFace)
- External API support (Claude, OpenAI, custom endpoints)
- 44 system tools with policy-controlled execution
- Full audit logging of every action
- Settings UI for backend selection, model management, and generation parameters
- Runs as Android home screen launcher

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

## How It Works

Every user request flows through a secure pipeline:

```
User Input → Intent Parser → Task Planner → Policy Engine → Tool Broker → System Tools
                                  ↓                              ↓
                            Consent UI                      Audit Log
```

1. **Parse** — understand the user's intent (via LLM or keyword matching)
2. **Plan** — break complex requests into executable steps
3. **Check** — policy engine evaluates risk and permissions
4. **Approve** — user confirms if needed (based on risk level)
5. **Execute** — tool broker runs the action via Android system APIs
6. **Log** — every action is recorded in the audit trail

## Key Components

| Component | Description | Language |
|-----------|-------------|----------|
| `shell-app` | Conversational Shell UI + AI launcher | Kotlin (Compose) |
| `shell-app/ai` | Multi-backend AI manager (Local/API/Remote) | Kotlin |
| `shell-app/cpp` | On-device LLM via llama.cpp + JNI bridge | C++ |
| `shell-app/broker` | Tool Broker — validates and executes actions | Kotlin |
| `shell-app/policy` | deny-by-default Policy Engine | Kotlin |
| `shell-app/tools` | 44 Android system tool implementations | Kotlin |
| `shell-app/planner` | Task decomposition and multi-step planning | Kotlin |
| `llm-server` | Development MLX server for remote inference | Python |

## AI Backends

AIOS supports three AI backends that can be switched at runtime:

| Backend | Use Case | Requirements |
|---------|----------|-------------|
| **Local (llama.cpp)** | Privacy-first, offline capable | GGUF model on device |
| **External API** | Best quality (Claude, OpenAI) | API key |
| **Remote Server** | Development with MLX | Mac with Apple Silicon |

### On-Device Models

Download GGUF models directly from the settings screen:

| Model | Size | Description |
|-------|------|-------------|
| Qwen 2.5 1.5B Q4_K_M | ~1.1 GB | Compact, good speed/quality balance |
| Qwen 2.5 3B Q4_K_M | ~2.1 GB | Best quality for mobile |
| SmolLM2 1.7B Q4_K_M | ~1.0 GB | Very fast, good for simple tasks |
| TinyLlama 1.1B Q8_0 | ~1.2 GB | Smallest model, fastest responses |

## Security Model

The AI operating system enforces strict security at every level:

- **Deny-by-default** — no action is allowed unless explicitly permitted
- **Capability-based rights** — `contacts.read`, `calendar.write`, not root access
- **Risk classification** — every tool has a risk level (LOW, MEDIUM, HIGH, CRITICAL)
- **Multi-level consent** — from silent allow to biometric confirmation
- **Four trust zones** — Secure Core, Privileged AI, User-space Tools, Untrusted Inputs
- **Full audit trail** — every action logged with timestamps and rollback info

### User Modes

| Mode | Description |
|------|-------------|
| **Assist** | Suggestions only, execute after confirmation |
| **Guided Autonomy** | Routine tasks in approved areas run independently |
| **Trusted Automation** | Extended agent rights in tightly scoped workspaces |

## Building

### Prerequisites

- Android Studio (Arctic Fox or newer)
- Android SDK 35 / NDK
- Java 21 (bundled with Android Studio)

### Build & Run

```bash
cd shell-app
./gradlew assembleDebug

# Install on device/emulator
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

> **Note:** On-device LLM inference requires a real Android device (ARM64) for reasonable performance. The emulator works for UI testing and API backends.

## Roadmap

- [x] **Phase 0** — Research: Architecture, tool schema, policy design
- [x] **Phase 1a** — MVP Shell: Conversational UI, tool broker, policy engine, 44 tools
- [x] **Phase 1b** — On-device AI: llama.cpp integration, model download, multi-backend
- [ ] **Phase 1c** — System prompt optimization, memory layer, context management
- [ ] **Phase 2** — Deep AOSP integration: Privileged system service, roles, multimodal input
- [ ] **Phase 3** — Developer platform: Tool SDK, certification, plugin store
- [ ] **Phase 4** — Standalone AI OS product: OEM partnerships, OTA, enterprise

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Kernel | AOSP, Android Common Kernel, SELinux |
| AI Runtime | llama.cpp (on-device), Claude/OpenAI APIs, MLX |
| System services | Kotlin, Android SDK |
| Core engines | Kotlin (migrating to Rust) |
| Database | SQLite (encrypted) |
| UI | Jetpack Compose (Material 3) |
| Native bridge | C++ / JNI / NDK |

## Contributing

We welcome contributions! See our [Contributing Guide](docs/CONTRIBUTING.md) for details.

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.
