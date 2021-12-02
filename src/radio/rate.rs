use core::marker::PhantomData;

use radio::modulation::lora::{CodingRate, LoRaChannel, SpreadingFactor};

use crate::radio::Region;

pub type Hz = u32;

#[derive(Debug, PartialEq)]
pub struct DataRate<R> {
    spreading_factor: SpreadingFactor,
    frequency: Hz,
    _region: PhantomData<R>,
}

impl<R> DataRate<R> {
    pub(in crate::radio) const fn new(spreading_factor: SpreadingFactor, frequency: Hz) -> Self {
        DataRate {
            spreading_factor,
            frequency,
            _region: PhantomData,
        }
    }
}

impl<R: Region> DataRate<R> {
    pub fn tx(&self, noise: usize) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::TX_FREQUENCIES[noise % R::TX_FREQUENCIES.len()] / 1000,
            bw_khz: (self.frequency / 1000) as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }

    pub fn rx1(&self, noise: usize) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::RX1_FREQUENCIES[noise % R::RX1_FREQUENCIES.len()] / 1000,
            bw_khz: (self.frequency / 1000) as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }

    pub fn rx2(&self, noise: usize) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::RX2_FREQUENCIES[noise % R::RX2_FREQUENCIES.len()] / 1000,
            bw_khz: (self.frequency / 1000) as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }
}

impl<R: Region> Default for DataRate<R> {
    /// Returns DR0 for this region.
    fn default() -> Self {
        R::DATA_RATES[0].clone()
    }
}

impl<R> Clone for DataRate<R> {
    fn clone(&self) -> Self {
        DataRate {
            spreading_factor: self.spreading_factor,
            frequency: self.frequency,
            _region: PhantomData,
        }
    }
}
