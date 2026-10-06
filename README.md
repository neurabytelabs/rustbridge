# RustBridge

RustBridge is a gateway written in Rust that reads Modbus TCP and RTU devices and publishes the values over a REST API, WebSocket, MQTT and Prometheus metrics.

**Status:** Prototype (v0.2.0). CI is green and the repo has 63 tests. I have not tested it against real PLCs or serial hardware, and I have not measured throughput. **Live demo:** none. An earlier demo host (`rustbridge.mustafasarac.com`) is offline and returns 503.

[![CI](https://github.com/neurabytelabs/rustbridge/actions/workflows/ci.yml/badge.svg)](https://github.com/neurabytelabs/rustbridge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Run it

```bash
git clone https://github.com/neurabytelabs/rustbridge.git
cd rustbridge
cargo test               # 63 tests
cargo run --release      # reads ./config.yaml, API on http://localhost:3000
curl http://localhost:3000/health
```

I ran this on macOS: `/health` and `/api/info` answered. The default `config.yaml` points at an MQTT broker on localhost; without one the log shows connection errors but the API keeps running.

With Docker: `docker compose up -d` builds the image from the `Dockerfile` and starts a Mosquitto broker next to it. I have not run this compose file myself.

Release binaries (Linux x86_64, macOS Intel, macOS Apple Silicon) are on the [releases page](https://github.com/neurabytelabs/rustbridge/releases).

## Release numbering

The tag order looks wrong: `v1.0.0` was created on 2025-12-27 and `v0.2.0` on 2026-01-02. The code at `v1.0.0` has `version = "0.1.0"` in `Cargo.toml`, so that release is really 0.1.0 and the 1.0.0 name was a mistake. The version in `Cargo.toml` and `CHANGELOG.md` is the real one; the latest release is `v0.2.0`. The old release was kept so existing download links keep working.

## Features

- Modbus TCP and RTU (serial) clients with per-device polling.
- Register types: holding, input, coil, discrete input. Data types: u16, i16, u32, i32, f32, bool, with scale and offset.
- REST API and a WebSocket stream of register updates.
- MQTT publisher with a configurable topic prefix and QoS.
- Prometheus metrics at `/metrics`.
- Optional API key authentication (`X-API-Key` header).

## Documentation

| Document | Description |
|----------|-------------|
| [Getting Started](docs/getting-started.md) | Quick installation and first steps |
| [Configuration](docs/configuration.md) | Complete configuration reference |
| [API Reference](docs/api-reference.md) | REST API and WebSocket documentation |
| [Modbus Guide](docs/modbus-guide.md) | Modbus protocol deep dive |
| [MQTT Integration](docs/mqtt-integration.md) | MQTT broker setup and topics |
| [Prometheus Metrics](docs/prometheus-metrics.md) | Monitoring and alerting |
| [Deployment](docs/deployment.md) | Production deployment strategies |
| [Troubleshooting](docs/troubleshooting.md) | Common issues and solutions |
| [Examples](docs/examples.md) | Real-world use cases |

## Configuration

Create a `config.yaml` file:

```yaml
server:
  host: "0.0.0.0"
  port: 3000
  metrics_enabled: true

mqtt:
  enabled: true
  host: "localhost"
  port: 1883
  client_id: "rustbridge"
  topic_prefix: "rustbridge"
  qos: 1
  retain: false

# API Authentication (optional)
auth:
  enabled: true
  api_keys:
    - "your-secret-key-1"
    - "your-secret-key-2"
  exclude_paths:
    - "/health"
    - "/metrics"

devices:
  # Modbus TCP device
  - id: "plc-01"
    name: "Main PLC"
    device_type: tcp
    connection:
      host: "192.168.1.100"
      port: 502
      unit_id: 1
    poll_interval_ms: 1000
    registers:
      - name: "temperature"
        address: 0
        register_type: holding
        count: 1
        data_type: u16
        unit: "°C"
        scale: 0.1

  # Modbus RTU (Serial) device
  - id: "sensor-01"
    name: "RTU Sensor"
    device_type: rtu
    connection:
      port: "/dev/ttyUSB0"
      baud_rate: 9600
      data_bits: 8
      stop_bits: 1
      parity: "none"
      unit_id: 1
    poll_interval_ms: 2000
    registers:
      - name: "humidity"
        address: 0
        register_type: input
        count: 1
        data_type: u16
        unit: "%"
```

> See [Configuration Reference](docs/configuration.md) for all options.

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/metrics` | GET | Prometheus metrics |
| `/api/info` | GET | API information |
| `/api/devices` | GET | List all devices |
| `/api/devices/:id` | GET | Get device details |
| `/api/devices/:id/registers` | GET | Get all register values |
| `/api/devices/:id/registers/:name` | GET/POST | Get/Write register |
| `/ws` | WebSocket | Real-time updates |

### Example Response

```json
{
  "name": "temperature",
  "value": 72.4,
  "raw": [724],
  "unit": "°C",
  "timestamp": "2025-12-26T23:15:00Z"
}
```

> See [API Reference](docs/api-reference.md) for full documentation.

## MQTT Topics

Data is published to: `{prefix}/{device_id}/{register_name}`

Example: `rustbridge/plc-01/temperature`

```json
{
  "value": 72.4,
  "raw": [724],
  "unit": "°C",
  "timestamp": "2025-12-26T23:15:00Z"
}
```

> See [MQTT Integration](docs/mqtt-integration.md) for broker setup.

## Prometheus Metrics

Available at `/metrics` when `metrics_enabled: true`:

| Metric | Type | Description |
|--------|------|-------------|
| `rustbridge_register_reads_total` | Counter | Total register read attempts |
| `rustbridge_read_duration_seconds` | Histogram | Read latency distribution |
| `rustbridge_register_value` | Gauge | Current register values |
| `rustbridge_errors_total` | Counter | Error count by type |
| `rustbridge_device_connected` | Gauge | Device connection status |
| `rustbridge_poll_cycle_seconds` | Histogram | Poll cycle duration |

> See [Prometheus Metrics](docs/prometheus-metrics.md) for Grafana dashboards and alerting.

## Production Deployment

### Docker Compose

```bash
# Production stack
docker compose up -d

# With monitoring (Prometheus + Grafana)
docker compose --profile monitoring up -d

# With Modbus simulator for testing
docker compose --profile dev up -d
```

Access:
- RustBridge API: http://localhost:3000
- Prometheus: http://localhost:9090 (with monitoring profile)
- Grafana: http://localhost:3001 (admin/rustbridge)

### systemd Service (Bare Metal)

```bash
# Install
cd deploy
sudo ./install.sh

# Control
sudo systemctl start rustbridge
sudo systemctl status rustbridge
sudo journalctl -u rustbridge -f
```

> See [Deployment Guide](docs/deployment.md) for HA setup, edge devices, and more.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        RUSTBRIDGE                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────┐    ┌──────────┐    ┌────────┐    ┌─────────┐  │
│  │ Modbus  │───▶│ Polling  │───▶│Broadcast│──▶│  MQTT   │  │
│  │TCP/RTU  │    │ Engine   │    │ Channel │   │Publisher│  │
│  └─────────┘    └──────────┘    └────────┘    └─────────┘  │
│                                      │                      │
│                                      ▼                      │
│                      ┌──────────────────────────┐          │
│                      │      REST API + WS       │          │
│                      │    (/api, /ws, /metrics) │          │
│                      └──────────────────────────┘          │
│                                      │                      │
│                                      ▼                      │
│                      ┌──────────────────────────┐          │
│                      │   Prometheus + Grafana   │          │
│                      └──────────────────────────┘          │
└─────────────────────────────────────────────────────────────┘
```

## Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Build release
cargo build --release

# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Project Structure

```
rustbridge/
├── docs/                # 📚 Documentation
│   ├── getting-started.md
│   ├── configuration.md
│   ├── api-reference.md
│   ├── modbus-guide.md
│   ├── mqtt-integration.md
│   ├── prometheus-metrics.md
│   ├── deployment.md
│   ├── troubleshooting.md
│   └── examples.md
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── config.rs        # Configuration parsing
│   ├── bridge.rs        # Main orchestration
│   ├── api/             # REST API + WebSocket
│   ├── modbus/          # Modbus TCP/RTU client
│   ├── mqtt/            # MQTT publisher
│   └── metrics/         # Prometheus metrics
├── deploy/
│   ├── mosquitto/       # MQTT broker config
│   ├── prometheus/      # Prometheus config
│   ├── grafana/         # Grafana provisioning
│   ├── systemd/         # systemd service file
│   ├── install.sh       # Installation script
│   └── uninstall.sh     # Uninstallation script
├── config.yaml          # Example configuration
├── Dockerfile           # Multi-stage build
└── docker-compose.yml   # Full stack deployment
```

## Security

- Optional API key authentication, configured under `auth:` in `config.yaml`. It is off unless `auth.enabled: true`.
- The Docker image runs as a non-root user and the systemd unit sets hardening options (see `Dockerfile` and `deploy/systemd/rustbridge.service`).
- MQTT TLS and rate limiting: not verified; do not rely on them.

Example with authentication enabled:

```bash
curl -H "X-API-Key: your-secret-api-key" http://localhost:3000/api/devices
```

## License

MIT. See [LICENSE](LICENSE).

## Support

Open an issue: [github.com/neurabytelabs/rustbridge/issues](https://github.com/neurabytelabs/rustbridge/issues)

Built by Mustafa Saraç ([NeuraByte Labs](https://neurabytelabs.com)).
