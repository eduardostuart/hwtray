//! Error types for API client operations.

/// Errors returned by [`crate::client::HwClient`] operations.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// An HTTP-level error (connection refused, TLS failure, etc.).
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// The response body could not be deserialized as JSON.
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    /// The device did not respond within the timeout window.
    #[error("Device not reachable at {0}")]
    Unreachable(String),
}
