/// Application Unique Identifier
#[derive(Clone, Debug)]
pub struct AppEui([u8; 8]);

impl AppEui {
    pub fn new(eui: u64) -> Self {
        AppEui(eui.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

/// Device Unique Identifier
#[derive(Clone, Debug)]
pub struct DevEui([u8; 8]);

impl DevEui {
    pub fn new(eui: u64) -> Self {
        DevEui(eui.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

/// Application Key
#[derive(Clone, Debug)]
pub struct AppKey([u8; 16]);

impl AppKey {
    pub fn new(key: u128) -> Self {
        AppKey(key.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

/// Device Address
#[derive(Clone, Debug)]
pub struct DevAddr([u8; 4]);

impl DevAddr {
    pub fn new(addr: u32) -> Self {
        DevAddr(addr.to_le_bytes())
    }

    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        DevAddr(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }
}

/// Network Session Key
#[derive(Clone, Debug)]
pub struct NwkSKey([u8; 16]);

impl NwkSKey {
    pub fn new(key: u128) -> Self {
        NwkSKey(key.to_le_bytes())
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        NwkSKey(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

/// Application Session Key
#[derive(Clone, Debug)]
pub struct AppSKey([u8; 16]);

impl AppSKey {
    pub fn new(key: u128) -> Self {
        AppSKey(key.to_le_bytes())
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        AppSKey(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

/// Device Nonce
#[derive(Clone, Debug)]
pub struct DevNonce([u8; 2]);

impl DevNonce {
    pub fn new(nonce: u16) -> Self {
        DevNonce(nonce.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 2] {
        &self.0
    }
}
