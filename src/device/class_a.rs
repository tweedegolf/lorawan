use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Busy, Channel, Receive, State, Transmit};
use radio::blocking::BlockingError;
use radio::modulation::lora::LoRaChannel;

use crate::device::{Device, DeviceState};
use crate::device::error::DeviceError;
use crate::lorawan::{Downlink, RECEIVE_DELAY1, RECEIVE_DELAY2, Uplink};
use crate::radio::{LoRaInfo, LoRaState, Region};

pub struct ClassA<R, C>(Device<R, C, DeviceState>);

impl<R, C, E> ClassA<R, C>
    where R: Transmit<Error=E>,
          R: Receive<Error=E, Info=LoRaInfo>,
          R: State<State=LoRaState, Error=E>,
          R: Channel<Channel=LoRaChannel, Error=E>,
          R: Busy<Error=E>,
          R: DelayUs<u32>,
          C: Region,
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

impl<R, C> From<Device<R, C, DeviceState>> for ClassA<R, C> {
    fn from(device: Device<R, C, DeviceState>) -> Self {
        ClassA(device)
    }
}
