# 📚 RustBridge Documentation

Welcome to the RustBridge documentation. This guide will help you install, configure, and operate RustBridge in production environments.

## 📖 Table of Contents

| Document | Description |
|----------|-------------|
| [Getting Started](getting-started.md) | Quick installation and first steps |
| [Configuration](configuration.md) | Complete configuration reference |
| [API Reference](api-reference.md) | REST API and WebSocket documentation |
| [Modbus Guide](modbus-guide.md) | Modbus protocol deep dive |
| [MQTT Integration](mqtt-integration.md) | MQTT broker setup and topics |
| [Prometheus Metrics](prometheus-metrics.md) | Monitoring and alerting |
| [Deployment](deployment.md) | Production deployment strategies |
| [Troubleshooting](troubleshooting.md) | Common issues and solutions |
| [Examples](examples.md) | Real-world use cases |

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                            RUSTBRIDGE                                    │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌──────────────┐     ┌──────────────┐     ┌──────────────┐           │
│   │   Modbus     │     │   Polling    │     │   Broadcast  │           │
│   │   Client     │────▶│   Engine     │────▶│   Channel    │           │
│   │  (TCP/RTU)   │     │  (Tokio)     │     │  (mpsc)      │           │
│   └──────────────┘     └──────────────┘     └──────┬───────┘           │
│                                                     │                   │
│          ┌──────────────────────────────────────────┼──────────┐       │
│          │                    │                     │          │       │
│          ▼                    ▼                     ▼          ▼       │
│   ┌────────────┐      ┌────────────┐      ┌────────────┐ ┌──────────┐ │
│   │  REST API  │      │ WebSocket  │      │   MQTT     │ │Prometheus│ │
│   │   (Axum)   │      │  (tokio)   │      │ Publisher  │ │ Metrics  │ │
│   └────────────┘      └────────────┘      └────────────┘ └──────────┘ │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## 🔑 Key Concepts

### Device
A Modbus device (PLC, sensor, meter) that RustBridge connects to. Each device has:
- Unique ID
- Connection settings (TCP host:port or RTU serial port)
- Polling interval
- List of registers to read

### Register
A Modbus register is a 16-bit memory location on the device. Types:
- **Holding Registers** (read/write) - Function codes 3/6/16
- **Input Registers** (read-only) - Function code 4
- **Coils** (read/write, boolean) - Function codes 1/5/15
- **Discrete Inputs** (read-only, boolean) - Function code 2

### Polling
RustBridge periodically reads registers from devices and broadcasts the values to all outputs (REST, WebSocket, MQTT, Prometheus).

## 🚀 Quick Links

- [GitHub Repository](https://github.com/mrsarac/rustbridge)
- [Docker Image](https://ghcr.io/mrsarac/rustbridge)
- [Release Downloads](https://github.com/mrsarac/rustbridge/releases)
- [Issue Tracker](https://github.com/mrsarac/rustbridge/issues)

## 📞 Support

- **GitHub Issues**: Report bugs and request features
- **Email**: mrsarac@gmail.com
- **Company**: [NeuraByte Labs](https://neurabytelabs.com)

---

*RustBridge is part of the Conatus Protocol - Industrial Edge AI Initiative*
