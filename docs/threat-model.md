# AIOS Threat Model

## 1. Overview

AIOS gives AI agents privileged system access. This creates a unique threat landscape where traditional mobile security concerns combine with AI-specific risks.

## 2. Threat Categories

### 2.1 Prompt Injection

**Description**: External content (emails, PDFs, websites, messages) contains hidden instructions that manipulate the AI agent into performing unauthorized actions.

**Attack vectors**:
- Email instructs model to ignore security rules
- PDF contains hidden agent commands
- Website manipulates tool usage
- Message contains social engineering for the agent

**Mitigations**:
- Data/Instruction separation at all input boundaries
- Origin marking on all external content
- Tool calls only from trusted plan channel
- Content sanitization pipeline
- Policy recheck before every sensitive action
- Safety Monitor detects instruction-like patterns in data

### 2.2 Data Exfiltration

**Description**: AI agent reads sensitive local data and transmits it to external destinations.

**Attack vectors**:
- Agent reads contacts/messages and sends to unknown API
- Agent composes message with sensitive data to wrong recipient
- Plugin tool collects and exfiltrates data

**Mitigations**:
- Capability-scoped file/data access (no global filesystem rights)
- Network access restricted to approved domains
- Policy Engine blocks `sensitive_source + untrusted_destination` combinations
- Audit logs flag unusual data access patterns
- Safety Monitor detects exfiltration patterns

### 2.3 Privilege Escalation

**Description**: Agent or plugin attempts to gain broader system access than granted.

**Attack vectors**:
- Agent chains low-risk tools to achieve high-risk outcome
- Plugin requests escalated permissions at runtime
- Agent modifies its own policy or capability set

**Mitigations**:
- Capability model: explicit, non-expandable rights
- Models cannot modify Policy Engine rules
- Tool chain analysis in Safety Monitor
- Quarantine mode for suspicious behavior

### 2.4 Plugin/Tool Abuse

**Description**: Third-party tools perform malicious actions or exceed their declared scope.

**Attack vectors**:
- Plugin has undeclared side effects
- Plugin communicates with unauthorized external services
- Plugin exploits system APIs beyond its manifest

**Mitigations**:
- Mandatory tool manifest with declared side effects
- Sandbox execution for third-party tools
- Signed tools with certificate verification
- Review process before marketplace publication
- Blocklist for compromised plugins
- Runtime monitoring of tool behavior

### 2.5 Model Hallucination with System Impact

**Description**: AI model generates incorrect tool calls or parameters that cause unintended system changes.

**Attack vectors**:
- Model hallucinates a contact name → message sent to wrong person
- Model misinterprets intent → deletes files instead of organizing them
- Model generates invalid parameters → system enters undefined state

**Mitigations**:
- Plan preview before execution (user sees what will happen)
- require-confirmation for all medium/high risk actions
- Rollback capability where possible
- Simulation mode for testing actions
- Safety Monitor detects unusual tool parameter patterns

### 2.6 Resource Exhaustion

**Description**: AI processing consumes excessive battery, CPU, memory, or network bandwidth.

**Attack vectors**:
- Infinite planning loops
- Large model loaded unnecessarily
- Continuous background inference
- Excessive API calls to remote models

**Mitigations**:
- Safety Monitor tracks resource consumption
- Automatic model downgrade under thermal/battery pressure
- Hard limits on inference cycles per request
- Background task restrictions
- Cost and energy budgets in Model Runtime Manager

## 3. Trust Boundaries

```
┌─────────────────────────────────────────────────┐
│                UNTRUSTED (Zone D)                │
│  Websites · Emails · PDFs · Messages · Apps     │
├─────────────────────────────────────────────────┤
│            USER-SPACE TOOLS (Zone C)             │
│  Connectors · File tools · App adapters          │
│  Third-party plugins                             │
├─────────────────────────────────────────────────┤
│          PRIVILEGED SYSTEM AI (Zone B)           │
│  Planner · Broker · Context · Runtime            │
├─────────────────────────────────────────────────┤
│              SECURE CORE (Zone A)                │
│  Boot · Keystore · Identity · Policy · Audit     │
└─────────────────────────────────────────────────┘
```

Each boundary crossing requires validation. Data moving from Zone D → Zone C must be sanitized. Actions from Zone B → Zone A require policy approval.

## 4. Security Principles

1. **deny-by-default**: No action is allowed unless explicitly permitted
2. **Least privilege**: Agents get minimum required capabilities
3. **No self-modification**: Models cannot expand their own rights
4. **Full auditability**: Every action is logged with context
5. **Recoverable**: Kill switch, safe mode, undo where possible
6. **Defense in depth**: Multiple layers of validation before any action
