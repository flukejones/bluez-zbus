use std::collections::{BTreeSet, HashMap};
use std::time::Duration;

use log::debug;
use uuid::Uuid;
use zbus::interface;
use zbus::zvariant::Type;

use super::gatt::SupportedIncludes;
use crate::{experimental_property, unused_property};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum AdvertisementType {
    #[default]
    Peripheral,
    Broadcast,
}

impl From<AdvertisementType> for String {
    fn from(value: AdvertisementType) -> Self {
        match value {
            AdvertisementType::Peripheral => "peripheral".to_string(),
            AdvertisementType::Broadcast => "broadcast".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Type)]
#[cfg(feature = "experimental")]
pub enum SecondaryChannel {
    OneM,
    TwoM,
    #[default]
    Coded,
}

#[cfg(feature = "experimental")]
impl From<&SecondaryChannel> for String {
    fn from(value: &SecondaryChannel) -> Self {
        match value {
            SecondaryChannel::OneM => "1M".to_string(),
            SecondaryChannel::TwoM => "2M".to_string(),
            SecondaryChannel::Coded => "coded".to_string(),
        }
    }
}

#[cfg(feature = "experimental")]
impl TryFrom<&str> for SecondaryChannel {
    type Error = zbus::fdo::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1M" => Ok(SecondaryChannel::OneM),
            "2M" => Ok(SecondaryChannel::TwoM),
            "coded" => Ok(SecondaryChannel::Coded),
            _ => Err(zbus::fdo::Error::IOError(format!(
                "{} is an invalid variant",
                value
            ))),
        }
    }
}

#[derive(Debug, Default)]
pub struct LEAdvertisement1 {
    /// Determines the type of advertising packet requested
    pub type_: AdvertisementType,
    /// List of UUIDs to include in the "Service UUID" field of the Advertising
    /// Data
    pub service_uuids: BTreeSet<Uuid>,
    /// Manufactuer Data fields to include in the Advertising Data.
    /// Keys are the Manufacturer ID to associate with the data.
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    /// Array of UUIDs to include in "Service Solicitation" Advertisement Data.
    pub solicit_uuids: BTreeSet<Uuid>,
    /// Service Data elements to include. The keys are the UUID to associate
    /// with the data.
    pub service_data: HashMap<Uuid, Vec<u8>>,
    /// Advertising Type to include in the Advertising Data. Key is the
    /// advertising type and value is the data as byte array.
    #[cfg(feature = "experimental")]
    pub data: HashMap<u8, Vec<u8>>,
    /// Advertise as general discoverable. When present this will override
    /// adapter Discoverable property.
    #[cfg(feature = "experimental")]
    pub discoverable: Option<bool>,
    /// The discoverable timeout in seconds. A value of zero means that the
    /// timeout is disabled and it will stay in discoverable/limited mode
    /// forever.
    #[cfg(feature = "experimental")]
    pub discoverable_timeout: Option<Duration>,
    /// List of features to be included in the advertising
    /// packet.
    /// Possible values: as found on LEAdvertisingManager.SupportedIncludes
    // TODO: this must be gotten fromt he manager dbus api
    pub includes: BTreeSet<SupportedIncludes>,
    pub local_name: Option<String>,
    /// Appearance to be used in the advertising report. Possible values: as
    /// found on GAP Service.
    pub appearance: Option<u16>,
    /// Rotation duration of the advertisement in seconds. If
    /// there are other applications advertising no duration is
    /// set the default is 2 seconds.
    pub duration: Option<Duration>,
    /// Timeout of the advertisement in seconds. This defines the lifetime of
    /// the advertisement.
    pub timeout: Option<Duration>,
    // #[cfg(feature = "experimental")]
    // pub secondary_channel: Option<SecondaryChannel>,
    #[cfg(feature = "experimental")]
    pub min_interval: Option<Duration>,
    #[cfg(feature = "experimental")]
    pub max_interval: Option<Duration>,
    #[cfg(feature = "experimental")]
    pub tx_power: Option<i16>,
}

#[interface(name = "org.bluez.LEAdvertisement1")]
impl LEAdvertisement1 {
    fn release(&self) -> zbus::fdo::Result<()> {
        debug!("LEAdvertisement1: release");
        Ok(())
    }

    /// Appearance property
    #[zbus(property)]
    fn appearance(&self) -> zbus::fdo::Result<u16> {
        debug!("LEAdvertisement1: appearance: {:?}", self.appearance);
        self.appearance.map_or_else(
            || {
                unused_property!("appearance", "LEAdvertisement1");
            },
            Ok,
        )
    }

    /// Data property
    #[zbus(property)]
    fn data(&self) -> zbus::fdo::Result<std::collections::HashMap<u8, Vec<u8>>> {
        experimental_property!("data", "LEAdvertisement1");
        #[cfg(feature = "experimental")]
        {
            debug!("LEAdvertisement1: data: {:?}", self.data);
            if self.data.is_empty() {
                unused_property!("data", "LEAdvertisement1");
            }
            Ok(self.data.clone())
        }
    }

    /// Discoverable property
    #[zbus(property)]
    fn discoverable(&self) -> zbus::fdo::Result<bool> {
        experimental_property!("discoverable", "LEAdvertisement1");
        #[cfg(feature = "experimental")]
        {
            debug!("LEAdvertisement1: discoverable: {:?}", self.discoverable);
            self.discoverable.map_or_else(
                || {
                    unused_property!("discoverable", "LEAdvertisement1");
                },
                Ok,
            )
        }
    }

