# AIOS Architecture

## Layer Model

### Layer 1 — Hardware
- ARM SoC
- NPU / DSP / GPU
- Camera, microphones, sensors
- Radio modules
- Secure Element / TEE

### Layer 2 — Kernel / Low-Level
- Android Common Kernel or compatible Linux kernel
- Drivers
- cgroups
- namespaces
- SELinux
- eBPF (optional for telemetry and control)

### Layer 3 — Android Core / System Services
- init
- Binder IPC
- HAL
- System Server
- Package Manager
- Activity Manager
- Media / Sensor / Location / Connectivity Services

### Layer 4 — AI Runtime Layer

The core product layer:

- **Model Runtime Manager** — Manages local and optional remote models
- **Context Service** — Provides device and user context to the AI
- **Tool Broker** — The only bridge between AI and system functions
- **Policy Engine** — Evaluates every action before execution
- **Memory Layer** — Controlled agent memory (session, personal, app, vault)
- **Task Planner** — Intent parsing, step decomposition, tool selection
- **Audit Service** — Full action logging
- **Consent Service** — Multi-level user approval
- **Safety Monitor** — Runtime anomaly detection

### Layer 5 — Experience Layer
- Conversational Shell
- Agent UI
- Task Feed
- Approval Center
- Logs / Explainability UI
- Settings / Permissions UI

### Layer 6 — App & Tool Ecosystem
- Android Apps
- Internal system tools
- Plugin tools
- Enterprise connectors

## Trust Zones

### Zone A — Secure Core
- Boot, Keystore, Identity
- Policy Engine, Audit Logs

### Zone B — Privileged System AI
- Planner, Broker
- Context Service, Runtime Manager

### Zone C — User-space Tools
- Connectors, file tools
- App adapters, third-party plugins

### Zone D — Untrusted Inputs
- Websites, emails, PDFs
- Messages, app content
- Voice transcripts

## Data Flow

```
User Input (text/voice)
    │
    ▼
┌─────────────────┐
│  Intent Parser   │  (local micro-model)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Task Planner    │  decompose into steps
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Tool Broker     │  validate & route tool calls
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Policy Engine   │  check permissions & risk
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
 allow     deny/confirm
    │         │
    ▼         ▼
┌────────┐ ┌──────────┐
│Execute │ │ Consent  │
│ Tool   │ │ Service  │
└───┬────┘ └────┬─────┘
    │           │
    ▼           ▼
┌─────────────────┐
│  Audit Service   │  log everything
└─────────────────┘
```

## Component Communication

- **aiosd ↔ Tool Broker**: Binder/AIDL
- **Tool Broker ↔ Policy Engine**: In-process (Rust library)
- **Tool Broker ↔ System Tools**: Structured IPC (JSON over Unix socket)
- **aiosd ↔ Shell App**: Binder/AIDL
- **Model Runtime ↔ aiosd**: Binder/AIDL
