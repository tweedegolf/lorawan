use core::fmt::Debug;

use crate::device::{Device, DeviceState};
use crate::device::error::DeviceError;
use crate::lorawan::{Downlink, RECEIVE_DELAY1, RECEIVE_DELAY2, Uplink};
use crate::radio::{LoRaInfo, LoRaRadio, Region};

pub struct ClassA<T, R>(Device<T, DeviceState<R>>);

impl<T, R, E> ClassA<T, R>
    where T: LoRaRadio<Error=E>,
          R: Region,
          E: Debug
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
    ) -> Result<Option<(usize, LoRaInfo)>, DeviceError<E>> {
        let uplink = Uplink::new(tx, 1, &mut self.0.state)?;
        let downlink = self.0.radio.lorawan_transmit(
            uplink.as_bytes(),
            rx,
            RECEIVE_DELAY1,
            RECEIVE_DELAY2,
            self.0.state.data_rate(),
        )?;
        match downlink {
            None => Ok(None),
            Some((n, info)) => {
                let downlink = Downlink::from_data(&mut rx[..n], &mut self.0.state)?;
                rx.copy_from_slice(downlink.as_bytes());
                Ok(Some((n, info)))
            }
        }
    }

    /// Returns the maximum size of a LoRaWAN packet using the current configuration. Note that the
    /// actual payload is shorter than this.
    pub fn packet_size_limit(&self) -> usize {
        R::packet_size_limit(self.0.state.data_rate())
    }
}

impl<T, R> From<Device<T, DeviceState<R>>> for ClassA<T, R> {
    fn from(device: Device<T, DeviceState<R>>) -> Self {
        ClassA(device)
    }
}
