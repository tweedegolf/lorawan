pub use crate::radio::config::*;
pub use crate::radio::constants::*;

mod config;
mod constants;

pub struct LoRaWANState {}

pub enum LoRaWANChannel {
    RX1,
    RX2,
}

pub struct LoRaWANInfo {
    rssi: i16,
    snr: i8,
}
