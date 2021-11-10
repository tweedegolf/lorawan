use core::fmt::Debug;
use core::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Channel, State};
use radio::blocking::{BlockingError, BlockingOptions, BlockingReceive, BlockingTransmit};

pub use crate::device::class_a::*;
use crate::device::error::DeviceError;
pub use crate::device::state::*;
use crate::lorawan::{DevNonce, JoinAccept, JoinRequest, MAX_PAYLOAD_SIZE};
use crate::radio::{JOIN_ACCEPT_1_DELAY, JOIN_ACCEPT_2_DELAY, LoRaChannel, LoRaInfo, LoRaState};

mod class_a;
pub mod error;
mod state;

const INTERVAL: Duration = Duration::from_millis(100);
const TIMEOUT: Duration = Duration::from_millis(200);

const BLOCKING_OPTIONS: BlockingOptions = BlockingOptions {
    poll_interval: INTERVAL,
    timeout: TIMEOUT,
};

/// Represents a generic LoRaWAN device. The state can be either [Credentials] for
/// devices that have not joined a network, or [DeviceState] for devices that have.
pub struct Device<R, S> {
    radio: R,
    state: S,
}

impl<R, E> Device<R, Credentials>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaInfo, E> + State<State=LoRaState, Error=E> + Channel<Channel=LoRaChannel, Error=E> + DelayUs<u32>,
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
    pub fn join(mut self) -> Result<Device<R, DeviceState>, DeviceError<E>> {
        let dev_nonce = DevNonce::new(37);

        let join_request = JoinRequest::new(&self.state, &dev_nonce);
        let mut buf = [0; MAX_PAYLOAD_SIZE];

        let _ = self.simple_transmit(join_request.payload(), &mut buf, JOIN_ACCEPT_1_DELAY, JOIN_ACCEPT_2_DELAY)?;

        let join_accept = JoinAccept::from_data(&mut buf)?;
        let state = join_accept.extract_state(&self.state, &dev_nonce);

        let device = Device {
            radio: self.radio,
            state,
        };

        Ok(device)
    }
}

impl<R, E> Device<R, DeviceState>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaInfo, E> + State<State=LoRaState, Error=E> + Channel<Channel=LoRaChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Creates a joined device through Activation By Personalization. Consider using [new_otaa]
    /// instead, as it is more secure.
    pub fn new_abp(radio: R, session: Session) -> Self {
        let state = DeviceState::new(session, Settings::default());

        Device {
            radio,
            state,
        }
    }

    /// Configures this device to have class A behavior: listening for downlinks only after
    /// transmitting an uplink.
    pub fn into_class_a(self) -> ClassA<R> {
        self.into()
    }
}

impl<R, E, S> Device<R, S>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaInfo, E> + State<State=LoRaState, Error=E> + Channel<Channel=LoRaChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    pub(in crate::device) fn simple_transmit(&mut self, tx: &[u8], rx: &mut [u8], delay_1: Duration, delay_2: Duration) -> Result<(usize, LoRaInfo), DeviceError<E>> {
        self.radio.do_transmit(tx, BLOCKING_OPTIONS)?;

        self.radio.set_channel(&LoRaChannel::RX1)?;
        self.radio.delay_us(delay_1.as_micros() as u32);

        match self.radio.do_receive(rx, BLOCKING_OPTIONS) {
            Err(BlockingError::Timeout) => {
                self.radio.set_channel(&LoRaChannel::RX2)?;
                self.radio.delay_us((delay_2 - delay_1 - TIMEOUT).as_micros() as u32);

                self.radio.do_receive(rx, BLOCKING_OPTIONS).map_err(|e| e.into())
            }
            result => result.map_err(|e| e.into())
        }
    }
}
