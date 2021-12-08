use core::marker::PhantomData;

use aes::{Aes128, Block, BlockEncrypt, NewBlockCipher};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::device::{Credentials, DeviceState};
use crate::lorawan::PacketError;

pub const MAX_F_OPTS_LENGTH: usize = 16;
pub const MAX_FRM_PAYLOAD_LENGTH: usize = 242;

/// Application Unique Identifier
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct JoinEui([u8; 8]);

impl JoinEui {
    pub const fn new(eui: u64) -> Self {
        JoinEui(eui.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

/// Device Unique Identifier
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DevEui([u8; 8]);

impl DevEui {
    pub const fn new(eui: u64) -> Self {
        DevEui(eui.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.0
    }
}

/// Application Key
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AppKey([u8; 16]);

impl AppKey {
    pub const fn new(key: u128) -> Self {
        AppKey(key.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

/// Device Address
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DevAddr([u8; 4]);

impl DevAddr {
    pub const fn new(addr: u32) -> Self {
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NwkSEncKey([u8; 16]);

impl NwkSEncKey {
    pub const fn new(key: u128) -> Self {
        NwkSEncKey(key.to_le_bytes())
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        NwkSEncKey(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

/// Application Session Key
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AppSKey([u8; 16]);

impl AppSKey {
    pub const fn new(key: u128) -> Self {
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DevNonce([u8; 2]);

impl DevNonce {
    pub const fn new(nonce: u16) -> Self {
        DevNonce(nonce.to_le_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 2] {
        &self.0
    }
}

/// Join-Server Nonce
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct JoinNonce([u8; 3]);

impl JoinNonce {
    pub fn as_bytes(&self) -> &[u8; 3] {
        &self.0
    }
}

/// Marker type for uplinks
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UL;

/// Marker type for downlinks
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DL;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Encrypted<K, T> {
    inner: T,
    _key: PhantomData<K>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyPayload<L, R> {
    Data(MHDR, MACPayload<L>, MIC),
    Request(MHDR, Request, MIC),
    Accept(MHDR, JoinAccept<R>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MHDR(u8);

impl MHDR {
    pub fn mtype(&self) -> MType {
        MType::from_u8(self.0 >> 5).expect("unsupported message type")
    }

    pub fn major(&self) -> Major {
        Major::from_u8(self.0 & 0b11).expect("unsupported major version")
    }
}

#[derive(Clone, Debug, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MType {
    JoinRequest = 0b000,
    JoinAccept = 0b001,
    UnconfirmedDataUp = 0b010,
    UnconfirmedDataDown = 0b011,
    ConfirmedDataUp = 0b100,
    ConfirmedDataDown = 0b101,
    RejoinRequest = 0b110,
    Proprietary = 0b111,
}

#[derive(Clone, Debug, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Major {
    LoRaWANR1 = 0b00,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MACPayload<L>(FHDR<L>, Option<FContent<L>>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FHDR<L>(DevAddr, FCtrl<L>, FCnt, Encrypted<NwkSEncKey, FOpts<L>>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FCtrl<L>(u8, PhantomData<L>);

impl<L> FCtrl<L> {
    pub fn adr(&self) -> bool {
        self.0 >> 7 == 1
    }

    pub fn ack(&self) -> bool {
        (self.0 >> 5) & 1 == 1
    }

    pub fn f_opts_len(&self) -> u8 {
        self.0 & 0b1111
    }
}

impl FCtrl<UL> {
    pub fn adr_ack_req(&self) -> bool {
        (self.0 >> 6) & 1 == 1
    }

    pub fn class_b(&self) -> bool {
        (self.0 >> 4) & 1 == 1
    }
}

impl FCtrl<DL> {
    pub fn f_pending(&self) -> bool {
        (self.0 >> 4) & 1 == 1
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FOpts<L>([u8; MAX_F_OPTS_LENGTH], PhantomData<L>);

impl<L> FOpts<L> {
    fn encrypt_with<R>(
        mut self,
        a: [u8; 16],
        state: DeviceState<R>,
    ) -> Encrypted<NwkSEncKey, FOpts<L>> {
        let nwk_s_enc_key = state.session().nwk_s_enc_key().as_bytes();
        let k = Aes128::new_from_slice(nwk_s_enc_key).expect("invalid key length");

        let mut block = Block::from(a);
        k.encrypt_block(&mut block);

        for i in 0..MAX_F_OPTS_LENGTH {
            self.0[i] ^= block[i];
        }

        Encrypted {
            inner: self,
            _key: Default::default(),
        }
    }
}

impl FOpts<UL> {
    pub fn encrypt<R>(self, state: DeviceState<R>) -> Encrypted<NwkSEncKey, FOpts<UL>> {
        let dev_addr = state.session().dev_addr().as_bytes();
        let f_cnt_up = state.fcnt_up().to_le_bytes();

        let a = [
            0x01,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            dev_addr[0],
            dev_addr[1],
            dev_addr[2],
            dev_addr[3],
            f_cnt_up[0],
            f_cnt_up[1],
            f_cnt_up[2],
            f_cnt_up[3],
            0x00,
            0x00,
        ];

        self.encrypt_with(a, state)
    }
}

impl FOpts<DL> {
    pub fn encrypt<R>(self, state: DeviceState<R>) -> Encrypted<NwkSEncKey, FOpts<DL>> {
        let dev_addr = state.session().dev_addr().as_bytes();
        let n_f_cnt_down = state.fcnt_down().to_le_bytes();

        let a = [
            0x01,
            0x00,
            0x00,
            0x00,
            0x00,
            0x01,
            dev_addr[0],
            dev_addr[1],
            dev_addr[2],
            dev_addr[3],
            n_f_cnt_down[0],
            n_f_cnt_down[1],
            n_f_cnt_down[2],
            n_f_cnt_down[3],
            0x00,
            0x00,
        ];

        self.encrypt_with(a, state)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FCnt([u8; 2]);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FContent<L> {
    Commands(Encrypted<NwkSEncKey, FRMPayload<L>>),
    Message(FPort, Encrypted<AppSKey, FRMPayload<L>>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FPort {
    Application(u8),
    Test,
}

impl TryFrom<u8> for FPort {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            n if (1..=223).contains(&n) => Ok(FPort::Application(n)),
            224 => Ok(FPort::Test),
            _ => Err("invalid port"),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FRMPayload<L>([u8; MAX_FRM_PAYLOAD_LENGTH], PhantomData<L>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Request {
    Join(JoinEui, DevEui, DevNonce),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct JoinAccept<R>(
    JoinNonce,
    HomeNetId,
    DevAddr,
    DlSettings,
    RxDelay,
    Option<CFList<R>>,
);

impl<R> JoinAccept<R> {
    pub fn from_data(data: &mut [u8]) -> Result<Self, PacketError> {
        // let payload = EncryptedJoinAcceptPayload::new(data)?;
        // Ok(JoinAccept(payload))
        todo!()
    }

    pub fn extract_state(self, credentials: &Credentials, dev_nonce: &DevNonce) -> DeviceState<R> {
        // let app_key = (*credentials.app_key().as_bytes()).into();
        // let dev_nonce = dev_nonce.as_bytes().into();
        //
        // let payload = self.0.decrypt(&app_key);
        //
        // let mut bytes = [0; 4];
        // bytes.copy_from_slice(payload.dev_addr().as_ref());
        // let dev_addr = DevAddr::from_bytes(bytes);
        // let nwk_skey = NwkSEncKey::from_bytes(payload.derive_newskey(&dev_nonce, &app_key).0);
        // let app_skey = AppSKey::from_bytes(payload.derive_appskey(&dev_nonce, &app_key).0);
        //
        // let session = Session::new(dev_addr, nwk_skey, app_skey);
        //
        // // TODO: Save state
        // let rx_delay = payload.rx_delay();
        // let dl_settings = payload.dl_settings();
        // let cf_list = payload.c_f_list().map(|frequencies| {
        //     frequencies.map(|frequency| {
        //         let mut buf = [0; 4];
        //         buf[1..3].copy_from_slice(frequency.as_ref());
        //         Hz::from_le_bytes(buf)
        //     })
        // });
        // let net_id = payload.net_id();
        //
        // DeviceState::new(session)
        todo!()
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HomeNetId([u8; 3]);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DlSettings(u8);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RxDelay(u8);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CFList<R>(PhantomData<R>);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MIC([u8; 4]);
