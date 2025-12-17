//! Prometheus metrics service

use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

/// Initialize the Prometheus metrics exporter
pub fn init_metrics() -> PrometheusHandle {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
}

/// Record an HTTP request
pub fn record_request(method: &str, path: &str, status: u16, duration_ms: f64) {
    counter!("omni_http_requests_total", "method" => method.to_string(), "path" => path.to_string(), "status" => status.to_string()).increment(1);
    histogram!("omni_http_request_duration_ms", "method" => method.to_string(), "path" => path.to_string()).record(duration_ms);
}

/// Update active sessions gauge
pub fn set_active_sessions(count: u64) {
    gauge!("omni_active_sessions").set(count as f64);
}

/// Update registered clients gauge
pub fn set_registered_clients(count: u64) {
    gauge!("omni_registered_clients").set(count as f64);
}

/// Update known servers gauge
pub fn set_known_servers(count: u64) {
    gauge!("omni_known_servers").set(count as f64);
}

/// Increment registration counter
pub fn increment_registrations() {
    counter!("omni_registrations_total").increment(1);
}

/// Increment federation sync counter
pub fn increment_federation_syncs(success: bool) {
    let status = if success { "success" } else { "failure" };
    counter!("omni_federation_syncs_total", "status" => status).increment(1);
}

/// Timer for measuring request duration
pub struct RequestTimer {
    start: Instant,
    method: String,
    path: String,
}

impl RequestTimer {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            start: Instant::now(),
            method: method.to_string(),
            path: path.to_string(),
        }
    }

    pub fn finish(self, status: u16) {
        let duration_ms = self.start.elapsed().as_secs_f64() * 1000.0;
        record_request(&self.method, &self.path, status, duration_ms);
    }
}
