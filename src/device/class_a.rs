use core::fmt::Debug;
use core::ops::{Deref, DerefMut};

use embedded_hal::blocking::delay::DelayUs;
use radio::{Channel, State};
use radio::blocking::{BlockingError, BlockingReceive, BlockingTransmit};

use crate::constants::{RX1_DELAY, RX2_DELAY};
use crate::device::Device;
use crate::device::error::DeviceError;
use crate::lorawan::{LoRaWANChannel, LoRaWANInfo, LoRaWANState, MAX_PAYLOAD_SIZE, Packet, Session};

pub struct ClassA<R>(Device<R, Session>);

impl<R, E> ClassA<R>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaWANInfo, E> + State<State=LoRaWANState, Error=E> + Channel<Channel=LoRaWANChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Transmits `packet` and waits for an optional response.
    pub fn uplink(&mut self, packet: Packet) -> Result<Option<Packet>, DeviceError<E>> {
        let mut buf = [0; MAX_PAYLOAD_SIZE];
        // TODO: Encrypt
        match self.simple_uplink(packet.payload(), &mut buf, RX1_DELAY, RX2_DELAY) {
            // No response
            Ok((n, _)) => Ok(Some(Packet::new(buf, n))),
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

impl<R> Deref for ClassA<R> {
    type Target = Device<R, Session>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> DerefMut for ClassA<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
