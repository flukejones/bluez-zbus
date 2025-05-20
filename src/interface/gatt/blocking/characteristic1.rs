use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use log::error;
use uuid::Uuid;
use zbus::blocking::object_server::InterfaceRef;
use zbus::blocking::Connection;
use zbus::fdo::Error as ZbusError;
use zbus::zvariant::{Array, ObjectPath, OwnedObjectPath, OwnedValue, Str};
use zbus::{interface, zvariant};

use super::{GattDescriptor1, GattDescriptorHandle};
use crate::interface::gatt::CharacteristicFlags;
use crate::unused_property;

/// The `GattCharacteristicHandle` provides a handle to the registered
/// `GattCharacteristic1` which is consumed by the zbus interface
pub struct GattCharacteristicHandle {
    data: Arc<Mutex<Vec<u8>>>,
    interface: InterfaceRef<GattCharacteristic1>,
    property_map: HashMap<String, OwnedValue>,
    path: OwnedObjectPath,
    descriptors: BTreeMap<Uuid, GattDescriptorHandle>,
}

impl GattCharacteristicHandle {
    pub fn data(&self) -> Arc<Mutex<Vec<u8>>> {
        self.data.clone()
    }

    pub fn zbus(&self) -> &InterfaceRef<GattCharacteristic1> {
        &self.interface
    }

    pub(crate) fn property_map(&self) -> HashMap<String, OwnedValue> {
        self.property_map.clone()
    }

    pub(crate) fn owned_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    pub fn descriptors(&self) -> &BTreeMap<Uuid, GattDescriptorHandle> {
        &self.descriptors
    }
}

pub struct GattCharacteristic1 {
    pub(crate) uuid: Uuid,
    data: Arc<Mutex<Vec<u8>>>,
    flags: Vec<CharacteristicFlags>,
    notifying: Option<bool>,
    notify_acquired: Option<bool>,
    write_acquired: Option<bool>,
    descriptors: Vec<OwnedObjectPath>,
    service_path: OwnedObjectPath,
}

impl GattCharacteristic1 {
    pub fn new(uuid: Uuid, data: Option<Vec<u8>>, flags: Vec<CharacteristicFlags>) -> Self {
        Self {
            uuid,
            data: Arc::new(Mutex::new(data.unwrap_or_default())),
            flags,
            notifying: None,
            notify_acquired: None,
            write_acquired: None,
            descriptors: Vec::default(),
            service_path: Default::default(),
        }
    }

    fn property_map(&self) -> HashMap<String, OwnedValue> {
        let mut props = HashMap::new();

        // TODO: could use try_from...
        props.insert(
            "UUID".to_string(),
            OwnedValue::from(Str::from(self.uuid.to_string())),
        );
        props.insert(
            "Service".to_string(),
            OwnedValue::from(self.service_path.as_ref()),
        );
        // TODO: make this efficient
        if let Some(write_acquired) = self.write_acquired {
            props.insert(
                "WriteAcquired".to_string(),
                OwnedValue::from(write_acquired),
            );
        }
        if let Some(notify_acquired) = self.notify_acquired {
            props.insert(
                "NotifyAcquired".to_string(),
                OwnedValue::from(notify_acquired),
            );
        }
        if let Some(notifying) = self.notifying {
            props.insert("Notifying".to_string(), OwnedValue::from(notifying));
        }
        let flags: Vec<String> = self
            .flags
            .iter()
            .map(|f| <&str>::from(f).to_string())
            .collect();
        if let Ok(flags) = OwnedValue::try_from(Array::from(flags))
            .map_err(|e| log::warn!("Could not convert flags: {e}"))
        {
            props.insert("Flags".to_string(), flags);
        }
        props.insert("Primary".to_string(), OwnedValue::from(true));

        props
    }

    // TODO: use for notifying prop change
    // TODO: move this method to GattCharacteristic1
    fn get_characteristic_interface(
        path: &ObjectPath,
        sys_connection: &Connection,
    ) -> zbus::Result<InterfaceRef<GattCharacteristic1>> {
        sys_connection
            .object_server()
            .interface::<_, GattCharacteristic1>(path)
            .map_err(|err| {
                error!("{}: getting interface {}", "path", err);
                err
            })
    }

