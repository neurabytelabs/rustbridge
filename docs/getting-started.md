# 🚀 Getting Started

This guide will help you get RustBridge up and running in under 5 minutes.

## Prerequisites

- Docker (recommended) or Rust 1.70+
- A Modbus device to connect to (or use the simulator)
- Network access to the device (TCP) or serial port (RTU)

## Installation

### Option 1: Docker (Recommended)

```bash
# Pull the latest image
docker pull ghcr.io/mrsarac/rustbridge:latest

# Run with default config
docker run -d \
  --name rustbridge \
  -p 3000:3000 \
  ghcr.io/mrsarac/rustbridge:latest

# Check if it's running
curl http://localhost:3000/health
```

### Option 2: Docker Compose (Full Stack)

```bash
# Clone the repository
git clone https://github.com/mrsarac/rustbridge.git
cd rustbridge

# Start with MQTT broker
docker compose up -d

# Start with monitoring (Prometheus + Grafana)
docker compose --profile monitoring up -d

# Start with Modbus simulator (for testing)
docker compose --profile dev up -d
```

### Option 3: Download Binary

Download from [GitHub Releases](https://github.com/mrsarac/rustbridge/releases):

| Platform | Binary |
|----------|--------|
| Linux x86_64 | `rustbridge-x86_64-unknown-linux-gnu` |
| Linux ARM64 | `rustbridge-aarch64-unknown-linux-gnu` |
| macOS Intel | `rustbridge-x86_64-apple-darwin` |
| macOS Apple Silicon | `rustbridge-aarch64-apple-darwin` |

```bash
# Download and make executable
chmod +x rustbridge-*

# Run
./rustbridge-x86_64-unknown-linux-gnu --config config.yaml
```

### Option 4: Build from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/mrsarac/rustbridge.git
cd rustbridge
cargo build --release

# Run
./target/release/rustbridge --config config.yaml
```

## First Configuration

Create a `config.yaml` file:

```yaml
# Basic configuration
server:
  host: "0.0.0.0"
  port: 3000
  metrics_enabled: true

# Connect to a Modbus TCP device
devices:
  - id: "my-first-device"
    name: "Temperature Sensor"
    device_type: tcp
    connection:
      host: "192.168.1.100"  # Your device IP
      port: 502              # Modbus TCP port
      unit_id: 1             # Slave ID
    poll_interval_ms: 1000   # Read every second
    registers:
      - name: "temperature"
        address: 0           # Register address
        register_type: holding
        count: 1
        data_type: u16
        unit: "°C"
        scale: 0.1           # Raw value / 10
```

## Verify Installation

```bash
# Health check
curl http://localhost:3000/health
# Response: {"status":"healthy","version":"1.0.0"}

# List devices
curl http://localhost:3000/api/devices
# Response: [{"id":"my-first-device","name":"Temperature Sensor",...}]

# Read register values
curl http://localhost:3000/api/devices/my-first-device/registers
# Response: [{"name":"temperature","value":23.5,"unit":"°C",...}]
```

## Testing Without Hardware

Use the built-in Modbus simulator:

```bash
# Start with simulator
docker compose --profile dev up -d

# The simulator creates fake devices at localhost:5020
# config.yaml is pre-configured to connect to it
```

## Next Steps

- [Configuration Reference](configuration.md) - All configuration options
- [API Reference](api-reference.md) - REST API documentation
- [MQTT Integration](mqtt-integration.md) - Set up real-time streaming
- [Examples](examples.md) - Real-world use cases

## Common Issues

### "Connection refused" error
- Check if the Modbus device is reachable: `nc -zv 192.168.1.100 502`
- Verify firewall settings
- Ensure the device supports Modbus TCP

### "Invalid unit ID" error
- Check the device's slave/unit ID configuration
- Common values: 1, 0, 255

### No data returned
- Verify register addresses in device manual
- Check register_type (holding vs input)
- Try different data_type (u16, i16, u32)

See [Troubleshooting](troubleshooting.md) for more solutions.
