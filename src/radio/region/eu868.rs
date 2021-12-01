use radio::modulation::lora::SpreadingFactor;

use crate::radio::{DataRate, Hz, Region};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EU868;

impl Region for EU868 {
    const JOIN_FREQUENCIES: &'static [Hz] = &[
        // Default channels
        868_100_000,
        868_300_000,
        868_500_000,
    ];

    const TX_FREQUENCIES: &'static [Hz] = Self::JOIN_FREQUENCIES;

    const RX1_FREQUENCIES: &'static [Hz] = Self::TX_FREQUENCIES;

    const RX2_FREQUENCIES: &'static [Hz] = &[869_525_000];

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
            .unwrap_or_else(|| panic!("Unsupported data rate: {:?}", rate))
    }
}
