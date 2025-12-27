# 🔧 Troubleshooting Guide

Common issues and solutions for RustBridge.

## Quick Diagnostics

```bash
# Check if RustBridge is running
curl http://localhost:3000/health

# View logs
docker logs rustbridge
# or
journalctl -u rustbridge -f

# Check device connectivity
nc -zv 192.168.1.100 502
```

## Connection Issues

### "Connection refused" Error

**Symptoms:**
```
ERROR modbus::tcp: Connection refused (os error 111)
```

**Solutions:**

1. **Check device is online:**
   ```bash
   ping 192.168.1.100
   ```

2. **Check port is open:**
   ```bash
   nc -zv 192.168.1.100 502
   # or
   telnet 192.168.1.100 502
   ```

3. **Check firewall:**
   ```bash
   # On RustBridge host
   sudo ufw status
   sudo iptables -L -n
   ```

4. **Check device firewall:**
   - Many PLCs have built-in firewalls
   - Check device configuration for allowed IPs

### "Connection timeout" Error

**Symptoms:**
```
ERROR modbus::tcp: Connection timeout after 3000ms
```

**Solutions:**

1. **Increase timeout:**
   ```yaml
   connection:
     timeout_ms: 10000  # 10 seconds
   ```

2. **Check network latency:**
   ```bash
   ping -c 10 192.168.1.100
   ```

3. **Check for packet loss:**
   ```bash
   mtr 192.168.1.100
   ```

4. **Verify routing:**
   ```bash
   traceroute 192.168.1.100
   ```

### "Invalid unit ID" Error

**Symptoms:**
```
ERROR modbus: Invalid unit ID response
```

**Solutions:**

1. **Check device documentation** for correct unit/slave ID
2. **Try common values:** 1, 0, 255
3. **Use Modbus scanner:**
   ```bash
   # Scan unit IDs 1-247
   for i in {1..247}; do
     echo "Testing unit $i"
     modpoll -m tcp -a $i -r 1 -c 1 192.168.1.100
   done
   ```

## Data Issues

### Wrong Values (Too Large/Small)

**Symptoms:**
- Temperature shows 2350 instead of 23.5
- Pressure shows 0.042 instead of 4.2

**Solutions:**

1. **Check scale factor:**
   ```yaml
   registers:
     - name: "temperature"
       scale: 0.1  # Divide by 10
   ```

2. **Check data type:**
   ```yaml
   # Try signed vs unsigned
   data_type: i16  # instead of u16
   ```

### Wrong Values (Completely Wrong)

**Symptoms:**
- Values make no sense (e.g., -32000 for temperature)

**Solutions:**

1. **Check byte order (endianness):**
   ```yaml
   # Try different byte orders
   data_type: f32_be  # Big-endian (most common)
   data_type: f32_le  # Little-endian
   ```

2. **Check register address:**
   ```yaml
   # Remember: 0-based addressing
   # Manual says 40001 → use address: 0
   address: 0
   ```

3. **Check register type:**
   ```yaml
   # Holding vs Input registers
   register_type: holding  # 40xxx
   register_type: input    # 30xxx
   ```

### Stale Values (Not Updating)

**Symptoms:**
- Values don't change even when device changes

**Solutions:**

1. **Check polling interval:**
   ```yaml
   poll_interval_ms: 1000  # 1 second
   ```

2. **Check device is connected:**
   ```bash
   curl http://localhost:3000/api/devices/plc-main
   # Look for "connected": true
   ```

3. **Check for errors in logs:**
   ```bash
   docker logs rustbridge 2>&1 | grep -i error
   ```

## Serial (RTU) Issues

### "Permission denied" on Serial Port

**Symptoms:**
```
ERROR modbus::rtu: Permission denied: /dev/ttyUSB0
```

**Solutions:**

1. **Add user to dialout group:**
   ```bash
   sudo usermod -a -G dialout $USER
   # Log out and back in
   ```

