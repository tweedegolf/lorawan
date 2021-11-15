use core::fmt::Debug;
use core::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Busy, Channel, Receive, Transmit};
use radio::blocking::BlockingError;

pub use crate::device::class_a::*;
use crate::device::error::DeviceError;
pub use crate::device::state::*;
use crate::lorawan::{DevNonce, JOIN_ACCEPT_DELAY1, JOIN_ACCEPT_DELAY2, JoinAccept, JoinRequest, MAX_PAYLOAD_SIZE};
use crate::radio::{LoRaChannel, LoRaInfo};

mod class_a;
pub mod error;
mod state;

/// Represents a generic LoRaWAN device. The state can be either [Credentials] for
/// devices that have not joined a network, or [DeviceState] for devices that have.
pub struct Device<R, S> {
    radio: R,
    state: S,
}

impl<R, E> Device<R, Credentials>
    where R: Transmit<Error=E> + Receive<Error=E, Info=LoRaInfo> + Channel<Channel=LoRaChannel, Error=E> + Busy<Error=E> + DelayUs<u32>,
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

        let _ = self.simple_transmit(join_request.payload(), &mut buf, JOIN_ACCEPT_DELAY1, JOIN_ACCEPT_DELAY2)?;

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
    where R: Transmit<Error=E> + Receive<Error=E, Info=LoRaInfo> + Channel<Channel=LoRaChannel, Error=E> + Busy<Error=E> + DelayUs<u32>,
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
    where R: Transmit<Error=E> + Receive<Error=E, Info=LoRaInfo> + Channel<Channel=LoRaChannel, Error=E> + Busy<Error=E> + DelayUs<u32>,
          E: Debug
{
    /// The time the radio will listen for a message on a channel. This must be long enough for the
    /// radio to receive a preamble, in which case it will continue listening for the message. It must
    /// not exceed one second, because the radio must switch to RX2 within that time if it does not
    /// receive a message on RX1.
    const TIMEOUT: Duration = Duration::from_millis(500);

    /// How often the radio will check whether a message has been received completely or not.
    const INTERVAL: Duration = Duration::from_millis(100);

    /// Basic LoRaWAN transmit. It transmits `tx`, then waits for a response on RX1, and if it does
    /// not receive anything, it waits for a response on RX2. The response is stored in `rx`. If no
    /// response is received, this method returns a timeout error.
    pub(in crate::device) fn simple_transmit(&mut self, tx: &[u8], rx: &mut [u8], delay_1: Duration, delay_2: Duration) -> Result<(usize, LoRaInfo), DeviceError<E>> {
        self.transmit(tx)?;

        self.radio.set_channel(&LoRaChannel::RX1)?;
        self.radio.delay_us(delay_1.as_micros() as u32);

        match self.receive(rx) {
            Err(BlockingError::Timeout) => {
                self.radio.set_channel(&LoRaChannel::RX2)?;
                self.radio.delay_us((delay_2 - delay_1 - Self::TIMEOUT).as_micros() as u32);

                self.receive(rx).map_err(|e| e.into())
            }
            result => result.map_err(|e| e.into())
        }
    }

    fn transmit(&mut self, data: &[u8]) -> Result<(), BlockingError<E>> {
        self.radio.start_transmit(data)?;

        let mut time = 0;
        loop {
            self.radio.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.radio.check_transmit()? {
                return Ok(());
            }

            time += Self::INTERVAL.as_micros();
            if time > Self::TIMEOUT.as_micros() {
                return Err(BlockingError::Timeout);
            }
        }
    }

    fn receive(&mut self, buf: &mut [u8]) -> Result<(usize, LoRaInfo), BlockingError<E>> {
        self.radio.start_receive()?;

        let mut time = 0;
        loop {
            self.radio.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.radio.check_receive(false)? {
                let (n, i) = self.radio.get_received(buf)?;
                return Ok((n, i));
            }

            time += Self::INTERVAL.as_micros();
            if time > Self::TIMEOUT.as_micros() && !self.radio.is_busy()? {
                return Err(BlockingError::Timeout);
            }
        }
    }
}
