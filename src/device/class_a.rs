use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Channel, State};
use radio::blocking::{BlockingError, BlockingReceive, BlockingTransmit};

use crate::device::{Device, DeviceState};
use crate::device::error::DeviceError;
use crate::lorawan::{RECEIVE_DELAY1, RECEIVE_DELAY2};
use crate::radio::{LoRaChannel, LoRaInfo, LoRaState};

pub struct ClassA<R>(Device<R, DeviceState>);

impl<R, E> ClassA<R>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaInfo, E> + State<State=LoRaState, Error=E> + Channel<Channel=LoRaChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<Option<(usize, LoRaInfo)>, DeviceError<E>> {
        // TODO: Encrypt
        match self.0.simple_transmit(tx, rx, RECEIVE_DELAY1, RECEIVE_DELAY2) {
            Ok((n, info)) => {
                // TODO: decrypt
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
