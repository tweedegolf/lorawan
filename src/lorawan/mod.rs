mod packet;

pub use packet::Packet;

// Everything below should eventually move to radio-hal

pub struct LoRaWANState {}

pub enum LoRaWANChannel {
    RX1,
    RX2,
}

pub struct LoRaWANInfo {
    rssi: i16,
    snr: i8,
}
