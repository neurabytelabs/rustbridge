# 🔧 Modbus Protocol Guide

This guide explains the Modbus protocol and how RustBridge implements it.

## What is Modbus?

Modbus is a serial communication protocol developed by Modicon in 1979. It's the de facto standard for industrial devices:

- **PLCs** (Programmable Logic Controllers)
- **Sensors** (temperature, pressure, flow)
- **Meters** (energy, water, gas)
- **Motor drives** (VFDs, servo controllers)
- **HVAC systems**

## Modbus Variants

### Modbus TCP

```
┌─────────────┐     Ethernet      ┌─────────────┐
│  RustBridge │ ◄───────────────► │   Device    │
│  (Client)   │    Port 502       │  (Server)   │
└─────────────┘                   └─────────────┘
```

- Uses TCP/IP over Ethernet
- Default port: 502
- Multiple devices on network
- Faster, more reliable

### Modbus RTU

```
┌─────────────┐     Serial        ┌─────────────┐
│  RustBridge │ ◄───────────────► │   Device    │
│  (Master)   │   RS-485/232      │   (Slave)   │
└─────────────┘                   └─────────────┘
```

- Uses serial communication (RS-485, RS-232)
- Binary framing with CRC
- Multiple devices on bus (RS-485)
- Common in older equipment

## Data Model

Modbus defines four types of data:

| Type | Access | Size | Address Range | Description |
|------|--------|------|---------------|-------------|
| **Coils** | R/W | 1 bit | 00001-09999 | Digital outputs |
| **Discrete Inputs** | R | 1 bit | 10001-19999 | Digital inputs |
| **Input Registers** | R | 16 bit | 30001-39999 | Analog inputs |
| **Holding Registers** | R/W | 16 bit | 40001-49999 | Configuration/data |

### Register Types in RustBridge

```yaml
registers:
  # Holding Register (read/write)
  - name: "setpoint"
    address: 100
    register_type: holding    # Function codes: 3 (read), 6/16 (write)
    
  # Input Register (read-only)
  - name: "temperature"
    address: 0
    register_type: input      # Function code: 4
    
  # Coil (digital output, read/write)
  - name: "pump_running"
    address: 0
    register_type: coil       # Function codes: 1 (read), 5/15 (write)
    data_type: bool
    
  # Discrete Input (digital input, read-only)
  - name: "door_open"
    address: 0
    register_type: discrete   # Function code: 2
    data_type: bool
```

## Addressing

### Zero-Based vs One-Based

**Important:** RustBridge uses **zero-based addressing** internally.

Many device manuals use one-based addressing with prefixes:

| Manual Shows | RustBridge Config | Type |
|--------------|-------------------|------|
| 40001 | address: 0 | holding |
| 40100 | address: 99 | holding |
| 30001 | address: 0 | input |
| 30010 | address: 9 | input |
| 00001 | address: 0 | coil |
| 10001 | address: 0 | discrete |

**Rule:** Subtract 1 and remove the prefix.

### Address Calculation Examples

```
Manual: "Temperature is at 30001"
Config: register_type: input, address: 0

Manual: "Setpoint is at 40100"
Config: register_type: holding, address: 99

Manual: "Pump status at 00005"
Config: register_type: coil, address: 4
```

## Data Types

Modbus registers are 16-bit. For larger values, multiple registers are combined:

### 16-bit Values (1 register)

```yaml
- name: "temperature"
  address: 0
  count: 1
  data_type: u16      # Unsigned 0-65535
  # or
  data_type: i16      # Signed -32768 to 32767
```

### 32-bit Values (2 registers)

```yaml
- name: "total_energy"
  address: 10
  count: 2
  data_type: u32_be   # Big-endian (most common)
  # or
  data_type: u32_le   # Little-endian
  # or
  data_type: f32_be   # 32-bit float, big-endian (IEEE 754)
```

### 64-bit Values (4 registers)

