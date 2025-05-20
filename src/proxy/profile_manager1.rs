use zbus::proxy;

#[proxy(
    interface = "org.bluez.ProfileManager1",
    default_service = "org.bluez",
    assume_defaults = true
)]
pub trait ProfileManager1 {
    /// RegisterProfile method
    fn register_profile(
        &self,
        profile: &zbus::zvariant::ObjectPath<'_>,
        uuid: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// UnregisterProfile method
    fn unregister_profile(&self, profile: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;
}
