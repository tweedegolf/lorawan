use core::fmt::Debug;
use core::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Channel, State};
use radio::blocking::{BlockingError, BlockingOptions, BlockingReceive, BlockingTransmit};

pub use class_a::ClassA;

use crate::constants::{JOIN_ACCEPT_1_DELAY, JOIN_ACCEPT_2_DELAY};
use crate::device::error::DeviceError;
use crate::lorawan::{Credentials, JoinAccept, JoinRequest, LoRaWANChannel, LoRaWANInfo, LoRaWANState, MAX_PAYLOAD_SIZE, Session};

mod class_a;
mod error;

const INTERVAL: Duration = Duration::from_millis(100);
const TIMEOUT: Duration = Duration::from_millis(200);

const BLOCKING_OPTIONS: BlockingOptions = BlockingOptions {
    poll_interval: INTERVAL,
    timeout: TIMEOUT,
};

/// Represents a generic LoRaWAN device. The state can be either [Credentials] for
/// devices that have not joined a network, or [Session] for devices that have.
pub struct Device<R, S> {
    radio: R,
    state: S,
}

impl<R, E> Device<R, Credentials>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaWANInfo, E> + State<State=LoRaWANState, Error=E> + Channel<Channel=LoRaWANChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Creates a new LoRaWAN device through Over-The-Air-Activation. It must join a network with
    /// [join] before it can be used. Alternatively, an ABP-joined device can be constructed with
    /// [new_abp].
    pub fn new_otaa(radio: R, credentials: Credentials) -> Self {
        Device {
            radio,
            state: credentials,
        }
    }

    /// Attempts to join this device to a network.
    pub fn join(mut self) -> Result<Device<R, Session>, DeviceError<E>> {
        let dev_nonce = 37;

        let join_request = JoinRequest::new(&self.state, dev_nonce);
        let mut buf = [0; MAX_PAYLOAD_SIZE];

        let _ = self.simple_uplink(join_request.payload(), &mut buf, JOIN_ACCEPT_1_DELAY, JOIN_ACCEPT_2_DELAY)?;

        let join_accept = JoinAccept::new(buf)?;
        let session = join_accept.extract(&self.state, dev_nonce);

        let device = Device {
            radio: self.radio,
            state: session,
        };

        Ok(device)
    }
}

impl<R, E> Device<R, Session>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaWANInfo, E> + State<State=LoRaWANState, Error=E> + Channel<Channel=LoRaWANChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Creates a joined device through Activation By Personalization.
    pub fn new_abp(radio: R, session: Session) -> Self {
        Device {
            radio,
            state: session,
        }
    }

    /// Configures this device to have class A behavior: listening for downlinks only after
    /// transmitting an uplink.
    pub fn into_class_a(self) -> ClassA<R> {
        self.into()
    }
}

impl<R, E, S> Device<R, S>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaWANInfo, E> + State<State=LoRaWANState, Error=E> + Channel<Channel=LoRaWANChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    pub(in crate::device) fn simple_uplink(&mut self, tx: &[u8], rx: &mut [u8], delay_1: Duration, delay_2: Duration) -> Result<(usize, LoRaWANInfo), DeviceError<E>> {
        self.radio.do_transmit(tx, BLOCKING_OPTIONS)?;

        self.radio.set_channel(&LoRaWANChannel::RX1)?;
        self.radio.delay_us(delay_1.as_micros() as u32);

        match self.radio.do_receive(rx, BLOCKING_OPTIONS) {
            Err(BlockingError::Timeout) => {
                self.radio.set_channel(&LoRaWANChannel::RX2)?;
                self.radio.delay_us((delay_2 - delay_1 - TIMEOUT).as_micros() as u32);

                self.radio.do_receive(rx, BLOCKING_OPTIONS).map_err(|e| e.into())
            }
            result => result.map_err(|e| e.into())
        }
    }
}
