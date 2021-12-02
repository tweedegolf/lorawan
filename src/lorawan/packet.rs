#![allow(unused_variables)]

use core::marker::PhantomData;

use lorawan_encoding::creator::{DataPayloadCreator, JoinRequestCreator};
use lorawan_encoding::default_crypto::DefaultFactory;
use lorawan_encoding::maccommands::MacCommand;
use lorawan_encoding::parser;
use lorawan_encoding::parser::{DataHeader, DataPayload, EncryptedJoinAcceptPayload, FCtrl,
                               FRMPayload, MHDRAble, PhyPayload};

use crate::device::{Credentials, DeviceState, Session};
use crate::lorawan::{AppSKey, DevAddr, DevNonce, NwkSKey};
use crate::radio::{Hz, Region};

pub const MAX_PACKET_SIZE: usize = 242;

pub struct Uplink([u8; MAX_PACKET_SIZE], usize);

impl Uplink {
    pub fn new<R: Region, E>(
        payload: &[u8],
        port: u8,
        state: &mut DeviceState<R>,
    ) -> Result<Self, PacketError<E>> {
        let session = state.session();
        let nwk_skey = (*session.nwk_skey().as_bytes()).into();
        let app_skey = (*session.app_skey().as_bytes()).into();

        let mut phy = DataPayloadCreator::new();
        phy.set_confirmed(false);
        phy.set_dev_addr(session.dev_addr().as_bytes());
        phy.set_f_port(port);
        phy.set_fcnt(state.fcnt_up());
        phy.set_fctrl(&FCtrl::new(0b10000000, true));
        phy.set_uplink(true);
        let payload = phy.build(payload, &[], &nwk_skey, &app_skey)?;

        let mut buf = [0; MAX_PACKET_SIZE];
        buf[0..payload.len()].copy_from_slice(payload);

        state.increment_fcnt_up();

        Ok(Uplink(buf, payload.len()))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..self.1]
    }
}

pub struct Downlink([u8; MAX_PACKET_SIZE], usize);

impl Downlink {
    pub fn from_data<R: Region, E>(
        data: &mut [u8],
        state: &mut DeviceState<R>,
    ) -> Result<Self, PacketError<E>> {
        let session = state.session();
        let nwk_skey = (*session.nwk_skey().as_bytes()).into();
        let app_skey = (*session.app_skey().as_bytes()).into();

        if let PhyPayload::Data(DataPayload::Encrypted(phy)) = parser::parse(data)? {
            let phy = phy
                .decrypt_if_mic_ok(&nwk_skey, &app_skey, state.fcnt_down())
                .map_err(|_| PacketError::MICMismatch)?;

            let mhdr = phy.mhdr();
            let fhdr = phy.fhdr();
            let f_port = phy.f_port();
            let frm_payload = phy.frm_payload();
            match f_port {
                None => {
                    // No FPort, hence no payload
                    todo!()
                }
                Some(port) => {
                    match port {
                        0 => {
                            // MAC Commands
                            if let Ok(FRMPayload::MACCommands(macs)) = frm_payload {
                                for mac in macs.mac_commands() {
                                    match mac {
                                        MacCommand::LinkCheckAns(_) => {}
                                        MacCommand::LinkADRReq(_) => {}
                                        MacCommand::DutyCycleReq(_) => {}
                                        MacCommand::RXParamSetupReq(_) => {}
                                        MacCommand::DevStatusReq(_) => {}
                                        MacCommand::NewChannelReq(_) => {}
                                        MacCommand::RXTimingSetupReq(_) => {}
                                        _ => return Err(PacketError::InvalidDownlinkMACCommand)
                                    }
                                }
                                todo!()
                            } else {
                                Err(PacketError::InvalidMACPort)
                            }
                        }
                        port if (1..=223).contains(&port) => {
                            // Application data
                            todo!()
                        }
                        224 => {
                            // Reserved
                            todo!()
                        }
                        port => {
                            Err(PacketError::InvalidPort(port))
                        }
                    }
                }
            }
        } else {
            Err(PacketError::Encoding("", PhantomData))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..self.1]
    }
}

pub struct JoinRequest([u8; 23]);

impl JoinRequest {
    pub fn new(credentials: &Credentials, dev_nonce: &DevNonce) -> Self {
        let app_key = (*credentials.app_key().as_bytes()).into();

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

    pub fn extract_state<R: Region>(
        self,
        credentials: &Credentials,
        dev_nonce: &DevNonce,
    ) -> DeviceState<R> {
        let app_key = (*credentials.app_key().as_bytes()).into();
        let dev_nonce = dev_nonce.as_bytes().into();

        let payload = self.0.decrypt(&app_key);

        let mut bytes = [0; 4];
        bytes.copy_from_slice(payload.dev_addr().as_ref());
        let dev_addr = DevAddr::from_bytes(bytes);
        let nwk_skey = NwkSKey::from_bytes(payload.derive_newskey(&dev_nonce, &app_key).0);
        let app_skey = AppSKey::from_bytes(payload.derive_appskey(&dev_nonce, &app_key).0);

        let session = Session::new(dev_addr, nwk_skey, app_skey);

        // TODO: Save state
        let rx_delay = payload.rx_delay();
        let dl_settings = payload.dl_settings();
        let cf_list = payload
            .c_f_list()
            .map(|frequencies| frequencies
                .map(|frequency| {
                    let mut buf = [0; 4];
                    buf[1..3].copy_from_slice(frequency.as_ref());
                    Hz::from_le_bytes(buf)
                })
            );
        let net_id = payload.net_id();

        DeviceState::new(session)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketError<E> {
    InvalidDownlinkMACCommand,
    MICMismatch,
    InvalidPort(u8),
    InvalidMACPort,
    Encoding(&'static str, PhantomData<E>),
}

impl<E> From<&'static str> for PacketError<E> {
    fn from(error: &'static str) -> Self {
        PacketError::Encoding(error, PhantomData)
    }
}
