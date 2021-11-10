/// Application Unique Identifier
#[derive(Clone, Debug)]
pub struct AppEui(u64);

impl AppEui {
    pub fn new(eui: u64) -> Self {
        AppEui(eui)
    }
}

impl From<&AppEui> for [u8; 8] {
    fn from(eui: &AppEui) -> Self {
        eui.0.to_le_bytes()
    }
}

/// Device Unique Identifier
#[derive(Clone, Debug)]
pub struct DevEui(u64);

impl DevEui {
    pub fn new(eui: u64) -> Self {
        DevEui(eui)
    }
}

impl From<&DevEui> for [u8; 8] {
    fn from(eui: &DevEui) -> Self {
        eui.0.to_le_bytes()
    }
}

/// Application Key
#[derive(Clone, Debug)]
pub struct AppKey(u128);

impl AppKey {
    pub fn new(key: u128) -> Self {
        AppKey(key)
    }
}

impl From<AppKey> for lorawan_encoding::keys::AES128 {
    fn from(key: AppKey) -> Self {
        key.0.to_le_bytes().into()
    }
}

/// Device Address
#[derive(Clone, Debug)]
pub struct DevAddr(u32);

impl DevAddr {
    pub fn new(addr: u32) -> Self {
        DevAddr(addr)
    }
}

impl From<&DevAddr> for [u8; 4] {
    fn from(addr: &DevAddr) -> Self {
        addr.0.to_le_bytes()
    }
}

impl From<lorawan_encoding::parser::DevAddr<&[u8]>> for DevAddr {
    fn from(addr: lorawan_encoding::parser::DevAddr<&[u8]>) -> Self {
        let mut buf = [0; 4];
        buf.copy_from_slice(&addr.as_ref()[0..4]);
        let dev_addr = u32::from_le_bytes(buf);
        DevAddr::new(dev_addr)
    }
}

/// Network Session Key
#[derive(Clone, Debug)]
pub struct NwkSKey(u128);

impl NwkSKey {
    pub fn new(key: u128) -> Self {
        NwkSKey(key)
    }
}

impl From<&NwkSKey> for lorawan_encoding::keys::AES128 {
    fn from(key: &NwkSKey) -> Self {
        key.0.to_le_bytes().into()
    }
}

impl From<lorawan_encoding::keys::AES128> for NwkSKey {
    fn from(key: lorawan_encoding::keys::AES128) -> Self {
        NwkSKey::new(u128::from_le_bytes(key.0))
    }
}

/// Application Session Key
#[derive(Clone, Debug)]
pub struct AppSKey(u128);

impl AppSKey {
    pub fn new(key: u128) -> Self {
        AppSKey(key)
    }
}

impl From<&AppSKey> for lorawan_encoding::keys::AES128 {
    fn from(key: &AppSKey) -> Self {
        key.0.to_le_bytes().into()
    }
}

impl From<lorawan_encoding::keys::AES128> for AppSKey {
    fn from(key: lorawan_encoding::keys::AES128) -> Self {
        AppSKey::new(u128::from_le_bytes(key.0))
    }
}

/// Device Nonce
#[derive(Clone, Debug)]
pub struct DevNonce(u16);

impl DevNonce {
    pub fn new(nonce: u16) -> Self {
        DevNonce(nonce)
    }
}

impl From<&DevNonce> for [u8; 2] {
    fn from(nonce: &DevNonce) -> Self {
        nonce.0.to_le_bytes()
    }
}
