use radio::modulation::lora::SpreadingFactor;

use crate::radio::{DataRate, Hz, Region};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EU868;

impl Region for EU868 {
    const JOIN_FREQUENCIES: &'static [Hz] = &[868_100_000, 868_300_000, 868_500_000];

    const TX_FREQUENCIES: &'static [Hz] = Self::JOIN_FREQUENCIES;

    const RX1_FREQUENCIES: &'static [Hz] = Self::TX_FREQUENCIES;

    const RX2_FREQUENCY: Hz = 869_525_000;

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
