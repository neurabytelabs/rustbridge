# 📡 MQTT Integration

RustBridge publishes register values to MQTT brokers for real-time streaming.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│   ┌───────────┐      ┌───────────┐      ┌───────────┐                  │
│   │  Device 1 │      │  Device 2 │      │  Device 3 │                  │
│   └─────┬─────┘      └─────┬─────┘      └─────┬─────┘                  │
│         │                  │                  │                         │
│         └──────────────────┼──────────────────┘                         │
│                            │                                            │
│                     ┌──────▼──────┐                                     │
│                     │ RUSTBRIDGE  │                                     │
│                     └──────┬──────┘                                     │
│                            │                                            │
│                            │ MQTT                                       │
│                            ▼                                            │
│                     ┌──────────────┐                                    │
│                     │ MQTT BROKER  │                                    │
│                     │ (Mosquitto)  │                                    │
│                     └──────┬───────┘                                    │
│                            │                                            │
│         ┌──────────────────┼──────────────────┐                         │
│         │                  │                  │                         │
│         ▼                  ▼                  ▼                         │
│   ┌───────────┐      ┌───────────┐      ┌───────────┐                  │
│   │ Dashboard │      │  Alarm    │      │  Data     │                  │
│   │   (Web)   │      │  System   │      │  Logger   │                  │
│   └───────────┘      └───────────┘      └───────────┘                  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Configuration

### Basic Setup

```yaml
mqtt:
  enabled: true
  host: "localhost"
  port: 1883
  client_id: "rustbridge-01"
  topic_prefix: "factory/line1"
```

### With Authentication

```yaml
mqtt:
  enabled: true
  host: "mqtt.example.com"
  port: 1883
  client_id: "rustbridge-01"
  username: "rustbridge"
  password: "secret123"
  topic_prefix: "rustbridge"
```

### With TLS

```yaml
mqtt:
  enabled: true
  host: "mqtt.example.com"
  port: 8883
  client_id: "rustbridge-01"
  username: "rustbridge"
  password: "secret123"
  use_tls: true
  ca_cert: "/etc/rustbridge/ca.crt"
  client_cert: "/etc/rustbridge/client.crt"
  client_key: "/etc/rustbridge/client.key"
  topic_prefix: "rustbridge"
```

### Full Options

```yaml
mqtt:
  enabled: true
  host: "localhost"
  port: 1883
  client_id: "rustbridge-01"
  username: ""
  password: ""
  topic_prefix: "rustbridge"
  qos: 1                    # 0=at most once, 1=at least once, 2=exactly once
  retain: false             # Retain last message
  clean_session: true       # Start fresh on reconnect
  keep_alive_secs: 60       # Keep-alive interval
  reconnect_delay_ms: 5000  # Delay before reconnect attempt
  use_tls: false
  ca_cert: ""
  client_cert: ""
  client_key: ""
```

## Topic Structure

### Default Format

```
{topic_prefix}/{device_id}/{register_name}
```

**Examples:**
```
rustbridge/plc-main/temperature
rustbridge/plc-main/pressure
rustbridge/sensor-01/humidity
```

### Subscribe Patterns

```bash
# All messages
mosquitto_sub -t "rustbridge/#"

# All registers from one device
mosquitto_sub -t "rustbridge/plc-main/#"

# Specific register
mosquitto_sub -t "rustbridge/plc-main/temperature"

# All temperature readings
mosquitto_sub -t "rustbridge/+/temperature"
```

## Message Format

### Register Value Message

```json
{
  "value": 23.5,
  "raw": [235],
  "unit": "°C",
  "quality": "good",
  "timestamp": "2025-12-27T10:30:00.123Z"
}
```

### Device Status Message

Published to: `{prefix}/{device_id}/$status`

```json
{
  "connected": true,
  "last_poll": "2025-12-27T10:30:00Z",
  "poll_count": 86400,
  "error_count": 5
}
```

## Docker Compose with Mosquitto

