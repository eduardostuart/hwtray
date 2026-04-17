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

    /// Build an `HwClient` pointing at the given wiremock server.
    fn client_for(server: &wiremock::MockServer) -> HwClient {
        let uri = server.uri();
        let without_scheme = uri.trim_start_matches("http://");
        let (host, port_str) = without_scheme.split_once(':').unwrap();
        HwClient::new(host, port_str.parse().unwrap())
    }

    #[tokio::test]
    async fn device_info_fetches_and_parses_json() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "product_type": "HWE-SKT",
                "product_name": "Energy Socket",
                "serial": "aabbccddeeff",
                "firmware_version": "4.19",
                "api_version": "v1"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let info = client_for(&server).device_info().await.unwrap();
        assert_eq!(info.serial, "aabbccddeeff");
        assert_eq!(info.firmware_version, "4.19");
    }

    #[tokio::test]
    async fn device_info_returns_parse_error_on_invalid_json() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&server)
            .await;

        let err = client_for(&server).device_info().await.unwrap_err();
        assert!(
            !matches!(err, ApiError::Unreachable(_)),
            "parse failure should not be classified as Unreachable"
        );
    }

    #[tokio::test]
    async fn device_info_returns_unreachable_when_port_closed() {
        let client = HwClient::new("127.0.0.1", 1);
        let err = client.device_info().await.unwrap_err();
        assert!(matches!(err, ApiError::Unreachable(_)));
    }

    #[tokio::test]
    async fn measurement_fetches_and_parses_json() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/data"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "wifi_ssid": "MyWiFi",
                "active_power_w": 543.0,
                "total_power_import_kwh": 1234.567
            })))
            .expect(1)
            .mount(&server)
            .await;

        let data = client_for(&server).measurement().await.unwrap();
        assert_eq!(data.active_power_w, Some(543.0));
        assert_eq!(data.wifi_ssid.as_deref(), Some("MyWiFi"));
    }

    #[tokio::test]
    async fn measurement_returns_unreachable_when_port_closed() {
        let client = HwClient::new("127.0.0.1", 1);
        let err = client.measurement().await.unwrap_err();
        assert!(matches!(err, ApiError::Unreachable(_)));
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

        client_for(&server).identify().await.unwrap();
    }

    #[tokio::test]
    async fn identify_propagates_http_error_on_non_2xx() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/identify"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let err = client_for(&server).identify().await.unwrap_err();
        assert!(matches!(err, ApiError::Http(_)));
    }

    #[tokio::test]
    async fn identify_returns_unreachable_when_port_closed() {
        // Port 1 is virtually always closed.
        let client = HwClient::new("127.0.0.1", 1);
        let err = client.identify().await.unwrap_err();
        assert!(matches!(err, ApiError::Unreachable(_)));
    }
}
