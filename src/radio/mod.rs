//! Compatible associated types for `radio`'s traits. These should eventually move to the `radio`
//! crate.

use radio::ReceiveInfo;

pub use crate::radio::config::*;

mod config;

#[derive(Debug)]
pub enum LoRaChannel {
    RX1,
    RX2,
}

#[derive(Debug, Default)]
pub struct LoRaInfo {
    rssi: i16,
    snr: i8,
}

impl ReceiveInfo for LoRaInfo {
    fn rssi(&self) -> i16 {
        self.rssi
    }
}
