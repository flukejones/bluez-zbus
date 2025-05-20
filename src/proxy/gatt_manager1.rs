use zbus::proxy;

#[proxy(
    interface = "org.bluez.GattManager1",
    default_service = "org.bluez",
    assume_defaults = true
)]
pub trait GattManager1 {
    /// RegisterApplication method
    fn register_application(
        &self,
        application: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// UnregisterApplication method
    fn unregister_application(
        &self,
        application: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;
}