```yaml
version: '3.8'

services:
  rustbridge:
    image: ghcr.io/mrsarac/rustbridge:latest
    ports:
      - "3000:3000"
    volumes:
      - ./config.yaml:/app/config.yaml
    depends_on:
      - mqtt

  mqtt:
    image: eclipse-mosquitto:2
    ports:
      - "1883:1883"
      - "9001:9001"
    volumes:
      - ./deploy/mosquitto/mosquitto.conf:/mosquitto/config/mosquitto.conf
      - mosquitto-data:/mosquitto/data
      - mosquitto-logs:/mosquitto/log

volumes:
  mosquitto-data:
  mosquitto-logs:
```

**mosquitto.conf:**
```
listener 1883
allow_anonymous true

listener 9001
protocol websockets
```

## Integration Examples

### Node-RED

1. Add MQTT-in node
2. Set topic: `rustbridge/#`
3. Connect to broker: `localhost:1883`
4. Process JSON payload

```javascript
// Function node to extract value
var payload = msg.payload;
var value = JSON.parse(payload).value;
msg.payload = value;
return msg;
```

### Home Assistant

```yaml
# configuration.yaml
mqtt:
  sensor:
    - name: "Factory Temperature"
      state_topic: "rustbridge/plc-main/temperature"
      value_template: "{{ value_json.value }}"
      unit_of_measurement: "°C"
      device_class: temperature
    
    - name: "Factory Pressure"
      state_topic: "rustbridge/plc-main/pressure"
      value_template: "{{ value_json.value }}"
      unit_of_measurement: "bar"
      device_class: pressure
```

### InfluxDB (Telegraf)

```toml
# telegraf.conf
[[inputs.mqtt_consumer]]
  servers = ["tcp://localhost:1883"]
  topics = ["rustbridge/#"]
  data_format = "json"
  json_time_key = "timestamp"
  json_time_format = "2006-01-02T15:04:05.000Z"
  tag_keys = ["device_id", "register"]

[[outputs.influxdb_v2]]
  urls = ["http://influxdb:8086"]
  token = "your-token"
  organization = "your-org"
  bucket = "rustbridge"
```

### Python Subscriber

```python
import json
import paho.mqtt.client as mqtt

def on_message(client, userdata, msg):
    topic = msg.topic
    payload = json.loads(msg.payload)
    
    # Parse topic: rustbridge/device/register
    parts = topic.split('/')
    device = parts[1]
    register = parts[2]
    
    print(f"{device}/{register}: {payload['value']} {payload.get('unit', '')}")

client = mqtt.Client()
client.on_message = on_message
client.connect("localhost", 1883)
client.subscribe("rustbridge/#")
client.loop_forever()
```

## QoS Levels

| Level | Name | Delivery Guarantee | Use Case |
|-------|------|-------------------|----------|
| 0 | At most once | Fire and forget | Non-critical data |
| 1 | At least once | Guaranteed (may duplicate) | Most cases |
| 2 | Exactly once | Guaranteed (no duplicates) | Critical data |

**Recommendation:** Use QoS 1 for most industrial applications.

## Retained Messages

When `retain: true`, the broker stores the last message for each topic:

- New subscribers immediately receive the last value
- Useful for dashboards that need current state on startup
- May cause confusion if device is offline

## Troubleshooting

### Connection Refused
```bash
# Check broker is running
docker ps | grep mosquitto

# Test connection
mosquitto_pub -h localhost -p 1883 -t test -m "hello"
```

### No Messages Received
```bash
# Subscribe to all topics
mosquitto_sub -h localhost -p 1883 -t "#" -v

# Check RustBridge logs
docker logs rustbridge | grep -i mqtt
```

### Authentication Errors
```bash
# Test credentials
mosquitto_pub -h localhost -p 1883 -u "user" -P "pass" -t test -m "hello"
```

### TLS Errors
```bash
# Test TLS connection
openssl s_client -connect mqtt.example.com:8883

# Verify certificate
openssl verify -CAfile ca.crt client.crt
```
