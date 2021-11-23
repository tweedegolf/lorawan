use core::marker::PhantomData;

use radio::modulation::lora::{LoRaChannel, SpreadingFactor};

use crate::radio::Frequency;
use crate::radio::region::Region;

#[derive(Debug)]
pub struct Config<R> {
    region: R,
    data_rate: DataRate<R>,
}

impl<R: Region> Config<R> {
    pub fn new(region: R) -> Self {
        Config {
            region,
            data_rate: R::DATA_RATES[0].clone(),
        }
    }

    pub fn rx1(&self) -> LoRaChannel {
        todo!()
    }

    pub fn rx2(&self) -> LoRaChannel {
        todo!()
    }
}

#[derive(Debug)]
pub struct DataRate<R> {
    spreading_factor: SpreadingFactor,
    frequency: Frequency,
    _region: PhantomData<R>,
}

impl<R> DataRate<R> {
    pub const fn new(spreading_factor: SpreadingFactor, frequency: Frequency) -> Self {
        DataRate {
            spreading_factor,
            frequency,
            _region: PhantomData,
        }
    }

    pub fn spreading_factor(&self) -> &SpreadingFactor {
        &self.spreading_factor
    }

    pub fn bandwidth(&self) -> &Frequency {
        &self.frequency
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