    pub fn register(
        mut self,
        path: OwnedObjectPath,
        service_path: OwnedObjectPath,
        descriptors: Vec<GattDescriptor1>,
        sys_connection: &Connection,
    ) -> Result<GattCharacteristicHandle, zbus::Error> {
        self.service_path = service_path.clone();
        let property_map = self.property_map();
        let data = self.data.clone();
        let mut descriptor_handles = BTreeMap::default();

        for (count, descriptor) in descriptors.into_iter().enumerate() {
            let descriptor_path = OwnedObjectPath::try_from(format!("{path}/descriptor{count}"))?;
            self.descriptors.push(descriptor_path.clone());
            descriptor_handles.insert(
                descriptor.uuid,
                descriptor.register(descriptor_path, path.clone(), sys_connection)?,
            );
        }

        log::debug!("GattCharacteristic1: Added UUID: {}", self.uuid);
        sys_connection
            .object_server()
            .at(&path, self)
            .map_err(|err| {
                error!("{}: add_to_server {}", "path", err);
                err
            })?;

        let interface = Self::get_characteristic_interface(&path, sys_connection)?;
        Ok(GattCharacteristicHandle {
            data,
            interface,
            property_map,
            path,
            descriptors: descriptor_handles,
        })
    }
}

#[interface(interface = "org.bluez.GattCharacteristic1")]
impl GattCharacteristic1 {
    /// AcquireNotify method
    fn acquire_notify(
        &self,
        _options: std::collections::HashMap<&str, zvariant::Value<'_>>,
    ) -> zbus::fdo::Result<(zvariant::OwnedFd, u16)> {
        Err(ZbusError::NotSupported(
            "AcquireNotify not supported on GattCharacteristic1".to_string(),
        ))
    }

    /// AcquireWrite method
    fn acquire_write(
        &self,
        _options: std::collections::HashMap<&str, zvariant::Value<'_>>,
    ) -> zbus::fdo::Result<(zvariant::OwnedFd, u16)> {
        Err(ZbusError::NotSupported(
            "AcquireWrite not supported on GattCharacteristic1".to_string(),
        ))
    }

    /// Confirm method
    ///
    /// This method doesn't expect a reply so it is just a confirmation that
    /// value was received. Possible Errors: `org.bluez.Error.Failed`
    fn confirm(&self) -> zbus::fdo::Result<()> {
        // TODO: record that the client recieved something
        Ok(())
    }

