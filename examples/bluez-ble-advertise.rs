use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;

use bluez_zbus::interface::{AdvertisementType, LEAdvertisement1};
use bluez_zbus::proxy::adapter1::Adapter1ProxyBlocking;
use bluez_zbus::proxy::le_advertising_manager1::LEAdvertisingManager1ProxyBlocking;
use log::{error, info};
use zbus::blocking::Connection;
use zbus::zvariant::ObjectPath;

fn power_on(adaptor: &Adapter1ProxyBlocking) -> Result<(), zbus::Error> {
    if !adaptor.powered()? {
        adaptor.set_powered(true)?;
        info!("Turned bluetooth on");
        println!("Turned bluetooth on");
        std::thread::sleep(Duration::from_millis(1000));
        assert!(adaptor.powered()?);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut logger = env_logger::Builder::new();
    logger
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let connection = Connection::system()?;
    let adaptor = Adapter1ProxyBlocking::builder(&connection)
        .path("/org/bluez/hci0")?
        .build()?;
    power_on(&adaptor)?;

    let mut advert = LEAdvertisement1 {
        type_: AdvertisementType::Peripheral,
        local_name: Some("Name goes here".to_string()),
        appearance: Some(0x00bf),
        duration: Some(Duration::from_secs(2)),
        timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    };

    #[cfg(feature = "experimental")]
    {
        advert.discoverable = Some(true);
        advert.discoverable_timeout = Some(Duration::from_secs(30));
        advert.min_interval = Some(Duration::from_millis(100));
        advert.max_interval = Some(Duration::from_secs(1));
    }

    connection
        .object_server()
        .at(
            &ObjectPath::from_str_unchecked(
                "/org/bluez/testing/advertisement/5935e8220fd3461f95a73ea26b4628ef",
            ),
            advert,
        )
        .map_err(|err| {
            error!("{}: add_to_server {}", "path", err);
            err
        })?;

    let advertising = LEAdvertisingManager1ProxyBlocking::builder(&connection)
        .path("/org/bluez/hci0")?
        .build()?;

    // advertise
    //  - Set name
    //  - Set advertise
    // gatt
    //  - register service 00000000-4fc7-4d40-8e54-3956e5e4ffb9
    //  - register characteristic 0000afb1-4fc7-4d40-8e54-3956e5e4ffb9 + props
    //  - register application

    dbg!(advertising.active_instances()?);
    dbg!(advertising.supported_includes()?);
    dbg!(advertising.supported_instances()?);
    println!("Registering");
    advertising.register_advertisement(
        &ObjectPath::from_static_str_unchecked(
            "/org/bluez/testing/advertisement/5935e8220fd3461f95a73ea26b4628ef",
        ),
        HashMap::default(),
    )?;
    dbg!(advertising.active_instances()?);

    let mut count_down = 30;
    while count_down != 0 {
        count_down -= 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("Unregistering");
    advertising.unregister_advertisement(&ObjectPath::from_static_str_unchecked(
        "/org/bluez/testing/advertisement/5935e8220fd3461f95a73ea26b4628ef",
    ))?;
    dbg!(advertising.active_instances()?);
    // toggle_pairable(&adaptor)?;
    Ok(())
}