    /// DiscoverableTimeout property
    #[zbus(property)]
    fn discoverable_timeout(&self) -> zbus::fdo::Result<u16> {
        experimental_property!("discoverable_timeout", "LEAdvertisement1");
        #[cfg(feature = "experimental")]
        {
            debug!(
                "LEAdvertisement1: discoverable_timeout: {:?}",
                self.discoverable_timeout
            );
            self.discoverable_timeout.map_or_else(
                || {
                    unused_property!("discoverable_timeout", "LEAdvertisement1");
                },
                |timeout| Ok(timeout.as_secs() as u16),
            )
        }
    }

    /// Duration property
    #[zbus(property)]
    fn duration(&self) -> zbus::fdo::Result<u16> {
        debug!("LEAdvertisement1: duration: {:?}", self.duration);
        self.duration.map_or_else(
            || {
                unused_property!("duration", "LEAdvertisement1");
            },
            |duration| Ok(duration.as_secs() as u16),
        )
    }

    /// Includes property
    #[zbus(property)]
    fn includes(&self) -> zbus::fdo::Result<Vec<String>> {
        debug!("LEAdvertisement1: includes: {:?}", self.includes);
        if self.includes.is_empty() {
            unused_property!("includes", "LEAdvertisement1");
        }
        Ok(self.includes.iter().map(String::from).collect())
    }

    /// LocalName property
    #[zbus(property)]
    fn local_name(&self) -> zbus::fdo::Result<String> {
        debug!("LEAdvertisement1: local_name: {:?}", self.local_name);
        self.local_name.as_ref().map_or_else(
            || {
                unused_property!("local_name", "LEAdvertisement1");
            },
            |name| Ok(name.clone()),
        )
    }

    /// ManufacturerData property
    #[zbus(property)]
    fn manufacturer_data(&self) -> zbus::fdo::Result<std::collections::HashMap<u16, Vec<u8>>> {
        debug!(
            "LEAdvertisement1: manufacturer_data: {:?}",
            self.manufacturer_data
        );
        if self.manufacturer_data.is_empty() {
            unused_property!("manufacturer_data", "LEAdvertisement1");
        }
        Ok(self.manufacturer_data.clone())
    }

    /// SolicitUUIDs property
    #[zbus(property, name = "SolicitUUIDs")]
    fn solicit_uuids(&self) -> zbus::fdo::Result<Vec<String>> {
        debug!("LEAdvertisement1: solicit_uuids: {:?}", self.solicit_uuids);
        if self.solicit_uuids.is_empty() {
            unused_property!("solicit_uuids", "LEAdvertisement1");
        }
        Ok(self
            .solicit_uuids
            .iter()
            .map(|uuid| uuid.to_string())
            .collect())
    }

    /// ServiceData property
    #[zbus(property)]
    fn service_data(&self) -> zbus::fdo::Result<std::collections::HashMap<String, Vec<u8>>> {
        debug!("LEAdvertisement1: service_data: {:?}", self.service_data);
        if self.service_data.is_empty() {
            unused_property!("service_data", "LEAdvertisement1");
        }
        Ok(self
            .service_data
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect())
    }

    /// ServiceUUIDs property
    #[zbus(property, name = "ServiceUUIDs")]
    fn service_uuids(&self) -> zbus::fdo::Result<Vec<String>> {
        debug!("LEAdvertisement1: service_uuids: {:?}", self.service_uuids);
        if self.service_uuids.is_empty() {
            unused_property!("service_uuids", "LEAdvertisement1");
        }
        Ok(self
            .service_uuids
            .iter()
            .map(|uuid| uuid.to_string())
            .collect())
    }

    /// Timeout property
    #[zbus(property)]
    fn timeout(&self) -> zbus::fdo::Result<u16> {
        debug!("LEAdvertisement1: timeout: {:?}", self.timeout);
        self.timeout.map_or_else(
            || {
                unused_property!("timeout", "LEAdvertisement1");
            },
            |timeout| Ok(timeout.as_secs() as u16),
        )
    }

    // #[zbus(property)]
    // #[cfg(feature = "experimental")]
    // fn secondary_channel(&self) -> zbus::fdo::Result<String> {
    //     debug!("LEAdvertisement1: secondary_channel");
    //     Ok(SecondaryChannel::OneM.into())
    // }

    #[zbus(property)]
    fn min_interval(&self) -> zbus::fdo::Result<u32> {
        experimental_property!("min_interval", "LEAdvertisement1");
        #[cfg(feature = "experimental")]
        {
            debug!("LEAdvertisement1: min_interval: {:?}", self.min_interval);
            self.min_interval.map_or_else(
                || {
                    unused_property!("min_interval", "LEAdvertisement1");
                },
                |min_interval| Ok(min_interval.as_millis() as u32),
            )
        }
    }

    #[zbus(property)]
    fn max_interval(&self) -> zbus::fdo::Result<u32> {
        experimental_property!("max_interval", "LEAdvertisement1");
        #[cfg(feature = "experimental")]
        {
            debug!("LEAdvertisement1: max_interval: {:?}", self.max_interval);
            self.max_interval.map_or_else(
                || {
                    unused_property!("max_interval", "LEAdvertisement1");
                },
                |max_interval| Ok(max_interval.as_millis() as u32),
            )
        }
    }

    #[zbus(property)]
    fn tx_power(&self) -> zbus::fdo::Result<i16> {
        experimental_property!("tx_power", "LEAdvertisement1");
        #[cfg(feature = "experimental")]
        {
            debug!("LEAdvertisement1: tx_power: {:?}", self.tx_power);
            self.tx_power.map_or_else(
                || {
                    unused_property!("tx_power", "LEAdvertisement1");
                },
                Ok,
            )
        }
    }

    /// Type property
    #[zbus(property)]
    fn type_(&self) -> zbus::fdo::Result<String> {
        debug!("LEAdvertisement1: type: {:?}", self.type_);
        Ok(self.type_.into())
    }
}
