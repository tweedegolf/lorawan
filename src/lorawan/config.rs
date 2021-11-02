use crate::lorawan::LoRaWANChannel;

type Frequency = u32;

pub struct Config {
    region: Region,
    data_rate: DataRate,
}

impl Config {
    pub fn new(region: Region, dr: usize) -> Self {
        let data_rate = region.datarates()[dr];
        Config {
            region,
            data_rate,
        }
    }
}

pub enum Region {
    EU868
}

impl Region {
    const fn join_frequencies(&self) -> &[Frequency] {
        match self {
            Region::EU868 => &[
                // Default channels
                868_100_000,
                868_300_000,
                868_500_000,
            ]
        }
    }

    const fn uplink_frequencies(&self) -> &[Frequency] {
        match self {
            Region::EU868 => &[
                // Default channels
                868_100_000,
                868_300_000,
                868_500_000,

                // The Things Network also supports these channels
                #[cfg(feature = "ttn")]
                    867_100_000,
                #[cfg(feature = "ttn")]
                    867_300_000,
                #[cfg(feature = "ttn")]
                    867_500_000,
                #[cfg(feature = "ttn")]
                    867_700_000,
                #[cfg(feature = "ttn")]
                    867_900_000,

                // FSK
                // 868_800_000
            ]
        }
    }

    const fn downlink_frequencies(&self, channel: &LoRaWANChannel) -> &[Frequency] {
        match self {
            Region::EU868 => match channel {
                LoRaWANChannel::RX1 => self.uplink_frequencies(),
                LoRaWANChannel::RX2 => &[869_525_000]
            }
        }
    }

    const fn datarates(&self) -> &[DataRate] {
        match self {
            Region::EU868 => &[
                DataRate::SF12_125,
                DataRate::SF11_125,
                DataRate::SF10_125,
                DataRate::SF9_125,
                DataRate::SF8_125,
                DataRate::SF7_125,
                DataRate::SF7_250
            ]
        }
    }
}

pub enum SpreadingFactor {
    SF7,
    SF8,
    SF9,
    SF10,
    SF11,
    SF12,
}

#[derive(Copy, Clone)]
enum DataRate {
    SF12_125,
    SF11_125,
    SF10_125,
    SF9_125,
    SF8_125,
    SF7_125,
    SF7_250,
}

impl DataRate {
    fn spreading_factor(&self) -> SpreadingFactor {
        match self {
            DataRate::SF12_125 => SpreadingFactor::SF12,
            DataRate::SF11_125 => SpreadingFactor::SF11,
            DataRate::SF10_125 => SpreadingFactor::SF10,
            DataRate::SF9_125 => SpreadingFactor::SF9,
            DataRate::SF8_125 => SpreadingFactor::SF8,
            DataRate::SF7_125 => SpreadingFactor::SF7,
            DataRate::SF7_250 => SpreadingFactor::SF7
        }
    }

    fn bandwidth(&self) -> Frequency {
        match self {
            DataRate::SF12_125 => 125_000,
            DataRate::SF11_125 => 125_000,
            DataRate::SF10_125 => 125_000,
            DataRate::SF9_125 => 125_000,
            DataRate::SF8_125 => 125_000,
            DataRate::SF7_125 => 125_000,
            DataRate::SF7_250 => 250_000
        }
    }

    fn max_payload_size(&self) -> usize {
        match self {
            DataRate::SF12_125 => 51,
            DataRate::SF11_125 => 51,
            DataRate::SF10_125 => 51,
            DataRate::SF9_125 => 115,
            DataRate::SF8_125 => 242,
            DataRate::SF7_125 => 242,
            DataRate::SF7_250 => 242
        }
    }
}
