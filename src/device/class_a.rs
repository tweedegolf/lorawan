use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Channel, State};
use radio::blocking::{BlockingError, BlockingReceive, BlockingTransmit};

use crate::device::{Device, Session};
use crate::device::error::DeviceError;
use crate::radio::{LoRaWANChannel, LoRaWANInfo, LoRaWANState, RX1_DELAY, RX2_DELAY};

pub struct ClassA<R>(Device<R, Session>);

impl<R, E> ClassA<R>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaWANInfo, E> + State<State=LoRaWANState, Error=E> + Channel<Channel=LoRaWANChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<Option<(usize, LoRaWANInfo)>, DeviceError<E>> {
        // TODO: Encrypt
        match self.0.simple_transmit(tx, rx, RX1_DELAY, RX2_DELAY) {
            Ok((n, info)) => {
                // TODO: decrypt
                Ok(Some((n, info)))
            }
            Err(DeviceError::Blocking(BlockingError::Timeout)) => Ok(None),
            Err(error) => Err(error)
        }
    }
}

impl<R> From<Device<R, Session>> for ClassA<R> {
    fn from(device: Device<R, Session>) -> Self {
        ClassA(device)
    }
}
