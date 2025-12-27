# 🚀 Production Deployment

This guide covers deploying RustBridge in production environments.

## Deployment Options

| Method | Best For | Complexity |
|--------|----------|------------|
| Docker Compose | Small/Medium deployments | Low |
| Docker Swarm | Multi-node clusters | Medium |
| Kubernetes | Large scale, cloud | High |
| systemd | Bare metal servers | Low |
| Edge devices | Raspberry Pi, gateways | Low |

## Docker Compose (Recommended)

### Basic Deployment

```bash
# Clone repository
git clone https://github.com/mrsarac/rustbridge.git
cd rustbridge

# Create production config
cp config.yaml config.prod.yaml
vim config.prod.yaml

# Start services
docker compose -f docker-compose.yml up -d
```

### With Monitoring Stack

```bash
# Start with Prometheus + Grafana
docker compose --profile monitoring up -d

# Access:
# - RustBridge: http://localhost:3000
# - Prometheus: http://localhost:9090
# - Grafana: http://localhost:3001 (admin/rustbridge)
```

### Production docker-compose.override.yml

```yaml
version: '3.8'

services:
  rustbridge:
    restart: always
    logging:
      driver: json-file
      options:
        max-size: "100m"
        max-file: "5"
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
```

## systemd (Bare Metal)

### Installation

```bash
# Download binary
wget https://github.com/mrsarac/rustbridge/releases/latest/download/rustbridge-x86_64-unknown-linux-gnu
mv rustbridge-x86_64-unknown-linux-gnu /usr/local/bin/rustbridge
chmod +x /usr/local/bin/rustbridge

# Create config directory
mkdir -p /etc/rustbridge
cp config.yaml /etc/rustbridge/config.yaml

# Create user
useradd -r -s /bin/false rustbridge

# Install service
cp deploy/systemd/rustbridge.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable rustbridge
systemctl start rustbridge
```

### systemd Service File

```ini
# /etc/systemd/system/rustbridge.service
[Unit]
Description=RustBridge Industrial Protocol Gateway
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=rustbridge
Group=rustbridge
ExecStart=/usr/local/bin/rustbridge --config /etc/rustbridge/config.yaml
Restart=on-failure
RestartSec=5

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ReadWritePaths=/var/log/rustbridge

# Resource limits
LimitNOFILE=65535
MemoryMax=512M
CPUQuota=100%

[Install]
WantedBy=multi-user.target
```

### Management Commands

```bash
# Status
systemctl status rustbridge

# Logs
journalctl -u rustbridge -f

# Restart
systemctl restart rustbridge

# Reload config (graceful)
kill -HUP $(pidof rustbridge)
```

## Edge Devices (Raspberry Pi)

### ARM64 Installation

```bash
# Download ARM64 binary
wget https://github.com/mrsarac/rustbridge/releases/latest/download/rustbridge-aarch64-unknown-linux-gnu
mv rustbridge-aarch64-unknown-linux-gnu /usr/local/bin/rustbridge
chmod +x /usr/local/bin/rustbridge

# For serial ports (Modbus RTU)
usermod -a -G dialout rustbridge

# Install as service
# (same as systemd above)
```

### Resource Optimization

```yaml
# config.yaml optimized for Pi
server:
  host: "0.0.0.0"
  port: 3000
  metrics_enabled: false  # Reduce CPU

mqtt:
  enabled: true
  qos: 0  # Reduce overhead

devices:
  - id: "sensor-01"
    poll_interval_ms: 5000  # Slower polling
```

## Kubernetes

### Deployment Manifest

```yaml
# rustbridge.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rustbridge
  labels:
    app: rustbridge
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rustbridge
  template:
    metadata:
      labels:
        app: rustbridge
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "3000"
    spec:
      containers:
      - name: rustbridge
        image: ghcr.io/mrsarac/rustbridge:latest
        ports:
        - containerPort: 3000
        volumeMounts:
        - name: config
          mountPath: /app/config.yaml
          subPath: config.yaml
        resources:
          limits:
            cpu: "500m"
            memory: "256Mi"
          requests:
            cpu: "100m"
            memory: "128Mi"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 10
      volumes:
      - name: config
        configMap:
          name: rustbridge-config
---
apiVersion: v1
kind: Service
metadata:
  name: rustbridge
spec:
  selector:
    app: rustbridge
  ports:
  - port: 3000
    targetPort: 3000
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: rustbridge-config
data:
  config.yaml: |
    server:
      host: "0.0.0.0"
      port: 3000
    # ... rest of config
```

### Deploy

```bash
kubectl apply -f rustbridge.yaml
kubectl get pods -l app=rustbridge
kubectl logs -f deployment/rustbridge
```

## High Availability

### Multiple Instances

For HA, run multiple RustBridge instances with load balancing:

```
                    ┌─────────────────┐
                    │  Load Balancer  │
                    └────────┬────────┘
                             │
           ┌─────────────────┼─────────────────┐
           │                 │                 │
           ▼                 ▼                 ▼
    ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
    │ RustBridge  │   │ RustBridge  │   │ RustBridge  │
    │   Node 1    │   │   Node 2    │   │   Node 3    │
    └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
           │                 │                 │
           └─────────────────┼─────────────────┘
                             │
                    ┌────────▼────────┐
                    │  Modbus Device  │
                    └─────────────────┘
```

**Note:** Only one instance should poll each device to avoid conflicts.

### Device Sharding

```yaml
# node1/config.yaml
devices:
  - id: "plc-01"  # Node 1 handles PLCs
  - id: "plc-02"

# node2/config.yaml
devices:
  - id: "sensor-01"  # Node 2 handles sensors
  - id: "sensor-02"
```

## Security

### Network Security

```bash
# Firewall rules
ufw allow from 10.0.0.0/8 to any port 3000  # API access
ufw allow from 10.0.0.0/8 to any port 502   # Modbus
ufw deny 3000  # Block external API access
```

### TLS Termination (Nginx)

```nginx
server {
    listen 443 ssl;
    server_name rustbridge.example.com;
    
    ssl_certificate /etc/ssl/rustbridge.crt;
    ssl_certificate_key /etc/ssl/rustbridge.key;
    
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
```

## Backup & Recovery

### Configuration Backup

```bash
# Backup
tar -czf rustbridge-backup-$(date +%Y%m%d).tar.gz \
  /etc/rustbridge/ \
  /var/lib/rustbridge/

# Restore
tar -xzf rustbridge-backup-20251227.tar.gz -C /
systemctl restart rustbridge
```

### Disaster Recovery

1. Keep config.yaml in version control (Git)
2. Use infrastructure as code (Terraform, Ansible)
3. Document device addresses and register maps
4. Test recovery procedures regularly
