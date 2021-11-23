use core::marker::PhantomData;

use radio::modulation::lora::{CodingRate, LoRaChannel, SpreadingFactor};

use crate::radio::{Frequency, Region};

#[derive(Debug, PartialEq)]
pub struct DataRate<R> {
    spreading_factor: SpreadingFactor,
    frequency: Frequency,
    _region: PhantomData<R>,
}

impl<R> DataRate<R> {
    pub(in crate::radio) const fn new(
        spreading_factor: SpreadingFactor,
        frequency: Frequency
    ) -> Self {
        DataRate {
            spreading_factor,
            frequency,
            _region: PhantomData,
        }
    }
}

impl<R: Region> DataRate<R> {
    pub fn rx1(&self) -> LoRaChannel {
        // TODO: Pick channel at random
        LoRaChannel {
            freq_khz: R::RX1_FREQUENCIES[0],
            bw_khz: self.frequency as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }

    pub fn rx2(&self) -> LoRaChannel {
        // TODO: Pick channel at random
        LoRaChannel {
            freq_khz: R::RX2_FREQUENCIES[0],
            bw_khz: self.frequency as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }
}

impl<R: Region> Default for DataRate<R> {
    /// Returns the appropriate DR0 for this region.
    fn default() -> Self {
        R::DATA_RATES[0].clone()
    }
}

impl<R> Clone for DataRate<R> {
    fn clone(&self) -> Self {
        DataRate {
            spreading_factor: self.spreading_factor.clone(),
            frequency: self.frequency.clone(),
            _region: PhantomData,
        }
    }
}
