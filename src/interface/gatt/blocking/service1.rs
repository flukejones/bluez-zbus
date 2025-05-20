//! # GattService1 implementation
//!
//! A service may have multiple characteristics, and a characteristic may have a
//! descriptor. An application (the user of this crate) may have mutliple
//! services available. It is up to the application to register each sevice.
//!
//! The service and all nested attributes are treated as immutable, except for
//! the characteristic data which is shared via `Arc<Mutex<T>>`.
//!
//! ```ignore
//! -> /com/example
//!   |   - org.freedesktop.DBus.ObjectManager
//!   |
//!   -> /com/example/service0
//!   | |   - org.freedesktop.DBus.Properties
//!   | |   - org.bluez.GattService1
//!   | |
//!   | -> /com/example/service0/char0
//!   | |     - org.freedesktop.DBus.Properties
//!   | |     - org.bluez.GattCharacteristic1
//!   | |
//!   | -> /com/example/service0/char1
//!   |   |   - org.freedesktop.DBus.Properties
//!   |   |   - org.bluez.GattCharacteristic1
//!   |   |
//!   |   -> /com/example/service0/char1/desc0
//!   |       - org.freedesktop.DBus.Properties
//!   |       - org.bluez.GattDescriptor1
//!   |
//!   -> /com/example/service1
//!     |   - org.freedesktop.DBus.Properties
//!     |   - org.bluez.GattService1
//!     |
//!     -> /com/example/service1/char0
//!         - org.freedesktop.DBus.Properties
//!         - org.bluez.GattCharacteristic1
//! ```

use std::collections::{BTreeMap, HashMap};

use log::error;
use uuid::Uuid;
use zbus::blocking::Connection;
use zbus::interface;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Str};

use super::characteristic1::{GattCharacteristic1, GattCharacteristicHandle};
use super::GattDescriptor1;

pub struct GattServiceHandle {
    characteristics: BTreeMap<Uuid, GattCharacteristicHandle>,
    _uuid: Uuid,
    _primary: bool,
    property_map: HashMap<String, OwnedValue>,
    path: OwnedObjectPath,
}

impl GattServiceHandle {
    pub fn characteristics(&self) -> &BTreeMap<Uuid, GattCharacteristicHandle> {
        &self.characteristics
    }

    pub(crate) fn property_map(&self) -> HashMap<String, OwnedValue> {
        self.property_map.clone()
    }

    pub(crate) fn owned_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }
}

#[derive(Debug)]
pub struct GattService1 {
    uuid: Uuid,
    primary: bool,
}

impl GattService1 {
    pub fn new(uuid: Uuid, primary: bool) -> Self {
        Self { uuid, primary }
    }

    fn property_map(&self) -> HashMap<String, OwnedValue> {
        let mut props = HashMap::new();

        // TODO: could use try_from...
        props.insert(
            "UUID".to_string(),
            OwnedValue::from(Str::from(self.uuid.to_string())),
        );
        props.insert("Primary".to_string(), OwnedValue::from(self.primary));
        // if !self.includes.is_empty() {
        //     let includes: Vec<String> = self.includes.iter().cloned().collect();
        //     let includes = Array::from(includes);
        //     props.insert("Includes".to_string(), OwnedValue::from(includes));
        // }

        props
    }

    pub fn register(
        self,
        characteristics: Vec<(GattCharacteristic1, Vec<GattDescriptor1>)>,
        sys_connection: &Connection,
        service_path: OwnedObjectPath,
    ) -> Result<GattServiceHandle, zbus::Error> {
        let mut service_handle = GattServiceHandle {
            characteristics: BTreeMap::new(),
            _uuid: self.uuid,
            _primary: self.primary,
            property_map: self.property_map(),
            path: service_path.clone(),
        };

        for (count, (gatt_char, descriptors)) in characteristics.into_iter().enumerate() {
            service_handle.characteristics.insert(
                gatt_char.uuid,
                gatt_char.register(
                    OwnedObjectPath::try_from(format!("{service_path}/characteristic{count}"))?,
                    service_path.clone(),
                    descriptors,
                    sys_connection,
                )?,
            );
            // TODO: push includes paths
        }

        log::debug!("GattService1: Added UUID: {}", self.uuid);
        sys_connection
            .object_server()
            .at(&service_path, self)
            .map_err(|err| {
                error!("{}: add_to_server {}", "path", err);
                err
            })?;

        Ok(service_handle)
    }
}

#[interface(interface = "org.bluez.GattService1")]
impl GattService1 {
    // /// Includes property
    // ///
    // /// Array of object paths representing the included services of this service.
    // #[zbus(property)]
    // fn includes(&self) -> zbus::fdo::Result<Vec<String>> {
    //     if self.includes.is_empty() {
    //         unused_property!("includes", "GattService1");
    //     }
    //     Ok(self.includes.iter().map(|inc| inc.to_string()).collect())
    // }

    /// Primary property
    ///
    /// Indicates whether or not this GATT service is a primary service. If
    /// false, the service is secondary.
    #[zbus(property)]
    fn primary(&self) -> zbus::fdo::Result<bool> {
        Ok(self.primary)
    }

    /// UUID property
    ///
    /// 128-bit service UUID
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<String> {
        Ok(self.uuid.to_string())
    }
}