    /// ReadValue method
    ///
    /// Issues a request to read the value of the characteristic and returns the
    /// value if the operation was successful.
    ///
    /// Possible options: "offset": uint16 offset
    /// 		  "mtu": Exchanged MTU (Server only)
    /// 		  "device": Object Device (Server only)
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, zvariant::Value<'_>>,
    ) -> zbus::fdo::Result<Vec<u8>> {
        let data = self
            .data
            .lock()
            .map_err(|e| zbus::fdo::Error::Failed(format!("Could not lock data: {e}")))?;
        let offset = if let Some(zvariant::Value::U16(ofs)) = options.get("offset") {
            if *ofs as usize >= data.len() - 1 {
                return Err(ZbusError::InvalidArgs("InvalidOffset".to_owned()));
            }
            *ofs
        } else {
            0
        } as usize;
        let data: Vec<u8> = data[offset..].to_vec();
        Ok(data)
    }

    /// StartNotify method
    ///
    /// Starts a notification session from this characteristic
    /// if it supports value notifications or indications.
    ///
    /// Possible Errors: org.bluez.Error.Failed
    ///             org.bluez.Error.NotPermitted
    ///             org.bluez.Error.InProgress
    ///             org.bluez.Error.NotConnected
    ///             org.bluez.Error.NotSupported
    fn start_notify(&self) -> zbus::fdo::Result<()> {
        // TODO: wire up the notification stuff
        Ok(())
    }

    /// StopNotify method
    ///
    /// This method will cancel any previous StartNotify
    /// transaction. Note that notifications from a
    /// characteristic are shared between sessions thus
    /// calling StopNotify will release a single session.
    ///
    /// Possible Errors: org.bluez.Error.Failed
    fn stop_notify(&self) -> zbus::fdo::Result<()> {
        // TODO: wire up the notification stuff
        Ok(())
    }

    /// WriteValue method
    ///
    /// Issues a request to write the value of the characteristic.
    fn write_value(
        &mut self,
        value: &[u8],
        options: std::collections::HashMap<&str, zvariant::Value<'_>>,
    ) -> zbus::fdo::Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| zbus::fdo::Error::Failed(format!("Could not lock data: {e}")))?;
        let offset = if let Some(zvariant::Value::U16(ofs)) = options.get("offset") {
            // if *ofs as usize >= data.len() || data.len() - (*ofs as usize) >= data.len()
            // {     return
            // Err(ZbusError::InvalidArgs("InvalidOffset".to_owned())); }
            *ofs
        } else {
            0
        } as usize;

        let data_len = data.len();
        let value_len = value.len();
        if offset + value_len > data_len {
            let max_len = if value_len < data_len {
                data_len
            } else {
                value_len
            };
            let mut new_data = vec![0; offset + max_len];
            new_data[..data_len].copy_from_slice(&data);
            new_data[offset..].copy_from_slice(value);
            *data = new_data;
        } else {
            data.truncate(value_len);
            data[offset..].copy_from_slice(value);
        }

        Ok(())
    }

    /// Descriptors property
    #[zbus(property)]
    fn descriptors(&self) -> zbus::fdo::Result<Vec<zvariant::OwnedObjectPath>> {
        // TODO: save the descriptors in this characteristic
        Ok(self.descriptors.clone())
    }

    // /// DirectedValue property
    // #[zbus(property)]
    // fn directed_value(
    //     &self,
    // ) -> zbus::fdo::Result<std::collections::HashMap<zvariant::OwnedFd,
    //   Vec<u8>>>;

    /// Flags property
    ///
    /// Defines how the characteristic value can be used. See
    /// Core spec "Table 3.5: Characteristic Properties bit
    /// field", and "Table 3.8: Characteristic Extended
    /// Properties bit field".
    ///
    /// The "x-notify" and "x-indicate" flags restrict access
    /// to notifications and indications by imposing write
    /// restrictions on a characteristic's client
    /// characteristic configuration descriptor.
    #[zbus(property)]
    fn flags(&self) -> zbus::fdo::Result<Vec<String>> {
        let flags: Vec<String> = self
            .flags
            .iter()
            .map(|f| <&str>::from(f).to_string())
            .collect();
        Ok(flags)
    }

    /// NotifyAcquired property
    ///
    /// True, if this characteristic has been acquired by any client using
    /// AcquireNotify.
    ///
    /// For server the presence of this property indicates that AcquireNotify is
    /// supported.
    #[zbus(property)]
    fn notify_acquired(&self) -> zbus::fdo::Result<bool> {
        self.notify_acquired.map_or_else(
            || {
                unused_property!("notify_acquired", "GattCharacteristic1");
            },
            Ok,
        )
    }

    /// Notifying property
    ///
    /// True, if notifications or indications on this characteristic are
    /// currently enabled.
    #[zbus(property)]
    fn notifying(&self) -> zbus::fdo::Result<bool> {
        self.notifying.map_or_else(
            || {
                unused_property!("notifying", "GattCharacteristic1");
            },
            Ok,
        )
    }

    /// Service property
    ///
    /// Object path of the GATT service the characteristic belongs to.
    // TODO:
    #[zbus(property)]
    fn service(&self) -> zbus::fdo::Result<zvariant::OwnedObjectPath> {
        Ok(self.service_path.clone())
    }

    /// UUID property
    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> zbus::fdo::Result<String> {
        Ok(self.uuid.to_string())
    }

    /// Value property
    #[zbus(property)]
    fn value(&self) -> zbus::fdo::Result<Vec<u8>> {
        Ok(Vec::default())
    }

    /// WriteAcquired property
    #[zbus(property)]
    fn write_acquired(&self) -> zbus::fdo::Result<bool> {
        self.write_acquired.map_or_else(
            || {
                unused_property!("write_acquired", "GattCharacteristic1");
            },
            Ok,
        )
    }
}
