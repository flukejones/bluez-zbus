use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::error;
use uuid::Uuid;
use zbus::blocking::object_server::InterfaceRef;
use zbus::blocking::Connection;
use zbus::fdo::Error as ZbusError;
use zbus::interface;
use zbus::zvariant::{self, Array, ObjectPath, OwnedObjectPath, OwnedValue, Str};

use crate::interface::gatt::GattDescriptorFlags;

pub struct GattDescriptorHandle {
    data: Arc<Mutex<Vec<u8>>>,
    interface: InterfaceRef<GattDescriptor1>,
    property_map: HashMap<String, OwnedValue>,
    path: OwnedObjectPath,
}

impl GattDescriptorHandle {
    pub fn data(&self) -> Arc<Mutex<Vec<u8>>> {
        self.data.clone()
    }

    pub fn zbus(&self) -> &InterfaceRef<GattDescriptor1> {
        &self.interface
    }

    pub(crate) fn property_map(&self) -> HashMap<String, OwnedValue> {
        self.property_map.clone()
    }

    pub(crate) fn owned_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }
}

pub struct GattDescriptor1 {
    pub(crate) uuid: Uuid,
    data: Arc<Mutex<Vec<u8>>>,
    flags: Vec<GattDescriptorFlags>,
    char_path: OwnedObjectPath,
}

impl GattDescriptor1 {
    pub fn new(uuid: Uuid, data: Option<Vec<u8>>, flags: Vec<GattDescriptorFlags>) -> Self {
        Self {
            uuid,
            data: Arc::new(Mutex::new(data.unwrap_or_default())),
            flags,
            char_path: Default::default(),
        }
    }

    fn data(&self) -> Arc<Mutex<Vec<u8>>> {
        self.data.clone()
    }

    fn property_map(&self) -> HashMap<String, OwnedValue> {
        let mut props = HashMap::new();

        // TODO: could use try_from...
        props.insert(
            "UUID".to_string(),
            OwnedValue::from(Str::from(self.uuid.to_string())),
        );
        props.insert(
            "Characteristic".to_string(),
            OwnedValue::from(self.char_path.as_ref()),
        );
        if let Ok(data) = self
            .data
            .lock()
            .map_err(|e| log::warn!("Could not lock data: {e}"))
        {
            if let Ok(data) = OwnedValue::try_from(Array::from(&*data))
                .map_err(|e| log::warn!("Could not convert data: {e}"))
            {
                props.insert("Value".to_string(), data);
            }
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
        props
    }

    // TODO: use for notifying prop change
    // TODO: move this method to GattCharacteristic1
    fn get_descriptor_interface(
        path: &ObjectPath,
        sys_connection: &Connection,
    ) -> zbus::Result<InterfaceRef<GattDescriptor1>> {
        sys_connection
            .object_server()
            .interface::<_, GattDescriptor1>(path)
            .map_err(|err| {
                error!("{}: getting interface {}", "path", err);
                err
            })
    }

    pub fn register(
        mut self,
        path: OwnedObjectPath,
        characteristic_path: OwnedObjectPath,
        sys_connection: &Connection,
    ) -> Result<GattDescriptorHandle, zbus::Error> {
        self.char_path = characteristic_path;
        let property_map = self.property_map();
        let data = self.data();

        log::debug!("GattDescriptor1: Added UUID: {}", self.uuid);
        sys_connection
            .object_server()
            .at(&path, self)
            .map_err(|err| {
                error!("{}: add_to_server {}", "path", err);
                err
            })?;

        let interface = Self::get_descriptor_interface(&path, sys_connection)?;
        Ok(GattDescriptorHandle {
            data,
            interface,
            property_map,
            path,
        })
    }
}

#[interface(interface = "org.bluez.GattDescriptor1")]
impl GattDescriptor1 {
    /// ReadValue method
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
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

    /// WriteValue method
    fn write_value(
        &self,
        value: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
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

    /// Characteristic property
    #[zbus(property)]
    fn characteristic(&self) -> zbus::fdo::Result<zbus::zvariant::OwnedObjectPath> {
        Ok(OwnedObjectPath::default())
    }

    /// Flags property
    #[zbus(property)]
    fn flags(&self) -> zbus::fdo::Result<Vec<String>> {
        Ok(self
            .flags
            .iter()
            .map(|f| <&str>::from(f).to_string())
            .collect())
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
}
