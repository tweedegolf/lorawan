pub use crate::radio::config::*;
pub use crate::radio::constants::*;

mod config;
mod constants;

pub struct LoRaState {}

pub enum LoRaChannel {
    RX1,
    RX2,
}

pub struct LoRaInfo {
    rssi: i16,
    snr: i8,
}
