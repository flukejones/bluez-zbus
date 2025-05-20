//! A crate to interface with the bluez daemon via DBUS

pub mod interface;
pub mod proxy;

#[macro_export]
macro_rules! experimental_property {
    ($prop_name:literal, $iface_name:literal) => {
        #[cfg(not(feature = "experimental"))]
        {
            let prop = $prop_name;
            let iface = $iface_name;
            let detail = format!("{iface}: experimental property '{prop}' disabled");
            log::trace!("{detail}");
            return Err(zbus::fdo::Error::UnknownProperty(detail));
        }
    };
}

/// Easy trace logging plus return `UnknownProperty` error
#[macro_export]
macro_rules! unused_property {
    ($prop_name:literal, $iface_name:literal) => {
        let prop = $prop_name;
        let iface = $iface_name;
        let detail = format!("{iface}: unused property '{prop}'");
        log::trace!("{detail}");
        return Err(zbus::fdo::Error::UnknownProperty(detail));
    };
}

#[macro_export]
macro_rules! enum_impl_to_from_str {
    ($type_name:ident, { $($variant:tt : $label:tt,)* }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $type_name {
            $($variant,)+
        }

        impl std::str::FromStr for $type_name {
            type Err = zbus::fdo::Error;

            fn from_str(m: &str) -> Result<Self, Self::Err> {
                let res = match m {
                    $($label => $type_name::$variant,)+
                    _ => return Err(zbus::fdo::Error::IOError(format!("{} is an invalid variant", m))),
                };
                Ok(res)
            }
        }

        impl From<$type_name> for &str {
            fn from(m: $type_name) -> Self {
                match m {
                    $($type_name::$variant => $label,)+
                }
            }
        }

        impl From<&$type_name> for &str {
            fn from(s: &$type_name) -> Self {
                match s {
                    $($type_name::$variant => $label,)+
                }
            }
        }
}}
