# Changelog

All notable changes to RustBridge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- CI/CD pipeline with GitHub Actions
- Automated release workflow with cross-platform binaries
- Docker image publishing to ghcr.io
- Dependabot for automated dependency updates
- Security audit in CI pipeline

## [0.1.0] - 2025-12-27

### Added
- Initial release
- Modbus TCP client with async polling
- REST API with Axum framework
  - `/health` - Health check endpoint
  - `/api/devices` - List all devices
  - `/api/devices/:id` - Get device details
  - `/api/devices/:id/registers` - Get all registers
  - `/api/devices/:id/registers/:name` - Get specific register
- MQTT publisher with rumqttc
- YAML configuration support
- Support for all Modbus register types:
  - Holding registers
  - Input registers
  - Coils
  - Discrete inputs
- Support for multiple data types:
  - u16, i16, u32, i32, f32, bool
- Scale and offset transformations
- Comprehensive test suite (35 tests)
  - Config parsing tests
  - Value conversion tests
  - API integration tests
- Docker support with multi-stage build
- Structured logging with tracing

### Planned (Week 2-3)
- Modbus RTU (serial) support
- WebSocket real-time streaming
- Web dashboard UI
- Prometheus metrics endpoint
- TLS support for MQTT
- Write register functionality

[Unreleased]: https://github.com/mrsarac/rustbridge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mrsarac/rustbridge/releases/tag/v0.1.0
