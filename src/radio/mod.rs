use core::fmt::Debug;

use radio::{RadioState, ReceiveInfo};
use radio::modulation::lora::{CodingRate, LoRaChannel};

pub use crate::radio::rate::*;
pub use crate::radio::region::*;

mod rate;
mod region;

pub type Frequency = u32;

#[derive(Debug)]
pub struct LoRaState<R> {
    rx_delay: u8,
    data_rate: DataRate<R>,
}

impl<R: Region> LoRaState<R> {
    pub fn set_rx_delay(&mut self, rx_delay: u8) -> &mut Self {
        self.rx_delay = rx_delay;
        self
    }

    pub fn rx1(&self) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::RX1_FREQUENCIES[0],
            bw_khz: *self.data_rate.bandwidth() as u16,
            sf: *self.data_rate.spreading_factor(),
            cr: CodingRate::Cr4_5,
        }
    }

    pub fn rx2(&self) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::RX2_FREQUENCIES[0],
            bw_khz: *self.data_rate.bandwidth() as u16,
            sf: *self.data_rate.spreading_factor(),
            cr: CodingRate::Cr4_5,
        }
    }
}

impl<R: Region> Default for LoRaState<R> {
    fn default() -> Self {
        LoRaState {
            rx_delay: 0,
            data_rate: R::DATA_RATES[0].clone(),
        }
    }
}

impl<R: Debug> RadioState for LoRaState<R> {
    fn idle() -> Self {
        todo!()
    }

    fn sleep() -> Self {
        todo!()
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