2. **Check permissions:**
   ```bash
   ls -la /dev/ttyUSB0
   # Should be: crw-rw---- 1 root dialout ...
   ```

3. **For Docker:**
   ```yaml
   services:
     rustbridge:
       devices:
         - /dev/ttyUSB0:/dev/ttyUSB0
       group_add:
         - dialout
   ```

### CRC Errors

**Symptoms:**
```
ERROR modbus::rtu: CRC mismatch
```

**Solutions:**

1. **Check baud rate matches device:**
   ```yaml
   connection:
     baud_rate: 9600  # Common values: 9600, 19200, 38400
   ```

2. **Check parity setting:**
   ```yaml
   connection:
     parity: "none"  # Options: none, even, odd
   ```

3. **Check cable quality:**
   - Use shielded cable for RS-485
   - Keep cables short (<100m)
   - Add termination resistor (120Ω)

4. **Check for interference:**
   - Keep away from power cables
   - Use proper grounding

### Serial Port Not Found

**Symptoms:**
```
ERROR modbus::rtu: No such file or directory: /dev/ttyUSB0
```

**Solutions:**

1. **List available ports:**
   ```bash
   ls -la /dev/tty*
   dmesg | grep tty
   ```

2. **Check USB adapter:**
   ```bash
   lsusb
   dmesg | tail -20
   ```

3. **Create udev rule for persistent naming:**
   ```bash
   # /etc/udev/rules.d/99-usb-serial.rules
   SUBSYSTEM=="tty", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", SYMLINK+="modbus0"
   ```

## MQTT Issues

### "Connection refused" to Broker

**Solutions:**

1. **Check broker is running:**
   ```bash
   docker ps | grep mosquitto
   netstat -tlnp | grep 1883
   ```

2. **Test connection:**
   ```bash
   mosquitto_pub -h localhost -t test -m "hello"
   ```

### No Messages Published

**Solutions:**

1. **Check MQTT is enabled:**
   ```yaml
   mqtt:
     enabled: true
   ```

2. **Subscribe to all topics:**
   ```bash
   mosquitto_sub -h localhost -t "#" -v
   ```

3. **Check logs for MQTT errors:**
   ```bash
   docker logs rustbridge 2>&1 | grep -i mqtt
   ```

## Performance Issues

### High CPU Usage

**Solutions:**

1. **Increase poll interval:**
   ```yaml
   poll_interval_ms: 5000  # 5 seconds instead of 1
   ```

2. **Reduce number of registers:**
   - Only poll what you need
   - Combine adjacent registers into single reads

3. **Disable metrics if not needed:**
   ```yaml
   server:
     metrics_enabled: false
   ```

### High Memory Usage

**Solutions:**

1. **Check for memory leaks:**
   ```bash
   # Monitor memory over time
   watch -n 5 'ps aux | grep rustbridge'
   ```

2. **Limit WebSocket connections:**
   - Close unused dashboard tabs
   - Implement connection limits

### Slow Response Times

**Solutions:**

1. **Check network latency:**
   ```bash
   ping 192.168.1.100
   ```

2. **Reduce concurrent reads:**
   ```yaml
   # Stagger poll intervals
   devices:
     - id: "plc-01"
       poll_interval_ms: 1000
     - id: "plc-02"
       poll_interval_ms: 1500  # Offset by 500ms
   ```

## Logging

### Enable Debug Logging

```yaml
server:
  log_level: "debug"  # trace, debug, info, warn, error
```

**Or via environment:**
```bash
RUST_LOG=debug ./rustbridge
```

### Log to File

```bash
./rustbridge 2>&1 | tee /var/log/rustbridge/rustbridge.log
```

## Getting Help

1. **Check logs** with debug level enabled
2. **Search GitHub Issues:** https://github.com/mrsarac/rustbridge/issues
3. **Open new issue** with:
   - RustBridge version
   - Configuration (remove sensitive data)
   - Error messages
   - Steps to reproduce
