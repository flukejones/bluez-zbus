use zbus::zvariant::Type;

use crate::enum_impl_to_from_str;

enum_impl_to_from_str! {
    CharacteristicFlags, {
        Broadcast : "broadcast",
        Read : "read",
        WriteWithoutResponse : "write-without-response",
        Write : "write",
        Notify : "notify",
        Indicate : "indicate",
        AuthenticatedSignedWrites : "authenticated-signed-writes",
        ExtendedProperties : "extended-properties",
        ReliableWrite : "reliable-write",
        WritableAuxiliaries : "writable-auxiliaries",
        EncryptRead : "encrypt-read",
        EncryptWrite : "encrypt-write",
        EncryptNotify : "encrypt-notify",
        EncryptIndicate : "encrypt-indicate",
        EncryptAuthenticatedRead : "encrypt-authenticated-read",
        EncryptAuthenticatedWrite : "encrypt-authenticated-write",
        EncryptAuthenticatedNotify : "encrypt-authenticated-notify",
        EncryptAuthenticatedIndicate : "encrypt-authenticated-indicate",
        SecureRead : "secure-read",
        SecureWrite : "secure-write",
        SecureNotify : "secure-notify",
        SecureIndicate : "secure-indicate",
        Authorize : "authorize",
    }
}

enum_impl_to_from_str! {
    GattDescriptorFlags, {
        Read : "read",
        Write : "write",
        Notify : "notify",
        EncryptRead : "encrypt-read",
        EncryptWrite : "encrypt-write",
        EncryptAuthenticatedRead : "encrypt-authenticated-read",
        EncryptAuthenticatedWrite : "encrypt-authenticated-write",
        SecureRead : "secure-read",
        SecureWrite : "secure-write",
        Authorize : "authorize",
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Type)]
pub enum SupportedIncludes {
    TxPower,
    Appearance,
    #[default]
    LocalName,
    RSI,
}

impl From<&SupportedIncludes> for String {
    fn from(value: &SupportedIncludes) -> Self {
        match value {
            SupportedIncludes::TxPower => "tx-power".to_string(),
            SupportedIncludes::Appearance => "appearance".to_string(),
            SupportedIncludes::LocalName => "local-name".to_string(),
            SupportedIncludes::RSI => "rsi".to_string(),
        }
    }
}
