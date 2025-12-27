# 📋 Real-World Examples

Practical configuration examples for common industrial scenarios.

## Factory Monitoring

### Multi-Line Manufacturing

Monitor 3 production lines with PLCs and sensors.

```yaml
server:
  host: "0.0.0.0"
  port: 3000
  metrics_enabled: true

mqtt:
  enabled: true
  host: "mqtt.factory.local"
  port: 1883
  topic_prefix: "factory"

devices:
  # Line 1 - Main PLC
  - id: "line1-plc"
    name: "Production Line 1 - Main Controller"
    device_type: tcp
    connection:
      host: "10.0.1.10"
      port: 502
      unit_id: 1
    poll_interval_ms: 500
    registers:
      - name: "line_speed"
        address: 0
        register_type: holding
        data_type: u16
        unit: "m/min"
        scale: 0.1
      - name: "product_count"
        address: 10
        register_type: holding
        count: 2
        data_type: u32_be
      - name: "line_status"
        address: 100
        register_type: coil
        data_type: bool
      - name: "emergency_stop"
        address: 101
        register_type: discrete
        data_type: bool

  # Line 1 - Temperature Sensors
  - id: "line1-temp"
    name: "Line 1 Temperature Sensors"
    device_type: rtu
    connection:
      port: "/dev/ttyUSB0"
      baud_rate: 9600
      unit_id: 1
    poll_interval_ms: 2000
    registers:
      - name: "zone1_temp"
        address: 0
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "zone2_temp"
        address: 1
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "zone3_temp"
        address: 2
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"

  # Line 2 - Main PLC
  - id: "line2-plc"
    name: "Production Line 2 - Main Controller"
    device_type: tcp
    connection:
      host: "10.0.1.20"
      port: 502
      unit_id: 1
    poll_interval_ms: 500
    registers:
      - name: "line_speed"
        address: 0
        register_type: holding
        data_type: u16
        unit: "m/min"
        scale: 0.1
      - name: "product_count"
        address: 10
        register_type: holding
        count: 2
        data_type: u32_be

  # Line 3 - VFD Motor Drive
  - id: "line3-vfd"
    name: "Line 3 - Main Motor VFD"
    device_type: tcp
    connection:
      host: "10.0.1.30"
      port: 502
      unit_id: 1
    poll_interval_ms: 1000
    registers:
      - name: "frequency"
        address: 1
        register_type: holding
        data_type: u16
        scale: 0.01
        unit: "Hz"
      - name: "motor_current"
        address: 3
        register_type: holding
        data_type: u16
        scale: 0.1
        unit: "A"
      - name: "motor_power"
        address: 5
        register_type: holding
        data_type: u16
        scale: 0.1
        unit: "kW"
      - name: "motor_speed"
        address: 7
        register_type: holding
        data_type: u16
        unit: "RPM"
```

## Energy Management

### Building Energy Monitoring

Monitor electricity, gas, and water meters.

```yaml
server:
  host: "0.0.0.0"
  port: 3000
  metrics_enabled: true

mqtt:
  enabled: true
  host: "localhost"
  port: 1883
  topic_prefix: "building/energy"

devices:
  # Main Electrical Meter (Eastron SDM630)
  - id: "meter-main"
    name: "Main Electrical Meter"
    device_type: tcp
    connection:
      host: "192.168.1.50"
      port: 502
      unit_id: 1
    poll_interval_ms: 5000
    registers:
      # Voltage
      - name: "voltage_l1"
        address: 0
        register_type: input
        count: 2
        data_type: f32_be
        unit: "V"
      - name: "voltage_l2"
        address: 2
        register_type: input
        count: 2
        data_type: f32_be
        unit: "V"
      - name: "voltage_l3"
        address: 4
        register_type: input
        count: 2
        data_type: f32_be
        unit: "V"
      # Current
      - name: "current_l1"
        address: 6
        register_type: input
        count: 2
        data_type: f32_be
        unit: "A"
      - name: "current_l2"
        address: 8
        register_type: input
        count: 2
        data_type: f32_be
        unit: "A"
      - name: "current_l3"
        address: 10
        register_type: input
        count: 2
        data_type: f32_be
        unit: "A"
      # Power
      - name: "power_total"
        address: 52
        register_type: input
        count: 2
        data_type: f32_be
        unit: "W"
      - name: "power_factor"
        address: 62
        register_type: input
        count: 2
        data_type: f32_be
      # Energy
      - name: "energy_import"
        address: 72
        register_type: input
        count: 2
        data_type: f32_be
        unit: "kWh"
      - name: "energy_export"
        address: 74
        register_type: input
        count: 2
        data_type: f32_be
        unit: "kWh"

  # HVAC Meter - Floor 1
  - id: "meter-hvac-f1"
    name: "HVAC Meter Floor 1"
    device_type: tcp
    connection:
      host: "192.168.1.51"
      port: 502
      unit_id: 1
    poll_interval_ms: 10000
    registers:
      - name: "power"
        address: 52
        register_type: input
        count: 2
        data_type: f32_be
        unit: "W"
      - name: "energy"
        address: 72
        register_type: input
        count: 2
        data_type: f32_be
        unit: "kWh"

  # Water Meter (Pulse counter)
  - id: "meter-water"
    name: "Water Meter"
    device_type: rtu
    connection:
      port: "/dev/ttyUSB1"
      baud_rate: 9600
      unit_id: 1
    poll_interval_ms: 60000  # Once per minute
    registers:
      - name: "total_volume"
        address: 0
        register_type: holding
        count: 2
        data_type: u32_be
        unit: "L"
      - name: "flow_rate"
        address: 10
        register_type: input
        data_type: u16
        unit: "L/min"
```

