use std::collections::HashMap;

use serde::Deserialize;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Type};

#[derive(Debug, Type, Deserialize)]
pub struct ManagedBluezObject {
    pub path: OwnedObjectPath,
    pub data: HashMap<String, HashMap<String, OwnedValue>>,
}

impl ManagedBluezObject {
    pub fn device_data(&self) -> Option<BluezDevice> {
        // TODO: change the path
        // TODO: could be multiple devices
        if let Some(stuff) = self.data.get("org.bluez.Device1") {
            return Some(BluezDevice::from(stuff));
        }
        None
    }
}

#[derive(Debug, Default, Type, Deserialize)]
#[serde(default)]
pub struct BluezDevice {
    trusted: bool,
    alias: String,
    address: String,
    address_type: String,
    rssi: i16,
    legacy_pairing: bool,
    blocked: bool,
    connected: bool,
    adapter: OwnedObjectPath,
    service_resolved: bool,
    bonded: bool,
    paired: bool,
    // uuids:
}

impl From<&HashMap<String, OwnedValue>> for BluezDevice {
    fn from(value: &HashMap<String, OwnedValue>) -> Self {
        Self {
            trusted: value
                .get("Trusted")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            alias: value
                .get("Alias")
                .map(|b| <&str>::try_from(b).unwrap_or_default())
                .unwrap_or_default()
                .to_string(),
            address: value
                .get("Address")
                .map(|b| <&str>::try_from(b).unwrap_or_default())
                .unwrap_or_default()
                .to_string(),
            address_type: value
                .get("AddressType")
                .map(|b| <&str>::try_from(b).unwrap_or_default())
                .unwrap_or_default()
                .to_string(),
            rssi: value
                .get("RSSI")
                .map(|b| i16::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            legacy_pairing: value
                .get("LegacyPairing")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            blocked: value
                .get("Blocked")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            connected: value
                .get("Connected")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            adapter: value
                .get("Adapter")
                .map(|b| OwnedObjectPath::try_from(b.clone()).unwrap_or_default())
                .unwrap_or_default(),
            service_resolved: value
                .get("ServicesResolved")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            bonded: value
                .get("Bonded")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
            paired: value
                .get("Paired")
                .map(|b| bool::try_from(b).unwrap_or_default())
                .unwrap_or_default(),
        }
    }
}
