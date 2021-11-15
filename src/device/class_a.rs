use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Busy, Channel, Receive, Transmit};
use radio::blocking::BlockingError;

use crate::device::{Device, DeviceState};
use crate::device::error::DeviceError;
use crate::lorawan::{Downlink, RECEIVE_DELAY1, RECEIVE_DELAY2, Uplink};
use crate::radio::{LoRaChannel, LoRaInfo};

pub struct ClassA<R>(Device<R, DeviceState>);

impl<R, E> ClassA<R>
    where R: Transmit<Error=E> + Receive<Error=E, Info=LoRaInfo> + Channel<Channel=LoRaChannel, Error=E> + Busy<Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<Option<(usize, LoRaInfo)>, DeviceError<E>> {
        let uplink = Uplink::new(tx, 1, &mut self.0.state)?;
        match self.0.simple_transmit(uplink.as_bytes(), rx, RECEIVE_DELAY1, RECEIVE_DELAY2) {
            Ok((n, info)) => {
                let downlink = Downlink::from_data(rx, &mut self.0.state)?;
                rx.copy_from_slice(downlink.as_bytes());
                Ok(Some((n, info)))
            }
            Err(DeviceError::Blocking(BlockingError::Timeout)) => Ok(None),
            Err(error) => Err(error)
        }
    }
}

impl<R> From<Device<R, DeviceState>> for ClassA<R> {
    fn from(device: Device<R, DeviceState>) -> Self {
        ClassA(device)
    }
}
