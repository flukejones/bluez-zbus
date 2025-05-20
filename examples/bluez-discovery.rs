use bluez_zbus::proxy::adapter1::Adapter1ProxyBlocking;
use bluez_zbus::proxy::object_manager::ManagedBluezObject;
use zbus::blocking::fdo::ObjectManagerProxy;
use zbus::blocking::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system()?;
    let adaptor = Adapter1ProxyBlocking::builder(&connection)
        .path("/org/bluez/hci0")?
        .build()?;
    let objman = ObjectManagerProxy::builder(&connection)
        .destination("org.bluez")?
        .interface("org.freedesktop.DBus.ObjectManager")?
        .path("/")?
        .build()?;

    // adaptor.set_discovery_filter(HashMap::default())?;
    adaptor.set_powered(true)?;
    if adaptor.discovering()? {
        adaptor.stop_discovery()?;
    }
    adaptor.start_discovery()?;
    dbg!(adaptor.discovering()?);
    // dbg!(objman.get_managed_objects()?);

    while let Some(wasd) = objman.receive_interfaces_added().unwrap().next() {
        dbg!(wasd.message().body().signature());
        let body: ManagedBluezObject = wasd.message().body().deserialize()?;
        dbg!(&body.path);
        dbg!(body.device_data().unwrap());
    }
    Ok(())
}