## HVAC Control

### Building Climate Control

```yaml
server:
  host: "0.0.0.0"
  port: 3000

devices:
  # AHU (Air Handling Unit) Controller
  - id: "ahu-01"
    name: "AHU Main Building"
    device_type: tcp
    connection:
      host: "192.168.2.10"
      port: 502
      unit_id: 1
    poll_interval_ms: 5000
    registers:
      # Temperatures
      - name: "supply_air_temp"
        address: 0
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "return_air_temp"
        address: 1
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "outside_air_temp"
        address: 2
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "temp_setpoint"
        address: 100
        register_type: holding
        data_type: i16
        scale: 0.1
        unit: "°C"
      # Dampers & Valves
      - name: "outside_air_damper"
        address: 10
        register_type: input
        data_type: u16
        unit: "%"
      - name: "heating_valve"
        address: 11
        register_type: input
        data_type: u16
        unit: "%"
      - name: "cooling_valve"
        address: 12
        register_type: input
        data_type: u16
        unit: "%"
      # Fan
      - name: "supply_fan_speed"
        address: 20
        register_type: input
        data_type: u16
        unit: "%"
      - name: "supply_fan_status"
        address: 0
        register_type: discrete
        data_type: bool
      # Alarms
      - name: "filter_alarm"
        address: 10
        register_type: discrete
        data_type: bool
      - name: "freeze_alarm"
        address: 11
        register_type: discrete
        data_type: bool

  # Zone Controllers (VAV boxes)
  - id: "vav-zone1"
    name: "VAV Zone 1 - Conference Room"
    device_type: tcp
    connection:
      host: "192.168.2.20"
      port: 502
      unit_id: 1
    poll_interval_ms: 10000
    registers:
      - name: "zone_temp"
        address: 0
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "zone_setpoint"
        address: 100
        register_type: holding
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "damper_position"
        address: 10
        register_type: input
        data_type: u16
        unit: "%"
      - name: "occupancy"
        address: 0
        register_type: discrete
        data_type: bool
```

## Solar Power Plant

### Photovoltaic Monitoring

```yaml
server:
  host: "0.0.0.0"
  port: 3000
  metrics_enabled: true

mqtt:
  enabled: true
  host: "localhost"
  port: 1883
  topic_prefix: "solar"

devices:
  # Inverter 1 (SMA Sunny Tripower)
  - id: "inverter-01"
    name: "Inverter 1 - East Array"
    device_type: tcp
    connection:
      host: "192.168.3.10"
      port: 502
      unit_id: 3
    poll_interval_ms: 5000
    registers:
      # DC Side
      - name: "dc_voltage_a"
        address: 30771
        register_type: holding
        count: 2
        data_type: u32_be
        scale: 0.01
        unit: "V"
      - name: "dc_current_a"
        address: 30769
        register_type: holding
        count: 2
        data_type: u32_be
        scale: 0.001
        unit: "A"
      - name: "dc_power"
        address: 30773
        register_type: holding
        count: 2
        data_type: u32_be
        unit: "W"
      # AC Side
      - name: "ac_power"
        address: 30775
        register_type: holding
        count: 2
        data_type: u32_be
        unit: "W"
      - name: "ac_frequency"
        address: 30803
        register_type: holding
        count: 2
        data_type: u32_be
        scale: 0.01
        unit: "Hz"
      # Energy
      - name: "energy_today"
        address: 30535
        register_type: holding
        count: 2
        data_type: u32_be
        unit: "Wh"
      - name: "energy_total"
        address: 30529
        register_type: holding
        count: 2
        data_type: u32_be
        unit: "Wh"
      # Status
      - name: "operating_state"
        address: 30201
        register_type: holding
        count: 2
        data_type: u32_be

  # Weather Station
  - id: "weather"
    name: "Weather Station"
    device_type: rtu
    connection:
      port: "/dev/ttyUSB0"
      baud_rate: 9600
      unit_id: 1
    poll_interval_ms: 60000
    registers:
      - name: "irradiance"
        address: 0
        register_type: input
        data_type: u16
        unit: "W/m²"
      - name: "module_temp"
        address: 1
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "ambient_temp"
        address: 2
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"
      - name: "wind_speed"
        address: 3
        register_type: input
        data_type: u16
        scale: 0.1
        unit: "m/s"
```

