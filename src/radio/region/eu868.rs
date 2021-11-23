use radio::modulation::lora::SpreadingFactor;

use crate::radio::{DataRate, Frequency, Region};

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
        match (self.spreading_factor(), self.bandwidth()) {
            (SpreadingFactor::Sf12, 125_000) => 51,
            (SpreadingFactor::Sf11, 125_000) => 51,
            (SpreadingFactor::Sf10, 125_000) => 51,
            (SpreadingFactor::Sf9, 125_000) => 115,
            (SpreadingFactor::Sf8, 125_000) => 222,
            (SpreadingFactor::Sf7, 125_000) => 222,
            (SpreadingFactor::Sf7, 250_000) => 222,
            _ => panic!("invalid data rate for EU868"),
        }
    }
}
