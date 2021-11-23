use radio::modulation::lora::SpreadingFactor;

use crate::radio::{DataRate, Frequency, Region};

#[derive(Debug, PartialEq)]
pub struct EU868;

impl Region for EU868 {
    const JOIN_FREQUENCIES: &'static [Frequency] = &[
        // Default channels
        868_100_000,
        868_300_000,
        868_500_000,
    ];

    const TX_FREQUENCIES: &'static [Frequency] = Self::JOIN_FREQUENCIES;

    const RX1_FREQUENCIES: &'static [Frequency] = Self::TX_FREQUENCIES;

    const RX2_FREQUENCIES: &'static [Frequency] = &[869_525_000];

    const DATA_RATES: &'static [DataRate<Self>] = &[
        DataRate::new(SpreadingFactor::Sf12, 125_000),
        DataRate::new(SpreadingFactor::Sf11, 125_000),
        DataRate::new(SpreadingFactor::Sf10, 125_000),
        DataRate::new(SpreadingFactor::Sf9, 125_000),
        DataRate::new(SpreadingFactor::Sf8, 125_000),
        DataRate::new(SpreadingFactor::Sf7, 125_000),
        DataRate::new(SpreadingFactor::Sf7, 250_000),
    ];
}

impl DataRate<EU868> {
    pub fn max_payload_size(&self) -> usize {
        const MAX_PAYLOAD_SIZE: [usize; 7] = [51, 51, 51, 115, 222, 222, 222];
        let size = EU868::DATA_RATES
            .iter()
            .enumerate()
            .find(|(_, dr)| **dr == *self)
            .map(|(index, _)| MAX_PAYLOAD_SIZE[index]);
        match size {
            Some(size) => size,
            None => {
                #[cfg(feature = "defmt")]
                defmt::error!("Unsupported data rate: {:?}", self);
                panic!("Unsupported data rate: {:?}", self)
            }
        }
    }
}
