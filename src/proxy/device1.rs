use zbus::proxy;

#[proxy(interface = "org.bluez.Device1", assume_defaults = true)]
pub trait Device1 {
    /// CancelPairing method
    fn cancel_pairing(&self) -> zbus::Result<()>;

    /// Connect method
    fn connect(&self) -> zbus::Result<()>;

    /// ConnectProfile method
    fn connect_profile(&self, uuid: &str) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// DisconnectProfile method
    fn disconnect_profile(&self, uuid: &str) -> zbus::Result<()>;

    /// Pair method
    fn pair(&self) -> zbus::Result<()>;

    /// Adapter property
    #[zbus(property)]
    fn adapter(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Address property
    #[zbus(property)]
    fn address(&self) -> zbus::Result<String>;

    /// AddressType property
    #[zbus(property)]
    fn address_type(&self) -> zbus::Result<String>;

    /// AdvertisingData property
    #[zbus(property)]
    fn advertising_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<u8, zbus::zvariant::OwnedValue>>;

    /// AdvertisingFlags property
    #[zbus(property)]
    fn advertising_flags(&self) -> zbus::Result<Vec<bool>>;

    /// Alias property
    #[zbus(property)]
    fn alias(&self) -> zbus::Result<String>;
    #[zbus(property, name = "Alias")]
    fn set_alias(&self, value: &str) -> zbus::Result<()>;

    /// Appearance property
    #[zbus(property)]
    fn appearance(&self) -> zbus::Result<u16>;

    /// Blocked property
    #[zbus(property)]
    fn blocked(&self) -> zbus::Result<bool>;
    #[zbus(property, name = "Blocked")]
    fn set_blocked(&self, value: bool) -> zbus::Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> zbus::Result<u32>;

    /// Connected property
    #[zbus(property)]
    fn connected(&self) -> zbus::Result<bool>;

    /// Icon property
    #[zbus(property)]
    fn icon(&self) -> zbus::Result<String>;

    /// LegacyPairing property
    #[zbus(property)]
    fn legacy_pairing(&self) -> zbus::Result<bool>;

    /// ManufacturerData property
    #[zbus(property)]
    fn manufacturer_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<u16, zbus::zvariant::OwnedValue>>;

    /// Modalias property
    #[zbus(property)]
    fn modalias(&self) -> zbus::Result<String>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Paired property
    #[zbus(property)]
    fn paired(&self) -> zbus::Result<bool>;

    /// RSSI property
    #[zbus(property, name = "RSSI")]
    fn rssi(&self) -> zbus::Result<i16>;

    /// ServiceData property
    #[zbus(property)]
    fn service_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ServicesResolved property
    #[zbus(property)]
    fn services_resolved(&self) -> zbus::Result<bool>;

    /// Trusted property
    #[zbus(property)]
    fn trusted(&self) -> zbus::Result<bool>;
    #[zbus(property, name = "Trusted")]
    fn set_trusted(&self, value: bool) -> zbus::Result<()>;

    /// TxPower property
    #[zbus(property)]
    fn tx_power(&self) -> zbus::Result<i16>;

    /// UUIDs property
    #[zbus(property, name = "UUIDs")]
    fn uuids(&self) -> zbus::Result<Vec<String>>;

    /// WakeAllowed property
    #[zbus(property)]
    fn wake_allowed(&self) -> zbus::Result<bool>;
    #[zbus(property, name = "WakeAllowed")]
    fn set_wake_allowed(&self, value: bool) -> zbus::Result<()>;
}
