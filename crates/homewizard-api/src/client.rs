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

    /// Ask the device to physically identify itself (LED blink, ~3s on sockets).
    ///
    /// Maps to `PUT /api/v1/identify`. Only supported by Energy Socket (HWE-SKT);
    /// other product types may return 404.
    pub async fn identify(&self) -> Result<(), ApiError> {
        let url = format!("{}/api/v1/identify", self.base_url());
        let resp = self.http.put(&url).send().await.map_err(|e| {
            if e.is_connect() || e.is_timeout() {
                ApiError::Unreachable(self.ip.clone())
            } else {
                ApiError::Http(e)
            }
        })?;
        resp.error_for_status()?;
        Ok(())
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

    #[tokio::test]
    async fn identify_sends_put_to_correct_path() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/identify"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let uri = server.uri();
        let without_scheme = uri.trim_start_matches("http://");
        let (host, port_str) = without_scheme.split_once(':').unwrap();
        let port: u16 = port_str.parse().unwrap();

        let client = HwClient::new(host, port);
        client.identify().await.unwrap();
    }

    #[tokio::test]
    async fn identify_returns_unreachable_when_port_closed() {
        // Port 1 is virtually always closed.
        let client = HwClient::new("127.0.0.1", 1);
        let err = client.identify().await.unwrap_err();
        assert!(matches!(err, ApiError::Unreachable(_)));
    }
}