## Water Treatment

### Water Treatment Plant

```yaml
server:
  host: "0.0.0.0"
  port: 3000

devices:
  # Inlet Pumping Station
  - id: "pump-inlet"
    name: "Inlet Pump Station"
    device_type: tcp
    connection:
      host: "10.10.1.10"
      port: 502
      unit_id: 1
    poll_interval_ms: 1000
    registers:
      - name: "pump1_running"
        address: 0
        register_type: coil
        data_type: bool
      - name: "pump2_running"
        address: 1
        register_type: coil
        data_type: bool
      - name: "inlet_flow"
        address: 0
        register_type: input
        count: 2
        data_type: f32_be
        unit: "m³/h"
      - name: "inlet_pressure"
        address: 10
        register_type: input
        data_type: u16
        scale: 0.01
        unit: "bar"
      - name: "wet_well_level"
        address: 20
        register_type: input
        data_type: u16
        scale: 0.1
        unit: "m"

  # Water Quality Analyzer
  - id: "analyzer-01"
    name: "Inlet Water Quality"
    device_type: rtu
    connection:
      port: "/dev/ttyUSB0"
      baud_rate: 9600
      unit_id: 1
    poll_interval_ms: 30000
    registers:
      - name: "ph"
        address: 0
        register_type: input
        data_type: u16
        scale: 0.01
      - name: "turbidity"
        address: 1
        register_type: input
        data_type: u16
        scale: 0.1
        unit: "NTU"
      - name: "chlorine"
        address: 2
        register_type: input
        data_type: u16
        scale: 0.01
        unit: "mg/L"
      - name: "conductivity"
        address: 3
        register_type: input
        data_type: u16
        unit: "µS/cm"
      - name: "temperature"
        address: 4
        register_type: input
        data_type: i16
        scale: 0.1
        unit: "°C"

  # Chemical Dosing
  - id: "dosing-chlorine"
    name: "Chlorine Dosing System"
    device_type: tcp
    connection:
      host: "10.10.1.30"
      port: 502
      unit_id: 1
    poll_interval_ms: 5000
    registers:
      - name: "pump_speed"
        address: 0
        register_type: holding
        data_type: u16
        unit: "%"
      - name: "flow_rate"
        address: 1
        register_type: input
        data_type: u16
        scale: 0.1
        unit: "L/h"
      - name: "tank_level"
        address: 10
        register_type: input
        data_type: u16
        unit: "%"
      - name: "tank_low_alarm"
        address: 0
        register_type: discrete
        data_type: bool
```

## Quick Reference

### Common Register Addresses

| Device Type | Register | Typical Address |
|-------------|----------|----------------|
| Temperature Sensor | Temperature | 0-10 (input) |
| Energy Meter | Voltage | 0-6 (input) |
| Energy Meter | Current | 6-12 (input) |
| Energy Meter | Power | 52-54 (input) |
| Energy Meter | Energy | 72-76 (input) |
| VFD | Frequency | 0-2 (holding) |
| VFD | Current | 2-4 (holding) |
| VFD | Run Command | 0 (coil) |
| PLC | Status Bits | 0-15 (coil) |
| PLC | Counters | 0-99 (holding) |

### Common Scale Factors

| Measurement | Scale | Example |
|-------------|-------|--------|
| Temperature | 0.1 | 235 → 23.5°C |
| Percentage | 0.1 | 500 → 50.0% |
| Voltage | 0.1 | 2300 → 230.0V |
| Current | 0.01 | 1234 → 12.34A |
| Frequency | 0.01 | 5000 → 50.00Hz |
| Pressure | 0.01 | 420 → 4.20 bar |