```yaml
- name: "total_volume"
  address: 20
  count: 4
  data_type: u64_be
  # or
  data_type: f64_be   # 64-bit float (double)
```

### Byte Order (Endianness)

```
32-bit value: 0x12345678

Big-endian (BE):    [0x1234] [0x5678]
                    Reg N    Reg N+1
                    
Little-endian (LE): [0x5678] [0x1234]
                    Reg N    Reg N+1

Word-swapped BE:    [0x3412] [0x7856]  (some devices)
```

**Tip:** If values look wrong, try different byte orders.

## Scaling and Offset

Many devices store scaled values:

```yaml
# Device stores 235 to represent 23.5°C
- name: "temperature"
  address: 0
  data_type: u16
  scale: 0.1          # value = raw * 0.1
  unit: "°C"

# Device stores 0-1000 to represent 0-100%
- name: "valve_position"
  address: 5
  data_type: u16
  scale: 0.1
  unit: "%"

# Device stores Kelvin, we want Celsius
- name: "temperature"
  address: 10
  data_type: u16
  scale: 0.01
  offset: -273.15     # value = (raw * 0.01) - 273.15
  unit: "°C"
```

## Common Devices

### Energy Meters (Eastron SDM)

```yaml
devices:
  - id: "sdm630"
    name: "Eastron SDM630"
    device_type: tcp
    connection:
      host: "192.168.1.50"
      port: 502
      unit_id: 1
    poll_interval_ms: 5000
    registers:
      - name: "voltage_l1"
        address: 0
        register_type: input
        count: 2
        data_type: f32_be
        unit: "V"
      - name: "current_l1"
        address: 6
        register_type: input
        count: 2
        data_type: f32_be
        unit: "A"
      - name: "power_total"
        address: 52
        register_type: input
        count: 2
        data_type: f32_be
        unit: "W"
      - name: "energy_total"
        address: 342
        register_type: input
        count: 2
        data_type: f32_be
        unit: "kWh"
```

### Temperature Controllers (Watlow)

```yaml
devices:
  - id: "watlow"
    name: "Watlow PM6"
    device_type: rtu
    connection:
      port: "/dev/ttyUSB0"
      baud_rate: 9600
      unit_id: 1
    poll_interval_ms: 1000
    registers:
      - name: "process_value"
        address: 100
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "setpoint"
        address: 300
        register_type: holding
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "output_percent"
        address: 104
        register_type: input
        data_type: u16
        scale: 0.1
        unit: "%"
```

### VFD (Variable Frequency Drive)

```yaml
devices:
  - id: "vfd-01"
    name: "ABB ACS580"
    device_type: tcp
    connection:
      host: "192.168.1.60"
      port: 502
      unit_id: 1
    poll_interval_ms: 500
    registers:
      - name: "frequency"
        address: 1
        register_type: holding
        data_type: u16
        scale: 0.1
        unit: "Hz"
      - name: "motor_current"
        address: 3
        register_type: holding
        data_type: u16
        scale: 0.1
        unit: "A"
      - name: "motor_speed"
        address: 5
        register_type: holding
        data_type: u16
        unit: "RPM"
      - name: "run_command"
        address: 0
        register_type: coil
        data_type: bool
```

## Troubleshooting

### No Response
1. Check network connectivity: `ping 192.168.1.100`
2. Check port is open: `nc -zv 192.168.1.100 502`
3. Verify unit_id matches device configuration

### Wrong Values
1. Check byte order (try `_be` vs `_le`)
2. Verify register address (0-based vs 1-based)
3. Check data type (u16 vs i16, f32 vs u32)
4. Verify scale factor

### Intermittent Errors
1. Increase timeout_ms
2. Reduce poll_interval_ms
3. Check cable quality (for RTU)
4. Verify network stability (for TCP)

### CRC Errors (RTU)
1. Check baud rate matches device
2. Verify parity setting
3. Check data bits and stop bits
4. Ensure proper RS-485 termination
