use crate::radio::{DataRate, Hz};
pub use crate::radio::region::eu868::EU868;

mod eu868;

pub trait Region: Sized + 'static {
    const JOIN_FREQUENCIES: &'static [Hz];

    const TX_FREQUENCIES: &'static [Hz];

    const RX1_FREQUENCIES: &'static [Hz];

    const RX2_FREQUENCIES: &'static [Hz];

    const DATA_RATES: &'static [DataRate<Self>];

    fn packet_size_limit(rate: &DataRate<Self>) -> usize;
}
