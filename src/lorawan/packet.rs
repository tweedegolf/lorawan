use crate::device::{Credentials, DeviceState};
use crate::lorawan::{DevNonce, MType, Major, Request, Write, MHDR};
use crate::radio::Region;
use aes::Aes128;
use cmac::{Cmac, Mac, NewMac};

pub const MAX_PACKET_SIZE: usize = 242;

#[derive(Debug)]
pub struct Uplink {
    bytes: [u8; MAX_PACKET_SIZE],
    size: usize,
}

impl Uplink {
    pub fn new<R: Region>(
        payload: &[u8],
        port: u8,
        state: &mut DeviceState<R>,
    ) -> Result<Self, PacketError> {
        // let session = state.session();
        // let nwk_skey = (*session.nwk_s_enc_key().as_bytes()).into();
        // let app_skey = (*session.app_skey().as_bytes()).into();
        //
        // let mut phy = DataPayloadCreator::new();
        // phy.set_confirmed(false);
        // phy.set_dev_addr(session.dev_addr().as_bytes());
        // phy.set_f_port(port);
        // phy.set_fcnt(state.fcnt_up());
        // phy.set_fctrl(&FCtrl::new(0b10000000, true));
        // phy.set_uplink(true);
        // let payload = phy.build(payload, &[], &nwk_skey, &app_skey)?;
        //
        // let mut buf = [0; MAX_PACKET_SIZE];
        // buf[0..payload.len()].copy_from_slice(payload);
        //
        // state.increment_fcnt_up();
        //
        // Ok(Uplink(buf, payload.len()))
        todo!()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.size]
    }
}

pub struct Downlink {
    bytes: [u8; MAX_PACKET_SIZE],
    size: usize,
}

impl Downlink {
    pub fn from_data<R: Region>(
        data: &mut [u8],
        state: &mut DeviceState<R>,
    ) -> Result<Self, PacketError> {
        // let session = state.session();
        // let nwk_skey = (*session.nwk_s_enc_key().as_bytes()).into();
        // let app_skey = (*session.app_skey().as_bytes()).into();
        //
        // if let PhyPayload::Data(DataPayload::Encrypted(phy)) = parser::parse(data)? {
        //     let phy = phy
        //         .decrypt_if_mic_ok(&nwk_skey, &app_skey, state.fcnt_down())
        //         .map_err(|_| PacketError::MICMismatch)?;
        //
        //     let mhdr = phy.mhdr();
        //     let fhdr = phy.fhdr();
        //     let f_port = phy.f_port();
        //     let frm_payload = phy.frm_payload();
        //     match f_port {
        //         None => {
        //             // No FPort, hence no payload
        //             todo!()
        //         }
        //         Some(port) => {
        //             match port {
        //                 0 => {
        //                     // MAC Commands
        //                     if let Ok(FRMPayload::MACCommands(macs)) = frm_payload {
        //                         for mac in macs.mac_commands() {
        //                             match mac {
        //                                 MacCommand::LinkCheckAns(_) => {}
        //                                 MacCommand::LinkADRReq(_) => {}
        //                                 MacCommand::DutyCycleReq(_) => {}
        //                                 MacCommand::RXParamSetupReq(_) => {}
        //                                 MacCommand::DevStatusReq(_) => {}
        //                                 MacCommand::NewChannelReq(_) => {}
        //                                 MacCommand::RXTimingSetupReq(_) => {}
        //                                 _ => return Err(PacketError::InvalidDownlinkMACCommand),
        //                             }
        //                         }
        //                         todo!()
        //                     } else {
        //                         Err(PacketError::InvalidMACPort)
        //                     }
        //                 }
        //                 port if (1..=223).contains(&port) => {
        //                     // Application data
        //                     todo!()
        //                 }
        //                 224 => {
        //                     // Reserved
        //                     todo!()
        //                 }
        //                 port => Err(PacketError::InvalidPort(port)),
        //             }
        //         }
        //     }
        // } else {
        //     Err(PacketError::Encoding(""))
        // }
        todo!()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.size]
    }
}

pub struct JoinRequest([u8; 23]);

impl JoinRequest {
    pub fn new(credentials: &Credentials, dev_nonce: &DevNonce) -> JoinRequest {
        let mhdr = MHDR::new(MType::JoinRequest, Major::LoRaWANR1);
        // TODO: Use references
        let request = Request::Join(
            credentials.join_eui().clone(),
            credentials.dev_eui().clone(),
            dev_nonce.clone(),
        );

        let mut bytes = [0; 23];
        let mut offset = 0;
        offset += mhdr.write_to(&mut bytes);
        offset += request.write_to(&mut bytes[offset..]);
        let mut mac = Cmac::<Aes128>::new_from_slice(credentials.nwk_key().as_bytes())
            .expect("failed to create MIC");
        mac.update(&bytes[..offset]);
        bytes[offset..].copy_from_slice(&mac.finalize().into_bytes().as_slice()[..4]);

        JoinRequest(bytes)
    }

    pub fn payload(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketError {}
