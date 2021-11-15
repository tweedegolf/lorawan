//! Compatible associated types for `radio`'s traits. These should eventually move to the `radio`
//! crate.

use radio::{RadioState, ReceiveInfo};

pub use crate::radio::config::*;

mod config;

#[derive(Debug)]
pub struct LoRaState {
    rx_delay: u8,
}

impl LoRaState {
    pub fn set_rx_delay(&mut self, rx_delay: u8) -> &mut Self {
        self.rx_delay = rx_delay;
        self
    }
}

impl RadioState for LoRaState {
    fn idle() -> Self {
        todo!()
    }

    fn sleep() -> Self {
        todo!()
    }
}

impl Default for LoRaState {
    fn default() -> Self {
        LoRaState {
            rx_delay: 0
        }
    }
}

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
