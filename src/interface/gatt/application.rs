// GattApplication1

use std::collections::HashMap;

use log::error;
use zbus::interface;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use zbus::Connection;

use super::characteristic1::GattCharacteristic1;
use super::service1::{GattService1, GattServiceHandle};
use super::GattDescriptor1;
use crate::proxy::gatt_manager1::GattManager1Proxy;

/// Mapped values to properties under this service
type Properties = HashMap<String, OwnedValue>;
/// Map of all the services under this path
type Services = HashMap<String, Properties>;
/// Map of all the pathes used by this application for gatts
type ManagedObjects = HashMap<OwnedObjectPath, Services>;

pub struct GattApplicationHandle {
    connection: Connection,
    services: Vec<GattServiceHandle>,
    path: OwnedObjectPath,
}

impl GattApplicationHandle {
    // TODO: use stored device path + actual used path for self
    pub async fn unregister(&self) -> Result<(), zbus::Error> {
        let proxy = GattManager1Proxy::builder(&self.connection)
            .path("/org/bluez/hci0")?
            .build()
            .await?;
        proxy.unregister_application(&self.path).await
    }

    pub fn services(&self) -> &[GattServiceHandle] {
        &self.services
    }
}

#[derive(Debug)]
pub struct GattApplication1 {
    connection: Connection,
    // TODO: store base path
    managed_objects: ManagedObjects,
}

impl GattApplication1 {
    #[allow(clippy::type_complexity)]
    pub async fn register_new(
        path: &str,
        connection: Connection,
        services: Vec<(
            GattService1,
            Vec<(GattCharacteristic1, Vec<GattDescriptor1>)>,
        )>,
    ) -> Result<GattApplicationHandle, zbus::Error> {
        let path = OwnedObjectPath::try_from(path)?;
        let mut application = Self {
            connection,
            managed_objects: HashMap::default(),
        };

        let connection = application.connection.clone();
        let mut serv_handles = Vec::new();
        for (count, serv) in services.into_iter().enumerate() {
            serv_handles.push(
                serv.0
                    .register(
                        serv.1,
                        &application.connection,
                        OwnedObjectPath::try_from(format!("{path}/service{count}"))?,
                    )
                    .await?,
            );
        }

        // This exists just to build the property list
        // TODO: make this better, move it somewhere else etc.
        for serv in &serv_handles {
            let props = serv.property_map();
            let mut service = HashMap::new();
            service.insert("org.bluez.GattService1".to_string(), props);
            application
                .managed_objects
                .insert(serv.owned_path(), service);

            for (_, char) in &mut serv.characteristics().iter() {
                let props = char.property_map();
                let mut service = HashMap::new();
                service.insert("org.bluez.GattCharacteristic1".to_string(), props);
                application
                    .managed_objects
                    .insert(char.owned_path(), service);
                for (_, desc) in char.descriptors().iter() {
                    let props = desc.property_map();
                    let mut descriptor = HashMap::new();
                    descriptor.insert("org.bluez.GattDescriptor1".to_string(), props);
                    application
                        .managed_objects
                        .insert(desc.owned_path(), descriptor);
                }
            }
        }

        connection
            .object_server()
            .at(&path, application)
            .await
            .map_err(|err| {
                error!("{}: add_to_server {}", path, err);
                err
            })?;

        let proxy = GattManager1Proxy::builder(&connection)
            .path("/org/bluez/hci0")?
            .build()
            .await?;
        // proxy.call_method("RegisterApplication", &{})?;
        proxy
            .register_application(&path, HashMap::default())
            .await?;

        Ok(GattApplicationHandle {
            services: serv_handles,
            connection,
            path,
        })
    }
}

#[interface(interface = "org.freedesktop.DBus.ObjectManager")]
impl GattApplication1 {
    /// Includes property
    ///
    /// Array of object paths representing the included services of this
    /// service.
    #[zbus(name = "GetManagedObjects")]
    fn get_managed_objects(&self) -> zbus::fdo::Result<ManagedObjects> {
        Ok(self.managed_objects.clone())
    }
}
