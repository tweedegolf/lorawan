use crate::radio::{DataRate, Frequency};
pub use crate::radio::region::eu868::EU868;

mod eu868;

pub trait Region: Sized + 'static {
    const JOIN_FREQUENCIES: &'static [Frequency];

    const TX_FREQUENCIES: &'static [Frequency];

    const RX1_FREQUENCIES: &'static [Frequency];

    const RX2_FREQUENCIES: &'static [Frequency];

    const DATA_RATES: &'static [DataRate<Self>];
}
