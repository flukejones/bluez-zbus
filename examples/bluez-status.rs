use std::time::Duration;

use bluez_zbus::proxy::adapter1::Adapter1ProxyBlocking;
use zbus::blocking::Connection;

fn toggle_pairable(adaptor: &Adapter1ProxyBlocking) -> Result<(), zbus::Error> {
    if !adaptor.pairable()? {
        // This call relies on the macro for it being set as:
        // #[dbus_proxy(property, name = "Pairable")]
        // in zbus.rs
        adaptor.set_pairable(true)?;
        // Another way to set a property is like this
        // adaptor.set_property("Pairable", true)?;
        println!("Set bluetooth pairable");
        assert!(adaptor.pairable()?);
    } else {
        adaptor.set_pairable(false)?;
        println!("Set bluetooth not pairable");
    }
    Ok(())
}

fn toggle_powered(adaptor: &Adapter1ProxyBlocking) -> Result<(), zbus::Error> {
    if !adaptor.powered()? {
        adaptor.set_powered(true)?;
        println!("Turned bluetooth on");
        std::thread::sleep(Duration::from_millis(100));
        assert!(adaptor.powered()?);
    } else {
        adaptor.set_powered(false)?;
        println!("Turned bluetooth off");
        std::thread::sleep(Duration::from_millis(100));
        assert!(!adaptor.powered()?);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system()?;
    let adaptor = Adapter1ProxyBlocking::builder(&connection)
        .path("/org/bluez/hci0")?
        .build()?;

    toggle_powered(&adaptor)?;
    toggle_pairable(&adaptor)?;

    assert!(!adaptor.discoverable()?);
    assert_eq!(!adaptor.discoverable_timeout()?, 4294967115);

    Ok(())
}
