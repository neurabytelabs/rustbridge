//! Prometheus metrics for RustBridge
//!
//! Exposes metrics at /metrics endpoint in Prometheus format:
//! - Register read counts
//! - Error counts
//! - Poll latency histograms
//! - Device connection status
//! - MQTT publish counts

use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;
use tracing::info;

/// Initialize Prometheus metrics exporter
/// Returns a handle to render metrics
pub fn init_metrics() -> PrometheusHandle {
    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    info!("Prometheus metrics initialized");
    handle
}

/// Metrics for register read operations
pub struct ReadMetrics {
    start: Instant,
    device_id: String,
    register_name: String,
}

impl ReadMetrics {
    /// Start timing a register read
    pub fn start(device_id: &str, register_name: &str) -> Self {
        Self {
            start: Instant::now(),
            device_id: device_id.to_string(),
            register_name: register_name.to_string(),
        }
    }

    /// Record successful read
    pub fn success(self, value: f64) {
        let duration = self.start.elapsed().as_secs_f64();

        // Increment read counter
        counter!(
            "rustbridge_register_reads_total",
            "device" => self.device_id.clone(),
            "register" => self.register_name.clone(),
            "status" => "success"
        )
        .increment(1);

        // Record latency histogram
        histogram!(
            "rustbridge_read_duration_seconds",
            "device" => self.device_id.clone(),
            "register" => self.register_name.clone()
        )
        .record(duration);

        // Set current value gauge
        gauge!(
            "rustbridge_register_value",
            "device" => self.device_id,
            "register" => self.register_name
        )
        .set(value);
    }

    /// Record failed read
    pub fn failure(self, error_type: &str) {
        let duration = self.start.elapsed().as_secs_f64();

        // Increment error counter
        counter!(
            "rustbridge_register_reads_total",
            "device" => self.device_id.clone(),
            "register" => self.register_name.clone(),
            "status" => "error"
        )
        .increment(1);

        // Increment specific error counter
        counter!(
            "rustbridge_errors_total",
            "device" => self.device_id.clone(),
            "type" => error_type.to_string()
        )
        .increment(1);

        // Still record the latency
        histogram!(
            "rustbridge_read_duration_seconds",
            "device" => self.device_id,
            "register" => self.register_name
        )
        .record(duration);
    }
}

/// Record device connection status
pub fn record_device_status(device_id: &str, connected: bool) {
    gauge!(
        "rustbridge_device_connected",
        "device" => device_id.to_string()
    )
    .set(if connected { 1.0 } else { 0.0 });
}

/// Record MQTT publish event
#[allow(dead_code)] // Available for MQTT integration
pub fn record_mqtt_publish(device_id: &str, register_name: &str, success: bool) {
    counter!(
        "rustbridge_mqtt_publishes_total",
        "device" => device_id.to_string(),
        "register" => register_name.to_string(),
        "status" => if success { "success" } else { "error" }
    )
    .increment(1);
}

/// Record MQTT connection status
#[allow(dead_code)] // Available for MQTT integration
pub fn record_mqtt_connection(connected: bool) {
    gauge!("rustbridge_mqtt_connected").set(if connected { 1.0 } else { 0.0 });
}

/// Record active polling devices count
#[allow(dead_code)] // Available for bridge stats
pub fn record_active_devices(count: usize) {
    gauge!("rustbridge_active_devices").set(count as f64);
}

/// Record poll cycle timing
pub fn record_poll_cycle(device_id: &str, duration_ms: u64) {
    histogram!(
        "rustbridge_poll_cycle_seconds",
        "device" => device_id.to_string()
    )
    .record(duration_ms as f64 / 1000.0);
}

/// Record WebSocket connections
#[allow(dead_code)] // Available for WebSocket stats
pub fn record_websocket_connections(count: usize) {
    gauge!("rustbridge_websocket_connections").set(count as f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_metrics_success() {
        // Initialize metrics for testing
        let _ = PrometheusBuilder::new().install_recorder();

        let metrics = ReadMetrics::start("test-device", "temperature");
        metrics.success(25.5);
        // No panic = success
    }

    #[test]
    fn test_read_metrics_failure() {
        let _ = PrometheusBuilder::new().install_recorder();

        let metrics = ReadMetrics::start("test-device", "temperature");
        metrics.failure("timeout");
        // No panic = success
    }

    #[test]
    fn test_device_status() {
        let _ = PrometheusBuilder::new().install_recorder();

        record_device_status("plc-001", true);
        record_device_status("plc-002", false);
        // No panic = success
    }

    #[test]
    fn test_mqtt_metrics() {
        let _ = PrometheusBuilder::new().install_recorder();

        record_mqtt_publish("plc-001", "temp", true);
        record_mqtt_publish("plc-001", "pressure", false);
        record_mqtt_connection(true);
        // No panic = success
    }

    #[test]
    fn test_poll_cycle_metrics() {
        let _ = PrometheusBuilder::new().install_recorder();

        record_poll_cycle("plc-001", 150);
        record_active_devices(5);
        record_websocket_connections(3);
        // No panic = success
    }
}
