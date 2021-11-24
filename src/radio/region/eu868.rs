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

    fn packet_size_limit(rate: &DataRate<Self>) -> usize {
        const PACKET_SIZE_LIMITS: [usize; 7] = [51, 51, 51, 115, 222, 222, 222];
        Self::DATA_RATES
            .iter()
            .enumerate()
            .find_map(|(index, other)| (*other == *rate)
                .then(|| PACKET_SIZE_LIMITS[index]))
            .unwrap_or_else(|| {
                #[cfg(feature = "defmt")]
                defmt::error!("Unsupported data rate: {:?}", rate);
                panic!("Unsupported data rate: {:?}", rate)
            })
    }
}
