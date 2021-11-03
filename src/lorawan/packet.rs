use core::marker::PhantomData;

use lorawan_encoding::creator::JoinRequestCreator;
use lorawan_encoding::default_crypto::DefaultFactory;
use lorawan_encoding::parser::EncryptedJoinAcceptPayload;

pub const MAX_PAYLOAD_SIZE: usize = 222;

pub struct Packet {
    payload: [u8; MAX_PAYLOAD_SIZE],
    size: usize,
}

impl Packet {
    pub fn new(payload: [u8; MAX_PAYLOAD_SIZE], size: usize) -> Self {
        Packet {
            payload,
            size,
        }
    }

    pub fn set_size(&mut self, length: usize) {
        self.size = length;
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload[0..self.size]
    }

    pub fn buf(&mut self) -> &mut [u8] {
        &mut self.payload
    }
}

impl Default for Packet {
    fn default() -> Self {
        Packet {
            payload: [0; MAX_PAYLOAD_SIZE],
            size: 0,
        }
    }
}

pub struct JoinRequest([u8; 23]);

impl JoinRequest {
    pub fn new(credentials: &Credentials, dev_nonce: u16) -> Self {
        let app_key = credentials.app_key.to_ne_bytes().into();
        let app_eui = credentials.app_eui.to_ne_bytes();
        let dev_eui = credentials.dev_eui.to_ne_bytes();
        let dev_nonce = dev_nonce.to_ne_bytes();

        let mut phy = JoinRequestCreator::new();
        phy.set_app_eui(&app_eui);
        phy.set_dev_eui(&dev_eui);
        phy.set_dev_nonce(&dev_nonce);
        // Despite the return type, build cannot fail
        let payload = phy.build(&app_key).unwrap();

        let mut buf = [0; 23];
        buf.copy_from_slice(payload);

        JoinRequest(buf)
    }

    pub fn payload(&self) -> &[u8] {
        &self.0
    }
}

pub struct JoinAccept(EncryptedJoinAcceptPayload<[u8; MAX_PAYLOAD_SIZE], DefaultFactory>);

impl JoinAccept {
    pub fn new<E>(buf: [u8; MAX_PAYLOAD_SIZE]) -> Result<Self, PacketError<E>> {
        let payload = EncryptedJoinAcceptPayload::new(buf)?;
        Ok(JoinAccept(payload))
    }

    pub fn extract(self, credentials: &Credentials, dev_nonce: u16) -> Session {
        let app_key = credentials.app_key.to_ne_bytes().into();
        let bytes = dev_nonce.to_ne_bytes();
        let dev_nonce = (&bytes).into();

        let payload = self.0.decrypt(&app_key);

        let mut buf = [0; 4];
        buf.copy_from_slice(&payload.dev_addr().as_ref()[0..4]);
        let dev_addr = u32::from_ne_bytes(buf);
        let nwk_skey = u128::from_ne_bytes(payload.derive_newskey(&dev_nonce, &app_key).0);
        let app_skey = u128::from_ne_bytes(payload.derive_appskey(&dev_nonce, &app_key).0);

        let session = Session {
            dev_addr,
            nwk_skey,
            app_skey,
        };

        // TODO: Extract settings

        session
    }
}

#[derive(Debug)]
pub struct PacketError<E> {
    error: &'static str,
    _phantom: PhantomData<E>,
}

impl<E> From<&'static str> for PacketError<E> {
    fn from(error: &'static str) -> Self {
        PacketError {
            error,
            _phantom: PhantomData,
        }
    }
}

/// Credentials needed to join a device to a network. A device that has not joined a network will
/// use this as state (see [crate::device::Device]).
pub struct Credentials {
    app_eui: u64,
    dev_eui: u64,
    app_key: u128,
}

/// Session data for a device joined to a network. It will use this as state (see
/// [crate::device::Device]).
pub struct Session {
    dev_addr: u32,
    nwk_skey: u128,
    app_skey: u128,
}

impl Session {
    /// Creates a session directly for ABP.
    pub fn new(dev_addr: u32, nwk_skey: u128, app_skey: u128) -> Self {
        Session {
            dev_addr,
            nwk_skey,
            app_skey,
        }
    }
}
