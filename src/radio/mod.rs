use radio::{RadioState, ReceiveInfo};

pub use crate::radio::config::*;
pub use crate::radio::region::*;

mod config;
mod region;

pub type Frequency = u32;

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
