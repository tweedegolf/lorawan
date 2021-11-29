use core::fmt::Debug;
use core::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Busy, Channel, RadioState, Receive, ReceiveInfo, State, Transmit};
use radio::blocking::BlockingError;
use radio::modulation::lora::LoRaChannel;

use crate::device::error::DeviceError;
pub use crate::radio::rate::*;
pub use crate::radio::region::*;

mod rate;
mod region;

pub type Hz = u32;

#[derive(Debug, Default)]
pub struct LoRaState {
    rx_delay: u8,
}

impl LoRaState {
    pub fn set_rx_delay(&mut self, rx_delay: u8) -> &mut Self {
        self.rx_delay = rx_delay;
        self
    }
}

impl RadioState for LoRaState {
    fn idle() -> Self {
        todo!()
    }

    fn sleep() -> Self {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct LoRaInfo {
    rssi: i16,
    snr: i8,
}

impl ReceiveInfo for LoRaInfo {
    fn rssi(&self) -> i16 {
        self.rssi
    }
}

/// Combines all the radio traits necessary for LoRa into one trait, and provides useful methods to
/// transmit messages.
pub trait LoRaRadio {
    /// The time the radio will listen for a message on a channel. This must be long enough for the
    /// radio to receive a preamble, in which case it will continue listening for the message. It
    /// must not exceed one second, because the radio must switch to RX2 within that time if it does
    /// not receive a message on RX1.
    const TIMEOUT: Duration = Duration::from_millis(500);

    /// How often the radio will check whether a message has been received completely or not.
    const INTERVAL: Duration = Duration::from_millis(100);

    type Error: Debug;

    /// Basic LoRaWAN transmit. It transmits `tx`, then waits for a response on RX1, and if it does
    /// not receive anything, it waits for a response on RX2. The response is stored in `rx`. If no
    /// response is received, this method returns a timeout error.
    fn lorawan_transmit<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        delay_1: Duration,
        delay_2: Duration,
        rate: &DataRate<R>,
    ) -> Result<(usize, LoRaInfo), DeviceError<Self::Error>>;

    /// Attempts to transmit a message.
    fn transmit(&mut self, data: &[u8]) -> Result<(), BlockingError<Self::Error>>;

    /// Attempts to receive a message. This returns within one second if no message is being
    /// received, giving enough time to switch to RX2 if necessary.
    fn receive(&mut self, buf: &mut [u8]) -> Result<(usize, LoRaInfo), BlockingError<Self::Error>>;
}

impl<T, C, E> LoRaRadio for T
    where T: Transmit<Error=E>,
          T: Receive<Error=E, Info=LoRaInfo>,
          T: State<State=LoRaState, Error=E>,
          T: Channel<Channel=C, Error=E>,
          T: Busy<Error=E>,
          T: DelayUs<u32>,
          C: From<LoRaChannel>,
          E: Debug
{
    type Error = E;

    fn lorawan_transmit<R: Region>(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
        delay_1: Duration,
        delay_2: Duration,
        rate: &DataRate<R>,
    ) -> Result<(usize, LoRaInfo), DeviceError<Self::Error>> {
        self.transmit(tx)?;

        self.set_channel(&rate.rx1().into())?;
        self.delay_us(delay_1.as_micros() as u32);

        match self.receive(rx) {
            Err(BlockingError::Timeout) => {
                self.set_channel(&rate.rx2().into())?;
                self.delay_us((delay_2 - delay_1 - Self::TIMEOUT).as_micros() as u32);

                self.receive(rx).map_err(|e| e.into())
            }
            result => result.map_err(|e| e.into())
        }
    }

    fn transmit(&mut self, data: &[u8]) -> Result<(), BlockingError<Self::Error>> {
        self.start_transmit(data)?;

        let mut time = 0;
        loop {
            self.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.check_transmit()? {
                return Ok(());
            }

            time += Self::INTERVAL.as_micros();
            if time > Self::TIMEOUT.as_micros() {
                return Err(BlockingError::Timeout);
            }
        }
    }

    fn receive(&mut self, buf: &mut [u8]) -> Result<(usize, LoRaInfo), BlockingError<Self::Error>> {
        self.start_receive()?;

        let mut time = 0;
        loop {
            self.delay_us(Self::INTERVAL.as_micros() as u32);

            if self.check_receive(false)? {
                let (n, i) = self.get_received(buf)?;
                return Ok((n, i));
            }

            time += Self::INTERVAL.as_micros();
            if time > Self::TIMEOUT.as_micros() && !self.is_busy()? {
                return Err(BlockingError::Timeout);
            }
        }
    }
}
