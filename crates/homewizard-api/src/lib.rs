//! # HomeWizard API
//!
//! Types and HTTP client for the [HomeWizard Energy](https://www.homewizard.com) local API.
//!
//! This crate provides:
//!
//! - **[`types`]** — Rust structs matching the JSON responses from the local API
//!   (`DeviceInfo`, `MeasurementData`, `ProductType`, etc.)
//! - **[`client`]** — async HTTP client (`HwClient`) for fetching device info and
//!   measurement data from a device on the local network.
//! - **[`error`]** — error types for HTTP and parsing failures.
//!
//! ## Supported devices
//!
//! - P1 Meter (`HWE-P1`) — electricity + gas
//! - Energy Socket (`HWE-SKT`)
//! - Water Meter (`HWE-WTR`)
//! - kWh Meter 1-phase (`HWE-KWH1` / `SDM230-wifi`)
//! - kWh Meter 3-phase (`HWE-KWH3` / `SDM630-wifi`)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use homewizard_api::client::HwClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = HwClient::new("192.168.1.50", 80);
//!
//! let info = client.device_info().await?;
//! println!("{} ({})", info.product_name, info.serial);
//!
//! let data = client.measurement().await?;
//! if let Some(power) = data.active_power_w {
//!     println!("Power: {} W", power);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## API reference
//!
//! Based on the [HomeWizard Energy API documentation](https://api-documentation.homewizard.com).
//! All field names match the official API v1 responses exactly.

pub mod client;
pub mod error;
pub mod types;
