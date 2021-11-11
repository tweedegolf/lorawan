use core::marker::PhantomData;

use lorawan_encoding::creator::{DataPayloadCreator, JoinRequestCreator};
use lorawan_encoding::default_crypto::DefaultFactory;
use lorawan_encoding::parser::{DataPayload, EncryptedJoinAcceptPayload, PhyPayload};

use crate::device::{Credentials, DeviceState, Session, Settings};
use crate::lorawan::{AppSKey, DevAddr, DevNonce, NwkSKey};
use crate::radio::Frequency;

pub const MAX_PAYLOAD_SIZE: usize = 222;

pub struct Uplink;

impl Uplink {
    pub fn new(payload: &[u8], state: &mut DeviceState) -> Self {
        let session = state.session();
        let mut phy = DataPayloadCreator::new();
        phy.set_confirmed(true);
        phy.set_dev_addr(session.dev_addr().as_bytes());
        // phy.set_f_port();
        phy.set_fcnt(state.fcnt_up());
        // phy.set_fctrl();
        phy.set_uplink(true);
        let payload = phy.build(payload, &[], &session.nwk_skey().as_bytes().clone().into(), &session.app_skey().as_bytes().clone().into()).unwrap();

        state.increment_fcnt_up();

        todo!()
    }
}

pub struct Downlink;

impl Downlink {
    pub fn from_data(data: &mut [u8], state: &mut DeviceState) -> Self {
        let session = state.session();
        if let Ok(PhyPayload::Data(DataPayload::Encrypted(phy))) = lorawan_encoding::parser::parse(data) {
            let phy = phy.decrypt(Some(&session.nwk_skey().as_bytes().clone().into()), Some(&session.app_skey().as_bytes().clone().into()), state.fcnt_down());
        }

        todo!()
    }
}

pub struct JoinRequest([u8; 23]);

impl JoinRequest {
    pub fn new(credentials: &Credentials, dev_nonce: &DevNonce) -> Self {
        let app_key = credentials.app_key().as_bytes().clone().into();

        let mut phy = JoinRequestCreator::new();
        phy.set_app_eui(credentials.app_eui().as_bytes());
        phy.set_dev_eui(credentials.dev_eui().as_bytes());
        phy.set_dev_nonce(dev_nonce.as_bytes());
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

pub struct JoinAccept<'a>(EncryptedJoinAcceptPayload<&'a mut [u8], DefaultFactory>);

impl<'a> JoinAccept<'a> {
    pub fn from_data<E>(data: &'a mut [u8]) -> Result<Self, PacketError<E>> {
        let payload = EncryptedJoinAcceptPayload::new(data)?;
        Ok(JoinAccept(payload))
    }

    pub fn extract_state(self, credentials: &Credentials, dev_nonce: &DevNonce) -> DeviceState {
        let app_key = credentials.app_key().as_bytes().clone().into();
        let dev_nonce = dev_nonce.as_bytes().into();

        let payload = self.0.decrypt(&app_key);

        let mut bytes = [0; 4];
        bytes.copy_from_slice(payload.dev_addr().as_ref());
        let dev_addr = DevAddr::from_bytes(bytes);
        let nwk_skey = NwkSKey::from_bytes(payload.derive_newskey(&dev_nonce, &app_key).0);
        let app_skey = AppSKey::from_bytes(payload.derive_appskey(&dev_nonce, &app_key).0);

        let session = Session::new(dev_addr, nwk_skey, app_skey);

        let mut settings = Settings::default();

        settings.set_rx_delay(payload.rx_delay());

        let dl_settings = payload.dl_settings();
        let cf_list = payload
            .c_f_list()
            .map(|frequencies| frequencies
                .map(|frequency| {
                    let mut buf = [0; 4];
                    buf[1..3].copy_from_slice(frequency.as_ref());
                    Frequency::from_le_bytes(buf)
                })
            );
        let net_id = payload.net_id();

        DeviceState::new(session, settings)
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
