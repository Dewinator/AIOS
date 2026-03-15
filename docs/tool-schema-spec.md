# AIOS Tool Schema Specification

## Overview

Every tool in AIOS is defined by a declarative JSON schema that describes its interface, permissions, risk level, and behavior. The Tool Broker uses these schemas to validate, route, and audit tool calls.

## Schema Format

```json
{
  "$schema": "https://aios.dev/schemas/tool/v1",
  "id": "string",
  "name": "string",
  "description": "string",
  "version": "string",
  "category": "string",
  "input": {
    "type": "object",
    "properties": { },
    "required": [ ]
  },
  "output": {
    "type": "object",
    "properties": { }
  },
  "capabilities": ["string"],
  "risk_class": "low | medium | high | critical",
  "side_effects": ["string"],
  "rollback": {
    "supported": "boolean",
    "method": "string"
  },
  "audit": {
    "log_input": "boolean",
    "log_output": "boolean",
    "retention_days": "integer"
  },
  "consent_level": "none | once | persistent | strong | biometric"
}
```

## Field Descriptions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | yes | Unique tool identifier (e.g., `system.files.read`) |
| `name` | string | yes | Human-readable name |
| `description` | string | yes | What this tool does |
| `version` | string | yes | Semantic version |
| `category` | string | yes | Tool category (files, calendar, messages, settings, device) |
| `input` | object | yes | JSON Schema for input parameters |
| `output` | object | yes | JSON Schema for output format |
| `capabilities` | array | yes | Required capability labels |
| `risk_class` | enum | yes | Risk classification |
| `side_effects` | array | yes | Declared side effects |
| `rollback` | object | yes | Whether and how the action can be undone |
| `audit` | object | yes | Logging requirements |
| `consent_level` | enum | yes | Default consent level required |

## Risk Classes

| Class | Description | Default Consent |
|-------|-------------|-----------------|
| `low` | Read-only, local, no sensitive data | `none` |
| `medium` | Write operations, local data modification | `once` |
| `high` | External communication, sensitive data access | `strong` |
| `critical` | Financial, identity, device management | `biometric` |

## Capability Labels

Capabilities follow a hierarchical naming scheme:

```
<domain>.<resource>.<action>

Examples:
  files.user_documents.read
  files.user_documents.write
  calendar.events.read
  calendar.events.write
  contacts.list.read
  messages.sms.send
  settings.audio.modify
  settings.network.modify
  apps.installed.launch
  device.info.read
  notifications.list.read
  notifications.list.summarize
```

## Consent Levels

| Level | Description | UI |
|-------|-------------|-----|
| `none` | No user interaction needed | Silent |
| `once` | Ask once, don't remember | Dialog |
| `persistent` | Ask once, remember for this tool class | Dialog + toggle |
| `strong` | Always ask, even if previously approved | Modal dialog |
| `biometric` | Require fingerprint/face | Biometric prompt |
