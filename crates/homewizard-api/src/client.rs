//! HTTP client for the HomeWizard local API.

use std::sync::OnceLock;

use crate::error::ApiError;
use crate::types::{DeviceInfo, MeasurementData};

static SHARED_HTTP: OnceLock<reqwest::Client> = OnceLock::new();

/// Build (or retrieve) the process-wide shared reqwest client.
///
/// Moving construction behind `OnceLock` keeps the theoretical
/// `ClientBuilder::build` failure confined to a single site and avoids
/// allocating a fresh connection pool per device.
fn shared_http() -> &'static reqwest::Client {
    SHARED_HTTP.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .expect("failed to build HTTP client")
    })
}

/// HTTP client for a single HomeWizard device.
#[derive(Debug, Clone)]
pub struct HwClient {
    ip: String,
    port: u16,
    http: reqwest::Client,
}

impl HwClient {
    /// Create a new client for a device at the given IP and port.
    pub fn new(ip: &str, port: u16) -> Self {
        Self {
            ip: ip.to_string(),
            port,
            http: shared_http().clone(),
        }
    }

    /// Base URL for this device.
    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.ip, self.port)
    }

    /// Fetch device info (GET /api).
    pub async fn device_info(&self) -> Result<DeviceInfo, ApiError> {
        let url = format!("{}/api", self.base_url());
        let resp = self.http.get(&url).send().await.map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                ApiError::Unreachable(self.ip.clone())
            } else {
                ApiError::Http(e)
            }
        })?;
        Ok(resp.json().await?)
    }

    /// Fetch current measurement data (GET /api/v1/data).
    pub async fn measurement(&self) -> Result<MeasurementData, ApiError> {
        let url = format!("{}/api/v1/data", self.base_url());
        let resp = self.http.get(&url).send().await.map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                ApiError::Unreachable(self.ip.clone())
            } else {
                ApiError::Http(e)
            }
        })?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_correct_url() {
        let client = HwClient::new("192.168.1.50", 80);
        assert_eq!(client.base_url(), "http://192.168.1.50:80");
    }

    #[test]
    fn custom_port() {
        let client = HwClient::new("10.0.0.5", 8080);
        assert_eq!(client.base_url(), "http://10.0.0.5:8080");
    }

    /// shared_http() is memoized so all HwClient instances reuse a single
    /// connection pool and the builder .expect() only runs once.
    #[test]
    fn shared_http_is_memoized() {
        let first = shared_http() as *const reqwest::Client;
        let second = shared_http() as *const reqwest::Client;
        assert_eq!(first, second);
    }
}
