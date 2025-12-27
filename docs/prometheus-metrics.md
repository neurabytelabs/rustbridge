# 📊 Prometheus Metrics

RustBridge exposes Prometheus metrics at `/metrics` for monitoring and alerting.

## Enabling Metrics

```yaml
server:
  metrics_enabled: true
```

## Available Metrics

### Register Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustbridge_register_value` | Gauge | device, register | Current register value |
| `rustbridge_register_reads_total` | Counter | device, status | Total read attempts |
| `rustbridge_read_duration_seconds` | Histogram | device | Read latency |

### Device Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustbridge_device_connected` | Gauge | device | Connection status (1=connected) |
| `rustbridge_device_errors_total` | Counter | device, error_type | Error count by type |
| `rustbridge_poll_cycle_seconds` | Histogram | device | Poll cycle duration |

### System Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `rustbridge_uptime_seconds` | Gauge | - | Process uptime |
| `rustbridge_info` | Gauge | version | Build information |
| `rustbridge_mqtt_messages_total` | Counter | status | MQTT publish count |
| `rustbridge_websocket_connections` | Gauge | - | Active WebSocket clients |

## Example Output

```
# HELP rustbridge_register_value Current register value
# TYPE rustbridge_register_value gauge
rustbridge_register_value{device="plc-main",register="temperature"} 23.5
rustbridge_register_value{device="plc-main",register="pressure"} 4.2
rustbridge_register_value{device="sensor-01",register="humidity"} 65.0

# HELP rustbridge_register_reads_total Total register read attempts
# TYPE rustbridge_register_reads_total counter
rustbridge_register_reads_total{device="plc-main",status="success"} 86350
rustbridge_register_reads_total{device="plc-main",status="error"} 50
rustbridge_register_reads_total{device="sensor-01",status="success"} 43200
rustbridge_register_reads_total{device="sensor-01",status="error"} 12

# HELP rustbridge_read_duration_seconds Read latency histogram
# TYPE rustbridge_read_duration_seconds histogram
rustbridge_read_duration_seconds_bucket{device="plc-main",le="0.001"} 10000
rustbridge_read_duration_seconds_bucket{device="plc-main",le="0.005"} 50000
rustbridge_read_duration_seconds_bucket{device="plc-main",le="0.01"} 80000
rustbridge_read_duration_seconds_bucket{device="plc-main",le="0.05"} 86000
rustbridge_read_duration_seconds_bucket{device="plc-main",le="0.1"} 86300
rustbridge_read_duration_seconds_bucket{device="plc-main",le="+Inf"} 86400
rustbridge_read_duration_seconds_sum{device="plc-main"} 432.5
rustbridge_read_duration_seconds_count{device="plc-main"} 86400

# HELP rustbridge_device_connected Device connection status
# TYPE rustbridge_device_connected gauge
rustbridge_device_connected{device="plc-main"} 1
rustbridge_device_connected{device="sensor-01"} 1

# HELP rustbridge_device_errors_total Device errors by type
# TYPE rustbridge_device_errors_total counter
rustbridge_device_errors_total{device="plc-main",error_type="timeout"} 45
rustbridge_device_errors_total{device="plc-main",error_type="connection"} 5

# HELP rustbridge_uptime_seconds Process uptime
# TYPE rustbridge_uptime_seconds gauge
rustbridge_uptime_seconds 86400

# HELP rustbridge_info Build information
# TYPE rustbridge_info gauge
rustbridge_info{version="1.0.0"} 1
```

## Prometheus Configuration

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'rustbridge'
    static_configs:
      - targets: ['rustbridge:3000']
    scrape_interval: 15s
    metrics_path: /metrics
```

## Docker Compose with Prometheus

```yaml
version: '3.8'

services:
  rustbridge:
    image: ghcr.io/mrsarac/rustbridge:latest
    ports:
      - "3000:3000"
    volumes:
      - ./config.yaml:/app/config.yaml

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./deploy/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--storage.tsdb.retention.time=30d'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    volumes:
      - ./deploy/grafana/provisioning:/etc/grafana/provisioning
      - grafana-data:/var/lib/grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=rustbridge

volumes:
  prometheus-data:
  grafana-data:
```

## Grafana Dashboard

Import the RustBridge dashboard from `deploy/grafana/dashboards/rustbridge.json` or use these panels:

### Panel: Register Values
```
rustbridge_register_value{device="$device"}
```

### Panel: Read Success Rate
```
sum(rate(rustbridge_register_reads_total{device="$device",status="success"}[5m])) /
sum(rate(rustbridge_register_reads_total{device="$device"}[5m])) * 100
```

### Panel: Average Latency
```
rate(rustbridge_read_duration_seconds_sum{device="$device"}[5m]) /
rate(rustbridge_read_duration_seconds_count{device="$device"}[5m])
```

### Panel: Error Rate
```
sum(rate(rustbridge_device_errors_total{device="$device"}[5m])) by (error_type)
```

### Panel: Device Connection Status
```
rustbridge_device_connected
```

## Alerting Rules

```yaml
# prometheus/rules/rustbridge.yml
groups:
  - name: rustbridge
    rules:
      # Device disconnected
      - alert: RustBridgeDeviceDown
        expr: rustbridge_device_connected == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Device {{ $labels.device }} is disconnected"
          
      # High error rate
      - alert: RustBridgeHighErrorRate
        expr: |
          sum(rate(rustbridge_register_reads_total{status="error"}[5m])) by (device) /
          sum(rate(rustbridge_register_reads_total[5m])) by (device) > 0.05
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} has >5% error rate"
          
      # High latency
      - alert: RustBridgeHighLatency
        expr: |
          rate(rustbridge_read_duration_seconds_sum[5m]) /
          rate(rustbridge_read_duration_seconds_count[5m]) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Device {{ $labels.device }} has >100ms average latency"
          
      # Temperature threshold
      - alert: TemperatureHigh
        expr: rustbridge_register_value{register="temperature"} > 80
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Temperature on {{ $labels.device }} is {{ $value }}°C"
```

## AlertManager Integration

```yaml
# alertmanager.yml
route:
  receiver: 'slack'
  routes:
    - match:
        severity: critical
      receiver: 'pagerduty'

receivers:
  - name: 'slack'
    slack_configs:
      - api_url: 'https://hooks.slack.com/services/xxx'
        channel: '#alerts'
        
  - name: 'pagerduty'
    pagerduty_configs:
      - service_key: 'xxx'
```
