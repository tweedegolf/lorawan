pub use crate::radio::config::*;

mod config;

pub struct LoRaState {}

pub enum LoRaChannel {
    RX1,
    RX2,
}

pub struct LoRaInfo {
    rssi: i16,
    snr: i8,
}
