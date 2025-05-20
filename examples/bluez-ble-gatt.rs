use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::time::Duration;

use bluez_zbus::interface::gatt::blocking::{
    GattApplication1, GattCharacteristic1, GattDescriptor1, GattService1,
};
use bluez_zbus::interface::gatt::{CharacteristicFlags, GattDescriptorFlags, SupportedIncludes};
use bluez_zbus::interface::{AdvertisementType, LEAdvertisement1};
use bluez_zbus::proxy::adapter1::Adapter1ProxyBlocking;
use bluez_zbus::proxy::le_advertising_manager1::LEAdvertisingManager1ProxyBlocking;
use uuid::Uuid;
use zbus::blocking::Connection;
use zbus::zvariant::ObjectPath;

fn power_on(adaptor: &Adapter1ProxyBlocking) -> Result<(), zbus::Error> {
    if !adaptor.powered()? {
        adaptor.set_powered(true)?;
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
    // connection.request_name("org.bluez.hci0.gatt")?;
    let adaptor = Adapter1ProxyBlocking::builder(&connection)
        .path("/org/bluez/hci0")?
        .build()?;
    power_on(&adaptor)?;
    dbg!(adaptor.discoverable()?);
    dbg!(adaptor.discoverable_timeout()?);
    adaptor.set_discoverable(true)?;
    adaptor.set_discoverable_timeout(Duration::from_secs(53).as_millis() as u32)?;
    std::thread::sleep(Duration::from_secs(2));
    dbg!(adaptor.discoverable()?);
    dbg!(adaptor.discoverable_timeout()?);

    let app = GattApplication1::register_new(
        "/org/bluezbus",
        connection.clone(),
        vec![(
            GattService1::new(Uuid::new_v4(), true),
            vec![
                (
                    GattCharacteristic1::new(
                        Uuid::new_v4(),
                        Some(vec![1, 2, 3, 4, 5, 6]),
                        vec![
                            CharacteristicFlags::Read,
                            CharacteristicFlags::Write,
                            CharacteristicFlags::Notify,
                        ],
                    ),
                    vec![GattDescriptor1::new(
                        Uuid::new_v4(),
                        Some(vec![41, 42, 6, 6, 6]),
                        vec![GattDescriptorFlags::Read],
                    )],
                ),
                (
                    GattCharacteristic1::new(
                        Uuid::new_v4(),
                        None,
                        vec![
                            CharacteristicFlags::Read,
                            CharacteristicFlags::Write,
                            CharacteristicFlags::Notify,
                        ],
                    ),
                    vec![],
                ),
            ],
        )],
    )?;

    //-------------------------------------------------------//
    let mut includes = BTreeSet::default();
    includes.insert(SupportedIncludes::LocalName);
    includes.insert(SupportedIncludes::Appearance);
    let mut advert = LEAdvertisement1 {
        type_: AdvertisementType::Peripheral,
        local_name: Some("Name goes here".to_string()),
        appearance: Some(0x00bf),
        duration: Some(Duration::from_secs(2)),
        timeout: Some(Duration::from_secs(60)),
        // includes,
        ..Default::default()
    };
    #[cfg(feature = "experimental")]
    {
        advert.discoverable = Some(true);
        advert.discoverable_timeout = Some(Duration::from_secs(30));
        advert.min_interval = Some(Duration::from_millis(100));
        advert.max_interval = Some(Duration::from_secs(1));
    }

    let path = ObjectPath::from_static_str_unchecked(
        "/org/bluezbus/advertisement/5935e8220fd3461f95a73ea26b4628ef",
    );
    connection.object_server().at(&path, advert)?;

    let advertising = LEAdvertisingManager1ProxyBlocking::builder(&connection)
        .path("/org/bluez/hci0")?
        .build()?;
    advertising.register_advertisement(&path, HashMap::default())?;
    dbg!(advertising.active_instances()?);

    let mut count_down = 60;
    while count_down != 0 {
        count_down -= 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("Unregistering");
    app.unregister()?;
    advertising.unregister_advertisement(&path)?;
    // let gattcha = GattCharacteristic1::new(Uuid::new_v4());
    // [variable prefix]/{hci0,hci1,...}/dev_XX_XX_XX_XX_XX_XX/serviceXX

    Ok(())
}
