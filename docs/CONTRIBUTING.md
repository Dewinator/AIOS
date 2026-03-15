# Contributing to AIOS

Thank you for your interest in contributing to AIOS! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run tests (see below)
5. Submit a pull request

## Development Setup

### Prerequisites

- **Rust** (latest stable) — for broker, policy-engine, safety-monitor
- **Android Studio** / **JDK 17+** — for aiosd and shell-app
- **AOSP build environment** — for full system builds (Linux recommended)

### Building Components

```bash
# Rust components
cd broker && cargo build
cd policy-engine && cargo build
cd safety-monitor && cargo build

# Android components (requires Android SDK)
cd shell-app && ./gradlew build
```

### Running Tests

```bash
# Rust tests
cd broker && cargo test
cd policy-engine && cargo test
cd safety-monitor && cargo test

# Schema validation tests
cd tests/tool_tests && cargo test
cd tests/policy_tests && cargo test
```

## Code Style

- **Rust**: Follow standard `rustfmt` formatting. Run `cargo fmt` before committing.
- **Kotlin**: Follow the official Kotlin coding conventions.
- **JSON/YAML schemas**: Use 2-space indentation.

## Pull Request Process

1. Ensure your PR has a clear title and description
2. Reference related issues if applicable
3. Include tests for new functionality
4. Ensure all existing tests pass
5. Update documentation if needed

## Security

If you discover a security vulnerability, please do **not** open a public issue. Instead, contact the maintainers directly.

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
