pub use crate::radio::region::eu868::EU868;
use crate::radio::{DataRate, Hz, RadioError};

mod eu868;

pub trait Region: Sized + 'static {
    const JOIN_FREQUENCIES: &'static [Hz];

    const TX_FREQUENCIES: &'static [Hz];

    const RX1_FREQUENCIES: &'static [Hz];

    const RX2_FREQUENCY: Hz;

    const DATA_RATES: &'static [DataRate<Self>];

    fn get_data_rate<'a, ERR>(index: usize) -> Result<&'a DataRate<Self>, RadioError<ERR>> {
        Self::DATA_RATES
            .get(index)
            .ok_or(RadioError::UnsupportedDataRate)
    }
}
