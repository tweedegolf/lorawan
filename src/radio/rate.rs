use core::marker::PhantomData;

use radio::modulation::lora::SpreadingFactor;

use crate::radio::Frequency;

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
