//! mDNS discovery of HomeWizard devices on the local network.

use mdns_sd::{ServiceDaemon, ServiceEvent};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

const SERVICE_TYPE: &str = "_hwenergy._tcp.local.";

/// A device discovered via mDNS on the local network.
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    /// Human-readable product name (from TXT record or hostname).
    pub name: String,
    /// IPv4 (preferred) or IPv6 address.
    pub ip: String,
    /// HTTP port advertised by the service.
    pub port: u16,
    /// Device serial number (from mDNS TXT `serial` property).
    pub serial: String,
    /// Product type string (from mDNS TXT `product_type` property).
    pub product_type: String,
    /// Whether the local API is enabled on this device.
    pub api_enabled: bool,
}

impl DiscoveredDevice {
    /// Unique identifier — uses the serial number.
    pub fn id(&self) -> &str {
        &self.serial
    }
}

/// Discovers HomeWizard devices on the local network via mDNS.
pub struct DiscoveryService {
    daemon: Option<ServiceDaemon>,
}

impl DiscoveryService {
    pub fn new() -> Self {
        Self { daemon: None }
    }

    /// Start browsing for HomeWizard devices.
    /// Returns a channel that receives discovered devices.
    ///
    /// Calling this while a daemon is already running transparently replaces
    /// the previous instance — the old daemon is shut down first so its
    /// spawned task exits cleanly.
    pub fn start(&mut self) -> Result<mpsc::Receiver<DiscoveredDevice>, String> {
        self.stop();

        let daemon =
            ServiceDaemon::new().map_err(|e| format!("Failed to start mDNS daemon: {e}"))?;

        let receiver = daemon
            .browse(SERVICE_TYPE)
            .map_err(|e| format!("Failed to browse: {e}"))?;

        let (tx, rx) = mpsc::channel(32);
        self.daemon = Some(daemon);

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv_async().await {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let addresses = info.get_addresses();
                        // Prefer IPv4 over IPv6 (link-local IPv6 often fails)
                        let ip = match addresses
                            .iter()
                            .find(|a| a.is_ipv4())
                            .or_else(|| addresses.iter().next())
                        {
                            Some(addr) => addr.to_string(),
                            None => {
                                warn!("Resolved service with no address: {}", info.get_fullname());
                                continue;
                            }
                        };

                        let properties = info.get_properties();
                        let serial = properties
                            .get_property_val_str("serial")
                            .unwrap_or_default()
                            .to_string();
                        let product_type = properties
                            .get_property_val_str("product_type")
                            .unwrap_or_default()
                            .to_string();
                        let api_enabled = properties
                            .get_property_val_str("api_enabled")
                            .unwrap_or("0")
                            == "1";

                        // Use product_name from TXT records, fall back to
                        // hostname without the service suffix.
                        let product_name = properties
                            .get_property_val_str("product_name")
                            .unwrap_or_default()
                            .to_string();
                        let name = if product_name.is_empty() {
                            info.get_hostname()
                                .trim_end_matches('.')
                                .trim_end_matches(".local")
                                .to_string()
                        } else {
                            product_name
                        };

                        let device = DiscoveredDevice {
                            name,
                            ip,
                            port: info.get_port(),
                            serial,
                            product_type,
                            api_enabled,
                        };

                        info!(
                            "Discovered device: {} at {}:{}",
                            device.name, device.ip, device.port
                        );

                        if tx.send(device).await.is_err() {
                            debug!("Discovery channel closed, stopping browse");
                            break;
                        }
                    }
                    ServiceEvent::SearchStarted(_) => {
                        debug!("mDNS search started");
                    }
                    ServiceEvent::ServiceRemoved(_, name) => {
                        debug!("Service removed: {name}");
                    }
                    _ => {}
                }
            }
        });

        Ok(rx)
    }

    /// Stop the mDNS discovery daemon.
    pub fn stop(&mut self) {
        if let Some(daemon) = self.daemon.take() {
            let _ = daemon.shutdown();
            info!("mDNS discovery stopped");
        }
    }
}

impl Default for DiscoveryService {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DiscoveryService {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_returns_serial() {
        let device = DiscoveredDevice {
            name: "P1 Meter".to_string(),
            ip: "192.168.1.50".to_string(),
            port: 80,
            serial: "aabbccddeeff".to_string(),
            product_type: "HWE-P1".to_string(),
            api_enabled: true,
        };
        assert_eq!(device.id(), "aabbccddeeff");
    }

    /// Calling start() twice without stop() must shut down the previous
    /// daemon — otherwise its internal channel stays open and the spawned
    /// task leaks until process exit. We observe this by checking that the
    /// first receiver closes within a reasonable window.
    #[tokio::test]
    async fn double_start_closes_previous_receiver() {
        let mut service = DiscoveryService::new();
        let Ok(mut rx1) = service.start() else {
            // mDNS may be unavailable in sandboxed environments
            eprintln!("skipping: mDNS daemon unavailable");
            return;
        };
        let _rx2 = service.start().expect("second start");

        let closed = tokio::time::timeout(std::time::Duration::from_secs(2), rx1.recv())
            .await
            .expect("first receiver should close after second start");
        assert!(
            closed.is_none(),
            "first receiver should yield None after second start, got: {closed:?}"
        );
    }
}
